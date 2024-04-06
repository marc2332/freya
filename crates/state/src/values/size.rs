use torin::geometry::Length;
use torin::size::{DynamicCalculation, Size};

use crate::Parse;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSizeError;

impl Parse for Size {
    type Err = ParseSizeError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        if value == "auto" {
            Ok(Size::Inner)
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
                    .map_err(|_| ParseSizeError)?,
            )))
        } else if value.contains('v') {
            Ok(Size::RootPercentage(Length::new(
                value
                    .replace('v', "")
                    .parse::<f32>()
                    .map_err(|_| ParseSizeError)?,
            )))
        } else {
            Ok(Size::Pixels(Length::new(
                value.parse::<f32>().map_err(|_| ParseSizeError)?,
            )))
        }
    }
}

pub fn parse_calc(mut value: &str) -> Result<Vec<DynamicCalculation>, ParseSizeError> {
    let mut calcs = Vec::new();

    value = value
        .strip_prefix("calc(")
        .ok_or(ParseSizeError)?
        .strip_suffix(')')
        .ok_or(ParseSizeError)?;

    let values = value.split_whitespace();

    for val in values {
        if val.contains('%') {
            calcs.push(DynamicCalculation::Percentage(
                val.replace('%', "").parse().map_err(|_| ParseSizeError)?,
            ));
        } else if val == "+" {
            calcs.push(DynamicCalculation::Add);
        } else if val == "-" {
            calcs.push(DynamicCalculation::Sub);
        } else if val == "/" {
            calcs.push(DynamicCalculation::Div);
        } else if val == "*" {
            calcs.push(DynamicCalculation::Mul);
        } else {
            calcs.push(DynamicCalculation::Pixels(
                val.parse::<f32>().map_err(|_| ParseSizeError)?,
            ));
        }
    }

    Ok(calcs)
}
