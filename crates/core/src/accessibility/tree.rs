use std::sync::{
    Arc,
    Mutex,
};

use accesskit::{
    Action,
    Affine,
    Node,
    NodeBuilder,
    NodeId as AccessibilityId,
    Rect,
    Role,
    TextDirection,
    Tree,
    TreeUpdate,
};
use freya_common::AccessibilityDirtyNodes;
use freya_engine::prelude::{
    Color,
    TextAlign,
    TextDecoration,
    TextDecorationStyle,
};
use freya_native_core::{
    node::NodeType,
    prelude::NodeImmutable,
    tags::TagName,
    NodeId,
};
use freya_node_state::{
    AccessibilityNodeState,
    Fill,
    FontStyleState,
    OverflowMode,
    StyleState,
    TransformState,
};
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};
use torin::{
    prelude::LayoutNode,
    torin::Torin,
};

use super::{
    AccessibilityFocusStrategy,
    NodeAccessibility,
};
use crate::dom::{
    DioxusDOM,
    DioxusNode,
};

pub const ACCESSIBILITY_ROOT_ID: AccessibilityId = AccessibilityId(0);

pub type SharedAccessibilityTree = Arc<Mutex<AccessibilityTree>>;

pub struct AccessibilityTree {
    pub map: FxHashMap<AccessibilityId, NodeId>,
    // Current focused Accessibility Node.
    pub focused_id: AccessibilityId,
}

impl AccessibilityTree {
    pub fn new(focused_id: AccessibilityId) -> Self {
        Self {
            focused_id,
            map: FxHashMap::default(),
        }
    }

