#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    borderless::BorderlessPlugin,
    prelude::*,
};

fn main() {
    launch(
        LaunchConfig::new()
            .with_plugin(BorderlessPlugin::new().with_corner_radius(12.))
            .with_window(
                WindowConfig::new(app)
                    .with_decorations(false)
                    .with_transparency(true)
                    .with_background(Color::TRANSPARENT),
            ),
    )
}

fn app() -> impl IntoElement {
    let close = move |_| {
        let platform = Platform::get();
        Platform::get().with_window(None, move |window| {
            platform.close_window(window.id());
        });
    };

    rect()
        .expanded()
        .vertical()
        .background((235, 235, 235))
        .child(
            rect()
                .horizontal()
                .background((215, 215, 215))
                .content(Content::Flex)
                .height(Size::px(32.))
                .cross_align(Alignment::Center)
                .padding((0., 0., 0., 8.))
                .child("Borderless Window")
                .child(
                    rect()
                        .window_drag()
                        .width(Size::flex(1.))
                        .height(Size::fill()),
                )
                .child(TitlebarButton::new(TitlebarAction::Close).on_press(close)),
        )
        .child(
            rect()
                .expanded()
                .center()
                .child("Drag the edges to resize, the corners are rounded by the plugin."),
        )
}
