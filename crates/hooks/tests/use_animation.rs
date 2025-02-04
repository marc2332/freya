use std::time::Duration;

use dioxus_core::use_hook;
use freya::prelude::*;
use freya_core::{
    Fill,
    Parse,
};
use freya_engine::prelude::Color;
use freya_testing::prelude::*;
use tokio::time::sleep;

#[tokio::test]
pub async fn track_progress() {
    fn use_animation_app() -> Element {
        let animation = use_animation(|_conf| AnimNum::new(0., 100.).time(50));

        let progress = animation.get().read().read();

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
        let animation = use_animation(|_conf| AnimNum::new(10., 100.).time(50));

        let progress = animation.get().read().read();

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
    utils.click_cursor((5., 5.)).await;

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
        let animation = use_animation(|_conf| AnimColor::new("red", "rgb(50, 100, 200)").time(50));

        let progress = animation.get().read().read();

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
        let animation = use_animation(|conf| {
            conf.auto_start(true);
            AnimNum::new(10., 100.).time(50)
        });

        let progress = animation.get().read().read();

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

#[tokio::test]
pub async fn sequential() {
    fn use_animation_app() -> Element {
        let animation = use_animation(|conf| {
            conf.auto_start(true);
            AnimSequential::new([
                AnimNum::new(10., 100.).time(50),
                AnimNum::new(10., 100.).time(50),
            ])
        });

        let progress_a = animation.get().read()[0].read();
        let progress_b = animation.get().read()[1].read();

        rsx!(
            rect {
                background: "white",
                height: "100%",
                width: "{progress_a}",
                rect {
                    background: "white",
                    height: "100%",
                    width: "{progress_b}",
                }
            }
        )
    }

    let mut utils = launch_test(use_animation_app);

    // Disable event loop ticker
    utils.config().event_loop_ticker = false;

    // Initial state
    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).area().unwrap().width(), 10.0);
    assert_eq!(utils.root().get(0).get(0).area().unwrap().width(), 10.0);

    // State somewhere in the middle
    sleep(Duration::from_millis(32)).await;
    utils.wait_for_update().await;

    let width_a = utils.root().get(0).area().unwrap().width();
    let width_b = utils.root().get(0).get(0).area().unwrap().width();
    assert!(width_a > 10.0);
    assert_eq!(width_b, 10.0);

    // Enable event loop ticker
    utils.config().event_loop_ticker = true;

    // Finished A and B
    utils.wait_for_update().await;
    sleep(Duration::from_millis(50)).await;
    utils.wait_for_update().await;
    utils.wait_for_update().await;

    let width_a = utils.root().get(0).area().unwrap().width();
    let width_b = utils.root().get(0).get(0).area().unwrap().width();
    assert_eq!(width_a, 100.0);
    assert_eq!(width_b, 100.0);
}
