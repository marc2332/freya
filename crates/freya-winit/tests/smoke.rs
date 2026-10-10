use std::{
    fs,
    io,
    path::PathBuf,
    time::{
        Duration,
        Instant,
    },
};

use freya::prelude::*;
use freya_engine::prelude::EncodedImageFormat;

fn main() -> io::Result<()> {
    let started = Instant::now();

    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_title("Freya nightly smoke")
                .with_size(720., 480.)
                .with_renderer(RendererPreference::Software),
        ),
    );

    if started.elapsed() < Duration::from_secs(10) {
        return Err(io::Error::other("the window closed before ten seconds"));
    }

    let screenshot = std::env::var_os("FREYA_SMOKE_SCREENSHOT")
        .ok_or_else(|| io::Error::other("FREYA_SMOKE_SCREENSHOT is not set"))?;
    let png = fs::read(screenshot)?;
    let image = ::image::load_from_memory_with_format(&png, ::image::ImageFormat::Png)
        .map_err(io::Error::other)?
        .to_rgb8();
    if !image.pixels().any(|pixel| {
        let [red, green, blue] = pixel.0;
        green > red.saturating_add(40) && green > blue.saturating_add(20)
    }) {
        return Err(io::Error::other(
            "screenshot does not contain the green rectangle",
        ));
    }

    Ok(())
}

fn app() -> impl IntoElement {
    let platform = Platform::get();
    let window_id = Platform::window_id();
    let screenshot = PathBuf::from(
        std::env::var_os("FREYA_SMOKE_SCREENSHOT").expect("FREYA_SMOKE_SCREENSHOT is not set"),
    );

    use_hook(move || {
        spawn(async move {
            timer(Duration::from_secs(10)).await;

            let screenshot = platform
                .post_render_callback(move |surface| {
                    let png = surface
                        .image_snapshot()
                        .encode(None, EncodedImageFormat::PNG, None)
                        .ok_or_else(|| io::Error::other("could not encode screenshot"))?;
                    if let Some(parent) = screenshot
                        .parent()
                        .filter(|parent| !parent.as_os_str().is_empty())
                    {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(&screenshot, png.as_bytes())
                })
                .await;
            let screenshot = match screenshot {
                Ok(result) => result,
                Err(error) => Err(io::Error::other(error)),
            };
            if let Err(error) = screenshot {
                eprintln!("Screenshot failed: {error}");
                std::process::exit(1);
            }

            platform.close_window(window_id);
        });
    });

    rect()
        .expanded()
        .padding(28.)
        .spacing(24.)
        .background((28, 36, 53))
        .color((250, 250, 250))
        .child("Freya nightly window smoke")
        .child(
            paragraph()
                .font_size(32.)
                .span("Text, emoji, and color: 🦀 🔍 🌈"),
        )
        .child(
            ImageViewer::new((
                "rust-logo",
                include_bytes!("../../../examples/rust_logo.png"),
            ))
            .width(Size::px(160.))
            .height(Size::px(160.)),
        )
        .child(
            rect()
                .width(Size::px(240.))
                .height(Size::px(32.))
                .background((94, 193, 130)),
        )
}
