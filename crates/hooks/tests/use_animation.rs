use std::time::Duration;

use crate::{use_animation, Animation};
use dioxus_core::use_hook;
use dioxus_hooks::to_owned;
use freya::prelude::*;
use freya_testing::{events::pointer::MouseButton, launch_test, FreyaEvent};
use tokio::time::sleep;

#[tokio::test]
pub async fn track_progress() {
    fn use_animation_app() -> Element {
        let mut animation = use_animation(|| 0.0);

        let progress = animation.value();

        let _ = use_memo(move || {
            animation.start(Animation::new_linear(0.0..=100.0, 50));
        });

        rsx!(rect {
            width: "{progress}",
        })
    }

    let mut utils = launch_test(use_animation_app);

    // Disable event loop ticker
    utils.config().enable_ticker(false);

    // Initial state
    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).layout().unwrap().width(), 0.0);

    // State somewhere in the middle
    sleep(Duration::from_millis(15)).await;
    utils.wait_for_update().await;

    let width = utils.root().get(0).layout().unwrap().width();
    assert!(width > 0.0);

    // Enable event loop ticker
    utils.config().enable_ticker(true);

    // Already finished
    sleep(Duration::from_millis(50)).await;

    // State in the end
    utils.wait_for_update().await;

    let width = utils.root().get(0).layout().unwrap().width();
    assert_eq!(width, 100.0);
}

#[tokio::test]
pub async fn restart_progress() {
    fn use_animation_app() -> Element {
        let mut animation = use_animation(|| 10.0);

        let progress = animation.value();

        let mut restart = {
            to_owned![animation];
            move || {
                animation.clear();
            }
        };

        use_hook(|| {
            animation.start(Animation::new_linear(10.0..=100.0, 50));
        });

        rsx!(rect {
            background: "white",
            height: "100%",
            onclick: move |_| restart(),
            width: "{progress}",
        })
    }

    let mut utils = launch_test(use_animation_app);

    // Disable event loop ticker
    utils.config().enable_ticker(false);

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
    utils.config().enable_ticker(true);

    // Already finished
    sleep(Duration::from_millis(50)).await;

    // State has been restarted
    utils.wait_for_update().await;
    utils.wait_for_update().await;

    let width = utils.root().get(0).layout().unwrap().width();
    assert_eq!(width, 10.0);
}

#[test]
pub fn animation_mode_settings() {
    let anim = Animation::new_sine_in_out(7.0..=99.0, 500);
    assert_eq!(anim.duration(), 500);
    assert_eq!(anim.initial_value(), 7.0);
    assert_eq!(anim.final_value(), 99.0);
}