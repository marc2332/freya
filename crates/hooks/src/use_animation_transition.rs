use dioxus_core::prelude::spawn;
use dioxus_hooks::{use_memo_with_dependencies, Dependency};
use dioxus_signals::{Readable, Signal, Writable};
use freya_engine::prelude::Color;
use freya_node_state::Parse;
use tokio::time::Instant;
use uuid::Uuid;

use crate::{use_platform, Animation, TransitionAnimation, UsePlatform};

/// Configure a `Transition` animation.
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Transition {
    /// Transition from one size to another.
    Size(f64, f64),
    /// Transition from one color to another.
    Color(Color, Color),
}

impl Transition {
    /// Create a Size transition.
    pub fn new_size(start: f64, end: f64) -> Self {
        Self::Size(start, end)
    }

    /// Create a Color transition.
    pub fn new_color(start: &str, end: &str) -> Self {
        let start = Color::parse(start).unwrap();
        let end = Color::parse(end).unwrap();

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
    /// Process the new value in this transition.
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

    /// Reset the current value back to the starting value.
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

    /// Cast as a size transition. This could panic if the tranistion of type Size.
    pub fn as_size(&self) -> f64 {
        self.to_size().unwrap()
    }

    /// Cast as a Color transition. This could panic if the tranistion of type Color.
    pub fn as_color(&self) -> String {
        self.to_color().unwrap()
    }

    /// Try casting to a Size transition.
    pub fn to_size(&self) -> Option<f64> {
        match self {
            Self::Size(current) => Some(*current),
            _ => None,
        }
    }

    /// Try casting to a stringified Color transition.
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

    /// Try casting to a raw Color transition.
    pub fn to_raw_color(&self) -> Option<Color> {
        match self {
            Self::Color(current) => Some(*current),
            _ => None,
        }
    }
}

/// Manage the lifecyle of a collection of transitions.
#[derive(Clone, PartialEq)]
pub struct TransitionsManager {
    /// Registered transitions
    transitions: Signal<Vec<Transition>>,
    /// The registered transition states
    transitions_storage: Signal<Vec<TransitionState>>,
    /// The transition animation type
    transition_animation: TransitionAnimation,
    /// Currently running animation.
    current_animation_id: Signal<Option<Uuid>>,
    /// Platform APIs
    platform: UsePlatform,
}

impl TransitionsManager {
    /// Animate from the end to the start.
    pub fn reverse(&mut self) {
        self.clear();
        let animation = self.transition_animation.to_animation(100.0..=0.0);
        self.run_with_animation(animation);
    }

    /// Animate from the start to the end.
    pub fn start(&mut self) {
        self.clear();
        let animation = self.transition_animation.to_animation(0.0..=100.0);
        self.run_with_animation(animation);
    }

    fn run_with_animation(&self, mut animation: Animation) {
        let animation_id = Uuid::new_v4();

        let platform = self.platform.clone();
        let mut ticker = platform.new_ticker();
        let transitions = self.transitions;
        let mut transitions_storage = self.transitions_storage;
        let mut current_animation_id = self.current_animation_id;

        // Set as current this new animation
        current_animation_id.set(Some(animation_id));

        spawn(async move {
            platform.request_animation_frame();

            let mut index = 0;
            let mut prev_frame = Instant::now();

            loop {
                // Wait for the event loop to tick
                ticker.tick().await;
                platform.request_animation_frame();

                // Stop running the animation if it's no longer selected
                if *current_animation_id.peek() == Some(animation_id) {
                    // Remove the current animation if it has finished
                    if animation.is_finished() {
                        current_animation_id.set(None);
                        break;
                    }

                    index += prev_frame.elapsed().as_millis() as i32;
                    let value = animation.move_value(index);
                    transitions_storage.with_mut(|storage| {
                        for (i, storage) in storage.iter_mut().enumerate() {
                            if let Some(conf) = transitions.peek().get(i) {
                                storage.set_value(conf, value);
                            }
                        }
                    });

                    prev_frame = Instant::now();
                } else {
                    break;
                }
            }
        });
    }

    /// Clear all the currently running [Transition]s.
    pub fn clear(&mut self) {
        self.current_animation_id.set(None);
        self.transitions_storage.with_mut(|storage| {
            for (i, storage) in storage.iter_mut().enumerate() {
                if let Some(conf) = self.transitions.peek().get(i) {
                    storage.clear(conf);
                }
            }
        })
    }

    /// Check whether there are [Transition]s running or not.
    pub fn is_animating(&self) -> bool {
        self.current_animation_id.peek().is_some()
    }

    /// Check whether the [Transition]s are at the start or at the end.
    pub fn is_at_start(&self) -> bool {
        if let Some(storage) = self.get(0) {
            let anim = self.transitions.peek()[0];
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
        self.transitions_storage.read().get(index).copied()
    }
}

/// Run a group of animated transitions.
///
/// ## Usage
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut animation = use_animation_transition(TransitionAnimation::new_linear(50), (), |_| vec![
///         Transition::new_size(0.0, 100.0)
///     ]);
///
///     let progress = animation.get(0).unwrap().as_size();
///
///     use_hook(move || {
///         animation.start();
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
pub fn use_animation_transition<D>(
    transition: TransitionAnimation,
    dependencies: D,
    init: impl Fn(D::Out) -> Vec<Transition> + 'static,
) -> TransitionsManager
where
    D: Dependency + 'static,
{
    use_memo_with_dependencies(dependencies.clone(), move |deps| {
        let platform = use_platform();
        let transitions = init(deps);
        let transitions_states = animations_map(&transitions);

        TransitionsManager {
            current_animation_id: Signal::new(None),
            transitions: Signal::new(transitions),
            transitions_storage: Signal::new(transitions_states),
            transition_animation: transition,
            platform,
        }
    })
    .read()
    .clone()
}

fn animations_map(animations: &[Transition]) -> Vec<TransitionState> {
    animations
        .iter()
        .map(TransitionState::from)
        .collect::<Vec<TransitionState>>()
}
