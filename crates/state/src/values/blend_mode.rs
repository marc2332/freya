use freya_engine::prelude::*;

use crate::{
    Parse,
    ParseError,
};

impl Parse for BlendMode {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "clear" => Self::Clear,
            "src" => Self::Src,
            "dst" => Self::Dst,
            "src-over" => Self::SrcOver,
            "dst-over" => Self::DstOver,
            "src-in" => Self::SrcIn,
            "dst-in" => Self::DstIn,
            "src-out" => Self::SrcOut,
            "dst-out" => Self::DstOut,
            "src-a-top" => Self::SrcATop,
            "dst-a-top" => Self::DstATop,
            "xor" => Self::Xor,
            "plus" => Self::Plus,
            "modulate" => Self::Modulate,
            "screen" => Self::Screen,
            "overlay" => Self::Overlay,
            "darken" => Self::Darken,
            "lighten" => Self::Lighten,
            "color-dodge" => Self::ColorDodge,
            "color-burn" => Self::ColorBurn,
            "hard-light" => Self::HardLight,
            "soft-light" => Self::SoftLight,
            "difference" => Self::Difference,
            "exclusion" => Self::Exclusion,
            "multiply" => Self::Multiply,
            "hue" => Self::Hue,
            "saturation" => Self::Saturation,
            "color" => Self::Color,
            "luminosity" => Self::Luminosity,
            _ => Err(ParseError)?,
        })
    }
}
