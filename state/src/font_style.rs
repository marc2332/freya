use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State};
use dioxus_native_core::SendAnyMap;
use dioxus_native_core_macro::partial_derive_state;
use skia_safe::textlayout::TextAlign;
use skia_safe::Color;
use smallvec::{smallvec, SmallVec};

use crate::{parse_color, CustomAttributeValues};

#[derive(Debug, Clone, PartialEq, Component)]
pub struct FontStyle {
    pub color: Color,
    pub font_family: SmallVec<[String; 2]>,
    pub font_size: f32,
    pub line_height: f32, // https://developer.mozilla.org/en-US/docs/Web/CSS/line-height,
    pub align: TextAlign,
    pub max_lines: Option<usize>,
    pub font_style: skia_safe::FontStyle,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            font_family: smallvec!["Fira Sans".to_string()],
            font_size: 16.0,
            line_height: 1.2,
            align: TextAlign::default(),
            max_lines: None,
            font_style: skia_safe::FontStyle::default(),
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
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<()>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let mut font_style = parent.map(|(v,)| v.clone()).unwrap_or_default();

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
                            if let Ok(font_size) = attr.parse() {
                                font_style.font_size = font_size;
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
                            font_style.font_style = parse_font_style(attr);
                        }
                    }
                    _ => {}
                }
            }
        }

        let changed = &font_style != self;
        *self = font_style;
        changed
    }
}

fn parse_font_style(style: &str) -> skia_safe::FontStyle {
    match style {
        "italic" => skia_safe::FontStyle::italic(),
        "bold" => skia_safe::FontStyle::bold(),
        "bold-italic" => skia_safe::FontStyle::bold_italic(),
        _ => skia_safe::FontStyle::default(),
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
