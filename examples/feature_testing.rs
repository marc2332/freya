#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_testing::TestingRunner;

fn app() -> impl IntoElement {
    let mut state = use_consume::<State<i32>>();

    rect()
        .expanded()
        .background((240, 240, 240))
        .on_mouse_up(move |_| *state.write() += 1)
        .child(format!("Clicked: {}", state.read()))
}

fn main() {
    // Create a headless testing runner and a shared state
    let (mut test, state) = TestingRunner::new(
        app,
        (300., 300.).into(),
        |runner| runner.provide_root_context(|| State::create(0)),
        1.,
    );

    test.sync_and_update();
    println!("Initial: {}", *state.peek());

    // Simulate a click by sending mouse down/up events at a point inside the window
    // Simulate a click with the testing helper
    test.click_cursor((15., 15.));

    println!("After click: {}", *state.peek());
}
