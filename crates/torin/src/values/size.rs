use std::{
    ops::Deref,
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
    InnerPercentage(Length),
    DynamicCalculations(Box<Vec<DynamicCalculation>>),
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
        matches!(
            self,
            Self::Inner | Self::FillMinimum | Self::InnerPercentage(_)
        )
    }

    pub fn inner_percentage_sized(&self) -> bool {
        matches!(self, Self::InnerPercentage(_))
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
            Size::InnerPercentage(p) => format!("{}% of auto", p.get()),
            Size::Flex(f) => format!("flex({}", f.get()),
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
            Size::DynamicCalculations(calculations) => Some(
                run_calculations(calculations.deref(), parent_value, root_value).unwrap_or(0.0),
            ),
            Size::Fill => Some(available_parent_value),
            Size::FillMinimum if phase == Phase::Final => Some(available_parent_value),
            Size::RootPercentage(per) => Some(root_value / 100.0 * per.get()),
            Size::Flex(_) if phase == Phase::Final => Some(available_parent_value),
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
            Self::Inner | Self::InnerPercentage(_) => available_size,
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
    OpenParenthesis,
    ClosedParenthesis,
    Percentage(f32),
    RootPercentage(f32),
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
            DynamicCalculation::OpenParenthesis => f.write_str("("),
            DynamicCalculation::ClosedParenthesis => f.write_str(")"),
            DynamicCalculation::Percentage(p) => f.write_fmt(format_args!("{p}%")),
            DynamicCalculation::RootPercentage(p) => f.write_fmt(format_args!("{p}v")),
            DynamicCalculation::Pixels(s) => f.write_fmt(format_args!("{s}")),
        }
    }
}

/// [Operator-precedence parser](https://en.wikipedia.org/wiki/Operator-precedence_parser#Precedence_climbing_method)
struct DynamicCalculationEvaluator<'a> {
    calcs: Iter<'a, DynamicCalculation>,
    parent_value: f32,
    root_value: f32,
    current: Option<&'a DynamicCalculation>,
}

impl<'a> DynamicCalculationEvaluator<'a> {
    pub fn new(calcs: Iter<'a, DynamicCalculation>, parent_value: f32, root_value: f32) -> Self {
        Self {
            calcs,
            parent_value,
            root_value,
            current: None,
        }
    }

    pub fn evaluate(&mut self) -> Option<f32> {
        // Parse and evaluate the expression
        let value = self.parse_expression(0);

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
    fn parse_expression(&mut self, min_precedence: usize) -> Option<f32> {
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
    fn parse_term(&mut self) -> Option<f32> {
        let prefix = self.parse_prefix()?;
        let mut lhs = None;
        // set to true so that the first value is multiplied and counts as normal syntax
        let mut last_is_separator = true;

        while let Some((rhs, seperator)) = self.parse_value() {
            if last_is_separator || seperator {
                lhs = Some(lhs.unwrap_or(1.0) * rhs);
            } else {
                return None;
            }
            last_is_separator = seperator;
        }
        if let Some(prefix) = prefix {
            match prefix {
                DynamicCalculation::Add => lhs,
                DynamicCalculation::Sub => lhs.map(|v| v * -1.0),
                _ => unreachable!("make sure to add the prefix here"),
            }
        } else {
            lhs
        }
    }
    /// parse and evaluate the value with the following grammar:
    /// ```ebnf
    ///     value      = percentage | pixels ;
    ///     percentage = number, "%" ;
    ///     pixels     = number ;
    /// `
    fn parse_value(&mut self) -> Option<(f32, bool)> {
        match self.current? {
            DynamicCalculation::Percentage(value) => {
                self.current = self.calcs.next();
                Some(((self.parent_value / 100.0 * value).round(), false))
            }
            DynamicCalculation::RootPercentage(value) => {
                self.current = self.calcs.next();
                Some(((self.root_value / 100.0 * value).round(), false))
            }
            DynamicCalculation::Pixels(value) => {
                self.current = self.calcs.next();
                Some((*value, false))
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
    parent_value: f32,
    root_value: f32,
) -> Option<f32> {
    DynamicCalculationEvaluator::new(calcs.iter(), parent_value, root_value).evaluate()
}
