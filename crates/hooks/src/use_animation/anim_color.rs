use std::time::Duration;

use freya_core::parsing::Parse;
use freya_engine::prelude::Color;

use super::{
    apply_value,
    AnimDirection,
    AnimatedValue,
    Ease,
    Function,
};

/// Animate a color.
#[derive(Clone, PartialEq)]
pub struct AnimColor {
    origin: Color,
    destination: Color,
    time: Duration,
    ease: Ease,
    function: Function,

    value: Color,
}

impl AnimColor {
    pub fn new(origin: &str, destination: &str) -> Self {
        Self {
            origin: Color::parse(origin).unwrap(),
            destination: Color::parse(destination).unwrap(),
            time: Duration::default(),
            ease: Ease::default(),
            function: Function::default(),

            value: Color::parse(origin).unwrap(),
        }
    }

    /// Reverses the `origin` and the `destination` of the [AnimColor].
    pub fn into_reversed(self) -> Self {
        Self {
            origin: self.destination,
            destination: self.origin,
            ..self
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

    /// Read the value of the [AnimColor] as a String.
    pub fn value(&self) -> String {
        format!(
            "rgb({}, {}, {}, {})",
            self.value.r(),
            self.value.g(),
            self.value.b(),
            self.value.a()
        )
    }
}

impl From<&AnimColor> for String {
    fn from(value: &AnimColor) -> Self {
        value.value()
    }
}

impl From<AnimColor> for String {
    fn from(value: AnimColor) -> Self {
        value.value()
    }
}

impl std::fmt::Display for AnimColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value())
    }
}

impl AnimatedValue for AnimColor {
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
                index >= self.time.as_millis()
                    && self.value.r() == self.destination.r()
                    && self.value.g() == self.destination.g()
                    && self.value.b() == self.destination.b()
                    && self.value.a() == self.destination.a()
            }
            AnimDirection::Reverse => {
                index >= self.time.as_millis()
                    && self.value.r() == self.origin.r()
                    && self.value.g() == self.origin.g()
                    && self.value.b() == self.origin.b()
                    && self.value.a() == self.origin.a()
            }
        }
    }

    fn advance(&mut self, index: u128, direction: AnimDirection) {
        let (origin, destination) = match direction {
            AnimDirection::Forward => (self.origin, self.destination),
            AnimDirection::Reverse => (self.destination, self.origin),
        };
        let r = apply_value(
            origin.r() as f32,
            destination.r() as f32,
            index.min(self.time.as_millis()),
            self.time,
            self.ease,
            self.function,
        );
        let g = apply_value(
            origin.g() as f32,
            destination.g() as f32,
            index.min(self.time.as_millis()),
            self.time,
            self.ease,
            self.function,
        );
        let b = apply_value(
            origin.b() as f32,
            destination.b() as f32,
            index.min(self.time.as_millis()),
            self.time,
            self.ease,
            self.function,
        );
        let a = apply_value(
            origin.a() as f32,
            destination.a() as f32,
            index.min(self.time.as_millis()),
            self.time,
            self.ease,
            self.function,
        );
        self.value = Color::from_argb(a as u8, r as u8, g as u8, b as u8);
    }

    fn finish(&mut self, direction: AnimDirection) {
        self.advance(self.time.as_millis(), direction);
    }
}
