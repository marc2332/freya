use accesskit::{
    Action,
    Node,
    Rect,
    Role,
    TreeId,
    TreeUpdate,
};
use ragnarok::ProcessedEvents;
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};
use torin::prelude::{
    CursorPoint,
    LayoutNode,
};

use crate::{
    accessibility::{
        focus_strategy::AccessibilityFocusStrategy,
        focusable::Focusable,
        id::AccessibilityId,
    },
    elements::label::Label,
    events::emittable::EmmitableEvent,
    integration::{
        EventName,
        EventsChunk,
    },
    node_id::NodeId,
    prelude::{
        AccessibilityFocusMovement,
        EventType,
        Paragraph,
        WheelEventData,
        WheelSource,
    },
    tree::Tree,
};

pub const ACCESSIBILITY_ROOT_ID: AccessibilityId = AccessibilityId(0);

pub struct AccessibilityTree {
    pub map: FxHashMap<AccessibilityId, NodeId>,
    // Current focused Accessibility Node.
    pub focused_id: AccessibilityId,
}

impl Default for AccessibilityTree {
    fn default() -> Self {
        Self::new(ACCESSIBILITY_ROOT_ID)
    }
}

impl AccessibilityTree {
    pub fn new(focused_id: AccessibilityId) -> Self {
        Self {
            focused_id,
            map: FxHashMap::default(),
        }
    }

    pub fn focused_node_id(&self) -> Option<NodeId> {
        self.map.get(&self.focused_id).cloned()
    }

    /// Initialize the Accessibility Tree
    pub fn init(&mut self, tree: &mut Tree) -> TreeUpdate {
        tree.accessibility_diff.clear();

        let mut nodes = vec![];

        tree.traverse_depth(|node_id| {
            let accessibility_state = tree.accessibility_state.get(&node_id).unwrap();
            let layout_node = tree.layout.get(&node_id).unwrap();
            let accessibility_node = Self::create_node(node_id, layout_node, tree);
            nodes.push((accessibility_state.a11y_id, accessibility_node));
            self.map.insert(accessibility_state.a11y_id, node_id);
        });

        #[cfg(debug_assertions)]
        tracing::info!(
            "Initialized the Accessibility Tree with {} nodes.",
            nodes.len()
        );

        if !self.map.contains_key(&self.focused_id) {
            self.focused_id = ACCESSIBILITY_ROOT_ID;
        }

        TreeUpdate {
            tree_id: TreeId::ROOT,
            nodes,
            tree: Some(accesskit::Tree::new(ACCESSIBILITY_ROOT_ID)),
            focus: self.focused_id,
        }
    }

    /// Process any pending Accessibility Tree update
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn process_updates(
        &mut self,
        tree: &mut Tree,
        events_sender: &futures_channel::mpsc::UnboundedSender<EventsChunk>,
    ) -> TreeUpdate {
        let requested_focus = tree.accessibility_diff.requested_focus.take();
        let removed_ids = tree
            .accessibility_diff
            .removed
            .drain()
            .collect::<FxHashMap<_, _>>();
        let mut added_or_updated_ids = tree
            .accessibility_diff
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

        // Remove all the removed nodes from the update list
        for (node_id, _) in removed_ids.iter() {
            added_or_updated_ids.remove(node_id);
            self.map.retain(|_, id| id != node_id);
        }

        // Mark the parent of the removed nodes as updated
        for (_, parent_id) in removed_ids.iter() {
            if !removed_ids.contains_key(parent_id) {
                added_or_updated_ids.insert(*parent_id);
            }
        }

        // Register the created/updated nodes
        for node_id in added_or_updated_ids.clone() {
            let accessibility_state = tree.accessibility_state.get(&node_id).unwrap();
            self.map.insert(accessibility_state.a11y_id, node_id);

            let node_parent_id = tree.parents.get(&node_id).unwrap_or(&NodeId::ROOT);
            added_or_updated_ids.insert(*node_parent_id);
        }

        // Create the updated nodes
        let mut nodes = Vec::new();
        for node_id in added_or_updated_ids {
            let accessibility_state = tree.accessibility_state.get(&node_id).unwrap();
            let layout_node = tree.layout.get(&node_id).unwrap();
            let accessibility_node = Self::create_node(node_id, layout_node, tree);
            nodes.push((accessibility_state.a11y_id, accessibility_node));
        }

        let has_request_focus = requested_focus.is_some();

        // Fallback the focused id to the root if the focused node no longer exists
        if !self.map.contains_key(&self.focused_id) {
            self.focused_id = ACCESSIBILITY_ROOT_ID;
        }

        // Focus the requested node id if there is one
        if let Some(requested_focus) = requested_focus {
            self.focus_node_with_strategy(requested_focus, tree);
        }

        if let Some(node_id) = self.focused_node_id()
            && has_request_focus
        {
            self.scroll_to(node_id, tree, events_sender);
        }

        TreeUpdate {
            tree_id: TreeId::ROOT,
            nodes,
            tree: Some(accesskit::Tree::new(ACCESSIBILITY_ROOT_ID)),
            focus: self.focused_id,
        }
    }

