use dioxus_core::ScopeState;
use dioxus_hooks::{to_owned, use_effect, use_state};
use std::{cell::RefCell, ops::RangeInclusive, time::Duration};
use tokio::time::interval;
use tween::{BounceIn, Linear, SineIn, SineInOut, Tweener};
use uuid::Uuid;

/// Type of animation to use.
#[derive(Clone)]
pub enum AnimationMode {
    BounceIn(RefCell<Tweener<f64, i32, BounceIn>>),
    SineIn(RefCell<Tweener<f64, i32, SineIn>>),
    SineInOut(RefCell<Tweener<f64, i32, SineInOut>>),
    Linear(RefCell<Tweener<f64, i32, Linear>>),
}

impl AnimationMode {
    pub fn new_bounce_in(range: RangeInclusive<f64>, time: i32) -> Self {
        Self::BounceIn(RefCell::new(Tweener::bounce_in(
            *range.start(),
            *range.end(),
            time,
        )))
    }
    pub fn new_sine_in(range: RangeInclusive<f64>, time: i32) -> Self {
        Self::SineIn(RefCell::new(Tweener::sine_in(
            *range.start(),
            *range.end(),
            time,
        )))
    }
    pub fn new_sine_in_out(range: RangeInclusive<f64>, time: i32) -> Self {
        Self::SineInOut(RefCell::new(Tweener::sine_in_out(
            *range.start(),
            *range.end(),
            time,
        )))
    }
    pub fn new_linear(range: RangeInclusive<f64>, time: i32) -> Self {
        Self::Linear(RefCell::new(Tweener::linear(
            *range.start(),
            *range.end(),
            time,
        )))
    }
}

impl AnimationMode {
    /// Get the duration of the animation mode
    fn duration(&self) -> i32 {
        match self {
            AnimationMode::BounceIn(tween) => tween.borrow().duration,
            AnimationMode::SineIn(tween) => tween.borrow().duration,
            AnimationMode::SineInOut(tween) => tween.borrow().duration,
            AnimationMode::Linear(tween) => tween.borrow().duration,
        }
    }

    /// Get the initial value of the animation mode
    fn initial_value(&self) -> f64 {
        match self {
            AnimationMode::BounceIn(tween) => tween.borrow().initial_value(),
            AnimationMode::SineIn(tween) => tween.borrow().initial_value(),
            AnimationMode::SineInOut(tween) => tween.borrow().initial_value(),
            AnimationMode::Linear(tween) => tween.borrow().initial_value(),
        }
    }

    /// Get the final value of the animation mode
    #[allow(dead_code)]
    fn final_value(&self) -> f64 {
        match self {
            AnimationMode::BounceIn(tween) => tween.borrow().final_value(),
            AnimationMode::SineIn(tween) => tween.borrow().final_value(),
            AnimationMode::SineInOut(tween) => tween.borrow().final_value(),
            AnimationMode::Linear(tween) => tween.borrow().final_value(),
        }
    }
}

/// More flexible animation hook than `use_animation`
pub fn use_animation_managed(
    cx: &ScopeState,
    init_value: f64,
) -> (impl Fn(AnimationMode) + '_, impl Fn(f64) + '_, f64, bool) {
    let current_anim_id = use_state(cx, || None);
    let value = use_state(cx, || init_value);

    let start_anim = move |mut anim: AnimationMode| {
        let new_id = Uuid::new_v4();
        let mut index = 0;

        to_owned![value, current_anim_id];

        // Set as current this new animation
        current_anim_id.set(Some(new_id));

        let duration = anim.duration();

        let mut run_with = move |index: i32| {
            match anim {
                AnimationMode::BounceIn(ref mut tween) => {
                    let tween = tween.get_mut();
                    let v = tween.move_to(index);
                    value.set(v);
                }
                AnimationMode::SineIn(ref mut tween) => {
                    let tween = tween.get_mut();
                    let v = tween.move_to(index);
                    value.set(v);
                }
                AnimationMode::SineInOut(ref mut tween) => {
                    let tween = tween.get_mut();
                    let v = tween.move_to(index);
                    value.set(v);
                }
                AnimationMode::Linear(ref mut tween) => {
                    let tween = tween.get_mut();
                    let v = tween.move_to(index);
                    value.set(v);
                }
            };
        };

        cx.spawn(async move {
            let mut ticker = interval(Duration::from_millis(1));
            loop {
                if *current_anim_id.current() == Some(new_id) {
                    if index > duration {
                        current_anim_id.set(None);
                        break;
                    }
                    run_with(index);
                    index += 1;
                    ticker.tick().await;
                } else {
                    break;
                }
            }
        });
    };

    let set_value = move |new_value: f64| {
        current_anim_id.set(None);
        value.set(new_value);
    };

    let current_value = *value.get();

    let is_animating = current_anim_id.is_some();

    (start_anim, set_value, current_value, is_animating)
}

