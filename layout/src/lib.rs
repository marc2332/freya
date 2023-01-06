use dioxus_native_core::{node::NodeType, NodeId};
use freya_common::{LayoutMemorizer, NodeArea, NodeLayoutInfo, NodeReferenceLayout};
use freya_layers::{DOMNode, Layers};
use freya_node_state::{
    CursorMode, CursorReference, DirectionMode, DisplayMode, NodeState, SizeMode,
};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use std::sync::{Arc, Mutex};

mod area_calc;
mod ops_calc;

use area_calc::calculate_area;
pub use ops_calc::run_calculations;

type NodeResolver<T> = fn(&NodeId, &T) -> Option<DOMNode>;

/// Collect all the texts and node states from a given array of children
fn get_inner_texts<T>(
    children: &[NodeId],
    node_resolver: NodeResolver<T>,
    resolver_options: &T,
) -> Vec<(NodeState, String)> {
    children
        .iter()
        .filter_map(|child_id| {
            let child = node_resolver(child_id, resolver_options)?;
            if let NodeType::Element { tag, .. } = child.get_type() {
                if tag != "text" {
                    return None;
                }
                let children = child.children?;
                let child_text_id = children.get(0)?;
                let child_text = node_resolver(child_text_id, resolver_options)?;

                if let NodeType::Text { text } = &child_text.get_type() {
                    Some((child.node.state, text.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<(NodeState, String)>>()
}

/// Get the info related to a cursor reference
fn get_cursor_reference(node_data: &DOMNode) -> Option<(&CursorReference, usize, (f32, f32))> {
    let cursor_ref = node_data.get_state().references.cursor_ref.as_ref()?;
    let positions = { *cursor_ref.positions.lock().unwrap().as_ref()? };
    let current_cursor_id = { *cursor_ref.id.lock().unwrap().as_ref()? };
    let cursor_id = node_data.get_state().cursor_settings.id.as_ref()?;

    if current_cursor_id == *cursor_id {
        Some((cursor_ref, *cursor_id, positions))
    } else {
        None
    }
}

/// Measure the layout of a given Node and all it's children
pub struct NodeLayoutMeasurer<'a, T> {
    dom_node: &'a DOMNode,
    remaining_area: &'a mut NodeArea,
    parent_area: NodeArea,
    layers: &'a mut Layers,
    resolver_options: &'a T,
    node_resolver: NodeResolver<T>,
    inherited_relative_layer: i16,
    font_collection: &'a mut FontCollection,
    layout_memorizer: &'a Arc<Mutex<LayoutMemorizer>>,
}

impl<'a, T> NodeLayoutMeasurer<'a, T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        node_data: &'a DOMNode,
        remaining_area: &'a mut NodeArea,
        parent_area: NodeArea,
        resolver_options: &'a T,
        layers: &'a mut Layers,
        node_resolver: NodeResolver<T>,
        inherited_relative_layer: i16,
        font_collection: &'a mut FontCollection,
        layout_memorizer: &'a Arc<Mutex<LayoutMemorizer>>,
    ) -> Self {
        Self {
            dom_node: node_data,
            remaining_area,
            parent_area,
            resolver_options,
            layers,
            node_resolver,
            inherited_relative_layer,
            font_collection,
            layout_memorizer,
        }
    }

    pub fn run_dirty_checks(&mut self) -> (bool, bool) {
        let is_parent_dirty = self
            .dom_node
            .parent_id
            .map(|p| self.layout_memorizer.lock().unwrap().is_dirty(&p))
            .unwrap_or(false);

        // If parent is dirty, mark this node as dirty too
        if is_parent_dirty {
            self.layout_memorizer
                .lock()
                .unwrap()
                .mark_as_dirty(*self.dom_node.get_id());
        }

        let is_dirty = self
            .layout_memorizer
            .lock()
            .unwrap()
            .is_dirty(self.dom_node.get_id());

        let is_cached = self
            .layout_memorizer
            .lock()
            .unwrap()
            .is_node_layout_memorized(self.dom_node.get_id());

        // If this node is dirty and parent is not dirty, mark this node dirty
        if is_dirty && !is_parent_dirty {
            if let Some(p) = self.dom_node.parent_id {
                self.layout_memorizer.lock().unwrap().mark_as_dirty(p)
            }
        }

        (is_dirty, is_cached)
    }

    /// Measure the area of a node
    pub fn measure_area(&mut self, must_memorize_layout: bool) -> NodeArea {
        let node_data = self.dom_node;
        let direction = &self.dom_node.get_state().size.direction;

        // Caculate the corresponding layer of this node
        let (node_layer, node_relative_layer) = self
            .layers
            .calculate_layer(node_data, self.inherited_relative_layer);

        let (is_dirty, is_cached) = self.run_dirty_checks();
        let must_recalculate = is_dirty || !is_cached;

        let padding = node_data.get_state().size.padding;

        let (
            mut node_area,
            mut remaining_inner_area,
            inner_area,
            mut inner_width,
            mut inner_height,
        ) = if must_recalculate {
            // TODO Change to the new version
            let node_area = calculate_area(self);

            let horizontal_padding = padding.1 + padding.3;
            let vertical_padding = padding.0 + padding.2;

            // Area that is available consideing the parent area
            let mut remaining_inner_area = NodeArea {
                x: node_area.x + padding.3,
                y: node_area.y + padding.0,
                width: node_area.width - horizontal_padding,
                height: node_area.height - vertical_padding,
            };

            // Visible area occupied by the child elements
            let inner_area = remaining_inner_area;

            // Increase the x and y axis with the node's scroll attributes
            remaining_inner_area.y += node_data.get_state().scroll.scroll_y;
            remaining_inner_area.x += node_data.get_state().scroll.scroll_x;

            if must_memorize_layout {
                // Memorize these layouts
                self.layout_memorizer.lock().unwrap().memorize_layout(
                    *node_data.get_id(),
                    NodeLayoutInfo {
                        area: node_area,
                        remaining_inner_area,
                        inner_area,
                        inner_width: horizontal_padding,
                        inner_height: vertical_padding,
                    },
                );
            }

            (
                node_area,
                remaining_inner_area,
                inner_area,
                horizontal_padding,
                vertical_padding,
            )
        } else {
            // Get the memorized layouts
            self.layout_memorizer
                .lock()
                .unwrap()
                .get_node_layout(node_data.get_id())
                .unwrap()
                .as_tuple()
        };

        // Calculate the children layouts  for the first time without the size and axis adjusted.
        if DisplayMode::Center == node_data.get_state().style.display {
            self.measure_inner_children(
                &mut node_area,
                inner_area,
                &mut remaining_inner_area,
                &mut inner_width,
                &mut inner_height,
                node_relative_layer,
                false, // By specifying `false` in the argument `must_memorize_layout` it will not cache any inner children layout in this first iteration
            );

            let space_left_vertically = (inner_area.height - inner_height) / 2.0;
            let space_left_horizontally = (inner_area.width - inner_width) / 2.0;

            match direction {
                DirectionMode::Vertical => {
                    remaining_inner_area.y = inner_area.y + space_left_vertically + padding.0;
                    remaining_inner_area.height =
                        inner_area.height - space_left_vertically - padding.2;
                }
                DirectionMode::Horizontal => {
                    remaining_inner_area.x = inner_area.x + space_left_horizontally + padding.1;
                    remaining_inner_area.width =
                        inner_area.width - space_left_horizontally - padding.3;
                }
                DirectionMode::Both => {
                    remaining_inner_area.x = inner_area.x + space_left_horizontally + padding.1;
                    remaining_inner_area.y = inner_area.y + space_left_vertically + padding.0;

                    remaining_inner_area.width =
                        inner_area.width - space_left_horizontally - padding.3;
                    remaining_inner_area.height =
                        inner_area.height - space_left_vertically - padding.2;
                }
            }
        }

        self.measure_inner_children(
            &mut node_area,
            inner_area,
            &mut remaining_inner_area,
            &mut inner_width,
            &mut inner_height,
            node_relative_layer,
            must_memorize_layout,
        );

        // Unmark the node as dirty as it's layout has been calculted
        if must_recalculate && must_memorize_layout {
            self.layout_memorizer
                .lock()
                .unwrap()
                .remove_as_dirty(node_data.get_id());
        }

        match &node_data.get_type() {
            NodeType::Text { .. } => {}
            _ => {
                if let SizeMode::Auto = node_data.get_state().size.width {
                    node_area.width = remaining_inner_area.x - node_area.x + padding.1;
                }
                if let SizeMode::Auto = node_data.get_state().size.height {
                    node_area.height = remaining_inner_area.y - node_area.y + padding.0;
                }
            }
        }

        // Registers the element in the Layers handler
        if must_memorize_layout {
            self.layers.add_element(node_data, &node_area, node_layer);
        }

        if must_recalculate {
            // Notify the node's reference about the new size layout
            if let Some(reference) = &node_data.get_state().references.node_ref {
                reference
                    .send(NodeReferenceLayout {
                        x: node_area.x,
                        y: node_area.y,
                        width: node_area.width,
                        height: node_area.height,
                        inner_height,
                        inner_width,
                    })
                    .ok();
            }
        }

        node_area
    }

    pub fn resolve_child(&'a self, child_id: &NodeId) -> Option<DOMNode> {
        (self.node_resolver)(child_id, self.resolver_options)
    }

    /// Measure the node of the inner children
    #[allow(clippy::too_many_arguments)]
    pub fn measure_inner_children(
        &mut self,
        node_area: &mut NodeArea,
        inner_area: NodeArea,
        remaining_area: &mut NodeArea,
        inner_width: &mut f32,
        inner_height: &mut f32,
        node_relative_layer: i16,
        must_memorize_layout: bool,
    ) {
        let direction = &self.dom_node.get_state().size.direction;
        let cursor_settings = &self.dom_node.get_state().cursor_settings;
        match &self.dom_node.get_type() {
            NodeType::Element { tag, .. } => {
                if let Some(children) = &self.dom_node.get_children() {
                    for child in children {
                        let child_node = self.resolve_child(child);
                        if let Some(mut child_node) = child_node {
                            let child_node_area = {
                                let mut child_measurer = NodeLayoutMeasurer {
                                    dom_node: &mut child_node,
                                    remaining_area,
                                    parent_area: inner_area,
                                    resolver_options: self.resolver_options,
                                    layers: self.layers,
                                    node_resolver: self.node_resolver,
                                    inherited_relative_layer: node_relative_layer,
                                    font_collection: self.font_collection,
                                    layout_memorizer: self.layout_memorizer,
                                };
                                child_measurer.measure_area(must_memorize_layout)
                            };

                            match direction {
                                DirectionMode::Vertical => {
                                    remaining_area.height -= child_node_area.height;
                                    remaining_area.y = child_node_area.y + child_node_area.height;

                                    // Accumulate all heights
                                    *inner_height += child_node_area.height;

                                    // Only save the biggest width
                                    if *inner_width < child_node_area.width {
                                        *inner_width = child_node_area.width;
                                    }
                                }
                                DirectionMode::Horizontal => {
                                    remaining_area.width -= child_node_area.width;
                                    remaining_area.x = child_node_area.x + child_node_area.width;

                                    // Accumulate all widths
                                    *inner_width += child_node_area.width;

                                    // Only save the biggest height
                                    if *inner_height < child_node_area.height {
                                        *inner_height = child_node_area.height;
                                    }
                                }
                                DirectionMode::Both => {
                                    remaining_area.height -= child_node_area.height;
                                    remaining_area.width -= child_node_area.width;
                                    remaining_area.y = child_node_area.y + child_node_area.height;
                                    remaining_area.x = child_node_area.x + child_node_area.width;

                                    // Accumulate all heights and widths
                                    *inner_height += child_node_area.height;
                                    *inner_width += child_node_area.width;
                                }
                            }

                            if child_node_area.width > remaining_area.width
                                || remaining_area.width == 0.0
                            {
                                remaining_area.width = child_node_area.width;
                            }

                            if child_node_area.height > remaining_area.height
                                || remaining_area.height == 0.0
                            {
                                remaining_area.height = child_node_area.height;
                            }
                        }
                    }
                    // Use SkParagraph to measure the layout of a `paragraph` node.
                    if tag == "paragraph" && CursorMode::Editable == cursor_settings.mode {
                        let font_size = self.dom_node.get_state().font_style.font_size;
                        let font_family = &self.dom_node.get_state().font_style.font_family;
                        let align = self.dom_node.get_state().font_style.align;
                        let max_lines = self.dom_node.get_state().font_style.max_lines;
                        let font_style = self.dom_node.get_state().font_style.font_style;

                        let mut paragraph_style = ParagraphStyle::default();
                        paragraph_style.set_text_align(align);
                        paragraph_style.set_max_lines(max_lines);
                        paragraph_style.set_replace_tab_characters(true);

                        let mut paragraph_builder =
                            ParagraphBuilder::new(&paragraph_style, self.font_collection.clone());

                        paragraph_builder.push_style(
                            TextStyle::new()
                                .set_font_style(font_style)
                                .set_font_size(font_size)
                                .set_font_families(&[font_family]),
                        );

                        let texts =
                            get_inner_texts(children, self.node_resolver, self.resolver_options);

                        for node_text in texts {
                            paragraph_builder.push_style(
                                TextStyle::new()
                                    .set_font_style(node_text.0.font_style.font_style)
                                    .set_height_override(true)
                                    .set_height(node_text.0.font_style.line_height)
                                    .set_color(node_text.0.font_style.color)
                                    .set_font_size(node_text.0.font_style.font_size)
                                    .set_font_families(&[node_text
                                        .0
                                        .font_style
                                        .font_family
                                        .clone()]),
                            );
                            paragraph_builder.add_text(node_text.1.clone());
                        }

                        let mut paragraph = paragraph_builder.build();
                        paragraph.layout(node_area.width);

                        if let Some((cursor_ref, cursor_id, positions)) =
                            get_cursor_reference(self.dom_node)
                        {
                            // Calculate the new cursor position
                            let char_position =
                                paragraph.get_glyph_position_at_coordinate(positions);

                            // Notify the cursor reference listener
                            cursor_ref
                                .agent
                                .send((char_position.position as usize, cursor_id))
                                .ok();
                        }
                    }
                }
            }
            NodeType::Text { text } => {
                let line_height = self.dom_node.get_state().font_style.line_height;
                let font_size = self.dom_node.get_state().font_style.font_size;
                let font_family = &self.dom_node.get_state().font_style.font_family;
                let align = self.dom_node.get_state().font_style.align;
                let max_lines = self.dom_node.get_state().font_style.max_lines;
                let font_style = self.dom_node.get_state().font_style.font_style;

                let mut paragraph_style = ParagraphStyle::default();
                paragraph_style.set_text_align(align);
                paragraph_style.set_max_lines(max_lines);
                paragraph_style.set_replace_tab_characters(true);

                let mut paragraph_builder =
                    ParagraphBuilder::new(&paragraph_style, self.font_collection.clone());

                paragraph_builder.push_style(
                    TextStyle::new()
                        .set_font_style(font_style)
                        .set_font_size(font_size)
                        .set_font_families(&[font_family]),
                );

                paragraph_builder.add_text(text);

                let mut paragraph = paragraph_builder.build();
                paragraph.layout(node_area.width);

                let lines_count = paragraph.line_number() as f32;
                node_area.width = paragraph.longest_line();
                node_area.height = (line_height * font_size) * lines_count;
            }
            NodeType::Placeholder => {}
        }
    }
}