    /// Focus a Node given the strategy.
    pub fn focus_node_with_strategy(
        &mut self,
        strategy: AccessibilityFocusStrategy,
        tree: &mut Tree,
    ) {
        if let AccessibilityFocusStrategy::Node(id) = strategy {
            if self.map.contains_key(&id) {
                self.focused_id = id;
            }
            return;
        }

        let (navigable_nodes, focused_id) = if strategy.mode()
            == Some(AccessibilityFocusMovement::InsideGroup)
        {
            // Get all accessible nodes in the current group
            let mut group_nodes = Vec::new();

            let node_id = self.map.get(&self.focused_id).unwrap();
            let accessibility_state = tree.accessibility_state.get(node_id).unwrap();
            let member_accessibility_id = accessibility_state.a11y_member_of;
            if let Some(member_accessibility_id) = member_accessibility_id {
                group_nodes = tree
                    .accessibility_groups
                    .get(&member_accessibility_id)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|id| {
                        let node_id = self.map.get(id).unwrap();
                        let accessibility_state = tree.accessibility_state.get(node_id).unwrap();
                        accessibility_state.a11y_focusable == Focusable::Enabled
                    })
                    .collect();
            }
            (group_nodes, self.focused_id)
        } else {
            let mut nodes = Vec::new();

            tree.traverse_depth(|node_id| {
                let accessibility_state = tree.accessibility_state.get(&node_id).unwrap();
                let member_accessibility_id = accessibility_state.a11y_member_of;

                // Exclude nodes that are members of groups except for the parent of the group
                if let Some(member_accessibility_id) = member_accessibility_id
                    && member_accessibility_id != accessibility_state.a11y_id
                {
                    return;
                }
                if accessibility_state.a11y_focusable == Focusable::Enabled {
                    nodes.push(accessibility_state.a11y_id);
                }
            });

            (nodes, self.focused_id)
        };

        let node_index = navigable_nodes
            .iter()
            .position(|accessibility_id| *accessibility_id == focused_id);

        let target_node = match strategy {
            AccessibilityFocusStrategy::Forward(_) => {
                // Find the next Node
                if let Some(node_index) = node_index {
                    if node_index == navigable_nodes.len() - 1 {
                        navigable_nodes.first().cloned()
                    } else {
                        navigable_nodes.get(node_index + 1).cloned()
                    }
                } else {
                    navigable_nodes.first().cloned()
                }
            }
            AccessibilityFocusStrategy::Backward(_) => {
                // Find the previous Node
                if let Some(node_index) = node_index {
                    if node_index == 0 {
                        navigable_nodes.last().cloned()
                    } else {
                        navigable_nodes.get(node_index - 1).cloned()
                    }
                } else {
                    navigable_nodes.last().cloned()
                }
            }
            _ => unreachable!(),
        };

        self.focused_id = target_node.unwrap_or(focused_id);

