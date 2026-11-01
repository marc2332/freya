#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut times_shown = use_state(|| 0);

    rect()
        .expanded()
        .child(label().text(match *times_shown.read() {
            0 => "Scroll down to reveal the blue rect".to_string(),
            times => format!("The blue rect has become visible {times} times"),
        }))
        .child(
            ScrollView::new()
                .expanded()
                .child(rect().height(Size::px(1000.)))
                .child(
                    rect()
                        .on_visible(move |_| *times_shown.write() += 1)
                        .width(Size::px(200.))
                        .height(Size::px(200.))
                        .background((0, 119, 182)),
                ),
        )
}
