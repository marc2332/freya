#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect().expanded().center().child(
        label()
            .text("Gradient text")
            .font_size(64.)
            .font_weight(FontWeight::EXTRA_BOLD)
            .color_linear_gradient(
                LinearGradient::new()
                    .angle(90.)
                    .stop(((255, 100, 50), 0.))
                    .stop(((255, 0, 128), 50.))
                    .stop(((100, 0, 255), 100.)),
            ),
    )
}
