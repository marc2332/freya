use std::sync::{Arc, Mutex};

use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node_ref::NodeView;
use dioxus_native_core::prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State};
use dioxus_native_core::NodeId;
use dioxus_native_core::SendAnyMap;
use dioxus_native_core_macro::partial_derive_state;
use skia_safe::font_style::Slant;
use skia_safe::font_style::Weight;
use skia_safe::font_style::Width;
use skia_safe::textlayout::TextAlign;
use skia_safe::Color;
use smallvec::{smallvec, SmallVec};
use torin::torin::Torin;

use crate::{CustomAttributeValues, Parse};

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
}

impl From<&FontStyle> for skia_safe::FontStyle {
    fn from(value: &FontStyle) -> Self {
        skia_safe::FontStyle::new(value.font_weight, value.font_width, value.font_slant)
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
            "font_style",
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
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(new_color) = Color::parse(value, None) {
                                font_style.color = new_color;
                            }
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
                    "align" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(align) = TextAlign::parse(value, None) {
                                font_style.align = align;
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
                    "font_style" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(font_slant) = Slant::parse(value, None) {
                                font_style.font_slant = font_slant;
                            }
                        }
                    }
                    "font_weight" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(font_weight) = Weight::parse(value, None) {
                                font_style.font_weight = font_weight;
                            }
                        }
                    }
                    "font_width" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(font_width) = Width::parse(value, None) {
                                font_style.font_width = font_width;
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
            || self.font_width != font_style.font_width;

        if changed_size {
            torin_layout.lock().unwrap().invalidate(node_view.node_id());
        }

        let changed = &font_style != self;
        *self = font_style;
        changed
    }
}