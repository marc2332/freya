use torin::{
    geometry::Length,
    size::{
        DynamicCalculation,
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
    let mut calcs = Vec::new();

    value = value
        .strip_prefix("calc(")
        .ok_or(ParseError)?
        .strip_suffix(')')
        .ok_or(ParseError)?;

    let values = value.split_whitespace();

    for val in values {
        if val.contains('%') {
            calcs.push(DynamicCalculation::Percentage(
                val.replace('%', "").parse().map_err(|_| ParseError)?,
            ));
        } else if val.contains('v') {
            calcs.push(DynamicCalculation::RootPercentage(
                val.replace('v', "").parse().map_err(|_| ParseError)?,
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
                val.parse::<f32>().map_err(|_| ParseError)?,
            ));
        }
    }

    Ok(calcs)
}
