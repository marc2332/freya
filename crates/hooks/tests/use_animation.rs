use std::time::Duration;

use dioxus_core::use_hook;
use freya::prelude::*;
use freya_testing::{events::pointer::MouseButton, launch_test, FreyaEvent};
use tokio::time::sleep;

#[tokio::test]
pub async fn track_progress() {
    fn use_animation_app() -> Element {
        let animation = use_animation(|ctx| ctx.with(AnimNum::new(0., 100.).time(50)));

        let progress = animation.read().get().read().as_f32();

        use_hook(|| {
            animation.read().start();
        });

        rsx!(rect {
            width: "{progress}",
        })
    }

    let mut utils = launch_test(use_animation_app);

    // Disable event loop ticker
    utils.config().event_loop_ticker = false;

    // Initial state
    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).layout().unwrap().width(), 0.0);

    // State somewhere in the middle
    sleep(Duration::from_millis(15)).await;
    utils.wait_for_update().await;

    let width = utils.root().get(0).layout().unwrap().width();
    assert!(width > 0.0);

    // Enable event loop ticker
    utils.config().event_loop_ticker = true;

    // Already finished
    sleep(Duration::from_millis(50)).await;

    // State in the end
    utils.wait_for_update().await;

    let width = utils.root().get(0).layout().unwrap().width();
    assert_eq!(width, 100.0);
}

#[tokio::test]
pub async fn reverse_progress() {
    fn use_animation_app() -> Element {
        let animation = use_animation(|ctx| ctx.with(AnimNum::new(10., 100.).time(50)));

        let progress = animation.read().get().read().as_f32();

        use_hook(|| {
            animation.read().start();
        });

        rsx!(rect {
            background: "white",
            height: "100%",
            onclick: move |_| {
                animation.read().reverse();
            },
            width: "{progress}",
        })
    }

    let mut utils = launch_test(use_animation_app);

    // Disable event loop ticker
    utils.config().event_loop_ticker = false;

    // Initial state
    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).layout().unwrap().width(), 10.0);

    // State somewhere in the middle
    sleep(Duration::from_millis(32)).await;
    utils.wait_for_update().await;

    let width = utils.root().get(0).layout().unwrap().width();
    assert!(width > 10.0);

    // Trigger the click event to restart the animation
    utils.push_event(FreyaEvent::Mouse {
        name: "click".to_string(),
        cursor: (5.0, 5.0).into(),
        button: Some(MouseButton::Left),
    });

    // Enable event loop ticker
    utils.config().event_loop_ticker = true;

    // Already finished
    sleep(Duration::from_millis(50)).await;

    // State has been restarted
    utils.wait_for_update().await;
    utils.wait_for_update().await;

    let width = utils.root().get(0).layout().unwrap().width();
    assert_eq!(width, 10.0);
}
