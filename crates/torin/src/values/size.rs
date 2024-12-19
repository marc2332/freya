use std::slice::Iter;

pub use euclid::Rect;

use crate::{
    geometry::Length,
    measure::Phase,
    prelude::Area,
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
        matches!(self, Self::Inner | Self::FillMinimum)
    }

    pub fn pretty(&self) -> String {
        match self {
            Self::Inner => "auto".to_string(),
            Self::Pixels(s) => format!("{}", s.get()),
            Self::DynamicCalculations(calcs) => format!(
                "calc({})",
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

    // i really do not want to make a new type just so this lint goes away
    #[allow(clippy::too_many_arguments)]
    pub fn eval(
        &self,
        dimension: EvalDimension,
        parent_value: &Area,
        available_parent_value: f32,
        parent_margin: f32,
        root_value: &Area,
        phase: Phase,
    ) -> Option<f32> {
        let current_parent_value = match dimension {
            EvalDimension::Width => parent_value.width(),
            EvalDimension::Height => parent_value.height(),
        };
        let current_root_value = match dimension {
            EvalDimension::Width => root_value.width(),
            EvalDimension::Height => root_value.height(),
        };
        match self {
            Self::Pixels(px) => Some(px.get() + parent_margin),
            Self::Percentage(per) => Some(parent_value / 100.0 * per.get()),
            Self::DynamicCalculations(calculations) => {
                Some(run_calculations(calculations, parent_value, root_value).unwrap_or(0.0))
            }
            Self::Fill => Some(available_parent_value),
            Self::RootPercentage(per) => Some(root_value / 100.0 * per.get()),
            Self::Flex(_) | Self::FillMinimum if phase == Phase::Final => {
                Some(available_parent_value)
            }
            Size::Pixels(px) => Some(px.get() + parent_margin),
            Size::Percentage(per) => Some(current_parent_value / 100.0 * per.get()),
            Size::DynamicCalculations(calculations) => Some(
                run_calculations(calculations.deref(), parent_value, root_value, dimension)
                    .unwrap_or(0.0),
            ),
            Size::Fill => Some(available_parent_value),
            Size::FillMinimum if phase == Phase::Final => Some(available_parent_value),
            Size::RootPercentage(per) => Some(current_root_value / 100.0 * per.get()),
            Size::Flex(_) if phase == Phase::Final => Some(available_parent_value),
            _ => None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn min_max(
        &self,
        value: f32,
        dimension: EvalDimension,
        parent_value: &Area,
        available_parent_value: f32,
        single_margin: f32,
        margin: f32,
        minimum: &Self,
        maximum: &Self,
        root_value: &Area,
        phase: Phase,
    ) -> f32 {
        let value = self
            .eval(
                dimension,
                parent_value,
                available_parent_value,
                margin,
                root_value,
                phase,
            )
            .unwrap_or(value + margin);

        let minimum_value = minimum
            .eval(
                dimension,
                parent_value,
                available_parent_value,
                margin,
                root_value,
                phase,
            )
            .map(|v| v + single_margin);
        let maximum_value = maximum.eval(
            dimension,
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
            Self::DynamicCalculations(calcs) => {
                calcs.iter_mut().for_each(|calc| calc.scale(scale_factor));
            }
            Size::Pixels(s) => *s *= scale_factor,
            Size::DynamicCalculations(calcs) => calcs.iter_mut().for_each(|v| {
                if v == &mut DynamicCalculation::ScalingFactor {
                    *v = DynamicCalculation::Pixels(scale_factor);
                }
            }),
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
    FunctionSeparator,
    // this one works real weird, we actually replace it with a pixel value when we run the scale
    // function
    ScalingFactor,
    Parent(Dimension),
    Root(Dimension),
    Pixels(f32),
    Function(LexFunction),
}

// a token of a function name, not the whole function, this is a lex token not a parse tree.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LexFunction {
    Min,
    Max,
    Clamp,
}

/// dimension as in height/width, allows for the use of the parent height and parent width inside
/// of one side, for example:
/// ```rust
/// rsx! {
///     rect {
///         width: "200",
///         height: "400",
///         rect {
///             width: "calc(min(100%, 100%'))", // the ' signifies the other side which would be
///             // the height
///             height: "calc(min(100%, 100%'))"
///         }
///     }
/// }
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Dimension {
    Current,
    Other,
    Width,
    Height,
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
            DynamicCalculation::Sub => f.write_str("-"),
            DynamicCalculation::Mul => f.write_str("*"),
            DynamicCalculation::Div => f.write_str("/"),
            DynamicCalculation::Add => f.write_str("+"),
            DynamicCalculation::OpenParenthesis => f.write_str("("),
            DynamicCalculation::ClosedParenthesis => f.write_str(")"),
            DynamicCalculation::FunctionSeparator => f.write_str(","),
            DynamicCalculation::Pixels(s) => f.write_fmt(format_args!("{s}")),
            DynamicCalculation::Function(LexFunction::Min) => f.write_str("min"),
            DynamicCalculation::Function(LexFunction::Max) => f.write_str("max"),
            DynamicCalculation::Function(LexFunction::Clamp) => f.write_str("clamp"),
            DynamicCalculation::ScalingFactor => f.write_str("scale"),
            DynamicCalculation::Parent(Dimension::Current) => f.write_str("parent"),
            DynamicCalculation::Parent(Dimension::Other) => f.write_str("parent.other"),
            DynamicCalculation::Parent(Dimension::Width) => f.write_str("parent.width"),
            DynamicCalculation::Parent(Dimension::Height) => f.write_str("parent.height"),
            DynamicCalculation::Root(Dimension::Current) => f.write_str("root"),
            DynamicCalculation::Root(Dimension::Other) => f.write_str("root.other"),
            DynamicCalculation::Root(Dimension::Width) => f.write_str("root.width"),
            DynamicCalculation::Root(Dimension::Height) => f.write_str("root.height"),
        }
    }
}

/// [Operator-precedence parser](https://en.wikipedia.org/wiki/Operator-precedence_parser#Precedence_climbing_method)
struct DynamicCalculationEvaluator<'a> {
    calcs: Iter<'a, DynamicCalculation>,
    dimension: EvalDimension,
    parent_value: &'a Area,
    root_value: &'a Area,
    current: Option<&'a DynamicCalculation>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EvalDimension {
    Width,
    Height,
}

impl<'a> DynamicCalculationEvaluator<'a> {
    pub const fn new(
        calcs: Iter<'a, DynamicCalculation>,
        parent_value: f32,
        root_value: f32,
    ) -> Self {
    pub fn new(
        calcs: Iter<'a, DynamicCalculation>,
        dimension: EvalDimension,
        parent_value: &'a Area,
        root_value: &'a Area,
    ) -> Self {
        Self {
            calcs,
            parent_value,
            dimension,
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
        prefix.map_or(lhs, |prefix| match prefix {
            DynamicCalculation::Add => lhs,
            DynamicCalculation::Sub => lhs.map(|v| v * -1.0),
            _ => unreachable!("make sure to add the prefix here"),
        })
    }
    /// parse and evaluate the value with the following grammar:
    /// ```ebnf
    ///     value      = percentage | pixels ;
    ///     percentage = number, "%" ;
    ///     pixels     = number ;
    /// `
    fn parse_value(&mut self) -> Option<(f32, bool)> {
        match self.current? {
            DynamicCalculation::Root(Dimension::Current) => {
                self.current = self.calcs.next();
                match self.dimension {
                    EvalDimension::Width => Some((self.root_value.width(), true)),
                    EvalDimension::Height => Some((self.root_value.height(), true)),
                }
            }
            DynamicCalculation::Root(Dimension::Other) => {
                self.current = self.calcs.next();
                match self.dimension {
                    EvalDimension::Width => Some((self.root_value.height(), true)),
                    EvalDimension::Height => Some((self.root_value.width(), true)),
                }
            }
            DynamicCalculation::Root(Dimension::Width) => {
                self.current = self.calcs.next();
                Some((self.root_value.width(), true))
            }
            DynamicCalculation::Root(Dimension::Height) => {
                self.current = self.calcs.next();
                Some((self.root_value.height(), true))
            }
            DynamicCalculation::Parent(Dimension::Current) => {
                self.current = self.calcs.next();
                match self.dimension {
                    EvalDimension::Width => Some((self.parent_value.width(), true)),
                    EvalDimension::Height => Some((self.parent_value.height(), true)),
                }
            }
            DynamicCalculation::Parent(Dimension::Other) => {
                self.current = self.calcs.next();
                match self.dimension {
                    EvalDimension::Width => Some((self.parent_value.height(), true)),
                    EvalDimension::Height => Some((self.parent_value.width(), true)),
                }
            }
            DynamicCalculation::Parent(Dimension::Width) => {
                self.current = self.calcs.next();
                Some((self.parent_value.width(), true))
            }
            DynamicCalculation::Parent(Dimension::Height) => {
                self.current = self.calcs.next();
                Some((self.parent_value.height(), true))
            }
            DynamicCalculation::ScalingFactor => {
                self.current = self.calcs.next();
                Some((1.0, true))
            }
            DynamicCalculation::Pixels(value) => {
                self.current = self.calcs.next();
                Some((*value, false))
            }
            DynamicCalculation::OpenParenthesis => {
                // function should return on DynamicCalculation::ClosedParenthesis because it does
                // not have a precedence, thats how it actually works
                let val = self.parse_expression(0)?;
                if self.current != Some(&DynamicCalculation::ClosedParenthesis) {
                    return None;
                }
                self.current = self.calcs.next();
                Some((val, true))
            }
            DynamicCalculation::Function(func) => {
                // oh god here we gg
                self.current = self.calcs.next();
                let vals = self.parse_function_inputs()?;
                let res = match func {
                    LexFunction::Min => vals.into_iter().reduce(f32::min),
                    LexFunction::Max => vals.into_iter().reduce(f32::max),
                    LexFunction::Clamp => {
                        if vals.len() != 3 {
                            None
                        } else {
                            Some(vals[0].max(vals[1].min(vals[2])))
                        }
                    }
                }?;

                Some((res, true))
            }
            _ => None,
        }
    }

    fn parse_function_inputs(&mut self) -> Option<Vec<f32>> {
        let mut res = vec![];
        loop {
            let val = self.parse_expression(0)?;
            match self.current? {
                DynamicCalculation::FunctionSeparator => {}
                DynamicCalculation::ClosedParenthesis => {
                    res.push(val);
                    break;
                }
                _ => return None,
            }
            res.push(val);
        }
        self.current = self.calcs.next();
        Some(res)
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
            _ => None,
        }
    }
}

/// Calculate dynamic expression with operator precedence.
/// This value could be for example the width of a node's parent area.
pub fn run_calculations(
    calcs: &[DynamicCalculation],
    parent_value: &Area,
    root_value: &Area,
    dimension: EvalDimension,
) -> Option<f32> {
    DynamicCalculationEvaluator::new(calcs.iter(), dimension, parent_value, root_value).evaluate()
}
