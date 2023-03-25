use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::ParentDepState;
use dioxus_native_core_macro::sorted_str_slice;
use freya_common::LayoutNotifier;
use skia_safe::textlayout::TextAlign;
use skia_safe::Color;
use smallvec::{smallvec, SmallVec};

use crate::{parse_color, CustomAttributeValues};

#[derive(Debug, Clone, PartialEq)]
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

/// Font style are inherited by default if not specified otherwise by some of the supported attributes.
impl ParentDepState<CustomAttributeValues> for FontStyle {
    type Ctx = LayoutNotifier;
    type DepState = (Self,);

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "color",
            "font_size",
            "font_family",
            "line_height",
            "align",
            "max_lines",
            "font_style"
        ])));

    fn reduce(
        &mut self,
        node: NodeView<CustomAttributeValues>,
        parent: Option<(&Self,)>,
        ctx: &Self::Ctx,
    ) -> bool {
        let mut font_style = parent.map(|(v,)| v.clone()).unwrap_or_default();
        let mut changed_size = false;

        if let Some(attributes) = node.attributes() {
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
                            changed_size = true;
                        }
                    }
                    "font_size" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Ok(font_size) = attr.parse() {
                                font_style.font_size = font_size;
                                changed_size = true;
                            }
                        }
                    }
                    "line_height" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            if let Ok(line_height) = attr.parse() {
                                font_style.line_height = line_height;
                                changed_size = true;
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
                                changed_size = true;
                            }
                        }
                    }
                    "font_style" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            font_style.font_style = parse_font_style(attr);
                            changed_size = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        if changed_size {
            *ctx.lock().unwrap() = true;
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
