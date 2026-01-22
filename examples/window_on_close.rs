#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    prelude::*,
    winit::window::WindowId,
};

fn main() {
    launch(
        LaunchConfig::new().with_window(WindowConfig::new(app).with_on_close(
            |mut ctx, window_id| {
                ctx.launch_window(
                    WindowConfig::new(move || close_dialog(window_id)).with_size(300., 150.),
                );
                CloseDecision::KeepOpen
            },
        )),
    )
}

fn close_dialog(window_id: WindowId) -> impl IntoElement {
    rect()
        .center()
        .expanded()
        .spacing(4.)
        .child("Are you sure?")
        .child(
            rect()
                .horizontal()
                .spacing(4.)
                .child(Button::new().child("No, keep open").on_press(move |_| {
                    let platform = Platform::get();
                    Platform::get().with_window(None, move |window| {
                        platform.close_window(window.id());
                    });
                }))
                .child(Button::new().child("Yes, Close").on_press(move |_| {
                    let platform = Platform::get();
                    platform.close_window(window_id);
                    Platform::get().with_window(None, move |window| {
                        platform.close_window(window.id());
                    });
                })),
        )
}

fn app() -> impl IntoElement {
    rect().center().expanded().child("Close this window!")
}
