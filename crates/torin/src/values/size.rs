use std::{
    ops::{
        AddAssign,
        DivAssign,
        Mul,
        MulAssign,
        SubAssign,
    },
    slice::Iter,
};

pub use euclid::Rect;

use crate::{
    geometry::Length,
    measure::Phase,
    scaled::Scaled,
};

#[derive(PartialEq, Clone, Debug)]
pub enum Size {
    Inner,
    Fill,
    FillMinimum,
    Percentage(Length),
    Pixels(Length),
    RootPercentage(Length),
    DynamicCalculations(f32, Box<Vec<DynamicCalculation>>),
    Flex(Length),
}

impl Default for Size {
    fn default() -> Self {
        Self::Inner
    }
}

impl Size {
    pub fn flex_grow(&self) -> Option<Length> {
        match self {
            Self::Flex(f) => Some(*f),
            _ => None,
        }
    }

    pub fn is_flex(&self) -> bool {
        matches!(self, Self::Flex(_))
    }

    pub fn inner_sized(&self) -> bool {
        matches!(self, Self::Inner)
    }

    pub fn pretty(&self) -> String {
        match self {
            Self::Inner => "auto".to_string(),
            Self::Pixels(s) => format!("{}", s.get()),
            Self::DynamicCalculations(scaling_factor, calcs) => format!(
                "calc({}) with scale {scaling_factor}",
                calcs
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Self::Percentage(p) => format!("{}%", p.get()),
            Self::Fill => "fill".to_string(),
            Self::FillMinimum => "fill-min".to_string(),
            Self::RootPercentage(p) => format!("{}% of root", p.get()),
            Self::Flex(f) => format!("flex({})", f.get()),
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
            Self::Pixels(px) => Some(px.get() + parent_margin),
            Self::Percentage(per) => Some(parent_value / 100.0 * per.get()),
            Self::DynamicCalculations(scaling_factor, calculations) => Some(
                run_calculations(calculations, *scaling_factor, parent_value, root_value)
                    .unwrap_or(0.0),
            ),
            Self::Fill => Some(available_parent_value),
            Self::RootPercentage(per) => Some(root_value / 100.0 * per.get()),
            Self::Flex(_) | Self::FillMinimum if phase == Phase::Final => {
                Some(available_parent_value)
            }
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
                final_value = maximum_value;
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
            Self::Pixels(s) => *s *= scale_factor,
            Self::DynamicCalculations(scaling_factor, _) => {
                *scaling_factor = scale_factor;
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
    OpenParenthesis,
    ClosedParenthesis,
    ScalingFactor(f32),
    Percentage(f32),
    RootPercentage(f32),
    Pixels(f32),
}

impl Scaled for DynamicCalculation {
    fn scale(&mut self, scale_factor: f32) {
        if let Self::Pixels(s) = self {
            *s *= scale_factor;
        }
    }
}

impl std::fmt::Display for DynamicCalculation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sub => f.write_str("-"),
            Self::Mul => f.write_str("*"),
            Self::Div => f.write_str("/"),
            Self::Add => f.write_str("+"),
            Self::OpenParenthesis => f.write_str("("),
            Self::ClosedParenthesis => f.write_str(")"),
            Self::Percentage(p) => f.write_fmt(format_args!("{p}%")),
            Self::RootPercentage(p) => f.write_fmt(format_args!("{p}v")),
            Self::Pixels(s) => f.write_fmt(format_args!("{s}")),
            Self::ScalingFactor(s) => f.write_fmt(format_args!("scaling_factor({s})")),
        }
    }
}

/// [Operator-precedence parser](https://en.wikipedia.org/wiki/Operator-precedence_parser#Precedence_climbing_method)
struct DynamicCalculationEvaluator<'a> {
    calcs: Iter<'a, DynamicCalculation>,
    parent_value: f32,
    scaling_factor: f32,
    root_value: f32,
    current: Option<&'a DynamicCalculation>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// the enum representing the type of value we use for calculations. It should only be created as the Scaled or the
/// UnScaled variant, the PartiallyScaled variant is created when doing an operation on two values of different types.
enum Val {
    /// represents pixels that will be scaled, eg. have itself multiplied by the scaling factor.
    Scaled(f32),
    /// represents pixels that will not be scaled.
    UnScaled(f32),
    /// represents a combination of scaled and unscaled pixels. It can be converted into the a number by scaling the scaled part and then adding that to the unscaled part.
    /// This is needed in cases like `calc(50% + 10)` where when you add `50%` and `10` you want to be able to represent the values in a way where the pixels that represent
    /// the `50%` arent scaled and the ones representing the `10` are.
    PartiallyScaled { scaled: f32, unscaled: f32 },
}

// below are the implementations of the operations used in the evaluation logic on the `Val` type.
// they define the rules for how a `Val` with a `Val` should interract.

impl SubAssign for Val {
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Val::Scaled(v) => match rhs {
                Val::Scaled(r) => v.sub_assign(r),
                Val::UnScaled(r) => {
                    *self = Self::PartiallyScaled {
                        scaled: *v,
                        unscaled: -r,
                    }
                }
                Val::PartiallyScaled { scaled, unscaled } => {
                    *self = Self::PartiallyScaled {
                        scaled: *v - scaled,
                        unscaled: -unscaled,
                    }
                }
            },
            Val::UnScaled(v) => match rhs {
                Val::Scaled(r) => {
                    *self = Self::PartiallyScaled {
                        scaled: -r,
                        unscaled: *v,
                    }
                }
                Val::UnScaled(r) => v.sub_assign(r),
                Val::PartiallyScaled { scaled, unscaled } => {
                    *self = Self::PartiallyScaled {
                        scaled: -scaled,
                        unscaled: *v - unscaled,
                    }
                }
            },
            Val::PartiallyScaled { scaled, unscaled } => match rhs {
                Val::Scaled(r) => {
                    *self = Self::PartiallyScaled {
                        scaled: *scaled - r,
                        unscaled: *unscaled,
                    }
                }
                Val::UnScaled(r) => {
                    *self = Self::PartiallyScaled {
                        scaled: *scaled,
                        unscaled: *unscaled - r,
                    }
                }
                Val::PartiallyScaled {
                    scaled: scaled_r,
                    unscaled: unscaled_r,
                } => {
                    scaled.sub_assign(scaled_r);
                    unscaled.sub_assign(unscaled_r);
                }
            },
        }
    }
}