    /// Initialize the Accessibility Tree
    pub fn init(
        &self,
        rdom: &DioxusDOM,
        layout: &Torin<NodeId>,
        dirty_nodes: &mut AccessibilityDirtyNodes,
    ) -> TreeUpdate {
        dirty_nodes.clear();

        let mut nodes = vec![];

        rdom.traverse_depth_first_advanced(|node_ref| {
            if !node_ref.node_type().is_element() {
                return false;
            }

            let accessibility_id = node_ref.get_accessibility_id();
            let layout_node = layout.get(node_ref.id());

            // Layout nodes might not exist yet when the app is lauched
            if let Some((accessibility_id, layout_node)) = accessibility_id.zip(layout_node) {
                let node_accessibility_state = node_ref.get::<AccessibilityNodeState>().unwrap();
                let accessibility_node =
                    Self::create_node(&node_ref, layout_node, &node_accessibility_state);
                nodes.push((accessibility_id, accessibility_node));
            }

            if let Some(tag) = node_ref.node_type().tag() {
                if *tag == TagName::Paragraph || *tag == TagName::Label {
                    return false;
                }
            }

            true
        });

        #[cfg(debug_assertions)]
        tracing::info!(
            "Initialized the Accessibility Tree with {} nodes.",
            nodes.len()
        );

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(ACCESSIBILITY_ROOT_ID)),
            focus: ACCESSIBILITY_ROOT_ID,
        }
    }

    /// Process any pending Accessibility Tree update
    pub fn process_updates(
        &mut self,
        rdom: &DioxusDOM,
        layout: &Torin<NodeId>,
        dirty_nodes: &mut AccessibilityDirtyNodes,
    ) -> TreeUpdate {
        let removed_ids = dirty_nodes.removed.drain().collect::<FxHashMap<_, _>>();
        let mut added_or_updated_ids = dirty_nodes
            .added_or_updated
            .drain()
            .collect::<FxHashSet<_>>();

        #[cfg(debug_assertions)]
        if !removed_ids.is_empty() || !added_or_updated_ids.is_empty() {
            tracing::info!(
                "Updating the Accessibility Tree with {} removals and {} additions/modifications",
                removed_ids.len(),
                added_or_updated_ids.len()
            );
        }

        // Mark the ancestors as modified
        for node_id in added_or_updated_ids.clone() {
            let node_ref = rdom.get(node_id).unwrap();
            let node_accessibility_state = node_ref.get::<AccessibilityNodeState>().unwrap();
            added_or_updated_ids.insert(
                node_accessibility_state
                    .closest_accessibility_node_id
                    .unwrap_or(rdom.root_id()),
            );
            self.map
                .insert(node_ref.get_accessibility_id().unwrap(), node_id);
        }

        // Mark the still existing ancenstors as modified
        for (node_id, ancestor_node_id) in removed_ids {
            added_or_updated_ids.insert(ancestor_node_id);
            self.map.retain(|_, id| *id != node_id);
        }

        // Create the updated nodes
        let mut nodes = Vec::new();
        for node_id in added_or_updated_ids {
            let node_ref = rdom.get(node_id).unwrap();
            let layout_node = layout.get(node_id).unwrap();
            let node_accessibility_state = node_ref.get::<AccessibilityNodeState>().unwrap();
            let accessibility_node =
                Self::create_node(&node_ref, layout_node, &node_accessibility_state);

            let accessibility_id = node_ref.get_accessibility_id().unwrap();

            nodes.push((accessibility_id, accessibility_node));
        }

        if !self.map.contains_key(&self.focused_id) {
            self.focused_id = ACCESSIBILITY_ROOT_ID;
        }

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(ACCESSIBILITY_ROOT_ID)),
            focus: self.focused_id,
        }
    }

    /// Update the focused Node ID and generate a TreeUpdate if necessary.
    pub fn set_focus_with_update(
        &mut self,
        new_focus_id: AccessibilityId,
    ) -> Option<(TreeUpdate, NodeId)> {
        self.focused_id = new_focus_id;

        // Only focus the element if it exists
        if let Some(node_id) = self.map.get(&new_focus_id).copied() {
            #[cfg(debug_assertions)]
            tracing::info!("Focused {new_focus_id:?} node.");

            Some((
                TreeUpdate {
                    nodes: Vec::new(),
                    tree: Some(Tree::new(ACCESSIBILITY_ROOT_ID)),
                    focus: self.focused_id,
                },
                node_id,
            ))
        } else {
            None
        }
    }

    /// Focus a Node given the strategy.
    pub fn set_focus_on_next_node(
        &mut self,
        stragegy: AccessibilityFocusStrategy,
        rdom: &DioxusDOM,
    ) -> (TreeUpdate, NodeId) {
        let mut nodes = Vec::new();

        rdom.traverse_depth_first_advanced(|node_ref| {
            if !node_ref.node_type().is_element() {
                return false;
            }

            let accessibility_id = node_ref.get_accessibility_id();

            if let Some(accessibility_id) = accessibility_id {
                nodes.push((accessibility_id, node_ref.id()))
            }

            if let Some(tag) = node_ref.node_type().tag() {
                if *tag == TagName::Paragraph || *tag == TagName::Label {
                    return false;
                }
            }

            true
        });

        let node_index = nodes
            .iter()
            .enumerate()
            .find(|(_, (accessibility_id, _))| *accessibility_id == self.focused_id)
            .map(|(i, _)| i);

        let target_node = if stragegy == AccessibilityFocusStrategy::Forward {
            // Find the next Node
            if let Some(node_index) = node_index {
                if node_index == nodes.len() - 1 {
                    nodes.first()
                } else {
                    nodes.get(node_index + 1)
                }
            } else {
                nodes.first()
            }
        } else {
            // Find the previous Node
            if let Some(node_index) = node_index {
                if node_index == 0 {
                    nodes.last()
                } else {
                    nodes.get(node_index - 1)
                }
            } else {
                nodes.last()
            }
        };

        let (accessibility_id, node_id) = target_node
            .copied()
            .unwrap_or((ACCESSIBILITY_ROOT_ID, rdom.root_id()));

        self.focused_id = accessibility_id;

        #[cfg(debug_assertions)]
        tracing::info!("Focused {accessibility_id:?} node.");

        (
            TreeUpdate {
                nodes: Vec::new(),
                tree: Some(Tree::new(ACCESSIBILITY_ROOT_ID)),
                focus: self.focused_id,
            },
            node_id,
        )
    }

    /// Create an accessibility node
    pub fn create_node(
        node_ref: &DioxusNode,
        layout_node: &LayoutNode,
        node_accessibility: &AccessibilityNodeState,
    ) -> Node {
        let font_style_state = &*node_ref.get::<FontStyleState>().unwrap();
        let style_state = &*node_ref.get::<StyleState>().unwrap();
        let transform_state = &*node_ref.get::<TransformState>().unwrap();
        let node_type = node_ref.node_type();

        let mut builder = NodeBuilder::new(Role::default());

        // Set children
        let children = node_ref.get_accessibility_children();
        builder.set_children(children);

        // Set the area
        let area = layout_node.area.to_f64();
        builder.set_bounds(Rect {
            x0: area.min_x(),
            x1: area.max_x(),
            y0: area.min_y(),
            y1: area.max_y(),
        });

        // Set focusable action
        // This will cause assistive technology to offer the user an option
        // to focus the current element if it supports it.
        if node_accessibility.focusable {
            builder.add_action(Action::Focus);
        }

        // Rotation transform
        if let Some((_, rotation)) = transform_state
            .rotations
            .iter()
            .find(|(id, _)| id == &node_ref.id())
        {
            builder.set_transform(Affine::rotate(rotation.to_radians() as _));
        }

        // Clipping overflow
        if style_state.overflow == OverflowMode::Clip {
            builder.set_clips_children();
        }

        // Foreground/Background color
        builder.set_foreground_color(skia_color_to_rgba_u32(font_style_state.color));
        if let Fill::Color(color) = style_state.background {
            builder.set_background_color(skia_color_to_rgba_u32(color));
        }

        // If the node is a block-level element in the layout, indicate that it will cause a linebreak.
        if !node_type.is_text() {
            if let NodeType::Element(node) = &*node_type {
                // This should be impossible currently but i'm checking for it just in case.
                // In the future, inline text spans should have their own own accessibility node,
                // but that's not a concern yet.
                if node.tag != TagName::Text {
                    builder.set_is_line_breaking_object();
                }
            }
        }

        // Font size
        builder.set_font_size(font_style_state.font_size as _);

        // If the font family has changed since the parent node, then we inform accesskit of this change.
        if let Some(parent_node) = node_ref.parent() {
            if parent_node.get::<FontStyleState>().unwrap().font_family
                != font_style_state.font_family
            {
                builder.set_font_family(font_style_state.font_family.join(", "));
            }
        } else {
            // Element has no parent elements, so we set the initial font style.
            builder.set_font_family(font_style_state.font_family.join(", "));
        }

        // Set bold flag for weights above 700
        if font_style_state.font_weight > 700.into() {
            builder.set_bold();
        }

        // Text alignment
        builder.set_text_align(match font_style_state.text_align {
            TextAlign::Center => accesskit::TextAlign::Center,
            TextAlign::Justify => accesskit::TextAlign::Justify,
            // TODO: change representation of `Start` and `End` once RTL text/writing modes are supported.
            TextAlign::Left | TextAlign::Start => accesskit::TextAlign::Left,
            TextAlign::Right | TextAlign::End => accesskit::TextAlign::Right,
        });

        // TODO: Adjust this once text direction support other than RTL is properly added
        builder.set_text_direction(TextDirection::LeftToRight);

        // Set italic property for italic/oblique font slants
        match font_style_state.font_slant as u32 {
            // Italic | Oblique
            1 | 2 => builder.set_italic(),
            _ => {}
        }

        // Text decoration
        if font_style_state
            .decoration
            .ty
            .contains(TextDecoration::LINE_THROUGH)
        {
            builder.set_strikethrough(skia_decoration_style_to_accesskit(
                font_style_state.decoration.style,
            ));
        }
        if font_style_state
            .decoration
            .ty
            .contains(TextDecoration::UNDERLINE)
        {
            builder.set_underline(skia_decoration_style_to_accesskit(
                font_style_state.decoration.style,
            ));
        }
        if font_style_state
            .decoration
            .ty
            .contains(TextDecoration::OVERLINE)
        {
            builder.set_overline(skia_decoration_style_to_accesskit(
                font_style_state.decoration.style,
            ));
        }

        // Set text value
        if let Some(alt) = &node_accessibility.alt {
            builder.set_value(alt.to_owned());
        } else if let Some(value) = node_ref.get_inner_texts() {
            builder.set_value(value);
            builder.set_role(Role::Label);
        }

        // Set name
        if let Some(name) = &node_accessibility.name {
            builder.set_name(name.to_owned());
        }

        // Set role
        if let Some(role) = node_accessibility.role {
            builder.set_role(role);
        }
        // Set root role
        if node_ref.id() == node_ref.real_dom().root_id() {
            builder.set_role(Role::Window);
        }

        builder.build()
    }
}

fn skia_decoration_style_to_accesskit(style: TextDecorationStyle) -> accesskit::TextDecoration {
    match style {
        TextDecorationStyle::Solid => accesskit::TextDecoration::Solid,
        TextDecorationStyle::Dotted => accesskit::TextDecoration::Dotted,
        TextDecorationStyle::Dashed => accesskit::TextDecoration::Dashed,
        TextDecorationStyle::Double => accesskit::TextDecoration::Double,
        TextDecorationStyle::Wavy => accesskit::TextDecoration::Wavy,
    }
}

fn skia_color_to_rgba_u32(color: Color) -> u32 {
    ((color.a() as u32) << 24)
        | ((color.b() as u32) << 16)
        | (((color.g() as u32) << 8) + (color.r() as u32))
}
