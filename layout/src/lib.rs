use dioxus_native_core::{node::NodeType, real_dom::RealDom, tree::TreeView, NodeId};
use freya_common::{LayoutMemorizer, NodeArea, NodeLayoutInfo, NodeReferenceLayout};
use freya_node_state::{
    CursorMode, CursorReference, CustomAttributeValues, DirectionMode, DisplayMode, FontStyle,
    NodeState, SizeMode,
};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use std::sync::{Arc, Mutex};

mod area_calc;
mod layers;
mod ops_calc;

use area_calc::calculate_area;
pub use layers::*;
pub use ops_calc::run_calculations;

/// Collect all the texts and node states from a given array of children
fn get_inner_texts(dom: &SafeDOM, node_id: &NodeId) -> Vec<(FontStyle, String)> {
    let children: Vec<DioxusNode> = dom
        .lock()
        .unwrap()
        .tree
        .children(*node_id)
        .unwrap()
        .cloned()
        .collect();
    children
        .iter()
        .filter_map(|child| {
            if let NodeType::Element { tag, .. } = &child.node_data.node_type {
                if tag != "text" {
                    return None;
                }

                let dom = dom.lock().unwrap();
                let child_text = dom
                    .tree
                    .children(child.node_data.node_id)
                    .unwrap()
                    .next()?
                    .clone();

                if let NodeType::Text { text } = child_text.node_data.node_type {
                    Some((child.state.font_style.clone(), text))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

/// Get the info related to a cursor reference
fn get_cursor_reference(node: &DioxusNode) -> Option<(&CursorReference, usize, (f32, f32))> {
    let cursor_ref = node.state.references.cursor_ref.as_ref()?;
    let positions = { *cursor_ref.positions.lock().unwrap().as_ref()? };
    let current_cursor_id = { *cursor_ref.id.lock().unwrap().as_ref()? };
    let cursor_id = node.state.cursor_settings.id.as_ref()?;

    if current_cursor_id == *cursor_id {
        Some((cursor_ref, *cursor_id, positions))
    } else {
        None
    }
}

pub type SafeDOM = Arc<Mutex<RealDom<NodeState, CustomAttributeValues>>>;

/// Measure the layout of a given Node and all it's children
pub struct NodeLayoutMeasurer<'a> {
    node: DioxusNode,
    node_id: NodeId,
    remaining_area: &'a mut NodeArea,
    parent_area: NodeArea,
    layers: &'a mut Layers,
    dom: &'a SafeDOM,
    inherited_relative_layer: i16,
    font_collection: &'a mut FontCollection,
    layout_memorizer: &'a Arc<Mutex<LayoutMemorizer>>,
}

impl<'a> NodeLayoutMeasurer<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        node: DioxusNode,
        remaining_area: &'a mut NodeArea,
        parent_area: NodeArea,
        dom: &'a SafeDOM,
        layers: &'a mut Layers,
        inherited_relative_layer: i16,
        font_collection: &'a mut FontCollection,
        layout_memorizer: &'a Arc<Mutex<LayoutMemorizer>>,
    ) -> Self {
        Self {
            node_id: node.node_data.node_id,
            node,
            remaining_area,
            parent_area,
            dom,
            layers,
            inherited_relative_layer,
            font_collection,
            layout_memorizer,
        }
    }

    pub fn run_dirty_checks(&mut self) -> (bool, bool) {
        let parent_id = self.dom.lock().unwrap().tree.parent_id(self.node_id);
        let is_parent_dirty = parent_id
            .map(|p| self.layout_memorizer.lock().unwrap().is_dirty(&p))
            .unwrap_or(false);

        // If parent is dirty, mark this node as dirty too
        if is_parent_dirty {
            self.layout_memorizer
                .lock()
                .unwrap()
                .mark_as_dirty(self.node_id);
        }

        let is_dirty = self
            .layout_memorizer
            .lock()
            .unwrap()
            .is_dirty(&self.node_id);

        let is_cached = self
            .layout_memorizer
            .lock()
            .unwrap()
            .is_node_layout_memorized(&self.node_id);

        // If this node is dirty and parent is not dirty, mark the parent as dirty
        if is_dirty && !is_parent_dirty {
            if let Some(p) = parent_id {
                self.layout_memorizer.lock().unwrap().mark_as_dirty(p)
            }
        }

        (is_dirty, is_cached)
    }

    /// Measure the area of a node
    pub fn measure_area(&mut self, must_memorize_layout: bool) -> NodeArea {
        let node_height = self.dom.lock().unwrap().tree.height(self.node_id).unwrap();

        let direction = self.node.state.size.direction;
        let padding = self.node.state.size.padding;
        let relative_layer = self.node.state.style.relative_layer;
        let scroll_y = self.node.state.scroll.scroll_y;
        let scroll_x = self.node.state.scroll.scroll_x;

        // Caculate the corresponding layer of this node
        let (node_layer, node_relative_layer) = self.layers.calculate_layer(
            relative_layer,
            node_height as i16,
            self.inherited_relative_layer,
        );

        let (is_dirty, is_cached) = self.run_dirty_checks();
        let must_recalculate = is_dirty || !is_cached;

        let (
            mut node_area,
            mut remaining_inner_area,
            inner_area,
            mut inner_width,
            mut inner_height,
        ) = if must_recalculate {
            // TODO Change to the new version
            let node_area = calculate_area(self, &self.node);

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
            remaining_inner_area.y += scroll_y;
            remaining_inner_area.x += scroll_x;

            if must_memorize_layout {
                // Memorize these layouts
                self.layout_memorizer.lock().unwrap().memorize_layout(
                    self.node_id,
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
                .get_node_layout(&self.node_id)
                .unwrap()
                .as_tuple()
        };

        // Calculate the children layouts  for the first time without the size and axis adjusted.
        if DisplayMode::Center == self.node.state.style.display {
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
                .remove_as_dirty(&self.node_id);
        }

        match &self.node.node_data.node_type {
            NodeType::Text { .. } => {}
            _ => {
                if let SizeMode::Auto = self.node.state.size.width {
                    node_area.width = remaining_inner_area.x - node_area.x + padding.1;
                }
                if let SizeMode::Auto = self.node.state.size.height {
                    node_area.height = remaining_inner_area.y - node_area.y + padding.0;
                }
            }
        }

        // Registers the element in the Layers handler
        if must_memorize_layout {
            let node_children = self
                .dom
                .lock()
                .unwrap()
                .tree
                .children_ids(self.node_id)
                .map(|v| v.to_vec());
            self.layers
                .add_element(&self.node, node_children, &node_area, node_layer);
        }

        if must_recalculate {
            // Notify the node's reference about the new size layout
            if let Some(reference) = &self.node.state.references.node_ref {
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
        let node = &self.node;
        let direction = node.state.size.direction;
        let cursor_settings = &node.state.cursor_settings;
        match &node.node_data.node_type {
            NodeType::Element { tag, .. } => {
                let node_children: Option<Vec<DioxusNode>> = self
                    .dom
                    .lock()
                    .unwrap()
                    .tree
                    .children(self.node_id)
                    .map(|children| children.cloned().collect());
                if let Some(children) = node_children {
                    for child in children.into_iter() {
                        let child_node_area = {
                            let mut child_measurer = NodeLayoutMeasurer {
                                node_id: child.node_data.node_id,
                                node: child,
                                remaining_area,
                                parent_area: inner_area,
                                dom: self.dom,
                                layers: self.layers,
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
                    // Use SkParagraph to measure the layout of a `paragraph` node.
                    if tag == "paragraph" && CursorMode::Editable == cursor_settings.mode {
                        let font_size = node.state.font_style.font_size;
                        let font_family = &node.state.font_style.font_family;
                        let align = node.state.font_style.align;
                        let max_lines = node.state.font_style.max_lines;
                        let font_style = node.state.font_style.font_style;

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

                        let texts = get_inner_texts(self.dom, &self.node_id);

                        for (font_style, text) in texts.into_iter() {
                            paragraph_builder.push_style(
                                TextStyle::new()
                                    .set_font_style(font_style.font_style)
                                    .set_height_override(true)
                                    .set_height(font_style.line_height)
                                    .set_color(font_style.color)
                                    .set_font_size(font_style.font_size)
                                    .set_font_families(&[font_style.font_family.clone()]),
                            );
                            paragraph_builder.add_text(text);
                        }

                        let mut paragraph = paragraph_builder.build();
                        paragraph.layout(node_area.width);

                        if let Some((cursor_ref, cursor_id, positions)) = get_cursor_reference(node)
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
                let line_height = node.state.font_style.line_height;
                let font_size = node.state.font_style.font_size;
                let font_family = &node.state.font_style.font_family;
                let align = node.state.font_style.align;
                let max_lines = node.state.font_style.max_lines;
                let font_style = node.state.font_style.font_style;

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