impl AddAssign for Val {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Val::Scaled(v) => match rhs {
                Val::Scaled(r) => v.add_assign(r),
                Val::UnScaled(r) => {
                    *self = Self::PartiallyScaled {
                        scaled: *v,
                        unscaled: r,
                    }
                }
                Val::PartiallyScaled { scaled, unscaled } => {
                    *self = Self::PartiallyScaled {
                        scaled: *v + scaled,
                        unscaled,
                    }
                }
            },
            Val::UnScaled(v) => match rhs {
                Val::Scaled(r) => {
                    *self = Self::PartiallyScaled {
                        scaled: r,
                        unscaled: *v,
                    }
                }
                Val::UnScaled(r) => v.add_assign(r),
                Val::PartiallyScaled { scaled, unscaled } => {
                    *self = Self::PartiallyScaled {
                        scaled,
                        unscaled: *v + unscaled,
                    }
                }
            },
            Val::PartiallyScaled { scaled, unscaled } => match rhs {
                Val::Scaled(r) => {
                    *self = Self::PartiallyScaled {
                        scaled: r + *scaled,
                        unscaled: *unscaled,
                    }
                }
                Val::UnScaled(r) => {
                    *self = Self::PartiallyScaled {
                        scaled: *scaled,
                        unscaled: r + *unscaled,
                    }
                }
                Val::PartiallyScaled {
                    scaled: scaled_r,
                    unscaled: unscaled_r,
                } => {
                    scaled.add_assign(scaled_r);
                    unscaled.add_assign(unscaled_r);
                }
            },
        }
    }
}

impl MulAssign for Val {
    fn mul_assign(&mut self, rhs: Self) {
        match self {
            Val::Scaled(v) => match rhs {
                Val::Scaled(r) => v.mul_assign(r),
                Val::UnScaled(r) => {
                    *self = Self::UnScaled(*v * r);
                }
                Val::PartiallyScaled { scaled, unscaled } => {
                    *self = Self::PartiallyScaled {
                        scaled: *v * scaled,
                        unscaled: *v * unscaled,
                    }
                }
            },
            Val::UnScaled(v) => match rhs {
                Val::Scaled(r) => {
                    *self = Self::UnScaled(*v * r);
                }
                Val::UnScaled(r) => v.mul_assign(r),
                Val::PartiallyScaled { scaled, unscaled } => {
                    *self = Self::PartiallyScaled {
                        scaled: *v * scaled,
                        unscaled: *v * unscaled,
                    }
                }
            },
            Val::PartiallyScaled { scaled, unscaled } => match rhs {
                Val::UnScaled(r) | Val::Scaled(r) => {
                    *self = Self::PartiallyScaled {
                        scaled: *scaled * r,
                        unscaled: *unscaled * r,
                    }
                }
                Val::PartiallyScaled {
                    scaled: scaled_r,
                    unscaled: unscaled_r,
                } => {
                    *self = Self::UnScaled((*scaled + *unscaled) * (scaled_r + unscaled_r));
                }
            },
        }
    }
}

