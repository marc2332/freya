use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::map,
    multi::many1,
    number::complete::float,
    sequence::preceded,
    IResult,
};
use torin::{
    geometry::Length,
    size::{
        Dimension,
        DynamicCalculation,
        LexFunction,
        Size,
    },
};

use crate::{
    Parse,
    ParseError,
};

impl Parse for Size {
    fn parse(value: &str) -> Result<Self, ParseError> {
        if value == "auto" {
            Ok(Size::Inner)
        } else if value == "flex" {
            Ok(Size::Flex(Length::new(1.0)))
        } else if value.contains("flex") {
            Ok(Size::Flex(Length::new(
                value
                    .replace("flex(", "")
                    .replace(')', "")
                    .parse::<f32>()
                    .map_err(|_| ParseError)?,
            )))
        } else if value == "fill" {
            Ok(Size::Fill)
        } else if value == "fill-min" {
            Ok(Size::FillMinimum)
        } else if value.contains("calc") {
            Ok(Size::DynamicCalculations(Box::new(parse_calc(value)?)))
        } else if value.contains('%') {
            Ok(Size::Percentage(Length::new(
                value
                    .replace('%', "")
                    .parse::<f32>()
                    .map_err(|_| ParseError)?,
            )))
        } else if value.contains('v') {
            Ok(Size::RootPercentage(Length::new(
                value
                    .replace('v', "")
                    .parse::<f32>()
                    .map_err(|_| ParseError)?,
            )))
        } else if value.contains('a') {
            Ok(Size::InnerPercentage(Length::new(
                value
                    .replace('a', "")
                    .parse::<f32>()
                    .map_err(|_| ParseError)?,
            )))
        } else {
            Ok(Size::Pixels(Length::new(
                value.parse::<f32>().map_err(|_| ParseError)?,
            )))
        }
    }
}

pub fn parse_calc(mut value: &str) -> Result<Vec<DynamicCalculation>, ParseError> {
    // No need to parse this using nom

    value = value
        .strip_prefix("calc(")
        .ok_or(ParseError)?
        .strip_suffix(')')
        .ok_or(ParseError)?;
    fn inner_parse(value: &str) -> IResult<&str, Vec<DynamicCalculation>> {
        many1(preceded(
            multispace0,
            alt((
                map(tag("min"), |_| {
                    DynamicCalculation::Function(LexFunction::Min)
                }),
                map(tag("max"), |_| {
                    DynamicCalculation::Function(LexFunction::Max)
                }),
                map(tag("clamp"), |_| {
                    DynamicCalculation::Function(LexFunction::Clamp)
                }),
                map(tag("scale"), |_| DynamicCalculation::ScalingFactor),
                map(tag("parent.width"), |_| {
                    DynamicCalculation::Parent(Dimension::Width)
                }),
                map(tag("parent.height"), |_| {
                    DynamicCalculation::Parent(Dimension::Height)
                }),
                map(tag("parent.cross"), |_| {
                    DynamicCalculation::Parent(Dimension::Cross)
                }),
                map(tag("parent"), |_| {
                    DynamicCalculation::Parent(Dimension::Current)
                }),
                map(tag("root.width"), |_| {
                    DynamicCalculation::Root(Dimension::Width)
                }),
                map(tag("root.height"), |_| {
                    DynamicCalculation::Root(Dimension::Height)
                }),
                map(tag("root.cross"), |_| {
                    DynamicCalculation::Root(Dimension::Cross)
                }),
                map(tag("root"), |_| {
                    DynamicCalculation::Root(Dimension::Current)
                }),
                map(tag("+"), |_| DynamicCalculation::Add),
                map(tag("-"), |_| DynamicCalculation::Sub),
                map(tag("*"), |_| DynamicCalculation::Mul),
                map(tag("/"), |_| DynamicCalculation::Div),
                map(tag("("), |_| DynamicCalculation::OpenParenthesis),
                map(tag(")"), |_| DynamicCalculation::ClosedParenthesis),
                map(tag(","), |_| DynamicCalculation::FunctionSeparator),
                map(float, DynamicCalculation::Pixels),
            )),
        ))(value)
    }
    let tokens = inner_parse(value).map_err(|_| ParseError)?.1;

    Ok(tokens)
}
