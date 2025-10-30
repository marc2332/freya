use std::ops::Deref;

use crate::hook::{
    AnimDirection,
    AnimatedValue,
};

/// Chain a sequence of animated values.
#[derive(Clone)]
pub struct AnimSequential<Animated: AnimatedValue, const N: usize> {
    values: [Animated; N],
    curr_value: usize,
    acc_index: u128,
}

impl<Animated: AnimatedValue + Default, const N: usize> Default for AnimSequential<Animated, N> {
    fn default() -> Self {
        Self {
            values: std::array::from_fn(|_| Animated::default()),
            curr_value: 0,
            acc_index: 0,
        }
    }
}

impl<Animated: AnimatedValue, const N: usize> AnimSequential<Animated, N> {
    pub fn new(values: [Animated; N]) -> Self {
        Self {
            values,
            curr_value: 0,
            acc_index: 0,
        }
    }
}

impl<Animated: AnimatedValue, const N: usize> Deref for AnimSequential<Animated, N> {
    type Target = [Animated; N];

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<Animated: AnimatedValue, const N: usize> AnimatedValue for AnimSequential<Animated, N> {
    fn advance(&mut self, index: u128, direction: AnimDirection) {
        let value = if direction == AnimDirection::Forward {
            self.values.get_mut(self.curr_value)
        } else {
            self.values.iter_mut().rev().nth(self.curr_value)
        };
        if let Some(value) = value {
            let index = index - self.acc_index;
            value.advance(index, direction);

            if value.is_finished(index, direction) {
                self.curr_value += 1;
                self.acc_index += index;
            }
        }
    }

    fn is_finished(&self, index: u128, direction: AnimDirection) -> bool {
        let value = if direction == AnimDirection::Forward {
            self.values.get(self.curr_value)
        } else {
            self.values.iter().rev().nth(self.curr_value)
        };
        if let Some(value) = value {
            value.is_finished(index, direction)
        } else {
            true
        }
    }

    fn prepare(&mut self, direction: AnimDirection) {
        self.acc_index = 0;
        self.curr_value = 0;
        match direction {
            AnimDirection::Forward => {
                for val in self.values.iter_mut() {
                    val.prepare(direction);
                }
            }
            AnimDirection::Reverse => {
                for val in self.values.iter_mut().rev() {
                    val.prepare(direction);
                }
            }
        }
    }

    fn finish(&mut self, direction: AnimDirection) {
        match direction {
            AnimDirection::Forward => {
                for value in &mut self.values {
                    value.finish(direction);
                }
            }
            AnimDirection::Reverse => {
                for value in &mut self.values {
                    value.finish(direction);
                }
            }
        }
    }

    fn into_reversed(self) -> Self {
        let mut values: [Animated; N] = self.values.map(|v| v.into_reversed());

        values.reverse();

        Self {
            values,
            curr_value: 0,
            acc_index: 0,
        }
    }
}
