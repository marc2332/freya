use std::sync::{Arc, Mutex};

use accesskit::{Action, Affine, Node, NodeBuilder, Rect, Role, TextDirection, Tree, TreeUpdate};
use freya_engine::prelude::{Color, TextAlign, TextDecoration, TextDecorationStyle};
use freya_node_state::{
    AccessibilityState, Fill, FontStyleState, OverflowMode, StyleState, TransformState,
};
use torin::prelude::LayoutNode;

use crate::{accessibility::*, dom::DioxusNode};

pub type SharedAccessibilityManager = Arc<Mutex<AccessibilityManager>>;

pub const ACCESSIBILITY_ROOT_ID: AccessibilityId = AccessibilityId(0);

/// Manages the Accessibility integration.
pub struct AccessibilityManager {
    /// Accessibility Nodes
    pub nodes: Vec<(AccessibilityId, Node)>,
    /// Current focused Accessibility Node.
    pub focused_id: AccessibilityId,
}

macro_rules! node_builder_property {
    ($builder:expr, $property:expr, $fn:ident) => {
        if let Some(p) = &$property {
            NodeBuilder::$fn(&mut $builder, p.clone())
        }
    };
}

impl AccessibilityManager {
    pub fn new(focused_id: AccessibilityId) -> Self {
        Self {
            focused_id,
            nodes: Vec::default(),
        }
    }

    /// Wrap it in a `Arc<Mutex<T>>`.
    pub fn wrap(self) -> SharedAccessibilityManager {
        Arc::new(Mutex::new(self))
    }

    /// Clear the Accessibility Nodes.
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn push_node(&mut self, id: AccessibilityId, node: Node) {
        self.nodes.push((id, node))
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
            | ((color.g() as u32) << 8) + (color.r() as u32)
    }

