use dioxus_native_core::{
    node::NodeType,
    prelude::{ElementNode, TextNode},
    real_dom::{NodeImmutable, RealDom},
    tree::TreeRef,
    NodeId,
};
use freya_common::{Area, CursorLayoutResponse, NodeReferenceLayout, Point2D};
use freya_dom::{DioxusNode, FreyaDOM};
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
pub fn get_inner_texts(node: &DioxusNode) -> Vec<(FontStyle, String)> {
    node.children()
        .iter()
        .filter_map(|child| {
            if let NodeType::Element(ElementNode { tag, .. }) = &*child.node_type() {
                if tag != "text" {
                    return None;
                }

                let children = child.children();
                let child_text = *children.first().unwrap();
                let child_text_type = &*child_text.node_type();

                if let NodeType::Text(TextNode { text, .. }) = child_text_type {
                    let font_style = child.get::<FontStyle>().unwrap();
                    Some((font_style.clone(), text.to_owned()))
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
fn get_cursor_reference(
    node: &DioxusNode,
) -> Option<(
    CursorReference,
    usize,
    Option<Point2D>,
    Option<(Point2D, Point2D)>,
)> {
    let node_references = node.get::<References>().unwrap();
    let cursor_settings = node.get::<CursorSettings>().unwrap();

    let cursor_ref = node_references.cursor_ref.clone()?;
    let cursor_id = cursor_settings.cursor_id?;

    let current_cursor_id = { *cursor_ref.cursor_id.lock().unwrap().as_ref()? };
    let cursor_selections = *cursor_ref.cursor_selections.lock().unwrap();
    let cursor_position = *cursor_ref.cursor_position.lock().unwrap();

    if current_cursor_id == cursor_id {
        Some((cursor_ref, cursor_id, cursor_position, cursor_selections))
    } else {
        None
    }
}

/// Measure the layout of a given Node and all it's children
pub struct NodeLayoutMeasurer<'a> {
    node: DioxusNode<'a>,
    node_id: NodeId,
    remaining_area: &'a mut Area,
    parent_area: Area,
    layers: &'a mut Layers,
    dom: &'a FreyaDOM,
    inherited_relative_layer: i16,
    font_collection: &'a mut FontCollection,
}

impl<'a> NodeLayoutMeasurer<'a> {
    /// Create a NodeLayoutMeasurer
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        node: DioxusNode<'a>,
        remaining_area: &'a mut Area,
        parent_area: Area,
        dom: &'a FreyaDOM,
        layers: &'a mut Layers,
        inherited_relative_layer: i16,
        font_collection: &'a mut FontCollection,
    ) -> Self {
        Self {
            node_id: node.id(),
            node,
            remaining_area,
            parent_area,
            dom,
            layers,
            inherited_relative_layer,
            font_collection,
        }
    }

    /// Measure the area of a Node
    pub fn measure_area(&mut self, is_measuring: bool, scale_factor: f32) -> Area {
        let node_height = self.dom.dom().tree_ref().height(self.node_id).unwrap();

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

        let mut inner_size = Area::from_size(
            (
                node_size.padding.1 + node_size.padding.3,
                node_size.padding.0 + node_size.padding.2,
            )
                .into(),
        );

        // Area that is available consideing the parent area
        let mut remaining_inner_area = Area {
            origin: (
                node_area.min_x() + node_size.padding.3,
                node_area.min_y() + node_size.padding.0,
            )
                .into(),
            size: (
                node_area.width() - inner_size.width(),
                node_area.height() - inner_size.height(),
            )
                .into(),
        };

        // Visible area occupied by the child elements
        let inner_area = remaining_inner_area;

        // Increase the x and y axis with the node's scroll attributes
        remaining_inner_area.origin.x += node_scroll.scroll_x;
        remaining_inner_area.origin.y += node_scroll.scroll_y;

        // Calculate the children layouts  for the first time without the size and axis adjusted.
        if DisplayMode::Center == node_style.display {
            self.measure_inner_children(
                &mut node_area,
                inner_area,
                &mut remaining_inner_area,
                &mut inner_size,
                node_relative_layer,
                false,
                scale_factor,
            );

            let space_left_horizontally = (inner_area.width() - inner_size.width()) / 2.0;
            let space_left_vertically = (inner_area.height() - inner_size.height()) / 2.0;

            match node_size.direction {
                DirectionMode::Vertical => {
                    remaining_inner_area.origin.y =
                        inner_area.min_y() + space_left_vertically + node_size.padding.0;
                    remaining_inner_area.size.height =
                        inner_area.height() - space_left_vertically - node_size.padding.2;
                }
                DirectionMode::Horizontal => {
                    remaining_inner_area.origin.x =
                        inner_area.min_x() + space_left_horizontally + node_size.padding.1;
                    remaining_inner_area.size.width =
                        inner_area.width() - space_left_horizontally - node_size.padding.3;
                }
                DirectionMode::Both => {
                    remaining_inner_area.origin.x =
                        inner_area.min_x() + space_left_horizontally + node_size.padding.1;
                    remaining_inner_area.origin.y =
                        inner_area.min_y() + space_left_vertically + node_size.padding.0;

                    remaining_inner_area.size.width =
                        inner_area.width() - space_left_horizontally - node_size.padding.3;
                    remaining_inner_area.size.height =
                        inner_area.height() - space_left_vertically - node_size.padding.2;
                }
            }
        }

        self.measure_inner_children(
            &mut node_area,
            inner_area,
            &mut remaining_inner_area,
            &mut inner_size,
            node_relative_layer,
            is_measuring,
            scale_factor,
        );

        match *self.node.node_type() {
            NodeType::Text { .. } => {}
            _ => {
                if let SizeMode::Auto = node_size.width {
                    if DirectionMode::Vertical == node_size.direction {
                        node_area.size.width = inner_size.width();
                    } else {
                        node_area.size.width =
                            remaining_inner_area.min_x() - node_area.min_x() + node_size.padding.3;
                    }
                }
                if let SizeMode::Auto = node_size.height {
                    if DirectionMode::Vertical == node_size.direction {
                        node_area.size.height = inner_size.height();
                    } else {
                        node_area.size.height =
                            remaining_inner_area.min_y() - node_area.min_y() + node_size.padding.2;
                    }
                }
            }
        }

        if is_measuring {
            // Add element to a layer
            self.layers.add_element(&self.node, &node_area, node_layer);

            // Notify the node's reference about the new size layout
            if let Some(reference) = &node_references.node_ref {
                let mut layout = NodeReferenceLayout {
                    area: node_area,
                    inner: inner_size,
                };
                layout.div(scale_factor);
                reference.send(layout).ok();
            }
        }

        node_area
    }

    /// Construct a paragraph with all it's inner texts and notify
    /// the cursor reference where the positions are located in the text
    fn notify_cursor_reference(&self, node_area: &Area) {
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
        paragraph.layout(node_area.width());

        if let Some((cursor_ref, id, cursor_position, cursor_selections)) =
            get_cursor_reference(&self.node)
        {
            if let Some(cursor_position) = cursor_position {
                // Calculate the new cursor position
                let char_position =
                    paragraph.get_glyph_position_at_coordinate(cursor_position.to_i32().to_tuple());

                // Notify the cursor reference listener
                cursor_ref
                    .agent
                    .send(CursorLayoutResponse::CursorPosition {
                        position: char_position.position as usize,
                        id,
                    })
                    .ok();
            }

            if let Some((origin, dist)) = cursor_selections {
                let origin_char =
                    paragraph.get_glyph_position_at_coordinate(origin.to_i32().to_tuple());
                let dist_char =
                    paragraph.get_glyph_position_at_coordinate(dist.to_i32().to_tuple());

                cursor_ref
                    .agent
                    .send(CursorLayoutResponse::TextSelection {
                        from: origin_char.position as usize,
                        to: dist_char.position as usize,
                        id,
                    })
                    .ok();
            }
        }
    }

    /// Measure the node of the inner children
    #[allow(clippy::too_many_arguments)]
    pub fn measure_inner_children(
        &mut self,
        node_area: &mut Area,
        inner_area: Area,
        remaining_area: &mut Area,
        inner_size: &mut Area,
        node_relative_layer: i16,
        must_memorize_layout: bool,
        scale_factor: f32,
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
                            dom: self.dom,
                            layers: self.layers,
                            inherited_relative_layer: node_relative_layer,
                            font_collection: self.font_collection,
                        };
                        child_measurer.measure_area(must_memorize_layout, scale_factor)
                    };

                    match node_size.direction {
                        DirectionMode::Vertical => {
                            remaining_area.size.height -= child_node_area.height();
                            remaining_area.origin.y =
                                child_node_area.origin.y + child_node_area.height();

                            // Accumulate all heights
                            inner_size.size.height += child_node_area.height();

                            // Only save the biggest width
                            if inner_size.size.width < child_node_area.width() {
                                inner_size.size.width = child_node_area.width();
                            }
                        }
                        DirectionMode::Horizontal => {
                            remaining_area.size.width -= child_node_area.width();
                            remaining_area.origin.x =
                                child_node_area.min_x() + child_node_area.width();

                            // Accumulate all widths
                            inner_size.size.width += child_node_area.width();

                            // Only save the biggest height
                            if inner_size.size.width < child_node_area.height() {
                                inner_size.size.width = child_node_area.height();
                            }
                        }
                        DirectionMode::Both => {
                            remaining_area.size.height -= child_node_area.height();
                            remaining_area.size.width -= child_node_area.width();
                            remaining_area.origin.y =
                                child_node_area.min_y() + child_node_area.height();
                            remaining_area.origin.x =
                                child_node_area.min_x() + child_node_area.width();

                            // Accumulate all heights and widths
                            inner_size.size.height += child_node_area.height();
                            inner_size.size.width += child_node_area.width();
                        }
                    }

                    if child_node_area.width() > remaining_area.width()
                        || remaining_area.width() == 0.0
                    {
                        remaining_area.size.width = child_node_area.width();
                    }

                    if child_node_area.height() > remaining_area.height()
                        || remaining_area.height() == 0.0
                    {
                        remaining_area.size.height = child_node_area.height();
                    }
                }

                // Use SkParagraph to measure the layout of a `paragraph` and calculate the position of the cursor
                if tag == "paragraph" && node_cursor_settings.mode == CursorMode::Editable {
                    self.notify_cursor_reference(node_area);
                }
            }
            NodeType::Text(TextNode { text, .. }) => {
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
                        .set_font_families(font_family),
                );

                paragraph_builder.add_text(text);

                let mut paragraph = paragraph_builder.build();
                paragraph.layout(node_area.width());

                let lines_count = paragraph.line_number() as f32;
                node_area.size.width = paragraph.longest_line();
                node_area.size.height = (line_height * font_size) * lines_count;
            }
            NodeType::Placeholder => {}
        }
    }
}
