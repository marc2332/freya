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
        .center()
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
    assert_eq!(*state.peek(), 0);

    // Simulate a click by sending mouse down/up events at a point inside the window
    test.click_cursor((15., 15.));
    assert_eq!(*state.peek(), 1);

    // Render the current ui state to a file
    test.render_to_file("./demo-1.png");
}