    /// Add a Node to the Accessibility Tree.
    pub fn add_node(
        &mut self,
        dioxus_node: &DioxusNode,
        layout_node: &LayoutNode,
        accessibility_state: &AccessibilityState,
    ) {
        let font_style_state = &*dioxus_node.get::<FontStyleState>().unwrap();
        let style_state = &*dioxus_node.get::<StyleState>().unwrap();
        let transform_state = &*dioxus_node.get::<TransformState>().unwrap();
        let options = &accessibility_state.options;
        let node_type = dioxus_node.node_type();

        let mut builder = NodeBuilder::new(options.role);

        // Set the area
        let area = layout_node.area.to_f64();
        builder.set_bounds(Rect {
            x0: area.min_x(),
            x1: area.max_x(),
            y0: area.min_y(),
            y1: area.max_y(),
        });

        // Set children
        let children = dioxus_node.get_accessibility_children();
        if !children.is_empty() {
            builder.set_children(children);
        }

        // Set focusable action
        // This will offer the user an abiliity to focus the element.
        if options.is_focusable {
            builder.add_action(Action::Focus);
        }

        // Rotation transform
        if let Some(rotation_degs) = transform_state.rotate_degs {
            builder.set_transform(Affine::rotate(rotation_degs.to_radians() as _))
        }

        // Clipping overflow
        if style_state.overflow == OverflowMode::Clip {
            builder.set_clips_children();
        }

        // Basic colors
        builder.set_foreground_color(Self::skia_color_to_rgba_u32(font_style_state.color));
        if let Fill::Color(color) = style_state.background {
            builder.set_background_color(Self::skia_color_to_rgba_u32(color));
        }

        // If the node is a block-level element in the layout, indicate that it will cause a linebreak.
        //
        // TODO: There is much more that could be done here for text, such as specifying character and word
        //       metrics. See: `character_widths`, `character_positions`, `character_lengths`.
        if !node_type.is_text() {
            // this should be impossible currently but i'm checking for it just in case.
            if let NodeType::Element(node) = &*node_type {
                if node.tag != TagName::Text {
                    builder.set_is_line_breaking_object();
                }
            }
        }

        // Font size
        builder.set_font_size(font_style_state.font_size as _);

        // If the font family has changed since the parent node, then we inform accesskit of this change.
        if let Some(parent_node) = dioxus_node.parent() {
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
            builder.set_strikethrough(Self::skia_decoration_style_to_accesskit(
                font_style_state.decoration.style,
            ));
        }
        if font_style_state
            .decoration
            .ty
            .contains(TextDecoration::UNDERLINE)
        {
            builder.set_underline(Self::skia_decoration_style_to_accesskit(
                font_style_state.decoration.style,
            ));
        }
        if font_style_state
            .decoration
            .ty
            .contains(TextDecoration::OVERLINE)
        {
            builder.set_overline(Self::skia_decoration_style_to_accesskit(
                font_style_state.decoration.style,
            ));
        }

        // Vec<NodeId>
        node_builder_property!(builder, options.controls, set_controls);
        node_builder_property!(builder, options.controls, set_controls);
        node_builder_property!(builder, options.details, set_details);
        node_builder_property!(builder, options.described_by, set_details);
        node_builder_property!(builder, options.named_by, set_labelled_by);
        node_builder_property!(builder, options.flow_to, set_flow_to);
        node_builder_property!(builder, options.owns, set_owns);
        node_builder_property!(builder, options.radio_group, set_radio_group);

        // NodeId
        node_builder_property!(builder, options.active_descendant, set_active_descendant);
        node_builder_property!(
            builder,
            options.in_page_link_target,
            set_in_page_link_target
        );
        node_builder_property!(builder, options.member_of, set_member_of);
        node_builder_property!(builder, options.next_on_line, set_next_on_line);
        node_builder_property!(builder, options.previous_on_line, set_previous_on_line);
        node_builder_property!(builder, options.popup_for, set_popup_for);

        // String
        node_builder_property!(builder, options.name, set_name);
        if let Some(name) = &options.name {
            builder.set_name(name.clone());
        } else if let Some(inner_text) = dioxus_node.get_inner_texts() {
            // If an accessible name was not explicitly provided, then we set it to
            // the node's inner text if the role supports it.
            //
            // Source for these roles: <https://w3c.github.io/aria/#namefromcontent>
            if matches!(
                options.role,
                Role::Button
                    | Role::Cell
                    | Role::CheckBox
                    | Role::ColumnHeader
                    | Role::Comment
                    | Role::Heading
                    | Role::Link
                    | Role::MenuItem
                    | Role::MenuItemCheckBox
                    | Role::MenuItemRadio
                    | Role::ListBoxOption
                    | Role::MenuListOption
                    | Role::RadioButton
                    | Role::Row
                    | Role::RowHeader
                    | Role::Switch
                    | Role::Tab
                    | Role::Tooltip
                    | Role::TreeItem
            ) {
                builder.set_name(inner_text);
            }
        }
        node_builder_property!(builder, options.description, set_description);
        node_builder_property!(builder, options.value, set_value);
        node_builder_property!(builder, options.author_id, set_author_id);
        node_builder_property!(builder, options.keyboard_shortcuts, set_keyboard_shortcut);
        node_builder_property!(builder, options.language, set_language);
        node_builder_property!(builder, options.placeholder, set_placeholder);
        node_builder_property!(builder, options.role_description, set_role_description);
        node_builder_property!(builder, options.state_description, set_state_description);
        node_builder_property!(builder, options.tooltip, set_tooltip);
        node_builder_property!(builder, options.url, set_url);

        // char
        node_builder_property!(
            builder,
            options.access_key.map(|k| k.to_string()),
            set_access_key
        );

        // usize
        node_builder_property!(builder, options.row_count, set_row_count);
        node_builder_property!(builder, options.column_count, set_column_count);
        node_builder_property!(builder, options.row_span, set_row_span);
        node_builder_property!(builder, options.column_span, set_column_span);
        node_builder_property!(builder, options.level, set_level);
        node_builder_property!(builder, options.size_of_set, set_size_of_set);
        node_builder_property!(builder, options.position_in_set, set_position_in_set);

        // Color
        if let Some(color_value) = options.color_value {
            builder.set_color_value(Self::skia_color_to_rgba_u32(color_value))
        }

        // Other
        node_builder_property!(builder, options.vertical_offset, set_vertical_offset);
        if let Some(numeric_value) = &options.numeric_value {
            builder.set_numeric_value(numeric_value.value);
            node_builder_property!(builder, numeric_value.min, set_min_numeric_value);
            node_builder_property!(builder, numeric_value.max, set_max_numeric_value);
            node_builder_property!(builder, numeric_value.step, set_numeric_value_step);
            node_builder_property!(builder, numeric_value.jump, set_numeric_value_jump);
        }
        if let Some(scroll_x) = options.scroll_x {
            builder.set_scroll_x(scroll_x.value);
            node_builder_property!(builder, scroll_x.min, set_scroll_x_min);
            node_builder_property!(builder, scroll_x.max, set_scroll_x_max);
        }
        if let Some(scroll_y) = options.scroll_y {
            builder.set_scroll_x(scroll_y.value);
            node_builder_property!(builder, scroll_y.min, set_scroll_y_min);
            node_builder_property!(builder, scroll_y.max, set_scroll_y_max);
        }
        if let Some(row_index) = &options.row_index {
            builder.set_row_index(row_index.index);
            node_builder_property!(builder, row_index.text, set_row_index_text);
        }
        if let Some(column_index) = &options.column_index {
            builder.set_column_index(column_index.index);
            node_builder_property!(builder, column_index.text, set_column_index_text);
        }
        node_builder_property!(builder, options.has_popup, set_has_popup);
        node_builder_property!(builder, options.list_style, set_list_style);
        node_builder_property!(builder, options.sort_direction, set_sort_direction);
        node_builder_property!(builder, options.auto_complete, set_auto_complete);
        node_builder_property!(builder, options.orientation, set_orientation);
        node_builder_property!(builder, options.current, set_aria_current);
        node_builder_property!(
            builder,
            options.default_action_verb,
            set_default_action_verb
        );
        node_builder_property!(builder, options.toggled, set_toggled);
        node_builder_property!(builder, options.live, set_live);
        if let Some(invalid) = &options.invalid {
            builder.set_invalid(invalid.reason);
            node_builder_property!(builder, invalid.error_message, set_error_message);
        }
        node_builder_property!(builder, options.custom_actions, set_custom_actions);
        node_builder_property!(builder, options.text_selection, set_text_selection);

        // Insert the node into the Tree
        let node = builder.build();
        self.push_node(accessibility_state.id, node);
    }

