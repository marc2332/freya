#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let count = use_state(|| 0);
    let mut windows = use_state(Vec::new);

    let on_open = move |_| {
        spawn(async move {
            let window_id = Platform::get()
                .launch_window(WindowConfig::new(move || sub_app(count)))
                .await;
            windows.write().push(window_id);
        });
    };

    let on_close_children = move |_| {
        spawn(async move {
            for window_id in windows.write().drain(..) {
                Platform::get().close_window(window_id);
            }
        });
    };

    rect()
        .expanded()
        .center()
        .child(Button::new().on_press(on_open).child("Open"))
        .child(
            Button::new()
                .on_press(on_close_children)
                .child("Close children"),
        )
}

fn sub_app(mut count: State<i32>) -> impl IntoElement {
    let on_press = move |_| {
        *count.write() += 1;
    };

    rect()
        .expanded()
        .center()
        .spacing(6.)
        .child(format!("Value is {}", count.read()))
        .child(Button::new().on_press(on_press).child("Increase"))
}
