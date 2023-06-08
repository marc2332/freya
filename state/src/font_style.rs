use std::sync::{Arc, Mutex};

use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node_ref::NodeView;
use dioxus_native_core::prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State};
use dioxus_native_core::NodeId;
use dioxus_native_core::SendAnyMap;
use dioxus_native_core_macro::partial_derive_state;
use skia_safe::textlayout::TextAlign;
use skia_safe::font_style::Weight;
use skia_safe::font_style::Slant;
use skia_safe::font_style::Width;
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

    pub fn to_skia_font_style(&self) -> skia_safe::font_style::FontStyle {
        skia_safe::font_style::FontStyle::new(
            self.font_weight, self.font_width, self.font_slant
        )
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
            "font_slant",
            "font_weight",
            "font_width",
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
                    "font_slant" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            font_style.font_slant = parse_font_slant(attr);
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
            || self.font_width != font_style.font_width;

        if changed_size {
            torin_layout.lock().unwrap().invalidate(node_view.node_id());
        }

        let changed = &font_style != self;
        *self = font_style;
        changed
    }
}

fn parse_font_slant(slant: &str) -> Slant {
    match slant {
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
    // I've left these two abnormal values commented for now. They can't be specified via the number syntax.
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
        // "50" => skia_safe::font_style::Weight::INVISIBLE,
        "100" => Weight::THIN,
        "200" => Weight::EXTRA_LIGHT,
        "300" => Weight::LIGHT,
        "400" => Weight::NORMAL,
        "500" => Weight::MEDIUM,
        "600" => Weight::SEMI_BOLD,
        "700" => Weight::BOLD,
        "800" => Weight::EXTRA_BOLD,
        "900" => Weight::BLACK,
        // "950" => Weight::EXTRA_BLACK,
        _ => Weight::NORMAL,
    }
}

fn parse_font_width(width: &str) -> Width {
    match width {
        "ultra-condensed" => Width::ULTRA_CONDENSED,
        "extra-condensed" => Width::EXTRA_CONDENSED,
        "condensed" => Width::CONDENSED,
        "semi-condensed" => Width::SEMI_CONDENSED,
        "normal" => Width::NORMAL,
        "semi-expanded" => Width::SEMI_EXPANDED,
        "expanded"  => Width::EXPANDED,
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