impl DivAssign for Val {
    fn div_assign(&mut self, rhs: Self) {
        match self {
            Val::Scaled(v) => match rhs {
                Val::Scaled(r) => v.div_assign(r),
                Val::UnScaled(r) => {
                    *self = Self::UnScaled(*v / r);
                }
                Val::PartiallyScaled { scaled, unscaled } => {
                    *self = Self::PartiallyScaled {
                        scaled: *v / scaled,
                        unscaled: *v / unscaled,
                    }
                }
            },
            Val::UnScaled(v) => match rhs {
                Val::Scaled(r) => {
                    *self = Self::UnScaled(*v / r);
                }
                Val::UnScaled(r) => v.div_assign(r),
                Val::PartiallyScaled { scaled, unscaled } => {
                    *self = Self::PartiallyScaled {
                        scaled: *v / scaled,
                        unscaled: *v / unscaled,
                    }
                }
            },
            Val::PartiallyScaled { scaled, unscaled } => match rhs {
                Val::UnScaled(r) | Val::Scaled(r) => {
                    *self = Self::PartiallyScaled {
                        scaled: *scaled / r,
                        unscaled: *unscaled / r,
                    }
                }
                Val::PartiallyScaled {
                    scaled: scaled_r,
                    unscaled: unscaled_r,
                } => {
                    *self = Self::UnScaled((*scaled + *unscaled) / (scaled_r + unscaled_r));
                }
            },
        }
    }
}

impl Mul for Val {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Val::Scaled(v) => match rhs {
                Val::Scaled(r) => Val::Scaled(v * r),
                Val::UnScaled(r) => Val::UnScaled(v * r),
                Val::PartiallyScaled { scaled, unscaled } => Val::PartiallyScaled {
                    scaled: v * scaled,
                    unscaled: v * unscaled,
                },
            },
            Val::UnScaled(v) => match rhs {
                Val::UnScaled(r) | Val::Scaled(r) => Val::UnScaled(v * r),
                Val::PartiallyScaled { scaled, unscaled } => Val::PartiallyScaled {
                    scaled: v * scaled,
                    unscaled: v * unscaled,
                },
            },
            Val::PartiallyScaled { scaled, unscaled } => match rhs {
                Val::UnScaled(r) | Val::Scaled(r) => Val::PartiallyScaled {
                    scaled: scaled * r,
                    unscaled: unscaled * r,
                },
                Val::PartiallyScaled {
                    scaled: scaled_r,
                    unscaled: unscaled_r,
                } => Val::UnScaled((scaled + unscaled) * (scaled_r + unscaled_r)),
            },
        }
    }
}

impl<'a> DynamicCalculationEvaluator<'a> {
    pub fn new(
        calcs: Iter<'a, DynamicCalculation>,
        parent_value: f32,
        root_value: f32,
        scaling_factor: f32,
    ) -> Self {
        Self {
            calcs,
            scaling_factor,
            parent_value,
            root_value,
            current: None,
        }
    }

    pub fn evaluate(&mut self) -> Option<f32> {
        let value = self.parse_expression(0);

        let value = value.map(|v| match v {
            Val::Scaled(v) => v * self.scaling_factor,
            Val::UnScaled(v) => v,
            Val::PartiallyScaled { scaled, unscaled } => scaled * self.scaling_factor + unscaled,
        });

        // Return the result if there are no more tokens
        match self.current {
            Some(_) => None,
            None => value,
        }
    }

