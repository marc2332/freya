use std::sync::{Arc, Mutex};

use dioxus_native_core::{
    exports::shipyard::Component,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    NodeId, SendAnyMap,
};
use dioxus_native_core_macro::partial_derive_state;
use freya_engine::prelude::*;
use smallvec::{smallvec, SmallVec};
use torin::torin::Torin;

use crate::{CustomAttributeValues, ExtSplit, Parse, TextOverflow};

#[derive(Debug, Clone, PartialEq, Component)]
pub struct FontStyleState {
    pub color: Color,
    pub text_shadows: Vec<TextShadow>,
    pub font_family: SmallVec<[String; 2]>,
    pub font_size: f32,
    pub font_slant: Slant,
    pub font_weight: Weight,
    pub font_width: Width,
    pub line_height: f32, // https://developer.mozilla.org/en-US/docs/Web/CSS/line-height,
    pub decoration: Decoration,
    pub word_spacing: f32,
    pub letter_spacing: f32,
    pub text_align: TextAlign,
    pub max_lines: Option<usize>,
    pub text_overflow: TextOverflow,
}

impl FontStyleState {
    fn default_with_scale_factor(scale_factor: f32) -> Self {
        Self {
            font_size: 16.0 * scale_factor,
            ..FontStyleState::default()
        }
    }
}

impl From<&FontStyleState> for TextStyle {
    fn from(value: &FontStyleState) -> Self {
        let mut text_style = TextStyle::new();

        text_style
            .set_color(value.color)
            .set_font_style(freya_engine::prelude::FontStyle::new(
                value.font_weight,
                value.font_width,
                value.font_slant,
            ))
            .set_font_size(value.font_size)
            .set_font_families(&value.font_family)
            .set_word_spacing(value.word_spacing)
            .set_letter_spacing(value.letter_spacing)
            .set_height_override(true)
            .set_height(value.line_height);

        for shadow in value.text_shadows.iter() {
            text_style.add_shadow(*shadow);
        }

        text_style.set_decoration(&value.decoration);

        text_style
    }
}

impl Default for FontStyleState {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            text_shadows: Vec::new(),
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
            text_align: TextAlign::default(),
            max_lines: None,
            text_overflow: TextOverflow::default(),
        }
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for FontStyleState {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            "color",
            "text_shadow",
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
            "decoration_style",
            "text_overflow",
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
            .unwrap_or_else(|| FontStyleState::default_with_scale_factor(*scale_factor));

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "color" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(new_color) = Color::parse(value) {
                                font_style.color = new_color;
                            }
                        }
                    }
                    "text_shadow" => {
                        if let Some(value) = attr.value.as_text() {
                            font_style.text_shadows = value
                                .split_excluding_group(',', '(', ')')
                                .map(|chunk| {
                                    let mut shadow = TextShadow::parse(chunk).unwrap_or_default();
                                    shadow.offset *= *scale_factor;
                                    shadow.blur_sigma *= *scale_factor as f64;

                                    shadow
                                })
                                .collect();
                        }
                    }
                    "font_family" => {
                        if let Some(value) = attr.value.as_text() {
                            let families = value.split(',');
                            font_style.font_family = SmallVec::from(
                                families
                                    .into_iter()
                                    .map(|f| f.trim().to_string())
                                    .collect::<Vec<String>>(),
                            );
                        }
                    }
                    "font_size" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(font_size) = value.parse::<f32>() {
                                font_style.font_size = font_size * scale_factor;
                            }
                        }
                    }
                    "line_height" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(line_height) = value.parse() {
                                font_style.line_height = line_height;
                            }
                        }
                    }
                    "text_align" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(text_align) = TextAlign::parse(value) {
                                font_style.text_align = text_align;
                            }
                        }
                    }
                    "max_lines" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(max_lines) = value.parse() {
                                font_style.max_lines = Some(max_lines);
                            }
                        }
                    }
                    "text_overflow" => {
                        let value = attr.value.as_text();
                        if let Some(value) = value {
                            if let Ok(text_overflow) = TextOverflow::parse(value) {
                                font_style.text_overflow = text_overflow;
                            }
                        }
                    }
                    "font_style" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(font_slant) = Slant::parse(value) {
                                font_style.font_slant = font_slant;
                            }
                        }
                    }
                    "font_weight" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(font_weight) = Weight::parse(value) {
                                font_style.font_weight = font_weight;
                            }
                        }
                    }
                    "font_width" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(font_width) = Width::parse(value) {
                                font_style.font_width = font_width;
                            }
                        }
                    }
                    "decoration" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(decoration) = TextDecoration::parse(value) {
                                font_style.decoration.ty = decoration;
                            }
                        }
                    }
                    "decoration_style" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(style) = TextDecorationStyle::parse(value) {
                                font_style.decoration.style = style;
                            }
                        }
                    }
                    "decoration_color" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(new_decoration_color) = Color::parse(value) {
                                font_style.decoration.color = new_decoration_color;
                            }
                        } else {
                            font_style.decoration.color = font_style.color;
                        }
                    }
                    "word_spacing" => {
                        let value = attr.value.as_text();
                        if let Some(value) = value {
                            if let Ok(word_spacing) = value.parse() {
                                font_style.word_spacing = word_spacing;
                            }
                        }
                    }
                    "letter_spacing" => {
                        let value = attr.value.as_text();
                        if let Some(value) = value {
                            if let Ok(letter_spacing) = value.parse() {
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
