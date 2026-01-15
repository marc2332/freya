use std::{
    borrow::Cow,
    hash::Hash,
    ops::{
        Deref,
        DerefMut,
    },
    rc::Rc,
};

use torin::{
    prelude::Area,
    torin::Torin,
};

use crate::{
    accessibility::{
        dirty_nodes::AccessibilityDirtyNodes,
        focusable::Focusable,
        groups::AccessibilityGroups,
        id::{
            AccessibilityGenerator,
            AccessibilityId,
        },
        tree::ACCESSIBILITY_ROOT_ID,
    },
    element::ElementExt,
    layers::{
        Layer,
        Layers,
    },
    node_id::NodeId,
    prelude::{
        AccessibilityFocusStrategy,
        CursorStyle,
    },
    style::{
        border::Border,
        color::Color,
        corner_radius::CornerRadius,
        fill::Fill,
        font_size::FontSize,
        font_slant::FontSlant,
        font_weight::FontWeight,
        font_width::FontWidth,
        scale::Scale,
        shadow::Shadow,
        text_align::TextAlign,
        text_height::TextHeightBehavior,
        text_overflow::TextOverflow,
        text_shadow::TextShadow,
    },
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct LayoutData {
    pub layout: torin::node::Node,
}

impl From<torin::node::Node> for LayoutData {
    fn from(layout: torin::node::Node) -> Self {
        LayoutData { layout }
    }
}

impl Deref for LayoutData {
    type Target = torin::node::Node;

    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}

impl DerefMut for LayoutData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.layout
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EffectData {
    pub overflow: Overflow,
    pub rotation: Option<f32>,
    pub scale: Option<Scale>,
    pub opacity: Option<f32>,
    pub scrollable: bool,
    pub interactive: Interactive,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StyleState {
    pub background: Fill,
    pub corner_radius: CornerRadius,
    pub borders: Vec<Border>,
    pub shadows: Vec<Shadow>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CursorStyleData {
    pub color: Color,
    pub highlight_color: Color,
    pub style: CursorStyle,
}

impl Default for CursorStyleData {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            highlight_color: Color::from_rgb(87, 108, 188),
            style: CursorStyle::default(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TextStyleState {
    pub font_size: FontSize,
    pub color: Color,
    pub text_align: TextAlign,
    pub font_families: Vec<Cow<'static, str>>,
    pub text_height: TextHeightBehavior,
    pub text_overflow: TextOverflow,
    pub text_shadows: Vec<TextShadow>,
    pub font_slant: FontSlant,
    pub font_weight: FontWeight,
    pub font_width: FontWidth,
}

impl Default for TextStyleState {
    fn default() -> Self {
        Self {
            font_size: FontSize::default(),
            color: Color::BLACK,
            text_align: TextAlign::default(),
            font_families: Vec::new(),
            text_height: TextHeightBehavior::default(),
            text_overflow: TextOverflow::default(),
            text_shadows: Vec::new(),
            font_slant: FontSlant::default(),
            font_weight: FontWeight::default(),
            font_width: FontWidth::default(),
        }
    }
}

impl TextStyleState {
    pub fn from_data(parent: &TextStyleState, data: &TextStyleData) -> Self {
        let color = data.color.unwrap_or(parent.color);

        let text_align = data.text_align.unwrap_or_default();
        let text_height = data.text_height.unwrap_or_default();
        let text_overflow = data.text_overflow.clone().unwrap_or_default();
        let text_shadows = data.text_shadows.clone();

        // Font values can be inherited
        let font_size = data.font_size.unwrap_or(parent.font_size);
        let font_slant = data.font_slant.unwrap_or(parent.font_slant);
        let font_weight = data.font_weight.unwrap_or(parent.font_weight);
        let font_width = data.font_width.unwrap_or(parent.font_width);
        let mut font_families = data.font_families.clone();
        font_families.extend_from_slice(&parent.font_families);

        Self {
            color,
            text_align,
            text_height,
            text_overflow,
            text_shadows,
            font_size,
            font_slant,
            font_weight,
            font_width,
            font_families,
        }
    }

    pub fn update(
        &mut self,
        node_id: NodeId,
        parent_text_style: &Self,
        element: &Rc<dyn ElementExt>,
        layout: &mut Torin<NodeId>,
    ) {
        let text_style_data = element.text_style();

        let text_style = Self::from_data(parent_text_style, &text_style_data);
        let is_equal = *self == text_style;

        *self = text_style;

        if !is_equal {
            // TODO: Only invalidate label and paragraphs
            layout.invalidate(node_id);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Hash)]
pub struct TextStyleData {
    pub color: Option<Color>,
    pub font_size: Option<FontSize>,
    pub font_families: Vec<Cow<'static, str>>,
    pub text_align: Option<TextAlign>,
    pub text_height: Option<TextHeightBehavior>,
    pub text_overflow: Option<TextOverflow>,
    pub text_shadows: Vec<TextShadow>,
    pub font_slant: Option<FontSlant>,
    pub font_weight: Option<FontWeight>,
    pub font_width: Option<FontWidth>,
}

#[derive(Debug, Default)]
pub struct LayerState {
    pub layer: i16,
}

impl LayerState {
    pub fn create_for_root(node_id: NodeId, layers: &mut Layers) -> Self {
        let layer = 0;

        layers.insert_node_in_layer(node_id, layer);

        Self { layer }
    }

    pub fn remove(self, node_id: NodeId, layers: &mut Layers) {
        layers.remove_node_from_layer(&node_id, self.layer);
    }

    pub fn update(
        &mut self,
        parent_layer: &Self,
        node_id: NodeId,
        element: &Rc<dyn ElementExt>,
        layers: &mut Layers,
    ) {
        let relative_layer = element.layer();

        // Old
        layers.remove_node_from_layer(&node_id, self.layer);

        // New
        self.layer = match relative_layer {
            Layer::Relative(relative_layer) => parent_layer.layer + relative_layer + 1,
            Layer::Overlay => i16::MAX / 2,
        };
        layers.insert_node_in_layer(node_id, self.layer);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub enum Overflow {
    #[default]
    None,
    Clip,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub enum Interactive {
    #[default]
    Yes,
    No,
}

impl From<bool> for Interactive {
    fn from(value: bool) -> Self {
        match value {
            true => Interactive::Yes,
            false => Interactive::No,
        }
    }
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct EffectState {
    pub overflow: Overflow,
    pub clips: Rc<[NodeId]>,

    pub rotations: Rc<[NodeId]>,
    pub rotation: Option<f32>,

    pub scales: Rc<[NodeId]>,
    pub scale: Option<Scale>,

    pub opacities: Rc<[f32]>,

    pub scrollables: Rc<[NodeId]>,

    pub interactive: Interactive,
}

impl EffectState {
    pub fn update(
        &mut self,
        parent_node_id: NodeId,
        parent_effect_state: &Self,
        node_id: NodeId,
        effect_data: Option<Cow<'_, EffectData>>,
    ) {
        *self = Self {
            overflow: Overflow::default(),
            ..parent_effect_state.clone()
        };

        if parent_effect_state.overflow == Overflow::Clip {
            let mut clips = parent_effect_state.clips.to_vec();
            clips.push(parent_node_id);
            if self.clips.as_ref() != clips {
                self.clips = Rc::from(clips);
            }
        }

        if let Some(effect_data) = effect_data {
            self.overflow = effect_data.overflow;

            if let Some(rotation) = effect_data.rotation {
                let mut rotations = parent_effect_state.rotations.to_vec();
                rotations.push(node_id);
                self.rotation = Some(rotation);
                if self.rotations.as_ref() != rotations {
                    self.rotations = Rc::from(rotations);
                }
            }

            if let Some(scale) = effect_data.scale {
                let mut scales = parent_effect_state.scales.to_vec();
                scales.push(node_id);
                self.scale = Some(scale);
                if self.scales.as_ref() != scales {
                    self.scales = Rc::from(scales);
                }
            }

            if let Some(opacity) = effect_data.opacity {
                let mut opacities = parent_effect_state.opacities.to_vec();
                opacities.push(opacity);
                if self.opacities.as_ref() != opacities {
                    self.opacities = Rc::from(opacities);
                }
            }

            if effect_data.scrollable {
                let mut scrolls = parent_effect_state.scrollables.to_vec();
                scrolls.push(node_id);
                if self.scrollables.as_ref() != scrolls {
                    self.scrollables = Rc::from(scrolls);
                }
            }

            self.interactive = effect_data.interactive;
        }
    }

    pub fn is_visible(&self, layout: &Torin<NodeId>, area: &Area) -> bool {
        // Skip elements that are completely out of any their parent's viewport
        for viewport_id in self.clips.iter() {
            let viewport = layout.get(viewport_id).unwrap().visible_area();
            if !viewport.intersects(area) {
                return false;
            }
        }
        true
    }
}

#[derive(PartialEq, Clone)]
pub struct AccessibilityState {
    pub a11y_id: AccessibilityId,
    pub a11y_focusable: Focusable,
    pub a11y_member_of: Option<AccessibilityId>,
}

impl AccessibilityState {
    pub fn create(
        node_id: NodeId,
        element: &Rc<dyn ElementExt>,
        accessibility_diff: &mut AccessibilityDirtyNodes,
        accessibility_generator: &AccessibilityGenerator,
        accessibility_groups: &mut AccessibilityGroups,
    ) -> Self {
        let data = element.accessibility();

        let a11y_id = if node_id == NodeId::ROOT {
            ACCESSIBILITY_ROOT_ID
        } else {
            data.a11y_id
                .unwrap_or_else(|| AccessibilityId(accessibility_generator.new_id()))
        };

        accessibility_diff.add_or_update(node_id);

        if let Some(member_of) = data.builder.member_of() {
            let group = accessibility_groups.entry(member_of).or_default();
            // This is not perfect as it assumes that order of creation is the same as the UI order
            // But we can't either assume that all the members are from the same parent so knowing their UI order gets trickier
            // So for no we just push to the end of the vector
            group.push(a11y_id);
        }

        if data.a11y_auto_focus {
            accessibility_diff.request_focus(AccessibilityFocusStrategy::Node(a11y_id));
        }

        Self {
            a11y_id,
            a11y_focusable: data.a11y_focusable.clone(),
            a11y_member_of: data.builder.member_of(),
        }
    }

    pub fn remove(
        self,
        node_id: NodeId,
        parent_id: NodeId,
        accessibility_diff: &mut AccessibilityDirtyNodes,
        accessibility_groups: &mut AccessibilityGroups,
    ) {
        accessibility_diff.remove(node_id, parent_id);

        if let Some(member_of) = self.a11y_member_of {
            let group = accessibility_groups.get_mut(&member_of).unwrap();
            group.retain(|id| *id != self.a11y_id);
        }
    }

    pub fn update(
        &mut self,
        node_id: NodeId,
        element: &Rc<dyn ElementExt>,
        accessibility_diff: &mut AccessibilityDirtyNodes,
        accessibility_groups: &mut AccessibilityGroups,
    ) {
        let data = element.accessibility();

        if let Some(member_of) = self.a11y_member_of
            && self.a11y_member_of != data.builder.member_of()
        {
            let group = accessibility_groups.get_mut(&member_of).unwrap();
            group.retain(|id| *id != self.a11y_id);
        }

        if let Some(a11y_id) = data.a11y_id
            && self.a11y_id != a11y_id
        {
            accessibility_diff.add_or_update(node_id);
            self.a11y_id = a11y_id;
        }

        if let Some(member_of) = data.builder.member_of() {
            let group = accessibility_groups.entry(member_of).or_default();
            // This is not perfect as it assumes that order of creation is the same as the UI order
            // But we can't either assume that all the members are from the same parent so knowing their UI order gets trickier
            // So for no we just push to the end of the vector
            group.push(self.a11y_id);

            self.a11y_member_of = Some(member_of);
        }

        self.a11y_focusable = data.a11y_focusable.clone();
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AccessibilityData {
    pub a11y_id: Option<AccessibilityId>,
    pub a11y_auto_focus: bool,
    pub a11y_focusable: Focusable,
    pub builder: accesskit::Node,
}
