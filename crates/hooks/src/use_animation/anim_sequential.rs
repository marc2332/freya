use std::ops::Deref;

use super::{
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

impl<Animated: AnimatedValue, const N: usize> AnimSequential<Animated, N> {
    pub fn new(values: [Animated; N]) -> Self {
        Self {
            values,
            curr_value: 0,
            acc_index: 0,
        }
    }

    pub fn values(&self) -> &[Animated; N] {
        &self.values
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
        if let Some(value) = self.values.get_mut(self.curr_value) {
            let index = index - self.acc_index;
            value.advance(index, direction);

            if value.is_finished(index, direction) {
                self.curr_value += 1;
                self.acc_index += index;
            }
        }
    }

    fn is_finished(&self, index: u128, direction: AnimDirection) -> bool {
        if let Some(value) = self.values.get(self.curr_value) {
            value.is_finished(index, direction)
        } else {
            true
        }
    }

    fn prepare(&mut self, direction: AnimDirection) {
        self.acc_index = 0;
        self.curr_value = 0;
        for val in &mut self.values {
            val.prepare(direction);
        }
    }

    fn finish(&mut self, direction: AnimDirection) {
        for value in &mut self.values {
            value.finish(direction);
        }
    }
}
