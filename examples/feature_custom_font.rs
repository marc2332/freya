use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new()
            .with_default_font("Samuel Morse")
            .with_font(
                "Samuel Morse",
                Bytes::from_static(include_bytes!("./SamuelMorse.otf")),
            )
            .with_window(WindowConfig::new(app)),
    )
}

fn app() -> Element {
    rect()
        .background((0, 0, 0))
        .color((255, 255, 255))
        .expanded()
        .center()
        .font_size(48.)
        .child("Hello, World!")
        .into()
}
