use std::sync::{Arc, Mutex};

use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node_ref::NodeView;
use dioxus_native_core::prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State};
use dioxus_native_core::NodeId;
use dioxus_native_core::SendAnyMap;
use dioxus_native_core_macro::partial_derive_state;
use skia_safe::font_style::{Slant, Weight, Width};
use skia_safe::textlayout::{TextStyle, Decoration, TextDecoration, TextDecorationStyle, TextAlign};
use skia_safe::Color;
use smallvec::{smallvec, SmallVec};
use torin::torin::Torin;

use crate::{parse_color, CustomAttributeValues};

#[derive(Debug, Clone, PartialEq, Component)]
pub struct FontStyle {
    pub color: Color,
    pub font_family: SmallVec<[String; 2]>,
    pub font_size: f32,
    pub font_slant: Slant,
    pub font_weight: Weight,
    pub font_width: Width,
    pub line_height: f32, // https://developer.mozilla.org/en-US/docs/Web/CSS/line-height,
    pub decoration: Decoration,
    pub word_spacing: f32,
    pub letter_spacing: f32,
    pub align: TextAlign,
    pub max_lines: Option<usize>,
}

impl FontStyle {
    fn default_with_scale_factor(scale_factor: f32) -> Self {
        Self {
            font_size: 16.0 * scale_factor,
            ..FontStyle::default()
        }
    }
}

impl From<&FontStyle> for TextStyle {
    fn from(value: &FontStyle) -> Self {
        let mut text_style = TextStyle::new();
        
        text_style
            .set_color(value.color)
            .set_font_style(skia_safe::FontStyle::new(value.font_weight, value.font_width, value.font_slant))
            .set_font_size(value.font_size)
            .set_font_families(&value.font_family)
            .set_word_spacing(value.word_spacing)
            .set_letter_spacing(value.letter_spacing)
            .set_height_override(true)
            .set_height(value.line_height);

        *text_style.decoration_mut() = value.decoration;

        text_style
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            font_family: smallvec!["Fira Sans".to_string()],
            font_size: 16.0,
            font_weight: Weight::NORMAL,
            font_slant: Slant::Upright,
            font_width: Width::NORMAL,
            line_height: 1.2,
            word_spacing: 0.0,
            letter_spacing: 0.0,
            decoration: Decoration {
                thickness_multiplier: 1.0, // Defaults to 0.0, even though 0.0 won't render anything
                ..Decoration::default()
            },
            align: TextAlign::default(),
            max_lines: None,
        }
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for FontStyle {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            "color",
            "font_size",
            "font_family",
            "line_height",
            "align",
            "max_lines",
            "font_style",
            "font_weight",
            "font_width",
            "word_spacing",
            "letter_spacing",
            "decoration",
            "decoration_color",
            "decoration_style"
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let torin_layout = context.get::<Arc<Mutex<Torin<NodeId>>>>().unwrap();
        let scale_factor = context.get::<f32>().unwrap();

        let mut font_style = parent
            .map(|(v,)| v.clone())
            .unwrap_or_else(|| FontStyle::default_with_scale_factor(*scale_factor));

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "color" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            let new_color = parse_color(attr);
                            if let Some(new_color) = new_color {
                                font_style.color = new_color;
                            }
                        }
                    }
                    "font_family" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            let families = attr.split(',');
                            font_style.font_family = SmallVec::from(
                                families
                                    .into_iter()
                                    .map(|f| f.trim().to_string())
                                    .collect::<Vec<String>>(),
                            );
                        }
                    }
                    "font_size" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Ok(font_size) = attr.parse::<f32>() {
                                font_style.font_size = font_size * scale_factor;
                            }
                        }
                    }
                    "line_height" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Ok(line_height) = attr.parse() {
                                font_style.line_height = line_height;
                            }
                        }
                    }
                    "align" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            font_style.align = parse_text_align(attr);
                        }
                    }
                    "max_lines" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Ok(max_lines) = attr.parse() {
                                font_style.max_lines = Some(max_lines);
                            }
                        }
                    }
                    "font_style" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            font_style.font_slant = parse_font_style(attr);
                        }
                    }
                    "font_weight" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            font_style.font_weight = parse_font_weight(attr);
                        }
                    }
                    "font_width" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            font_style.font_width = parse_font_width(attr);
                        }
                    }
                    "decoration" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            font_style.decoration.ty = parse_decoration(attr);
                        }
                    }
                    "decoration_style" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            font_style.decoration.style = parse_decoration_style(attr);
                        }
                    }
                    "decoration_color" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Some(new_decoration_color) = parse_color(attr) {
                                font_style.decoration.color = new_decoration_color;
                            }
                        } else {
                            font_style.decoration.color = font_style.color;
                        }
                    }
                    "word_spacing" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Ok(word_spacing) = attr.parse() {
                                font_style.word_spacing = word_spacing;
                            }
                        }
                    }
                    "letter_spacing" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Ok(letter_spacing) = attr.parse() {
                                font_style.letter_spacing = letter_spacing;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let changed_size = self.max_lines != font_style.max_lines
            || self.line_height != font_style.line_height
            || self.font_size != font_style.font_size
            || self.font_family != font_style.font_family
            || self.font_slant != font_style.font_slant
            || self.font_weight != font_style.font_weight
            || self.font_width != font_style.font_width
            || self.word_spacing != font_style.word_spacing
            || self.letter_spacing != font_style.letter_spacing;

        if changed_size {
            torin_layout.lock().unwrap().invalidate(node_view.node_id());
        }

        let changed = &font_style != self;
        *self = font_style;
        changed
    }
}

