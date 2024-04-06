use std::ops::Deref;

pub use euclid::Rect;

use crate::scaled::Scaled;
use crate::{geometry::Length, measure::Phase};

#[derive(PartialEq, Clone, Debug)]
pub enum Size {
    Inner,
    Fill,
    FillMinimum,
    Percentage(Length),
    Pixels(Length),
    RootPercentage(Length),
    DynamicCalculations(Box<Vec<DynamicCalculation>>),
}

impl Default for Size {
    fn default() -> Self {
        Self::Inner
    }
}

impl Size {
    pub fn inner_sized(&self) -> bool {
        matches!(self, Self::Inner | Self::FillMinimum)
    }

    pub fn pretty(&self) -> String {
        match self {
            Size::Inner => "auto".to_string(),
            Size::Pixels(s) => format!("{}", s.get()),
            Size::DynamicCalculations(calcs) => format!(
                "calc({})",
                calcs
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Size::Percentage(p) => format!("{}%", p.get()),
            Size::Fill => "fill".to_string(),
            Size::FillMinimum => "fill-min".to_string(),
            Size::RootPercentage(p) => format!("{}% of root", p.get()),
        }
    }

    pub fn eval(
        &self,
        parent_value: f32,
        available_parent_value: f32,
        parent_margin: f32,
        root_value: f32,
        phase: Phase,
    ) -> Option<f32> {
        match self {
            Size::Pixels(px) => Some(px.get() + parent_margin),
            Size::Percentage(per) => Some(parent_value / 100.0 * per.get()),
            Size::DynamicCalculations(calculations) => {
                Some(run_calculations(calculations.deref(), parent_value))
            }
            Size::Fill => Some(available_parent_value),
            Size::FillMinimum => {
                if phase == Phase::Initial {
                    None
                } else {
                    Some(available_parent_value)
                }
            }
            Size::RootPercentage(per) => Some(root_value / 100.0 * per.get()),
            _ => None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn min_max(
        &self,
        value: f32,
        parent_value: f32,
        available_parent_value: f32,
        single_margin: f32,
        margin: f32,
        minimum: &Self,
        maximum: &Self,
        root_value: f32,
        phase: Phase,
    ) -> f32 {
        let value = self
            .eval(
                parent_value,
                available_parent_value,
                margin,
                root_value,
                phase,
            )
            .unwrap_or(value + margin);

        let minimum_value = minimum
            .eval(
                parent_value,
                available_parent_value,
                margin,
                root_value,
                phase,
            )
            .map(|v| v + single_margin);
        let maximum_value = maximum.eval(
            parent_value,
            available_parent_value,
            margin,
            root_value,
            phase,
        );

        let mut final_value = value;

        if let Some(minimum_value) = minimum_value {
            if minimum_value > final_value {
                final_value = minimum_value;
            }
        }

        if let Some(maximum_value) = maximum_value {
            if final_value > maximum_value {
                final_value = maximum_value
            }
        }

        final_value
    }

    pub fn most_fitting_size<'a>(&self, size: &'a f32, available_size: &'a f32) -> &'a f32 {
        match self {
            Self::Inner => available_size,
            _ => size,
        }
    }
}

impl Scaled for Size {
    fn scale(&mut self, scale_factor: f32) {
        match self {
            Size::Pixels(s) => *s *= scale_factor,
            Size::DynamicCalculations(calcs) => {
                calcs.iter_mut().for_each(|calc| calc.scale(scale_factor));
            }
            _ => (),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DynamicCalculation {
    Sub,
    Mul,
    Div,
    Add,
    Percentage(f32),
    Pixels(f32),
}

impl Scaled for DynamicCalculation {
    fn scale(&mut self, scale_factor: f32) {
        if let DynamicCalculation::Pixels(s) = self {
            *s *= scale_factor;
        }
    }
}

impl std::fmt::Display for DynamicCalculation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynamicCalculation::Sub => f.write_str("-"),
            DynamicCalculation::Mul => f.write_str("*"),
            DynamicCalculation::Div => f.write_str("/"),
            DynamicCalculation::Add => f.write_str("+"),
            DynamicCalculation::Percentage(p) => f.write_fmt(format_args!("{p}%")),
            DynamicCalculation::Pixels(s) => f.write_fmt(format_args!("{s}")),
        }
    }
}

/// Calculate some chained operations with a given value.
/// This value could be for example the width of a node's parent area.
pub fn run_calculations(calcs: &[DynamicCalculation], value: f32) -> f32 {
    let mut prev_number: Option<f32> = None;
    let mut prev_op: Option<DynamicCalculation> = None;

    let mut calc_with_op = |val: f32, prev_op: Option<DynamicCalculation>| {
        if let Some(op) = prev_op {
            match op {
                DynamicCalculation::Sub => {
                    prev_number = Some(prev_number.unwrap() - val);
                }
                DynamicCalculation::Add => {
                    prev_number = Some(prev_number.unwrap() + val);
                }
                DynamicCalculation::Mul => {
                    prev_number = Some(prev_number.unwrap() * val);
                }
                DynamicCalculation::Div => {
                    prev_number = Some(prev_number.unwrap() / val);
                }
                _ => {}
            }
        } else {
            prev_number = Some(val);
        }
    };

    for calc in calcs {
        match calc {
            DynamicCalculation::Percentage(per) => {
                let val = (value / 100.0 * per).round();

                calc_with_op(val, prev_op);

                prev_op = None;
            }
            DynamicCalculation::Pixels(val) => {
                calc_with_op(*val, prev_op);
                prev_op = None;
            }
            _ => prev_op = Some(*calc),
        }
    }

    prev_number.unwrap()
}
