use std::{
    fmt,
    time::Duration,
};

use easer::functions::*;

use super::AnimDirection;

pub trait AnimatedValue: Clone + 'static {
    fn prepare(&mut self, direction: AnimDirection);

    fn is_finished(&self, index: u128, direction: AnimDirection) -> bool;

    fn advance(&mut self, index: u128, direction: AnimDirection);

    fn finish(&mut self, direction: AnimDirection);
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Ease {
    In,
    #[default]
    Out,
    InOut,
}

pub fn apply_value(
    origin: f32,
    destination: f32,
    index: u128,
    time: Duration,
    ease: Ease,
    function: Function,
) -> f32 {
    let (t, b, c, d) = (
        index as f32,
        origin,
        destination - origin,
        time.as_millis() as f32,
    );
    match function {
        Function::Back => match ease {
            Ease::In => Back::ease_in(t, b, c, d),
            Ease::InOut => Back::ease_in_out(t, b, c, d),
            Ease::Out => Back::ease_out(t, b, c, d),
        },
        Function::Bounce => match ease {
            Ease::In => Bounce::ease_in(t, b, c, d),
            Ease::InOut => Bounce::ease_in_out(t, b, c, d),
            Ease::Out => Bounce::ease_out(t, b, c, d),
        },
        Function::Circ => match ease {
            Ease::In => Circ::ease_in(t, b, c, d),
            Ease::InOut => Circ::ease_in_out(t, b, c, d),
            Ease::Out => Circ::ease_out(t, b, c, d),
        },
        Function::Cubic => match ease {
            Ease::In => Cubic::ease_in(t, b, c, d),
            Ease::InOut => Cubic::ease_in_out(t, b, c, d),
            Ease::Out => Cubic::ease_out(t, b, c, d),
        },
        Function::Elastic => match ease {
            Ease::In => Elastic::ease_in(t, b, c, d),
            Ease::InOut => Elastic::ease_in_out(t, b, c, d),
            Ease::Out => Elastic::ease_out(t, b, c, d),
        },
        Function::Expo => match ease {
            Ease::In => Expo::ease_in(t, b, c, d),
            Ease::InOut => Expo::ease_in_out(t, b, c, d),
            Ease::Out => Expo::ease_out(t, b, c, d),
        },
        Function::Linear => match ease {
            Ease::In => Linear::ease_in(t, b, c, d),
            Ease::InOut => Linear::ease_in_out(t, b, c, d),
            Ease::Out => Linear::ease_out(t, b, c, d),
        },
        Function::Quad => match ease {
            Ease::In => Quad::ease_in(t, b, c, d),
            Ease::InOut => Quad::ease_in_out(t, b, c, d),
            Ease::Out => Quad::ease_out(t, b, c, d),
        },
        Function::Quart => match ease {
            Ease::In => Quart::ease_in(t, b, c, d),
            Ease::InOut => Quart::ease_in_out(t, b, c, d),
            Ease::Out => Quart::ease_out(t, b, c, d),
        },
        Function::Sine => match ease {
            Ease::In => Sine::ease_in(t, b, c, d),
            Ease::InOut => Sine::ease_in_out(t, b, c, d),
            Ease::Out => Sine::ease_out(t, b, c, d),
        },
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Function {
    Back,
    Bounce,
    Circ,
    Cubic,
    Elastic,
    Expo,
    #[default]
    Linear,
    Quad,
    Quart,
    Sine,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