        #[cfg(debug_assertions)]
        tracing::info!("Focused {:?} node.", self.focused_id);
    }

    /// Send the necessary wheel events to scroll views so that the given focused [NodeId] is visible on screen.
    fn scroll_to(
        &self,
        node_id: NodeId,
        tree: &mut Tree,
        events_sender: &futures_channel::mpsc::UnboundedSender<EventsChunk>,
    ) {
        let Some(effect_state) = tree.effect_state.get(&node_id) else {
            return;
        };
        let mut target_node = node_id;
        let mut emmitable_events = Vec::new();
        // Iterate over the inherited scrollables from the closes to the farthest
        for closest_scrollable in effect_state.scrollables.iter().rev() {
            // Every scrollable has a target node, the first scrollable target is the focused node that we want to make visible,
            // the rest scrollables will in the other hand just have the previous scrollable as target
            let target_layout_node = tree.layout.get(&target_node).unwrap();
            let target_area = target_layout_node.area;
            let scrollable_layout_node = tree.layout.get(closest_scrollable).unwrap();
            let scrollable_target_area = scrollable_layout_node.area;

            // We only want to scroll if it is not visible
            if !effect_state.is_visible(&tree.layout, &target_area) {
                let element = tree.elements.get(closest_scrollable).unwrap();
                let scroll_x = element
                    .accessibility()
                    .builder
                    .scroll_x()
                    .unwrap_or_default() as f32;
                let scroll_y = element
                    .accessibility()
                    .builder
                    .scroll_y()
                    .unwrap_or_default() as f32;

                // Get the relative diff from where the scrollable scroll starts
                let diff_x = target_area.min_x() - scrollable_target_area.min_x() - scroll_x;
                let diff_y = target_area.min_y() - scrollable_target_area.min_y() - scroll_y;

                // And get the distance it needs to scroll in order to make the target visible
                let delta_y = -(scroll_y + diff_y);
                let delta_x = -(scroll_x + diff_x);
                emmitable_events.push(EmmitableEvent {
                    name: EventName::Wheel,
                    source_event: EventName::Wheel,
                    node_id: *closest_scrollable,
                    data: EventType::Wheel(WheelEventData::new(
                        delta_x as f64,
                        delta_y as f64,
                        WheelSource::Custom,
                        CursorPoint::default(),
                        CursorPoint::default(),
                    )),
                    bubbles: false,
                });
                // Change the target to the current scrollable, so that the next scrollable makes sure this one is visible
                target_node = *closest_scrollable;
            }
        }
        events_sender
            .unbounded_send(EventsChunk::Processed(ProcessedEvents {
                emmitable_events,
                ..Default::default()
            }))
            .unwrap();
    }

    /// Create an accessibility node
    pub fn create_node(node_id: NodeId, layout_node: &LayoutNode, tree: &Tree) -> Node {
        let element = tree.elements.get(&node_id).unwrap();
        let mut accessibility_data = element.accessibility().into_owned();

        if node_id == NodeId::ROOT {
            accessibility_data.builder.set_role(Role::Window);
        }

        // Set children
        let children = tree
            .children
            .get(&node_id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|child| tree.accessibility_state.get(&child).unwrap().a11y_id)
            .collect::<Vec<_>>();
        accessibility_data.builder.set_children(children);

        // Set the area
        let area = layout_node.area.to_f64();
        accessibility_data.builder.set_bounds(Rect {
            x0: area.min_x(),
            x1: area.max_x(),
            y0: area.min_y(),
            y1: area.max_y(),
        });

        // Set inner text
        if let Some(children) = tree.children.get(&node_id) {
            for child in children {
                let children_element = tree.elements.get(child).unwrap();
                // TODO: Maybe support paragraphs too, or use a new trait
                if let Some(label) = Label::try_downcast(children_element.as_ref()) {
                    accessibility_data.builder.set_label(label.text);
                } else if let Some(paragraph) = Paragraph::try_downcast(children_element.as_ref()) {
                    accessibility_data.builder.set_label(
                        paragraph
                            .spans
                            .iter()
                            .map(|span| span.text.to_string())
                            .collect::<String>(),
                    );
                };
            }
        }

        // Set focusable action
        // This will cause assistive technology to offer the user an option
        // to focus the current element if it supports it.
        if accessibility_data.a11y_focusable.is_enabled() {
            accessibility_data.builder.add_action(Action::Focus);
            // accessibility_data.builder.add_action(Action::Click);
        }

        // // Rotation transform
        // if let Some((_, rotation)) = transform_state
        //     .rotations
        //     .iter()
        //     .find(|(id, _)| id == &node_ref.id())
        // {
        //     let rotation = rotation.to_radians() as f64;
        //     let (s, c) = rotation.sin_cos();
        //     builder.set_transform(Affine::new([c, s, -s, c, 0.0, 0.0]));
        // }

        // // Clipping overflow
        // if style_state.overflow == OverflowMode::Clip {
        //     builder.set_clips_children();
        // }

        // Foreground/Background color
        // builder.set_foreground_color(font_style_state.color.into());
        // if let Fill::Color(color) = style_state.background {
        //     builder.set_background_color(color.into());
        // }

        // // If the node is a block-level element in the layout, indicate that it will cause a linebreak.
        // if !node_type.is_text() {
        //     if let NodeType::Element(node) = &*node_type {
        //         // This should be impossible currently but i'm checking for it just in case.
        //         // In the future, inline text spans should have their own own accessibility node,
        //         // but that's not a concern yet.
        //         if node.tag != TagName::Text {
        //             builder.set_is_line_breaking_object();
        //         }
        //     }
        // }

        // Font size
        // builder.set_font_size(font_style_state.font_size as _);

        // // If the font family has changed since the parent node, then we inform accesskit of this change.
        // if let Some(parent_node) = node_ref.parent() {
        //     if parent_node.get::<FontStyleState>().unwrap().font_family
        //         != font_style_state.font_family
        //     {
        //         builder.set_font_family(font_style_state.font_family.join(", "));
        //     }
        // } else {
        //     // Element has no parent elements, so we set the initial font style.
        //     builder.set_font_family(font_style_state.font_family.join(", "));
        // }

        // // Set bold flag for weights above 700
        // if font_style_state.font_weight > 700.into() {
        //     builder.set_bold();
        // }

        // // Text alignment
        // builder.set_text_align(match font_style_state.text_align {
        //     TextAlign::Center => accesskit::TextAlign::Center,
        //     TextAlign::Justify => accesskit::TextAlign::Justify,
        //     // TODO: change representation of `Start` and `End` once RTL text/writing modes are supported.
        //     TextAlign::Left | TextAlign::Start => accesskit::TextAlign::Left,
        //     TextAlign::Right | TextAlign::End => accesskit::TextAlign::Right,
        // });

        // // TODO: Adjust this once text direction support other than RTL is properly added
        // builder.set_text_direction(TextDirection::LeftToRight);

        // // Set italic property for italic/oblique font slants
        // match font_style_state.font_slant {
        //     FontSlant::Italic | FontSlant::Oblique => builder.set_italic(),
        //     _ => {}
        // }

        // // Text decoration
        // if font_style_state
        //     .text_decoration
        //     .contains(TextDecoration::LINE_THROUGH)
        // {
        //     builder.set_strikethrough(skia_decoration_style_to_accesskit(
        //         font_style_state.text_decoration_style,
        //     ));
        // }
        // if font_style_state
        //     .text_decoration
        //     .contains(TextDecoration::UNDERLINE)
        // {
        //     builder.set_underline(skia_decoration_style_to_accesskit(
        //         font_style_state.text_decoration_style,
        //     ));
        // }
        // if font_style_state
        //     .text_decoration
        //     .contains(TextDecoration::OVERLINE)
        // {
        //     builder.set_overline(skia_decoration_style_to_accesskit(
        //         font_style_state.text_decoration_style,
        //     ));
        // }

        accessibility_data.builder
    }
}

// fn skia_decoration_style_to_accesskit(style: TextDecorationStyle) -> accesskit::TextDecoration {
//     match style {
//         TextDecorationStyle::Solid => accesskit::TextDecoration::Solid,
//         TextDecorationStyle::Dotted => accesskit::TextDecoration::Dotted,
//         TextDecorationStyle::Dashed => accesskit::TextDecoration::Dashed,
//         TextDecorationStyle::Double => accesskit::TextDecoration::Double,
//         TextDecorationStyle::Wavy => accesskit::TextDecoration::Wavy,
//     }
// }
