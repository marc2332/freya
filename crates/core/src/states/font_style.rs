use std::sync::{
    Arc,
    Mutex,
};

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
    custom_attributes::CustomAttributeValues,
    dom::CompositorDirtyNodes,
    parsing::{
        ExtSplit,
        Parse,
        ParseAttribute,
        ParseError,
    },
    values::{
        TextHeight,
        TextOverflow,
    },
};

#[derive(Debug, Clone, PartialEq, Component)]
pub struct FontStyleState {
    pub color: Color,
    pub text_shadows: Arc<[TextShadow]>,
    pub font_family: Arc<[String]>,
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
    pub text_height: TextHeightBehavior,
}

impl FontStyleState {
    pub fn text_style(
        &self,
        default_font_family: &[String],
        scale_factor: f32,
        paragraph_text_height: TextHeightBehavior,
    ) -> TextStyle {
        let mut text_style = TextStyle::new();

        let mut font_family = self.font_family.to_vec();

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

        if paragraph_text_height.needs_custom_height() {
            text_style.set_height_override(true);
            text_style.set_half_leading(true);
        }

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
            text_shadows: Arc::default(),
            font_family: Arc::default(),
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
            text_height: TextHeightBehavior::DisableAll,
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
                let value = attr.value.as_text().ok_or(ParseError)?;
                // Make an exception for the "inherit" as in this case we don't want to pass
                //  a color at all but use the inherited one.
                if value != "inherit" {
                    self.color = Color::parse(value)?;
                }
            }
            AttributeName::TextShadow => {
                self.text_shadows = attr
                    .value
                    .as_text()
                    .ok_or(ParseError)?
                    .split_excluding_group(',', '(', ')')
                    .map(|chunk| TextShadow::parse(chunk).unwrap_or_default())
                    .collect();
            }
            AttributeName::FontFamily => {
                self.font_family = attr
                    .value
                    .as_text()
                    .ok_or(ParseError)?
                    .split(',')
                    .map(|f| f.trim().to_string())
                    .collect();
            }
            AttributeName::FontSize => {
                self.font_size = attr
                    .value
                    .as_text()
                    .ok_or(ParseError)?
                    .parse()
                    .map_err(|_| ParseError)?;
            }
            AttributeName::LineHeight => {
                self.line_height = Some(
                    attr.value
                        .as_text()
                        .ok_or(ParseError)?
                        .parse()
                        .map_err(|_| ParseError)?,
                );
            }
            AttributeName::TextAlign => {
                self.text_align = TextAlign::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::MaxLines => {
                self.max_lines = Some(
                    attr.value
                        .as_text()
                        .ok_or(ParseError)?
                        .parse()
                        .map_err(|_| ParseError)?,
                );
            }
            AttributeName::TextOverflow => {
                self.text_overflow = TextOverflow::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::FontStyle => {
                self.font_slant = Slant::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::FontWeight => {
                self.font_weight = Weight::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::FontWidth => {
                self.font_width = Width::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::Decoration => {
                self.decoration.ty =
                    TextDecoration::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::DecorationStyle => {
                self.decoration.style =
                    TextDecorationStyle::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::DecorationColor => {
                self.decoration.color = match attr.value.as_text() {
                    Some(v) => Color::parse(v)?,
                    None => self.color,
                };
            }
            AttributeName::WordSpacing => {
                self.word_spacing = attr
                    .value
                    .as_text()
                    .ok_or(ParseError)?
                    .parse()
                    .map_err(|_| ParseError)?;
            }
            AttributeName::LetterSpacing => {
                self.letter_spacing = attr
                    .value
                    .as_text()
                    .ok_or(ParseError)?
                    .parse()
                    .map_err(|_| ParseError)?;
            }
            AttributeName::TextHeight => {
                self.text_height =
                    TextHeightBehavior::parse(attr.value.as_text().ok_or(ParseError)?)?;
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
            AttributeName::TextHeight,
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let root_id = context.get::<NodeId>().unwrap();
        let torin_layout = context.get::<Arc<Mutex<Torin<NodeId>>>>().unwrap();
        let compositor_dirty_nodes = context.get::<Arc<Mutex<CompositorDirtyNodes>>>().unwrap();

        let mut font_style = parent.map(|(v,)| v.clone()).unwrap_or_default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                font_style.parse_safe(attr);
            }
        }

        let changed = &font_style != self;

        let is_orphan = node_view.height() == 0 && node_view.node_id() != *root_id;

        if changed && !is_orphan {
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
