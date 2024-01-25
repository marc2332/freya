use dioxus_core::{prelude::spawn, use_hook};
use dioxus_signals::{Readable, Signal, Writable};
use tokio::time::Instant;
use uuid::Uuid;

use crate::{use_platform, Animation, UsePlatform};

/// Manage the lifecyle of an [Animation].
#[derive(Clone)]
pub struct AnimationManager {
    init_value: f64,
    current_animation_id: Signal<Option<Uuid>>,
    value: Signal<f64>,
    platform: UsePlatform,
}

impl AnimationManager {
    /// Start the given [Animation].
    pub fn start(&mut self, mut anim: Animation) {
        let new_id = Uuid::new_v4();

        // Set as current this new animation
        self.current_animation_id.set(Some(new_id));

        let platform = self.platform.clone();
        let mut ticker = platform.new_ticker();
        let mut value = self.value.clone();
        let mut current_animation_id = self.current_animation_id.clone();

        // Spawn the animation that will run at 1ms speed
        spawn(async move {
            platform.request_animation_frame();

            let mut index = 0;
            let mut prev_frame = Instant::now();

            loop {
                // Wait for the event loop to tick
                ticker.tick().await;
                platform.request_animation_frame();

                // Stop running the animation if it was removed
                if *current_animation_id.peek() == Some(new_id) {
                    // Remove the current animation if it has finished
                    if anim.is_finished() {
                        current_animation_id.set(None);
                        break;
                    }

                    index += prev_frame.elapsed().as_millis() as i32;
                    value.set(anim.move_value(index));

                    prev_frame = Instant::now();
                } else {
                    break;
                }
            }
        });
    }

    /// Clear the currently running [Animation].
    pub fn clear(&mut self) {
        self.current_animation_id.set(None);
        self.set_value(self.init_value);
    }

    /// Check whether there is an [Animation] running or not.
    pub fn is_animating(&self) -> bool {
        self.current_animation_id.read().is_some()
    }

    /// Get the current value of the [Animation].
    pub fn value(&self) -> f64 {
        *self.value.read()
    }

    /// Get the current value of the [Animation], silently.
    pub fn peek_value(&self) -> f64 {
        *self.value.peek()
    }

    /// Set a new value for the [Animation].
    pub fn set_value(&mut self, new_value: f64) {
        self.value.set(new_value);
    }
}

/// Run animations.
///
/// ## Usage
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let animation = use_animation(|| 0.0);
///
///     let progress = animation.value();
///
///     use_hook(move || {
///         animation.start(Animation::new_linear(0.0..=100.0, 50));
///     });
///
///     rsx!(
///         rect {
///             width: "{progress}",
///         }
///     )
/// }
/// ```
///
pub fn use_animation(init_value: impl FnOnce() -> f64) -> AnimationManager {
    use_hook(|| {
        let value = init_value();
        AnimationManager {
            current_animation_id: Signal::new(None),
            value: Signal::new(value),
            init_value: value,
            platform: use_platform(),
        }
    })
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::{use_animation, Animation};
    use dioxus_hooks::to_owned;
    use freya::prelude::*;
    use freya_testing::{events::pointer::MouseButton, launch_test, FreyaEvent};
    use tokio::time::sleep;

    #[tokio::test]
    pub async fn track_progress() {
        fn use_animation_app() -> Element {
            let animation = use_animation(|| 0.0);

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

        // State in the end
        utils.wait_for_update().await;

        let width = utils.root().get(0).layout().unwrap().width();
        assert_eq!(width, 100.0);
    }

    #[tokio::test]
    pub async fn restart_progress() {
        fn use_animation_app() -> Element {
            let animation = use_animation(|| 10.0);

            let progress = animation.value();

            let restart = {
                to_owned![animation];
                move || {
                    animation.clear();
                }
            };

            let _ = use_memo(move || {
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
}
