//! WebView example demonstrating how to embed web content in a Freya application.
//!
//! Run with:
//! ```sh
//! cargo run --example feature_webview
//! ```

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_webview::prelude::*;

fn main() {
    tracing_subscriber::fmt::init();

    launch(
        LaunchConfig::new()
            .with_plugin(WebViewPlugin::new())
            .with_window(WindowConfig::new(app).with_size(1024., 768.)),
    )
}

fn app() -> impl IntoElement {
    rect()
        .width(Size::fill())
        .height(Size::fill())
        .background((35, 35, 35))
        .vertical()
        .child(
            // Header bar
            rect()
                .width(Size::fill())
                .height(Size::px(50.))
                .padding(Gaps::new(12., 16., 12., 16.))
                .background((50, 50, 50))
                .horizontal()
                .cross_align(Alignment::Center)
                .child(
                    label()
                        .text("Freya WebView Example")
                        .color(Color::WHITE)
                        .font_size(18.),
                ),
        )
        .child(
            rect()
                .expanded()
                .horizontal()
                .child(
                    webview("https://freyaui.dev")
                        .width(Size::percent(50.))
                        .height(Size::fill())
                )
                .child(
                    webview("https://github.com")
                        .width(Size::percent(50.))
                        .height(Size::fill())
                ),
        )
}
