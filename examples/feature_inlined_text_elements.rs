#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(650., 400.)))
}

fn app() -> impl IntoElement {
    let mut count = use_state(|| 0);

    let logo = ("rust-logo", include_bytes!("./rust_logo.png"));

    rect().expanded().padding(40.).child(
        paragraph()
            .expanded()
            .font_size(16.)
            .line_height(1.5)
            .text_overflow(TextOverflow::Ellipsis)
            .max_lines(4)
            .span("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed ")
            .child(
                ImageViewer::new(logo)
                    .width(Size::px(32.))
                    .height(Size::px(32.)),
            )
            .span(" do eiusmod tempor incididunt ut labore et dolore magna aliqua ")
            .child(
                Link::new("https://google.com").child("https://google.com"),
            )
            .span(" Ut enim ad minim veniam ")
            .child(
                Button::new()
                    .rounded_full()
                    .on_press(move |_| *count.write() += 1)
                    .child(count.read().to_string()),
            )
            .span("  quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."),
    )
}
