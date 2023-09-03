pub use euclid::Rect;

use crate::geometry::Length;
use crate::scaled::Scaled;

#[derive(PartialEq, Clone, Debug)]
pub enum Size {
    Inner,
    Percentage(Length),
    Pixels(Length),
    DynamicCalculations(Vec<DynamicCalculation>),
}

impl Default for Size {
    fn default() -> Self {
        Self::Inner
    }
}

impl Size {
    pub fn pretty(&self) -> String {
        match self {
            Size::Inner => "inner".to_string(),
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
        }
    }

    pub fn eval(&self, parent_value: f32) -> Option<f32> {
        match self {
            Size::Pixels(px) => Some(px.get()),
            Size::Percentage(per) => Some(parent_value / 100.0 * per.get()),
            Size::DynamicCalculations(calculations) => {
                Some(run_calculations(calculations, parent_value))
            }
            _ => None,
        }
    }

    pub fn min_max(
        &self,
        value: f32,
        parent_value: f32,
        margin: f32,
        minimum: &Self,
        maximum: &Self,
    ) -> f32 {
        let value = self.eval(parent_value).unwrap_or(value) + margin;

        let minimum_value = minimum.eval(parent_value);
        let maximum_value = maximum.eval(parent_value);

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
