use dioxus_core::ScopeState;
use dioxus_hooks::{use_effect, use_memo, use_state, UseFutureDep, UseState};
use freya_node_state::parse_color;
use skia_safe::Color;
use std::time::Duration;
use tokio::time::interval;
use uuid::Uuid;

use crate::{Animation, TransitionAnimation};

/// Configure a `Transition` animation.
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Transition {
    Size(f64, f64),
    Color(Color, Color),
}

impl Transition {
    pub fn new_size(start: f64, end: f64) -> Self {
        Self::Size(start, end)
    }

    pub fn new_color(start: &str, end: &str) -> Self {
        let start = parse_color(start).unwrap();
        let end = parse_color(end).unwrap();

        Self::Color(start, end)
    }
}

/// Stores the current state for a [`Transition`].
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum TransitionState {
    Size(f64),
    Color(Color),
}

impl From<&Transition> for TransitionState {
    fn from(value: &Transition) -> Self {
        match *value {
            Transition::Size(start, _) => Self::Size(start),
            Transition::Color(start, _) => Self::Color(start),
        }
    }
}

impl TransitionState {
    pub fn set_value(&mut self, animate: &Transition, value: f64) {
        match (self, animate) {
            (Self::Size(current), Transition::Size(start, end)) => {
                let road = *end - *start;
                let walked = (road / 100.0) * value;
                *current = walked;
            }
            (Self::Color(current), Transition::Color(start, end)) => {
                let apply_index = |v: u8, d: u8, value: f64| -> u8 {
                    let road = if d > v { d - v } else { v - d };
                    let walked = (road as f64 / 100.0) * value;

                    if d > v {
                        v + walked.round() as u8
                    } else {
                        v - walked.round() as u8
                    }
                };
                let r = apply_index(start.r(), end.r(), value);
                let g = apply_index(start.g(), end.g(), value);
                let b = apply_index(start.b(), end.b(), value);
                *current = Color::from_rgb(r, g, b)
            }
            _ => {}
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
            Self::Size(current) => Some(*current),
            _ => None,
        }
    }

    pub fn to_color(&self) -> Option<String> {
        match self {
            Self::Color(current) => Some(format!(
                "rgb({}, {}, {})",
                current.r(),
                current.g(),
                current.b()
            )),
            _ => None,
        }
    }

    pub fn to_raw_color(&self) -> Option<Color> {
        match self {
            Self::Color(current) => Some(*current),
            _ => None,
        }
    }

    pub fn clear(&mut self, animate: &Transition) {
        match (self, animate) {
            (Self::Size(current), Transition::Size(start, _)) => {
                *current = *start;
            }
            (Self::Color(current), Transition::Color(start, _)) => {
                *current = *start;
            }
            _ => {}
        }
    }
}

/// Manage the lifecyle of an [AnimationTransitionManager].
#[derive(Clone)]
pub struct TransitionsManager<'a> {
    current_animation_id: &'a UseState<Option<Uuid>>,
    animations: &'a Vec<Transition>,
    storage: &'a UseState<Vec<TransitionState>>,
    cx: &'a ScopeState,
    transition: TransitionAnimation,
}

impl<'a> TransitionsManager<'a> {
    /// Animate from the end to the start.
    pub fn reverse(&self) {
        self.clear();
        let anim = self.transition.to_animation(100.0..=0.0);
        self.run_with(anim);
    }

    /// Animate from the start to the end.
    pub fn start(&self) {
        self.clear();
        let anim = self.transition.to_animation(0.0..=100.0);
        self.run_with(anim);
    }

    pub fn run_with(&self, mut anim: Animation) {
        let new_id = Uuid::new_v4();
        let mut index = 0;

        let animations = self.animations.clone();
        let storage = self.storage.clone();
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
                    storage.with_mut(|storage| {
                        for (i, storage) in storage.iter_mut().enumerate() {
                            if let Some(conf) = animations.get(i) {
                                storage.set_value(conf, value);
                            }
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

    /// Clear all the currently running [Transition]
    pub fn clear(&self) {
        self.current_animation_id.set(None);
        self.storage.with_mut(|storage| {
            for (i, storage) in storage.iter_mut().enumerate() {
                if let Some(conf) = self.animations.get(i) {
                    storage.clear(conf);
                }
            }
        })
    }

    /// Check whether there are [Transition]s running or not.
    pub fn is_animating(&self) -> bool {
        self.current_animation_id.is_some()
    }

    /// Check whether the [Transition]s are at the start or at the end.
    pub fn is_at_start(&self) -> bool {
        if let Some(storage) = self.get(0) {
            let anim = self.animations[0];
            match anim {
                Transition::Size(start, _) => start == storage.to_size().unwrap_or(start),
                Transition::Color(start, _) => start == storage.to_raw_color().unwrap_or(start),
            }
        } else {
            true
        }
    }

    /// Get an [TransitionState]
    pub fn get(&self, index: usize) -> Option<TransitionState> {
        self.storage.current().get(index).copied()
    }
}

/// Run a group of animated transitions.
///
/// ## Usage
/// ```rust
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     let animation = use_animation_transition(cx, TransitionAnimation::new_linear(50), (), |_| vec![
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
pub fn use_animation_transition<D>(
    cx: &ScopeState,
    transition: TransitionAnimation,
    dependencies: D,
    mut init: impl Fn(D::Out) -> Vec<Transition>,
) -> TransitionsManager
where
    D: UseFutureDep,
{
    let current_animation_id = use_state(cx, || None);
    let animations = use_memo(cx, dependencies.clone(), &mut init);
    let storage = use_state(cx, || animations_map(animations));
    let storage_setter = storage.setter();

    use_effect(cx, dependencies, move |v| {
        storage_setter(animations_map(&init(v)));
        async move {}
    });

    TransitionsManager {
        current_animation_id,
        animations,
        storage,
        cx,
        transition,
    }
}

fn animations_map(animations: &[Transition]) -> Vec<TransitionState> {
    animations
        .iter()
        .map(TransitionState::from)
        .collect::<Vec<TransitionState>>()
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::{use_animation_transition, Transition, TransitionAnimation};
    use dioxus_hooks::use_effect;
    use freya::prelude::*;
    use freya_testing::launch_test;
    use tokio::time::sleep;

    #[tokio::test]
    pub async fn track_progress() {
        fn use_animation_transition_app(cx: Scope) -> Element {
            let animation =
                use_animation_transition(cx, TransitionAnimation::new_linear(50), (), |_| {
                    vec![Transition::new_size(0.0, 100.0)]
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
