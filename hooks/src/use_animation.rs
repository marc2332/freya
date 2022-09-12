use dioxus::prelude::{use_effect, use_state, ScopeState};
use std::{cell::RefCell, ops::RangeInclusive};
use tween::{BounceIn, SineIn, SineInOut, Tween};

#[derive(Clone)]
pub enum AnimationMode {
    BounceIn(RefCell<BounceIn<f64, i32>>),
    SineIn(RefCell<SineIn<f64, i32>>),
    SineInOut(RefCell<SineInOut<f64, i32>>),
}

impl AnimationMode {
    pub fn new_bounce_in(range: RangeInclusive<f64>, time: i32) -> Self {
        Self::BounceIn(RefCell::new(BounceIn::new(range, time)))
    }
    pub fn new_sine_in(range: RangeInclusive<f64>, time: i32) -> Self {
        Self::SineIn(RefCell::new(SineIn::new(range, time)))
    }
    pub fn new_sine_in_out(range: RangeInclusive<f64>, time: i32) -> Self {
        Self::SineInOut(RefCell::new(SineInOut::new(range, time)))
    }
}

impl AnimationMode {
    fn duration(&self) -> i32 {
        match self {
            AnimationMode::BounceIn(tween) => tween.borrow().duration(),
            AnimationMode::SineIn(tween) => tween.borrow().duration(),
            AnimationMode::SineInOut(tween) => tween.borrow().duration(),
        }
    }
}

pub fn use_animation(
    cx: &ScopeState,
    mode: impl FnOnce() -> AnimationMode,
) -> (impl Fn(), impl Fn(), f64) {
    let tween = use_state::<AnimationMode>(&cx, || mode());
    let tween = tween.get().clone();
    let index = use_state(&cx, || 0);
    let value = use_state(&cx, || 0.0);
    let started = use_state(&cx, || false);

    let started_setter = started.setter();
    let value_setter = value.setter();
    let index_setter = index.setter();

    let started_setter_b = started_setter.clone();
    let value_setter_b = value_setter.clone();
    let index_setter_b = index_setter.clone();

    use_effect(&cx, (started, index), move |(started, index)| {
        let started_setter = started_setter_b.clone();
        async move {
            if *started.get() {
                if index.get() > &tween.duration() {
                    started_setter(false);
                    index_setter_b(0);
                    return;
                }
                match tween {
                    AnimationMode::BounceIn(mut tween) => {
                        let tween = tween.get_mut();
                        let v = tween.run(*index.get());
                        value_setter_b(v);
                        index_setter_b(index.get() + 1);
                    }
                    AnimationMode::SineIn(mut tween) => {
                        let tween = tween.get_mut();
                        let v = tween.run(*index.get());
                        value_setter_b(v);
                        index_setter_b(index.get() + 1);
                    }
                    AnimationMode::SineInOut(mut tween) => {
                        let tween = tween.get_mut();
                        let v = tween.run(*index.get());
                        value_setter_b(v);
                        index_setter_b(index.get() + 1);
                    }
                }
            }
        }
    });

    (
        {
            let started_setter = started_setter.clone();
            move || started_setter(true)
        },
        move || {
            started_setter(false);
            index_setter(0);
            value_setter(0.0);
        },
        *value.get(),
    )
}
