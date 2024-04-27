use std::time::Duration;

use dioxus_core::use_hook;
use freya::events::pointer::MouseButton;
use freya::prelude::*;
use freya_engine::prelude::Color;
use freya_node_state::{Fill, Parse};
use freya_testing::prelude::*;
use tokio::time::sleep;

#[tokio::test]
pub async fn track_progress() {
    fn use_animation_app() -> Element {
        let animation = use_animation(|ctx| ctx.with(AnimNum::new(0., 100.).time(50)));

        let progress = animation.get().read().as_f32();

        use_hook(|| {
            animation.start();
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

    assert_eq!(utils.root().get(0).area().unwrap().width(), 0.0);

    // State somewhere in the middle
    sleep(Duration::from_millis(15)).await;
    utils.wait_for_update().await;

    let width = utils.root().get(0).area().unwrap().width();
    assert!(width > 0.0);

    // Enable event loop ticker
    utils.config().event_loop_ticker = true;

    // Already finished
    sleep(Duration::from_millis(50)).await;

    // State in the end
    utils.wait_for_update().await;

    let width = utils.root().get(0).area().unwrap().width();
    assert_eq!(width, 100.0);
}

#[tokio::test]
pub async fn reverse_progress() {
    fn use_animation_app() -> Element {
        let animation = use_animation(|ctx| ctx.with(AnimNum::new(10., 100.).time(50)));

        let progress = animation.get().read().as_f32();

        use_hook(|| {
            animation.start();
        });

        rsx!(rect {
            background: "white",
            height: "100%",
            onclick: move |_| {
                animation.reverse();
            },
            width: "{progress}",
        })
    }

    let mut utils = launch_test(use_animation_app);

    // Disable event loop ticker
    utils.config().event_loop_ticker = false;

    // Initial state
    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).area().unwrap().width(), 10.0);

    // State somewhere in the middle
    sleep(Duration::from_millis(32)).await;
    utils.wait_for_update().await;

    let width = utils.root().get(0).area().unwrap().width();
    assert!(width > 10.0);

    // Trigger the click event to restart the animation
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (5.0, 5.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;

    // Enable event loop ticker
    utils.config().event_loop_ticker = true;

    // Already finished
    sleep(Duration::from_millis(50)).await;

    // State has been restarted
    utils.wait_for_update().await;

    let width = utils.root().get(0).area().unwrap().width();
    assert_eq!(width, 10.0);
}

#[tokio::test]
pub async fn animate_color() {
    fn use_animation_app() -> Element {
        let animation =
            use_animation(|ctx| ctx.with(AnimColor::new("red", "rgb(50, 100, 200)").time(50)));

        let progress = animation.get().read().as_string();

        use_hook(|| {
            animation.start();
        });

        rsx!(rect {
            background: "{progress}",
        })
    }

    let mut utils = launch_test(use_animation_app);

    // Disable event loop ticker
    utils.config().event_loop_ticker = false;

    utils.wait_for_update().await;

    // Initial color
    assert_eq!(
        utils.root().get(0).style().background,
        Fill::Color(Color::RED)
    );

    // Color somewhere in the middle
    sleep(Duration::from_millis(15)).await;
    utils.wait_for_update().await;

    assert_ne!(
        utils.root().get(0).style().background,
        Fill::Color(Color::RED)
    );

    // Enable event loop ticker
    utils.config().event_loop_ticker = true;

    // Already finished
    sleep(Duration::from_millis(50)).await;

    // Color in the end
    utils.wait_for_update().await;

    assert_eq!(
        utils.root().get(0).style().background,
        Fill::Color(Color::parse("rgb(50, 100, 200)").unwrap())
    );
}

#[tokio::test]
pub async fn auto_start() {
    fn use_animation_app() -> Element {
        let animation = use_animation(|ctx| {
            ctx.auto_start(true);
            ctx.with(AnimNum::new(10., 100.).time(50))
        });

        let progress = animation.get().read().as_f32();

        rsx!(rect {
            background: "white",
            height: "100%",
            width: "{progress}",
        })
    }

    let mut utils = launch_test(use_animation_app);

    // Disable event loop ticker
    utils.config().event_loop_ticker = false;

    // Initial state
    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).area().unwrap().width(), 10.0);

    // State somewhere in the middle
    sleep(Duration::from_millis(32)).await;
    utils.wait_for_update().await;

    let width = utils.root().get(0).area().unwrap().width();
    assert!(width > 10.0);

    // Already finished
    sleep(Duration::from_millis(32)).await;

    // State has been restarted
    utils.wait_for_update().await;

    let width = utils.root().get(0).area().unwrap().width();
    assert_eq!(width, 100.0);
}
