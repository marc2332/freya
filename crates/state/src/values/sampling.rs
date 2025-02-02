use std::fmt;

use crate::{
    Parse,
    ParseError,
};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum SamplingMode {
    #[default]
    Nearest,
    Bilinear,
    Trilinear,
    Mitchell,
    CatmullRom,
}

impl Parse for SamplingMode {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "bilinear" => SamplingMode::Bilinear,
            "trilinear" => SamplingMode::Trilinear,
            "mitchell" => SamplingMode::Mitchell,
            "catmull-rom" => SamplingMode::CatmullRom,
            _ => SamplingMode::Nearest,
        })
    }
}

impl fmt::Display for SamplingMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            SamplingMode::Nearest => "nearest",
            SamplingMode::Bilinear => "bilinear",
            SamplingMode::Trilinear => "trilinear",
            SamplingMode::Mitchell => "mitchell",
            SamplingMode::CatmullRom => "catmull-rom",
        })
    }
}