    /// Update the focused Node ID and generate a TreeUpdate if necessary.
    pub fn set_focus_with_update(&mut self, new_focus_id: AccessibilityId) -> Option<TreeUpdate> {
        self.focused_id = new_focus_id;

        // Only focus the element if it exists
        let node_focused_exists = new_focus_id == ACCESSIBILITY_ROOT_ID
            || self.nodes.iter().any(|node| node.0 == new_focus_id);
        if node_focused_exists {
            Some(TreeUpdate {
                nodes: Vec::new(),
                tree: None,
                focus: self.focused_id,
            })
        } else {
            None
        }
    }

    /// Create the root Accessibility Node.
    pub fn build_root(&mut self, root_name: &str) -> Node {
        let mut builder = NodeBuilder::new(Role::Window);
        builder.set_name(root_name.to_string());
        builder.set_children(
            self.nodes
                .iter()
                .map(|(id, _)| *id)
                .collect::<Vec<AccessibilityId>>(),
        );

        builder.build()
    }

    /// Process the Nodes accessibility Tree
    pub fn process(&mut self, root_id: AccessibilityId, root_name: &str) -> TreeUpdate {
        let root = self.build_root(root_name);
        let mut nodes = vec![(root_id, root)];
        nodes.extend(self.nodes.clone());
        nodes.reverse();

        let focus = self
            .nodes
            .iter()
            .find_map(|node| {
                if node.0 == self.focused_id {
                    Some(node.0)
                } else {
                    None
                }
            })
            .unwrap_or(ACCESSIBILITY_ROOT_ID);

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(root_id)),
            focus,
        }
    }

    /// Focus the next/previous Node starting from the currently focused Node.
    pub fn set_focus_on_next_node(&mut self, direction: AccessibilityFocusDirection) -> TreeUpdate {
        let node_index = self
            .nodes
            .iter()
            .enumerate()
            .find(|(_, node)| node.0 == self.focused_id)
            .map(|(i, _)| i);

        let target_node = if direction == AccessibilityFocusDirection::Forward {
            // Find the next Node
            if let Some(node_index) = node_index {
                if node_index == self.nodes.len() - 1 {
                    self.nodes.first()
                } else {
                    self.nodes.get(node_index + 1)
                }
            } else {
                self.nodes.first()
            }
        } else {
            // Find the previous Node
            if let Some(node_index) = node_index {
                if node_index == 0 {
                    self.nodes.last()
                } else {
                    self.nodes.get(node_index - 1)
                }
            } else {
                self.nodes.last()
            }
        };

        self.focused_id = target_node
            .map(|(id, _)| *id)
            .unwrap_or(ACCESSIBILITY_ROOT_ID);

        TreeUpdate {
            nodes: Vec::new(),
            tree: None,
            focus: self.focused_id,
        }
    }
}
