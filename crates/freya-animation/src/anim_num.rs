use std::time::Duration;

use crate::{
    easing::{
        Function,
        apply_value,
    },
    hook::{
        AnimDirection,
        AnimatedValue,
        Ease,
        ReadAnimatedValue,
    },
};

/// Animate a numeric value.
#[derive(Clone, PartialEq, Default)]
pub struct AnimNum {
    origin: f32,
    destination: f32,
    time: Duration,
    ease: Ease,
    function: Function,

    value: f32,
}

impl AnimNum {
    pub fn new(origin: f32, destination: f32) -> Self {
        Self {
            origin,
            destination,
            time: Duration::default(),
            ease: Ease::default(),
            function: Function::default(),

            value: origin,
        }
    }

    /// Set the animation duration using milliseconds. Use `Self::duration` if you want to specify the duration in another form.
    pub fn time(mut self, time: u64) -> Self {
        self.time = Duration::from_millis(time);
        self
    }

    /// Set the animation duration using milliseconds.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.time = duration;
        self
    }

    /// Set the easing type. See `Ease` for all the types.
    pub fn ease(mut self, ease: Ease) -> Self {
        self.ease = ease;
        self
    }

    /// Set the easing function. See `Function` for all the types.
    pub fn function(mut self, function: Function) -> Self {
        self.function = function;
        self
    }

    /// Read the value of the [AnimNum] as a f32.
    pub fn value(&self) -> f32 {
        self.value
    }
}

impl From<&AnimNum> for f32 {
    fn from(value: &AnimNum) -> Self {
        value.value()
    }
}

impl From<AnimNum> for f32 {
    fn from(value: AnimNum) -> Self {
        value.value()
    }
}

impl AnimatedValue for AnimNum {
    fn prepare(&mut self, direction: AnimDirection) {
        match direction {
            AnimDirection::Forward => self.value = self.origin,
            AnimDirection::Reverse => {
                self.value = self.destination;
            }
        }
    }

    fn is_finished(&self, index: u128, direction: AnimDirection) -> bool {
        match direction {
            AnimDirection::Forward => {
                index >= self.time.as_millis() && self.value == self.destination
            }
            AnimDirection::Reverse => index >= self.time.as_millis() && self.value == self.origin,
        }
    }

    fn advance(&mut self, index: u128, direction: AnimDirection) {
        let (origin, destination) = match direction {
            AnimDirection::Forward => (self.origin, self.destination),
            AnimDirection::Reverse => (self.destination, self.origin),
        };
        self.value = apply_value(
            origin,
            destination,
            index.min(self.time.as_millis()),
            self.time,
            self.ease,
            self.function,
        );
    }

    fn finish(&mut self, direction: AnimDirection) {
        self.advance(self.time.as_millis(), direction);
    }

    /// Reverses the `origin` and the `destination` of the [AnimNum].
    fn into_reversed(self) -> AnimNum {
        Self {
            origin: self.destination,
            destination: self.origin,
            ..self
        }
    }
}

impl ReadAnimatedValue for AnimNum {
    type Output = f32;
    fn value(&self) -> Self::Output {
        self.value()
    }
}
