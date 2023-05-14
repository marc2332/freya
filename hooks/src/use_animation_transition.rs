use dioxus_core::ScopeState;
use dioxus_hooks::{use_state, UseState};
use freya_node_state::parse_color;
use skia_safe::Color;
use std::time::Duration;
use tokio::time::interval;
use uuid::Uuid;

use crate::{Animation, TransitionAnimation};

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Animate {
    Size(f64, f64, f64),
    Color(Color, Color, Color),
}

impl Animate {
    pub fn new_size(origin: f64, end: f64) -> Self {
        Self::Size(origin, end, origin)
    }

    pub fn new_color(origin: &str, end: &str) -> Self {
        let origin = parse_color(origin).unwrap();
        let end = parse_color(end).unwrap();

        Self::Color(origin, end, origin)
    }

    pub fn set_value(&mut self, value: f64) {
        match self {
            Self::Size(origin, end, current) => {
                let road = *end - *origin;
                let walked = (road / 100.0) * value;
                *current = walked;
            }
            Self::Color(origin, end, current) => {
                let apply_index = |v: u8, d: u8, value: f64| -> u8 {
                    let road = if d > v { d - v } else { v - d };
                    let walked = (road as f64 / 100.0) * value;

                    if d > v {
                        v + walked.round() as u8
                    } else {
                        v - walked.round() as u8
                    }
                };
                let r = apply_index(origin.r(), end.r(), value);
                let g = apply_index(origin.g(), end.g(), value);
                let b = apply_index(origin.b(), end.b(), value);
                *current = Color::from_rgb(r, g, b)
            }
        }
    }

    pub fn as_size(&self) -> f64 {
        self.to_size().unwrap()
    }

    pub fn as_color(&self) -> String {
        self.to_color().unwrap()
    }

    pub fn to_size(&self) -> Option<f64> {
        match self {
            Self::Size(_, _, current) => Some(*current),
            _ => None,
        }
    }

    pub fn to_color(&self) -> Option<String> {
        match self {
            Self::Color(_, _, current) => Some(format!(
                "rgb({}, {}, {})",
                current.r(),
                current.g(),
                current.b()
            )),
            _ => None,
        }
    }

    pub fn clear(&mut self) {
        match self {
            Self::Size(origin, _, current) => {
                *current = *origin;
            }
            Self::Color(origin, _, current) => {
                *current = *origin;
            }
        }
    }
}

/// Manage the lifecyle of an [Animation].
#[derive(Clone)]
pub struct AnimationTransitionManager<'a> {
    current_animation_id: &'a UseState<Option<Uuid>>,
    animations: &'a UseState<Vec<Animate>>,
    cx: &'a ScopeState,
    transition: TransitionAnimation,
}

impl<'a> AnimationTransitionManager<'a> {
    pub fn reverse(&self) {
        let anim = self.transition.to_animation(100.0..=0.0);
        self.run_with(anim);
    }

    pub fn start(&self) {
        let anim = self.transition.to_animation(0.0..=100.0);
        self.run_with(anim);
    }

    pub fn run_with(&self, mut anim: Animation) {
        let new_id = Uuid::new_v4();
        let mut index = 0;

        let animations = self.animations.clone();
        let current_animation_id = self.current_animation_id.clone();

        // Set as current this new animation
        current_animation_id.set(Some(new_id));

        // Spawn the animation that will run at 1ms speed
        self.cx.spawn(async move {
            let mut ticker = interval(Duration::from_millis(1));
            loop {
                // Stop running the animation if it was removed
                if *current_animation_id.current() == Some(new_id) {
                    // Remove the current animation if it has finished
                    if anim.is_finished() {
                        current_animation_id.set(None);
                        break;
                    }

                    // Advance one tick
                    let value = anim.move_value(index);
                    animations.with_mut(|animations| {
                        for animation in animations {
                            animation.set_value(value);
                        }
                    });

                    index += 1;

                    // Wait 1ms
                    ticker.tick().await;
                } else {
                    break;
                }
            }
        });
    }

    /// Clear the currently running [Animation].
    pub fn clear(&self) {
        self.current_animation_id.set(None);
        self.animations.with_mut(|animations| {
            for animation in animations {
                animation.clear()
            }
        })
    }

    /// Check whether there is an [Animation] running or not.
    pub fn is_animating(&self) -> bool {
        self.current_animation_id.is_some()
    }

    /// Get an animation
    pub fn get(&self, index: usize) -> Option<Animate> {
        self.animations.current().get(index).copied()
    }
}

/// Run a group of animated transitions.
///
/// ## Usage
/// ```rust
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     let animation = use_animation_transition(cx, TransitionAnimation::new_linear(50), || vec![
///         Animate::new_size(0.0, 100.0)
///     ]);
///
///     let progress = animation.get(0).unwrap().as_size();
///
///     use_effect(cx, (), move |_| {
///         animation.start();
///         async move {}
///     });
///
///     render!(
///         rect {
///             width: "{progress}",
///         }
///     )
/// }
/// ```
///
pub fn use_animation_transition(
    cx: &ScopeState,
    transition: TransitionAnimation,
    init: impl FnOnce() -> Vec<Animate>,
) -> AnimationTransitionManager {
    let current_animation_id = use_state(cx, || None);
    let animations = use_state(cx, init);

    AnimationTransitionManager {
        current_animation_id,
        animations,
        cx,
        transition,
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::{use_animation_transition, Animate, TransitionAnimation};
    use dioxus_hooks::use_effect;
    use freya::prelude::*;
    use freya_testing::launch_test;
    use tokio::time::sleep;

    #[tokio::test]
    pub async fn track_progress() {
        fn use_animation_transition_app(cx: Scope) -> Element {
            let animation =
                use_animation_transition(cx, TransitionAnimation::new_linear(50), || {
                    vec![Animate::new_size(0.0, 100.0)]
                });

            let progress = animation.get(0).unwrap().as_size();

            use_effect(cx, (), move |_| {
                animation.start();
                async move {}
            });

            render!(rect {
                width: "{progress}",
            })
        }

        let mut utils = launch_test(use_animation_transition_app);

        // Initial state
        utils.wait_for_update().await;

        assert_eq!(utils.root().get(0).layout().unwrap().width(), 0.0);

        // State somewhere in the middle
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let width = utils.root().get(0).layout().unwrap().width();
        assert!(width > 0.0);
        assert!(width < 100.0);

        sleep(Duration::from_millis(50)).await;

        // State in the end
        utils.wait_for_update().await;

        let width = utils.root().get(0).layout().unwrap().width();
        assert_eq!(width, 100.0);
    }
}
