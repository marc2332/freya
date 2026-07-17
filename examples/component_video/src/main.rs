#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::{
    prelude::*,
    video::*,
};

fn main() {
    ensure_ffmpeg().expect("failed to prepare ffmpeg");
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(1200., 720.)))
}

fn app() -> impl IntoElement {
    use_init_theme(dark_theme);
    let mut player = use_video(|| {
        std::env::args()
            .nth(1)
            .expect("pass a video path as the first argument")
    });

    let state = player.state();
    let frame = player.frame();
    let progress = player.progress();
    let position = player.position();
    let duration = player.duration();
    let volume = player.volume();

    let toggle_icon = match state {
        PlaybackState::Playing => freya::icons::lucide::pause(),
        PlaybackState::Errored => freya::icons::lucide::bug(),
        PlaybackState::Loading => freya::icons::lucide::loader(),
        _ => freya::icons::lucide::play(),
    };

    let time_label = format!(
        "{} / {}",
        format_duration(position),
        duration
            .map(format_duration)
            .unwrap_or_else(|| "--:--".into()),
    );

    rect()
        .theme_background()
        .theme_color()
        .expanded()
        .center()
        .spacing(8.)
        .content(Content::Flex)
        .child(
            rect()
                .width(Size::fill())
                .height(Size::flex(1.))
                .center()
                .child(match (frame, state) {
                    (Some(frame), _) => image(frame)
                        .expanded()
                        .aspect_ratio(AspectRatio::Min)
                        .image_cover(ImageCover::Center)
                        .overflow(Overflow::Clip)
                        .into(),
                    (_, PlaybackState::Errored) => "Failed to load video.".into_element(),
                    _ => CircularLoader::new().into(),
                }),
        )
        .child(
            rect()
                .horizontal()
                .content(Content::Flex)
                .cross_align(Alignment::Center)
                .spacing(12.)
                .padding(12.)
                .width(Size::percent(70.))
                .child(
                    Button::new()
                        .rounded_full()
                        .expanded()
                        .flat()
                        .on_press(move |_| player.toggle())
                        .child(
                            SvgViewer::new(toggle_icon)
                                .color((200, 200, 200))
                                .width(Size::px(16.))
                                .height(Size::px(16.)),
                        ),
                )
                .child(
                    Slider::new(move |per: f64| {
                        if let Some(duration) = player.duration() {
                            let target =
                                Duration::from_secs_f64(duration.as_secs_f64() * per / 100.0);
                            player.seek(target, Duration::from_millis(150));
                        }
                    })
                    .value(progress)
                    .size(Size::flex(1.)),
                )
                .child(label().text(time_label).max_lines(1))
                .child(
                    SvgViewer::new(freya::icons::lucide::volume_2())
                        .color((200, 200, 200))
                        .width(Size::px(16.))
                        .height(Size::px(16.)),
                )
                .child(
                    Slider::new(move |per: f64| player.set_volume((per / 100.0) as f32))
                        .value(volume as f64 * 100.0)
                        .size(Size::px(90.)),
                ),
        )
}

fn format_duration(d: Duration) -> String {
    let total = d.as_secs();
    let minutes = total / 60;
    let seconds = total % 60;
    format!("{minutes}:{seconds:02}")
}
