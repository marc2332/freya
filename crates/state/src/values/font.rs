use crate::Parse;
use freya_engine::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTextAlignError;

impl Parse for TextAlign {
    type Err = ParseTextAlignError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "center" => TextAlign::Center,
            "justify" => TextAlign::Justify,
            "start" => TextAlign::Start,
            "end" => TextAlign::End,
            "left" => TextAlign::Left,
            "right" => TextAlign::Right,
            _ => TextAlign::default(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSlantError;

impl Parse for Slant {
    type Err = ParseSlantError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "upright" => Slant::Upright,
            "italic" => Slant::Italic,
            "oblique" => Slant::Oblique,
            _ => Slant::Upright,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseWidthError;

impl Parse for Width {
    type Err = ParseWidthError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "ultra-condensed" => Width::ULTRA_CONDENSED,
            "extra-condensed" => Width::EXTRA_CONDENSED,
            "condensed" => Width::CONDENSED,
            "semi-condensed" => Width::SEMI_CONDENSED,
            "normal" => Width::NORMAL,
            "semi-expanded" => Width::SEMI_EXPANDED,
            "expanded" => Width::EXPANDED,
            "extra-expanded" => Width::EXTRA_EXPANDED,
            "ultra-expanded" => Width::ULTRA_EXPANDED,
            _ => Width::NORMAL,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseWeightError;

impl Parse for Weight {
    type Err = ParseWeightError;

    // NOTES:
    // This is mostly taken from the OpenType specification (https://learn.microsoft.com/en-us/typography/opentype/spec/os2#usweightclass)
    // CSS has one deviation from this spec, which uses the value "950" for extra_black.
    // skia_safe also has an "invisible" weight smaller than the thin weight, which could fall under CSS's interpretation of OpenType's
    // version. In this case it would be font_weight: "50".
    fn parse(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "invisible" => Weight::INVISIBLE,
            "thin" => Weight::THIN,
            "extra-light" => Weight::EXTRA_LIGHT,
            "light" => Weight::LIGHT,
            "normal" => Weight::NORMAL,
            "medium" => Weight::MEDIUM,
            "semi-bold" => Weight::SEMI_BOLD,
            "bold" => Weight::BOLD,
            "extra-bold" => Weight::EXTRA_BOLD,
            "black" => Weight::BLACK,
            "extra-black" => Weight::EXTRA_BLACK,
            "50" => Weight::INVISIBLE,
            "100" => Weight::THIN,
            "200" => Weight::EXTRA_LIGHT,
            "300" => Weight::LIGHT,
            "400" => Weight::NORMAL,
            "500" => Weight::MEDIUM,
            "600" => Weight::SEMI_BOLD,
            "700" => Weight::BOLD,
            "800" => Weight::EXTRA_BOLD,
            "900" => Weight::BLACK,
            "950" => Weight::EXTRA_BLACK,
            _ => Weight::NORMAL,
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTextOverflowError;

impl Parse for TextOverflow {
    type Err = ParseTextOverflowError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "ellipsis" => TextOverflow::Ellipsis,
            "clip" => TextOverflow::Clip,
            value => TextOverflow::Custom(value.to_string()),
        })
    }
}
