use dioxus::prelude::{use_effect, use_state, ScopeState};
use std::{cell::RefCell, ops::RangeInclusive, time::Duration};
use tokio::time::interval;
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
    let mut tween = tween.get().clone();
    let value = use_state(&cx, || 0.0);
    let started = use_state(&cx, || false);

    let started_setter = started.setter();
    let value_setter = value.setter();

    {
        let started_setter = started_setter.clone();
        let value_setter = value_setter.clone();
        use_effect(&cx, started, move |started| {
            let mut index = 0;

            let dut = tween.duration();

            let mut run_with = move |index: i32| match tween {
                AnimationMode::BounceIn(ref mut tween) => {
                    let tween = tween.get_mut();
                    let v = tween.run(index);
                    value_setter(v);
                }
                AnimationMode::SineIn(ref mut tween) => {
                    let tween = tween.get_mut();
                    let v = tween.run(index);
                    value_setter(v);
                }
                AnimationMode::SineInOut(ref mut tween) => {
                    let tween = tween.get_mut();
                    let v = tween.run(index);
                    value_setter(v);
                }
            };

            async move {
                let mut ticker = interval(Duration::from_millis(1));
                loop {
                    if *started.get() {
                        if index > dut {
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
            value_setter(0.0);
        },
        *value.get(),
    )
}
