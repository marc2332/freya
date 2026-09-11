#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_html::prelude::*;

fn main() {
    tracing_subscriber::fmt::init();

    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(1024., 768.)))
}

fn app() -> impl IntoElement {
    use_init_theme(dark_theme);

    let mut handle = use_html_handle(|| HtmlSource::url("https://news.ycombinator.com/"));
    let mut input = use_state(|| String::from("https://news.ycombinator.com/"));

    // Keep the input in sync when the page navigates.
    use_side_effect_with_deps(&handle.current_url(), move |url| {
        if let Some(url) = url {
            input.set(url.clone());
        }
    });

    rect()
        .expanded()
        .background((30, 30, 40))
        .child(
            rect()
                .horizontal()
                .width(Size::fill())
                .cross_align(Alignment::Center)
                .spacing(4.)
                .padding(4.)
                .child(
                    Button::new()
                        .child("←")
                        .enabled(handle.can_go_back())
                        .on_press(move |_| handle.back()),
                )
                .child(
                    Button::new()
                        .child("→")
                        .enabled(handle.can_go_forward())
                        .on_press(move |_| handle.forward()),
                )
                .child(
                    Input::new(input)
                        .flat()
                        .width(Size::fill())
                        .placeholder("Enter a URL")
                        .on_submit(move |value: String| handle.navigate(value)),
                ),
        )
        .child(HtmlViewer::new(handle))
}
