use std::{cell::RefCell, ops::RangeInclusive};

use tween::{BounceIn, Linear, SineIn, SineInOut, Tweener};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransitionAnimation {
    BounceIn(i32),
    SineIn(i32),
    SineInOut(i32),
    Linear(i32),
}

impl TransitionAnimation {
    /// New BounceIn [Animation]
    pub fn new_bounce_in(time: i32) -> Self {
        Self::BounceIn(time)
    }

    /// New SineIn [Animation]
    pub fn new_sine_in(time: i32) -> Self {
        Self::SineIn(time)
    }

    /// New SineInOut [Animation]
    pub fn new_sine_in_out(time: i32) -> Self {
        Self::SineInOut(time)
    }

    /// New Linear [Animation]
    pub fn new_linear(time: i32) -> Self {
        Self::Linear(time)
    }

    pub fn to_animation(self, range: RangeInclusive<f64>) -> Animation {
        match self {
            TransitionAnimation::BounceIn(time) => Animation::new_bounce_in(range, time),
            TransitionAnimation::SineIn(time) => Animation::new_sine_in(range, time),
            TransitionAnimation::SineInOut(time) => Animation::new_sine_in_out(range, time),
            TransitionAnimation::Linear(time) => Animation::new_linear(range, time),
        }
    }
}

/// Animation mode and configuration.
#[derive(Clone)]
pub enum Animation {
    BounceIn(RefCell<Tweener<f64, i32, BounceIn>>),
    SineIn(RefCell<Tweener<f64, i32, SineIn>>),
    SineInOut(RefCell<Tweener<f64, i32, SineInOut>>),
    Linear(RefCell<Tweener<f64, i32, Linear>>),
}

impl Animation {
    /// New BounceIn [Animation]
    pub fn new_bounce_in(range: RangeInclusive<f64>, time: i32) -> Self {
        Self::BounceIn(RefCell::new(Tweener::bounce_in(
            *range.start(),
            *range.end(),
            time,
        )))
    }

    /// New SineIn [Animation]
    pub fn new_sine_in(range: RangeInclusive<f64>, time: i32) -> Self {
        Self::SineIn(RefCell::new(Tweener::sine_in(
            *range.start(),
            *range.end(),
            time,
        )))
    }

    /// New SineInOut [Animation]
    pub fn new_sine_in_out(range: RangeInclusive<f64>, time: i32) -> Self {
        Self::SineInOut(RefCell::new(Tweener::sine_in_out(
            *range.start(),
            *range.end(),
            time,
        )))
    }

    /// New Linear [Animation]
    pub fn new_linear(range: RangeInclusive<f64>, time: i32) -> Self {
        Self::Linear(RefCell::new(Tweener::linear(
            *range.start(),
            *range.end(),
            time,
        )))
    }

    /// Get the duration of the animation.
    pub fn duration(&self) -> i32 {
        match self {
            Animation::BounceIn(tween) => tween.borrow().duration,
            Animation::SineIn(tween) => tween.borrow().duration,
            Animation::SineInOut(tween) => tween.borrow().duration,
            Animation::Linear(tween) => tween.borrow().duration,
        }
    }

    /// Get the initial value of the animation.
    #[allow(dead_code)]
    pub fn initial_value(&self) -> f64 {
        match self {
            Animation::BounceIn(tween) => tween.borrow().initial_value(),
            Animation::SineIn(tween) => tween.borrow().initial_value(),
            Animation::SineInOut(tween) => tween.borrow().initial_value(),
            Animation::Linear(tween) => tween.borrow().initial_value(),
        }
    }

    /// Get the final value of the animation.
    #[allow(dead_code)]
    pub fn final_value(&self) -> f64 {
        match self {
            Animation::BounceIn(tween) => tween.borrow().final_value(),
            Animation::SineIn(tween) => tween.borrow().final_value(),
            Animation::SineInOut(tween) => tween.borrow().final_value(),
            Animation::Linear(tween) => tween.borrow().final_value(),
        }
    }

    /// Move the animation to the given index.
    pub fn move_value(&mut self, index: i32) -> f64 {
        match self {
            Animation::BounceIn(ref mut tween) => {
                let tween = tween.get_mut();
                tween.move_to(index)
            }
            Animation::SineIn(ref mut tween) => {
                let tween = tween.get_mut();
                tween.move_to(index)
            }
            Animation::SineInOut(ref mut tween) => {
                let tween = tween.get_mut();
                tween.move_to(index)
            }
            Animation::Linear(ref mut tween) => {
                let tween = tween.get_mut();
                tween.move_to(index)
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_finished(&self) -> bool {
        match self {
            Animation::BounceIn(tween) => tween.borrow().is_finished(),
            Animation::SineIn(tween) => tween.borrow().is_finished(),
            Animation::SineInOut(tween) => tween.borrow().is_finished(),
            Animation::Linear(tween) => tween.borrow().is_finished(),
        }
    }
}
