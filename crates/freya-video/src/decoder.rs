use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
};

use anyhow::Context as AnyhowContext;
use ffmpeg::{
    ChannelLayout,
    Dictionary,
    format::{
        Pixel,
        sample::{self, Sample},
    },
    frame::Audio as AudioFrame,
    media::Type as MediaType,
    software::{
        resampling::context::Context as ResamplerCtx,
        scaling::{context::Context as ScalerContext, flag::Flags as ScalerFlags},
    },
    util::log,
};
use ffmpeg_next as ffmpeg;

use crate::audio::{AUDIO_SAMPLE_RATE, AudioFrameData};

/// Returns FFmpeg input options appropriate for the given input string.
/// For network URLs (HLS, HTTP, RTMP...) this allows any segment extension so that
/// streams whose segments use non-standard extensions (e.g. `.jpg`) are accepted.
/// `extra_headers` are appended to the default browser-like headers.
fn input_options(input: &str, extra_headers: &[(String, String)]) -> Dictionary<'static> {
    let mut opts = Dictionary::new();
    let is_url = input.starts_with("http://")
        || input.starts_with("https://")
        || input.starts_with("rtmp://")
        || input.starts_with("rtsp://");
    if is_url {
        // Allow HLS segments with any file extension (e.g. .jpg used by some CDNs).
        opts.set("allowed_extensions", "ALL");
        // Allow HTTPS, TLS and crypto sub-protocols used by HLS over HTTPS.
        opts.set("protocol_whitelist", "file,http,https,tcp,tls,crypto");
        // Automatically reconnect if a segment download is interrupted.
        opts.set("reconnect", "1");
        opts.set("reconnect_on_network_error", "1");
        opts.set("reconnect_streamed", "1");
        // Build HTTP headers: start with browser defaults, append caller-supplied extras.
        // If the caller provides a `user-agent` header, skip the default one.
        let has_user_agent = extra_headers
            .iter()
            .any(|(k, _)| k.eq_ignore_ascii_case("user-agent"));
        let mut header_str = String::from("Accept: */*\r\nAccept-Language: en-US,en;q=0.9\r\n");
        if !has_user_agent {
            header_str.push_str(
                "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
                 AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36\r\n",
            );
        }
        for (name, value) in extra_headers {
            header_str.push_str(name);
            header_str.push_str(": ");
            header_str.push_str(value);
            header_str.push_str("\r\n");
        }
        // `av_dict_set` copies the value, so the local String can be dropped after this call.
        opts.set("headers", &header_str);
    }
    opts
}

/// Raw video frame data sent from the decoder thread to the async receive task.
pub(crate) struct VideoFrameData {
    pub rgba: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub pts_secs: f64,
    pub total_secs: f64,
}

/// Decodes only the video stream. Runs in its own OS thread.
/// `input` is either a local file path or a network URL (HTTP, HLS, RTMP, etc.).
pub(crate) fn decode_video_thread(
    input: String,
    extra_headers: Vec<(String, String)>,
    start_secs: f64,
    tx_video: mpsc::SyncSender<VideoFrameData>,
    abort: Arc<AtomicBool>,
) -> anyhow::Result<()> {
    ffmpeg::init()?;
    log::set_level(log::Level::Warning);

    let mut ictx = ffmpeg::format::input_with_dictionary(
        &Path::new(&input),
        input_options(&input, &extra_headers),
    )?;

    if start_secs > 0.0 {
        let target_pts = (start_secs * ffmpeg::ffi::AV_TIME_BASE as f64) as i64;
        ictx.seek(target_pts, target_pts..).ok();
    }

    let total_secs = if ictx.duration() > 0 {
        ictx.duration() as f64 / ffmpeg::ffi::AV_TIME_BASE as f64
    } else {
        0.0
    };

    let input = ictx
        .streams()
        .best(MediaType::Video)
        .context("No video stream found")?;
    let video_stream_index = input.index();
    let time_base = input.time_base();
    let codec_ctx = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
    let mut decoder = codec_ctx.decoder().video()?;
    let width = decoder.width();
    let height = decoder.height();
    let mut scaler = ScalerContext::get(
        decoder.format(),
        width,
        height,
        Pixel::RGBA,
        width,
        height,
        ScalerFlags::BILINEAR,
    )?;

    let mut flush_video = |decoder: &mut ffmpeg::decoder::Video| -> anyhow::Result<bool> {
        let mut decoded = ffmpeg::util::frame::video::Video::empty();
        while decoder.receive_frame(&mut decoded).is_ok() {
            if abort.load(Ordering::Relaxed) {
                return Ok(false);
            }
            let mut rgba_frame = ffmpeg::util::frame::video::Video::empty();
            scaler.run(&decoded, &mut rgba_frame)?;

            let pts_secs = decoded
                .pts()
                .map(|pts| pts as f64 * f64::from(time_base))
                .unwrap_or(0.0);

            let stride = rgba_frame.stride(0);
            let data = rgba_frame.data(0);
            let row_bytes = (width * 4) as usize;

            let rgba = if stride == row_bytes {
                data.to_vec()
            } else {
                let mut buf = Vec::with_capacity(row_bytes * height as usize);
                for row in 0..height as usize {
                    let start = row * stride;
                    buf.extend_from_slice(&data[start..start + row_bytes]);
                }
                buf
            };

            if tx_video
                .send(VideoFrameData {
                    rgba,
                    width,
                    height,
                    pts_secs,
                    total_secs,
                })
                .is_err()
            {
                return Ok(false);
            }
        }
        Ok(true)
    };

    for (stream, packet) in ictx.packets() {
        if abort.load(Ordering::Relaxed) {
            return Ok(());
        }
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            if !flush_video(&mut decoder)? {
                return Ok(());
            }
        }
    }

    decoder.send_eof().ok();
    flush_video(&mut decoder)?;

    Ok(())
}

