use dioxus_native_core::{
    node::NodeType,
    prelude::ElementNode,
    real_dom::{NodeImmutable, RealDom},
    tree::TreeRef,
    NodeId,
};
use freya_common::{NodeArea, NodeReferenceLayout};
use freya_node_state::{
    CursorMode, CursorReference, CursorSettings, CustomAttributeValues, DirectionMode, DisplayMode,
    FontStyle, References, Scroll, Size, SizeMode, Style,
};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};

mod area_calc;
mod layers;
mod ops_calc;

use area_calc::calculate_area;
pub use layers::*;
pub use ops_calc::run_calculations;

pub type DioxusDOM = RealDom<CustomAttributeValues>;

/// Collect all the texts and node states from a given array of children
fn get_inner_texts(node: &DioxusNode) -> Vec<(FontStyle, String)> {
    node.children()
        .iter()
        .filter_map(|child| {
            if let NodeType::Element(ElementNode { tag, .. }) = &*child.node_type() {
                if tag != "text" {
                    return None;
                }

                let children = child.children();
                let child_text = children.first().unwrap().clone();
                let child_text_type = &*child_text.node_type();

                if let NodeType::Text(text) = child_text_type {
                    let font_style = child_text.get::<FontStyle>().unwrap();
                    Some((font_style.clone(), text.text.clone()))
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
fn get_cursor_reference<'a>(
    node: &'a DioxusNode<'a>,
) -> Option<(CursorReference, usize, (f32, f32))> {
    let node_references = node.get::<References>().unwrap();
    let cursor_settings = node.get::<CursorSettings>().unwrap();

    let cursor_ref = node_references.cursor_ref.clone()?;
    let cursor_id = cursor_settings.cursor_id.clone()?;

    let positions = { *cursor_ref.positions.lock().unwrap().as_ref()? };
    let current_cursor_id = { *cursor_ref.cursor_id.lock().unwrap().as_ref()? };

    if current_cursor_id == cursor_id {
        Some((cursor_ref, cursor_id, positions))
    } else {
        None
    }
}

/// Measure the layout of a given Node and all it's children
pub struct NodeLayoutMeasurer<'a> {
    node: DioxusNode<'a>,
    node_id: NodeId,
    remaining_area: &'a mut NodeArea,
    parent_area: NodeArea,
    layers: &'a mut Layers,
    rdom: &'a DioxusDOM,
    inherited_relative_layer: i16,
    font_collection: &'a mut FontCollection,
}

impl<'a> NodeLayoutMeasurer<'a> {
    /// Create a NodeLayoutMeasurer
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        node: DioxusNode<'a>,
        remaining_area: &'a mut NodeArea,
        parent_area: NodeArea,
        dom: &'a DioxusDOM,
        layers: &'a mut Layers,
        inherited_relative_layer: i16,
        font_collection: &'a mut FontCollection,
    ) -> Self {
        Self {
            node_id: node.id(),
            node,
            remaining_area,
            parent_area,
            rdom: dom,
            layers,
            inherited_relative_layer,
            font_collection,
        }
    }

    /// Measure the area of a Node
    pub fn measure_area(&mut self, is_measuring: bool) -> NodeArea {
        let node = self.rdom.get(self.node_id).unwrap();
        let node_height = self.rdom.tree_ref().height(self.node_id).unwrap();

        let node_references = self.node.get::<References>().unwrap().clone();
        let node_size = self.node.get::<Size>().unwrap().clone();
        let node_style = self.node.get::<Style>().unwrap().clone();
        let node_scroll = self.node.get::<Scroll>().unwrap().clone();

        // Caculate the corresponding layer of this node
        let (node_layer, node_relative_layer) = self.layers.calculate_layer(
            node_style.relative_layer,
            node_height as i16,
            self.inherited_relative_layer,
        );

        let mut node_area = calculate_area(self);

        let mut inner_width = node_size.padding.1 + node_size.padding.3;
        let mut inner_height = node_size.padding.0 + node_size.padding.2;

        // Area that is available consideing the parent area
        let mut remaining_inner_area = NodeArea {
            x: node_area.x + node_size.padding.3,
            y: node_area.y + node_size.padding.0,
            width: node_area.width - inner_width,
            height: node_area.height - inner_height,
        };

        // Visible area occupied by the child elements
        let inner_area = remaining_inner_area;

        // Increase the x and y axis with the node's scroll attributes
        remaining_inner_area.y += node_scroll.scroll_y;
        remaining_inner_area.x += node_scroll.scroll_x;

        // Calculate the children layouts  for the first time without the size and axis adjusted.
        if DisplayMode::Center == node_style.display {
            self.measure_inner_children(
                &mut node_area,
                inner_area,
                &mut remaining_inner_area,
                &mut inner_width,
                &mut inner_height,
                node_relative_layer,
                false,
            );

            let space_left_vertically = (inner_area.height - inner_height) / 2.0;
            let space_left_horizontally = (inner_area.width - inner_width) / 2.0;

            match node_size.direction {
                DirectionMode::Vertical => {
                    remaining_inner_area.y =
                        inner_area.y + space_left_vertically + node_size.padding.0;
                    remaining_inner_area.height =
                        inner_area.height - space_left_vertically - node_size.padding.2;
                }
                DirectionMode::Horizontal => {
                    remaining_inner_area.x =
                        inner_area.x + space_left_horizontally + node_size.padding.1;
                    remaining_inner_area.width =
                        inner_area.width - space_left_horizontally - node_size.padding.3;
                }
                DirectionMode::Both => {
                    remaining_inner_area.x =
                        inner_area.x + space_left_horizontally + node_size.padding.1;
                    remaining_inner_area.y =
                        inner_area.y + space_left_vertically + node_size.padding.0;

                    remaining_inner_area.width =
                        inner_area.width - space_left_horizontally - node_size.padding.3;
                    remaining_inner_area.height =
                        inner_area.height - space_left_vertically - node_size.padding.2;
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
            is_measuring,
        );

        match *node.node_type() {
            NodeType::Text { .. } => {}
            _ => {
                if let SizeMode::Auto = node_size.width {
                    node_area.width = remaining_inner_area.x - node_area.x + node_size.padding.3;
                }
                if let SizeMode::Auto = node_size.height {
                    node_area.height = remaining_inner_area.y - node_area.y + node_size.padding.2;
                }
            }
        }

        // Registers the element in the Layers handler
        if is_measuring {
            self.layers.add_element(&self.node, &node_area, node_layer);
        }

        if is_measuring {
            // Notify the node's reference about the new size layout
            if let Some(reference) = &node_references.node_ref {
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

    /// Construct a paragraph with all it's inner texts and notify
    /// the cursor reference where the positions are located in the text
    fn notify_cursor_reference(&self, node_area: &NodeArea) {
        let font_style = self.node.get::<FontStyle>().unwrap();

        let mut paragraph_style = ParagraphStyle::default();
        paragraph_style.set_text_align(font_style.align);
        paragraph_style.set_max_lines(font_style.max_lines);
        paragraph_style.set_replace_tab_characters(true);

        let mut paragraph_builder =
            ParagraphBuilder::new(&paragraph_style, self.font_collection.clone());

        paragraph_builder.push_style(
            TextStyle::new()
                .set_font_style(font_style.font_style)
                .set_font_size(font_style.font_size)
                .set_font_families(&font_style.font_family),
        );

        let texts = get_inner_texts(&self.node);

        for (font_style, text) in texts.into_iter() {
            paragraph_builder.push_style(
                TextStyle::new()
                    .set_font_style(font_style.font_style)
                    .set_height_override(true)
                    .set_height(font_style.line_height)
                    .set_color(font_style.color)
                    .set_font_size(font_style.font_size)
                    .set_font_families(&font_style.font_family),
            );
            paragraph_builder.add_text(text);
        }

        let mut paragraph = paragraph_builder.build();
        paragraph.layout(node_area.width);

        if let Some((cursor_ref, cursor_id, positions)) = get_cursor_reference(&self.node) {
            // Calculate the new cursor position
            let char_position = paragraph.get_glyph_position_at_coordinate(positions);

            // Notify the cursor reference listener
            cursor_ref
                .agent
                .send((char_position.position as usize, cursor_id))
                .ok();
        }
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

        let node_size = &*self.node.get::<Size>().unwrap();
        let node_cursor_settings = &*self.node.get::<CursorSettings>().unwrap();

        match &*node.node_type() {
            NodeType::Element(ElementNode { tag, .. }) => {
                for child in node.children() {
                    let child_node_area = {
                        let mut child_measurer = NodeLayoutMeasurer {
                            node_id: child.id(),
                            node: child,
                            remaining_area,
                            parent_area: inner_area,
                            rdom: self.rdom,
                            layers: self.layers,
                            inherited_relative_layer: node_relative_layer,
                            font_collection: self.font_collection,
                        };
                        child_measurer.measure_area(must_memorize_layout)
                    };

                    match node_size.direction {
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

                    if child_node_area.width > remaining_area.width || remaining_area.width == 0.0 {
                        remaining_area.width = child_node_area.width;
                    }

                    if child_node_area.height > remaining_area.height
                        || remaining_area.height == 0.0
                    {
                        remaining_area.height = child_node_area.height;
                    }
                }

                // Use SkParagraph to measure the layout of a `paragraph` and calculate the position of the cursor
                if tag == "paragraph" && CursorMode::Editable == node_cursor_settings.mode {
                    self.notify_cursor_reference(node_area);
                }
            }
            NodeType::Text(text) => {
                let FontStyle {
                    font_family,
                    font_size,
                    line_height,
                    align,
                    max_lines,
                    font_style,
                    ..
                } = &*node.get::<FontStyle>().unwrap();

                let mut paragraph_style = ParagraphStyle::default();
                paragraph_style.set_text_align(*align);
                paragraph_style.set_max_lines(*max_lines);
                paragraph_style.set_replace_tab_characters(true);

                let mut paragraph_builder =
                    ParagraphBuilder::new(&paragraph_style, self.font_collection.clone());

                paragraph_builder.push_style(
                    TextStyle::new()
                        .set_font_style(*font_style)
                        .set_font_size(*font_size)
                        .set_font_families(&font_family),
                );

                paragraph_builder.add_text(&text.text);

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
