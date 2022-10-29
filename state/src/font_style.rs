use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::ParentDepState;
use dioxus_native_core_macro::sorted_str_slice;
use skia_safe::textlayout::TextAlign;
use skia_safe::Color;

use crate::parse_color;

#[derive(Debug, Clone, PartialEq)]
pub struct FontStyle {
    pub color: Color,
    pub font_family: String,
    pub font_size: f32,
    pub line_height: f32, // https://developer.mozilla.org/en-US/docs/Web/CSS/line-height,
    pub align: TextAlign,
    pub max_lines: Option<usize>,
    pub font_style: skia_safe::FontStyle,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            font_family: "Fira Sans".to_string(),
            font_size: 16.0,
            line_height: 1.2,
            align: TextAlign::default(),
            max_lines: None,
            font_style: skia_safe::FontStyle::default(),
        }
    }
}

/// Font style are inherited by default if not specified otherwise by some of the supported attributes.
impl ParentDepState for FontStyle {
    type Ctx = ();
    type DepState = Self;

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

    fn reduce<'a>(
        &mut self,
        node: NodeView,
        parent: Option<&'a Self::DepState>,
        _ctx: &Self::Ctx,
    ) -> bool {
        let mut font_style = parent.cloned().unwrap_or_default();

        for attr in node.attributes() {
            match attr.name {
                "color" => {
                    let new_color = parse_color(&attr.value.to_string());
                    if let Some(new_color) = new_color {
                        font_style.color = new_color;
                    }
                }
                "font_family" => {
                    font_style.font_family = attr.value.to_string();
                }
                "font_size" => {
                    if let Ok(font_size) = attr.value.to_string().parse() {
                        font_style.font_size = font_size;
                    }
                }
                "line_height" => {
                    if let Ok(line_height) = attr.value.to_string().parse() {
                        font_style.line_height = line_height;
                    }
                }
                "align" => {
                    font_style.align = parse_text_align(&attr.value.to_string());
                }
                "max_lines" => {
                    if let Ok(max_lines) = attr.value.to_string().parse() {
                        font_style.max_lines = Some(max_lines);
                    }
                }
                "font_style" => {
                    font_style.font_style = parse_font_style(&attr.value.to_string());
                }
                _ => {}
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
