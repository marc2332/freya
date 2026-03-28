use anyhow::Context as AnyhowContext;
use freya_engine::prelude::{
    AlphaType, ColorType, Data, ISize, ImageInfo, SkImage, raster_from_data,
};

pub(crate) fn format_time(secs: f64) -> String {
    let total = secs as u64;
    let m = total / 60;
    let s = total % 60;
    format!("{m}:{s:02}")
}

pub(crate) fn make_sk_image(rgba: &[u8], width: u32, height: u32) -> anyhow::Result<SkImage> {
    let row_bytes = (width * 4) as usize;
    let data = Data::new_copy(rgba);
    raster_from_data(
        &ImageInfo::new(
            ISize::new(width as i32, height as i32),
            ColorType::RGBA8888,
            AlphaType::Unpremul,
            None,
        ),
        data,
        row_bytes,
    )
    .context("Failed to create SkImage from video frame")
}
