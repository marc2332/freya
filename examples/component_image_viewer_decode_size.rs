#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

const SOURCE: &str = "https://images.pexels.com/photos/842711/pexels-photo-842711.jpeg";

const PRESETS: [(&str, DecodeMode); 4] = [
    ("Source (default)", DecodeMode::Source),
    ("FromLayout", DecodeMode::FromLayout),
    ("Custom 64×64", DecodeMode::Custom(Size2D::new(64., 64.))),
    (
        "Custom 256×256",
        DecodeMode::Custom(Size2D::new(256., 256.)),
    ),
];

fn app() -> impl IntoElement {
    let mut preset = use_state(|| 0);
    let (label, mode) = PRESETS[preset()];

    rect()
        .expanded()
        .center()
        .spacing(16.)
        .child(format!("Decode mode: {label}"))
        .child(
            ImageViewer::new(SOURCE)
                .width(Size::px(600.))
                .height(Size::px(400.))
                .decode_mode(mode),
        )
        .child(
            rect()
                .horizontal()
                .spacing(12.)
                .children(PRESETS.iter().enumerate().map(|(i, (label, _))| {
                    Button::new()
                        .on_press(move |_| *preset.write() = i)
                        .child(*label)
                        .into()
                })),
        )
}