fn parse_font_style(style: &str) -> Slant {
    match style {
        "upright" => Slant::Upright,
        "italic" => Slant::Italic,
        "oblique" => Slant::Oblique,
        _ => Slant::Upright,
    }
}

fn parse_font_weight(weight: &str) -> Weight {
    // NOTES:
    // This is mostly taken from the OpenType specification (https://learn.microsoft.com/en-us/typography/opentype/spec/os2#usweightclass)
    // CSS has one deviation from this spec, which uses the value "950" for extra_black.
    // skia_safe also has an "invisible" weight smaller than the thin weight, which could fall under CSS's interpretation of OpenType's
    // version. In this case it would be font_weight: "50".
    match weight {
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
    }
}

fn parse_font_width(width: &str) -> Width {
    // NOTES:
    // CSS also supports some percentage mappings for different stretches.
    // https://developer.mozilla.org/en-US/docs/Web/CSS/font-stretch#keyword_to_numeric_mapping
    match width {
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
    }
}

pub fn parse_text_align(align: &str) -> TextAlign {
    match align {
        "center" => TextAlign::Center,
        "end" => TextAlign::End,
        "justify" => TextAlign::Justify,
        "left" => TextAlign::Left,
        "right" => TextAlign::Right,
        "start" => TextAlign::Start,
        _ => TextAlign::Left,
    }
}

pub fn parse_decoration(value: &str) -> TextDecoration {
    let decoration_values = value.split_ascii_whitespace();
    let mut decoration = TextDecoration::default();

    for value in decoration_values {
        decoration.set(match value {
            "underline" => TextDecoration::UNDERLINE,
            "overline" => TextDecoration::OVERLINE,
            "line-through" => TextDecoration::LINE_THROUGH,
            _ => TextDecoration::NO_DECORATION
        }, true);
    }

    decoration
}

pub fn parse_decoration_style(style: &str) -> TextDecorationStyle {
    match style {
        "solid" => TextDecorationStyle::Solid,
        "double" => TextDecorationStyle::Double,
        "dotted" => TextDecorationStyle::Dotted,
        "dashed" => TextDecorationStyle::Dashed,
        "wavy" => TextDecorationStyle::Wavy,
        _ => TextDecorationStyle::Solid
    }
}