    /// Parse and evaluate the expression with operator precedence and following grammar:
    /// ```ebnf
    ///     expression = value, { operator, value } ;
    ///     operator   = "+" | "-" | "*" | "/" ;
    /// ```
    fn parse_expression(&mut self, min_precedence: usize) -> Option<Val> {
        // Parse left-hand side value
        self.current = self.calcs.next();
        let mut lhs = self.parse_term()?;

        while let Some(operator_precedence) = self.operator_precedence() {
            // Return if minimal precedence is reached.
            if operator_precedence < min_precedence {
                return Some(lhs);
            }

            // Save operator to apply after parsing right-hand side value.
            let operator = self.current?;

            // Parse right-hand side value.
            //
            // Next precedence is the current precedence + 1
            // because all operators are left associative.
            let rhs = self.parse_expression(operator_precedence + 1)?;

            // Apply operator
            match operator {
                DynamicCalculation::Add => lhs += rhs,
                DynamicCalculation::Sub => lhs -= rhs,
                DynamicCalculation::Mul => lhs *= rhs,
                DynamicCalculation::Div => lhs /= rhs,
                // Precedence will return None for other tokens
                // and loop will break if it's not an operator
                _ => unreachable!(),
            }
        }

        Some(lhs)
    }

    /// Parse and evaluate the term, implements implicit multiplication. only parenthesis count as
    /// a seperator, so syntax like 50 50 isnt correct, but 50(50) is because the parenthesis act
    /// as a seperator
    fn parse_term(&mut self) -> Option<Val> {
        let prefix = self.parse_prefix()?;
        let mut lhs = None;
        // set to true so that the first value is multiplied and counts as normal syntax
        let mut last_is_separator = true;

        while let Some((rhs, seperator)) = self.parse_value() {
            if last_is_separator || seperator {
                lhs = Some(lhs.unwrap_or(Val::Scaled(1.0)) * rhs);
            } else {
                return None;
            }
            last_is_separator = seperator;
        }
        prefix.map_or(lhs, |prefix| match prefix {
            DynamicCalculation::Add => lhs,
            DynamicCalculation::Sub => lhs.map(|v| v * Val::Scaled(-1.0)),
            _ => unreachable!("make sure to add the prefix here"),
        })
    }
    /// parse and evaluate the value with the following grammar:
    /// ```ebnf
    ///     value      = percentage | pixels ;
    ///     percentage = number, "%" ;
    ///     pixels     = number ;
    /// `
    fn parse_value(&mut self) -> Option<(Val, bool)> {
        match self.current? {
            DynamicCalculation::Percentage(value) => {
                self.current = self.calcs.next();
                Some((Val::UnScaled(value / 100.0 * self.parent_value), false))
            }
            DynamicCalculation::RootPercentage(value) => {
                self.current = self.calcs.next();
                Some((Val::UnScaled(value / 100.0 * self.root_value), false))
            }
            DynamicCalculation::Pixels(value) => {
                self.current = self.calcs.next();
                Some((Val::Scaled(*value), false))
            }
            DynamicCalculation::OpenParenthesis => {
                // function should return on DynamicCalculation::ClosedParenthesis because it does
                // not have a precedence, thats how it actually works
                let val = self.parse_expression(0);
                if self.current != Some(&DynamicCalculation::ClosedParenthesis) {
                    return None;
                }
                self.current = self.calcs.next();
                Some((val?, true))
            }
            _ => None,
        }
    }

    /// parses out the prefix, like a + or -
    fn parse_prefix(&mut self) -> Option<Option<DynamicCalculation>> {
        match self.current? {
            DynamicCalculation::Add => {
                self.current = self.calcs.next();
                Some(Some(DynamicCalculation::Add))
            }
            DynamicCalculation::Sub => {
                self.current = self.calcs.next();
                Some(Some(DynamicCalculation::Sub))
            }
            _ => Some(None),
        }
    }

    /// Get the precedence of the operator if current token is an operator or None otherwise.
    fn operator_precedence(&self) -> Option<usize> {
        match self.current? {
            DynamicCalculation::Add | DynamicCalculation::Sub => Some(1),
            DynamicCalculation::Mul | DynamicCalculation::Div => Some(2),
            DynamicCalculation::OpenParenthesis => Some(0),
            _ => None,
        }
    }
}

/// Calculate dynamic expression with operator precedence.
/// This value could be for example the width of a node's parent area.
pub fn run_calculations(
    calcs: &[DynamicCalculation],
    scaling_factor: f32,
    parent_value: f32,
    root_value: f32,
) -> Option<f32> {
    DynamicCalculationEvaluator::new(calcs.iter(), parent_value, root_value, scaling_factor)
        .evaluate()
}