/// Decodes only the audio stream and resamples to stereo f32 at [`AUDIO_SAMPLE_RATE`].
/// Runs in its own OS thread, independently of video decoding.
/// `input` is either a local file path or a network URL (HTTP, HLS, RTMP, etc.).
pub(crate) fn decode_audio_thread(
    input: String,
    extra_headers: Vec<(String, String)>,
    start_secs: f64,
    tx_audio: mpsc::SyncSender<AudioFrameData>,
    abort: Arc<AtomicBool>,
) -> anyhow::Result<()> {
    ffmpeg::init()?;
    log::set_level(log::Level::Warning);

    let mut ictx = ffmpeg::format::input_with_dictionary(
        &Path::new(&input),
        input_options(&input, &extra_headers),
    )?;

    if start_secs > 0.0 {
        let target_pts = (start_secs * ffmpeg::ffi::AV_TIME_BASE as f64) as i64;
        ictx.seek(target_pts, target_pts..).ok();
    }

    let audio_stream = match ictx.streams().best(MediaType::Audio) {
        Some(s) => s,
        None => return Ok(()),
    };
    let audio_idx = audio_stream.index();
    let audio_time_base = audio_stream.time_base();
    let ctx = ffmpeg::codec::context::Context::from_parameters(audio_stream.parameters())?;
    let mut dec = ctx.decoder().audio()?;
    let mut resampler = ResamplerCtx::get(
        dec.format(),
        dec.channel_layout(),
        dec.rate(),
        Sample::F32(sample::Type::Planar),
        ChannelLayout::STEREO,
        AUDIO_SAMPLE_RATE,
    )?;

    for (stream, packet) in ictx.packets() {
        if abort.load(Ordering::Relaxed) {
            return Ok(());
        }
        if stream.index() != audio_idx {
            continue;
        }
        dec.send_packet(&packet).ok();
        let mut frame = AudioFrame::empty();
        while dec.receive_frame(&mut frame).is_ok() {
            let pts_secs = frame
                .pts()
                .map(|p| p as f64 * f64::from(audio_time_base))
                .unwrap_or(0.0);
            let mut resampled = AudioFrame::empty();
            if resampler.run(&frame, &mut resampled).is_ok() {
                let n = resampled.samples();
                if n > 0 {
                    let left = resampled.plane::<f32>(0);
                    let left_n = n.min(left.len());
                    let mut samples = Vec::with_capacity(left_n * 2);
                    if resampled.planes() >= 2 {
                        let right = resampled.plane::<f32>(1);
                        let right_n = left_n.min(right.len());
                        for i in 0..right_n {
                            samples.push(left[i].clamp(-1.0, 1.0));
                            samples.push(right[i].clamp(-1.0, 1.0));
                        }
                    } else {
                        for i in 0..left_n {
                            let s = left[i].clamp(-1.0, 1.0);
                            samples.push(s);
                            samples.push(s);
                        }
                    }
                    if tx_audio.send(AudioFrameData { samples, pts_secs }).is_err() {
                        return Ok(());
                    }
                }
            }
        }
    }

    Ok(())
}