/// Create and configure an animation.
pub fn use_animation(
    cx: &ScopeState,
    anim_mode: impl FnOnce() -> AnimationMode,
) -> (impl Fn(), impl Fn(), f64) {
    let anim = use_state(cx, anim_mode);
    let initial_value = anim.get().initial_value();
    let value = use_state(cx, || initial_value);
    let started = use_state(cx, || false);

    let started_setter = started.setter();
    let value_setter = value.setter();

    {
        let started_setter = started_setter.clone();
        let value_setter = value_setter.clone();
        use_effect(cx, started, move |started| {
            let mut index = 0;
            let anim = anim.clone();

            let duration = anim.duration();

            let run_with = move |index: i32| {
                anim.with_mut(|anim| {
                    match anim {
                        AnimationMode::BounceIn(ref mut tween) => {
                            let tween = tween.get_mut();
                            let v = tween.move_to(index);
                            value_setter(v);
                        }
                        AnimationMode::SineIn(ref mut tween) => {
                            let tween = tween.get_mut();
                            let v = tween.move_to(index);
                            value_setter(v);
                        }
                        AnimationMode::SineInOut(ref mut tween) => {
                            let tween = tween.get_mut();
                            let v = tween.move_to(index);
                            value_setter(v);
                        }
                        AnimationMode::Linear(ref mut tween) => {
                            let tween = tween.get_mut();
                            let v = tween.move_to(index);
                            value_setter(v);
                        }
                    };
                });
            };

            async move {
                let mut ticker = interval(Duration::from_millis(1));
                loop {
                    if *started.current() {
                        if index > duration {
                            started_setter(false);
                            break;
                        }
                        run_with(index);
                        index += 1;
                        ticker.tick().await;
                    } else {
                        break;
                    }
                }
            }
        });
    }

    (
        {
            let started_setter = started_setter.clone();
            move || started_setter(true)
        },
        move || {
            started_setter(false);
            value_setter(initial_value);
        },
        *value.get(),
    )
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::{use_animation, use_animation_managed, AnimationMode};
    use dioxus_hooks::use_effect;
    use freya::prelude::*;
    use freya_testing::{launch_test, FreyaEvent, MouseButton};
    use tokio::time::sleep;

    #[tokio::test]
    pub async fn track_progress() {
        fn use_animation_app(cx: Scope) -> Element {
            let (start, _, progress) =
                use_animation(cx, || AnimationMode::new_linear(0.0..=100.0, 50));
            use_effect(cx, (), move |_| async move {
                start();
            });

            render!(rect {
                width: "{progress}",
            })
        }

        let mut utils = launch_test(use_animation_app);

        // Initial state
        utils.wait_for_update((500.0, 500.0)).await;

        assert_eq!(utils.root().child(0).unwrap().layout().unwrap().width, 0.0);

        // State somewhere in the middle
        utils.wait_for_update((500.0, 500.0)).await;
        utils.wait_for_update((500.0, 500.0)).await;
        utils.wait_for_work((500.0, 500.0)).await;
        let width = utils.root().child(0).unwrap().layout().unwrap().width;
        assert!(width > 0.0);
        assert!(width < 100.0);

        sleep(Duration::from_millis(50)).await;

        // State in the end
        utils.wait_for_update((500.0, 500.0)).await;
        utils.wait_for_work((500.0, 500.0)).await;
        let width = utils.root().child(0).unwrap().layout().unwrap().width;
        assert_eq!(width, 100.0);
    }

    #[tokio::test]
    pub async fn restart_progress() {
        fn use_animation_app(cx: Scope) -> Element {
            let (start, restart, progress) =
                use_animation(cx, || AnimationMode::new_linear(10.0..=100.0, 50));

            use_effect(cx, (), move |_| async move {
                start();
            });

            render!(rect {
                background: "white",
                height: "100%",
                onclick: move |_| restart(),
                width: "{progress}",
            })
        }

        let mut utils = launch_test(use_animation_app);

        // Initial state
        utils.wait_for_update((500.0, 500.0)).await;

        assert_eq!(utils.root().child(0).unwrap().layout().unwrap().width, 10.0);

        // State somewhere in the middle
        utils.wait_for_update((500.0, 500.0)).await;
        utils.wait_for_update((500.0, 500.0)).await;
        utils.wait_for_work((500.0, 500.0)).await;
        let width = utils.root().child(0).unwrap().layout().unwrap().width;
        assert!(width > 10.0);

        utils.send_event(FreyaEvent::Mouse {
            name: "click",
            cursor: (5.0, 5.0),
            button: Some(MouseButton::Left),
        });

        // State has been restarted
        utils.wait_for_update((500.0, 500.0)).await;
        utils.wait_for_work((500.0, 500.0)).await;
        let width = utils.root().child(0).unwrap().layout().unwrap().width;
        assert_eq!(width, 10.0);
    }

    #[test]
    pub fn animation_mode_settings() {
        let anim = AnimationMode::new_sine_in_out(7.0..=99.0, 500);
        assert_eq!(anim.duration(), 500);
        assert_eq!(anim.initial_value(), 7.0);
        assert_eq!(anim.final_value(), 99.0);
    }

    #[tokio::test]
    pub async fn test_use_animation_managed() {
        fn use_animation_app(cx: Scope) -> Element {
            let (anim, _, progress, _) = use_animation_managed(cx, 0.0);

            use_effect(cx, &(progress), move |v| {
                if v == 0.0 {
                    anim(AnimationMode::new_sine_in_out(0.0..=100.0, 50));
                } else if v == 100.0 {
                    anim(AnimationMode::new_sine_in_out(100.0..=0.0, 50));
                }
                async move {}
            });

            render!(rect {
                width: "{progress}",
            })
        }

        let mut utils = launch_test(use_animation_app);
        let element = utils.root().child(0).unwrap();

        // Initial state
        utils.wait_for_update((500.0, 500.0)).await;

        assert_eq!(element.layout().unwrap().width, 0.0);

        // State somewhere in the middle
        utils.wait_for_update((500.0, 500.0)).await;
        utils.wait_for_work((500.0, 500.0)).await;
        let width = element.layout().unwrap().width;
        assert!(width > 0.0);
        assert!(width < 100.0);

        // Reached 100.0
        utils.wait_until_cleanup((500.0, 500.0)).await;
        let width = element.layout().unwrap().width;
        assert_eq!(width, 100.0);

        // Back to 0.0
        utils.wait_until_cleanup((500.0, 500.0)).await;
        let width = element.layout().unwrap().width;
        assert_eq!(width, 0.0);
    }
}
