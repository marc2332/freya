#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_testing::prelude::*;

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        rect {
            height: "fill",
            width: "fill",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            onclick: move |_| count += 1,
            label {
                font_size: "100",
                font_weight: "bold",
                "{count}"
            }
        }
    )
}

#[tokio::main]
async fn main() {
    let mut utils = launch_test(app);

    // Initial render
    utils.wait_for_update().await;
    utils.save_snapshot("./snapshot_before.png");

    // Emit click event
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (100., 100.).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;

    // Render after click
    utils.save_snapshot("./snapshot_after.png");
}
