use std::time::Duration;

use crate::{use_animation_transition, Transition, TransitionAnimation};
use dioxus_core::use_hook;
use freya::prelude::*;
use freya_testing::launch_test;
use tokio::time::sleep;

#[tokio::test]
pub async fn track_progress() {
    fn use_animation_transition_app() -> Element {
        let mut animation =
            use_animation_transition(TransitionAnimation::new_linear(50), (), |_| {
                vec![Transition::new_size(0.0, 100.0)]
            });

        let progress = animation.get(0).unwrap().as_size();

        use_hook(move || {
            animation.start();
        });

        rsx!(rect {
            width: "{progress}",
        })
    }

    let mut utils = launch_test(use_animation_transition_app);

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
