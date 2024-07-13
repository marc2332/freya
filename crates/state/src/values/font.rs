use freya_engine::prelude::*;

use crate::{
    Parse,
    ParseError,
    Parser,
};

impl Parse for TextAlign {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|token| {
            token.as_string().and_then(|value| match value {
                "center" => Some(TextAlign::Center),
                "justify" => Some(TextAlign::Justify),
                "start" => Some(TextAlign::Start),
                "end" => Some(TextAlign::End),
                "left" => Some(TextAlign::Left),
                "right" => Some(TextAlign::Right),
                _ => None,
            })
        })
    }
}

impl Parse for Slant {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|token| {
            token.as_string().and_then(|value| match value {
                "upright" => Some(Slant::Upright),
                "italic" => Some(Slant::Italic),
                "oblique" => Some(Slant::Oblique),
                _ => None,
            })
        })
    }
}

impl Parse for Width {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|token| {
            token.as_string().and_then(|value| match value {
                "ultra-condensed" => Some(Width::ULTRA_CONDENSED),
                "extra-condensed" => Some(Width::EXTRA_CONDENSED),
                "condensed" => Some(Width::CONDENSED),
                "semi-condensed" => Some(Width::SEMI_CONDENSED),
                "normal" => Some(Width::NORMAL),
                "semi-expanded" => Some(Width::SEMI_EXPANDED),
                "expanded" => Some(Width::EXPANDED),
                "extra-expanded" => Some(Width::EXTRA_EXPANDED),
                "ultra-expanded" => Some(Width::ULTRA_EXPANDED),
                _ => None,
            })
        })
    }
}

impl Parse for Weight {
    // NOTES:
    // This is mostly taken from the OpenType specification (https://learn.microsoft.com/en-us/typography/opentype/spec/os2#usweightclass)
    // CSS has one deviation from this spec, which uses the value "950" for extra_black.
    // skia_safe also has an "invisible" weight smaller than the thin weight, which could fall under CSS's interpretation of OpenType's
    // version. In this case it would be font_weight: "50".
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|token| {
            if token.is_number() {
                token.as_number().and_then(|value| match value {
                    50 => Some(Weight::INVISIBLE),
                    100 => Some(Weight::THIN),
                    200 => Some(Weight::EXTRA_LIGHT),
                    300 => Some(Weight::LIGHT),
                    400 => Some(Weight::NORMAL),
                    500 => Some(Weight::MEDIUM),
                    600 => Some(Weight::SEMI_BOLD),
                    700 => Some(Weight::BOLD),
                    800 => Some(Weight::EXTRA_BOLD),
                    900 => Some(Weight::BLACK),
                    950 => Some(Weight::EXTRA_BLACK),
                    _ => None,
                })
            } else {
                token.as_string().and_then(|value| match value {
                    "invisible" => Some(Weight::INVISIBLE),
                    "thin" => Some(Weight::THIN),
                    "extra-light" => Some(Weight::EXTRA_LIGHT),
                    "light" => Some(Weight::LIGHT),
                    "normal" => Some(Weight::NORMAL),
                    "medium" => Some(Weight::MEDIUM),
                    "semi-bold" => Some(Weight::SEMI_BOLD),
                    "bold" => Some(Weight::BOLD),
                    "extra-bold" => Some(Weight::EXTRA_BOLD),
                    "black" => Some(Weight::BLACK),
                    "extra-black" => Some(Weight::EXTRA_BLACK),
                    _ => None,
                })
            }
        })
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub enum TextOverflow {
    #[default]
    Clip,
    Ellipsis,
    Custom(String),
}

impl TextOverflow {
    pub fn get_ellipsis(&self) -> Option<&str> {
        match self {
            Self::Clip => None,
            Self::Ellipsis => Some("..."),
            Self::Custom(custom) => Some(custom),
        }
    }

    pub fn pretty(&self) -> String {
        match self {
            TextOverflow::Clip => "clip".to_string(),
            TextOverflow::Ellipsis => "ellipsis".to_string(),
            TextOverflow::Custom(text_overflow) => text_overflow.to_string(),
        }
    }
}

impl Parse for TextOverflow {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|token| {
            token.as_string().map(|value| match value {
                "ellipsis" => TextOverflow::Ellipsis,
                "clip" => TextOverflow::Clip,
                value => TextOverflow::Custom(value.to_string()),
            })
        })
    }
}
