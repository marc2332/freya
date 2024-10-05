use std::{
    num::{
        ParseFloatError,
        ParseIntError,
    },
    sync::{
        Arc,
        Mutex,
    },
};

use freya_common::CompositorDirtyNodes;
use freya_engine::prelude::*;
use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        Dependancy,
        NodeMaskBuilder,
        State,
    },
    NodeId,
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;
use torin::torin::Torin;

use crate::{
    CustomAttributeValues,
    Parse,
    ParseAttribute,
    ParseError,
    TextOverflow,
    Token,
};

#[derive(Debug, Clone, PartialEq, Component)]
pub struct FontStyleState {
    pub color: Color,
    pub text_shadows: Vec<TextShadow>,
    pub font_family: Vec<String>,
    pub font_size: f32,
    pub font_slant: Slant,
    pub font_weight: Weight,
    pub font_width: Width,
    pub line_height: Option<f32>,
    pub decoration: Decoration,
    pub word_spacing: f32,
    pub letter_spacing: f32,
    pub text_align: TextAlign,
    pub max_lines: Option<usize>,
    pub text_overflow: TextOverflow,
}

impl FontStyleState {
    pub fn text_style(&self, default_font_family: &[String], scale_factor: f32) -> TextStyle {
        let mut text_style = TextStyle::new();
        let mut font_family = self.font_family.clone();

        font_family.extend_from_slice(default_font_family);

        text_style
            .set_color(self.color)
            .set_font_style(FontStyle::new(
                self.font_weight,
                self.font_width,
                self.font_slant,
            ))
            .set_font_size(self.font_size * scale_factor)
            .set_font_families(&font_family)
            .set_word_spacing(self.word_spacing)
            .set_letter_spacing(self.letter_spacing);

        if let Some(line_height) = self.line_height {
            text_style.set_height_override(true).set_height(line_height);
        }

        for text_shadow in self.text_shadows.iter() {
            text_style.add_shadow(*text_shadow);
        }

        text_style.set_decoration_style(self.decoration.style);
        text_style.set_decoration_type(self.decoration.ty);
        text_style.set_decoration_color(self.decoration.color);

        text_style
    }
}

impl Default for FontStyleState {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            text_shadows: Vec::new(),
            font_family: Vec::new(),
            font_size: 16.0,
            font_weight: Weight::NORMAL,
            font_slant: Slant::Upright,
            font_width: Width::NORMAL,
            line_height: None,
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

impl ParseAttribute for FontStyleState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), ParseError> {
        match attr.attribute {
            AttributeName::Color => {
                if let Some(value) = attr.value.as_text() {
                    // Make an exception for the "inherit" as in this case we don't want to pass
                    //  a color at all but use the inherited one.
                    if value != "inherit" {
                        self.color = Color::parse(value)?;
                    }
                }
            }
            AttributeName::TextShadow => {
                if let Some(value) = attr.value.as_text() {
                    self.text_shadows = TextShadow::parse_with_separator(value, &Token::Comma)?;
                }
            }
            AttributeName::FontFamily => {
                if let Some(value) = attr.value.as_text() {
                    let families = value.split(',');

                    self.font_family = families
                        .into_iter()
                        .map(|f| f.trim().to_string())
                        .collect::<Vec<String>>();
                }
            }
            AttributeName::FontSize => {
                if let Some(value) = attr.value.as_text() {
                    if let Ok(font_size) = value.parse::<f32>() {
                        self.font_size = font_size;
                    }
                }
            }
            AttributeName::LineHeight => {
                if let Some(value) = attr.value.as_text() {
                    if let Ok(line_height) = value.parse::<f32>() {
                        self.line_height = Some(line_height);
                    }
                }
            }
            AttributeName::TextAlign => {
                if let Some(value) = attr.value.as_text() {
                    self.text_align = TextAlign::parse(value)?;
                }
            }
            AttributeName::MaxLines => {
                if let Some(value) = attr.value.as_text() {
                    self.max_lines = Some(
                        value
                            .parse()
                            .map_err(|err: ParseIntError| ParseError(err.to_string()))?,
                    );
                }
            }
            AttributeName::TextOverflow => {
                if let Some(value) = attr.value.as_text() {
                    self.text_overflow = TextOverflow::parse(value)?;
                }
            }
            AttributeName::FontStyle => {
                if let Some(value) = attr.value.as_text() {
                    self.font_slant = Slant::parse(value)?;
                }
            }
            AttributeName::FontWeight => {
                if let Some(value) = attr.value.as_text() {
                    self.font_weight = Weight::parse(value)?;
                }
            }
            AttributeName::FontWidth => {
                if let Some(value) = attr.value.as_text() {
                    self.font_width = Width::parse(value)?;
                }
            }
            AttributeName::Decoration => {
                if let Some(value) = attr.value.as_text() {
                    self.decoration.ty = TextDecoration::parse(value)?;
                }
            }
            AttributeName::DecorationStyle => {
                if let Some(value) = attr.value.as_text() {
                    self.decoration.style = TextDecorationStyle::parse(value)?;
                }
            }
            AttributeName::DecorationColor => {
                if let Some(value) = attr.value.as_text() {
                    self.decoration.color = Color::parse(value)?;
                } else {
                    self.decoration.color = self.color;
                }
            }
            AttributeName::WordSpacing => {
                if let Some(value) = attr.value.as_text() {
                    self.word_spacing = value
                        .parse()
                        .map_err(|err: ParseFloatError| ParseError(err.to_string()))?;
                }
            }
            AttributeName::LetterSpacing => {
                if let Some(value) = attr.value.as_text() {
                    self.letter_spacing = value
                        .parse()
                        .map_err(|err: ParseFloatError| ParseError(err.to_string()))?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for FontStyleState {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::Color,
            AttributeName::TextAlign,
            AttributeName::TextShadow,
            AttributeName::FontSize,
            AttributeName::FontFamily,
            AttributeName::LineHeight,
            AttributeName::MaxLines,
            AttributeName::FontStyle,
            AttributeName::FontWeight,
            AttributeName::FontWidth,
            AttributeName::WordSpacing,
            AttributeName::LetterSpacing,
            AttributeName::Decoration,
            AttributeName::DecorationColor,
            AttributeName::DecorationStyle,
            AttributeName::TextOverflow,
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
        let compositor_dirty_nodes = context.get::<Arc<Mutex<CompositorDirtyNodes>>>().unwrap();

        let mut font_style = parent.map(|(v,)| v.clone()).unwrap_or_default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                font_style.parse_safe(attr);
            }
        }

        let changed = &font_style != self;

        if changed {
            torin_layout.lock().unwrap().invalidate(node_view.node_id());
            compositor_dirty_nodes
                .lock()
                .unwrap()
                .invalidate(node_view.node_id());
        }

        *self = font_style;
        changed
    }
}
