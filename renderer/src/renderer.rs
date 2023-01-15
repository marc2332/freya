use dioxus_native_core::node::NodeType;
use dioxus_native_core::tree::TreeView;
use dioxus_native_core::NodeId;
use freya_layers::{DioxusNode, RenderData};
use freya_node_state::NodeState;
use freya_processor::ViewportsCollection;
use skia_safe::textlayout::{Paragraph, RectHeightStyle, RectWidthStyle, TextHeightBehavior};
use skia_safe::Color;
use skia_safe::{
    svg,
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
    BlurStyle, Canvas, ClipOp, Data, IRect, Image, MaskFilter, Paint, PaintStyle, Path,
    PathDirection, Rect,
};

use crate::SafeDOM;

pub fn render_skia(
    dom: &SafeDOM,
    canvas: &mut Canvas,
    node: &RenderData,
    font_collection: &mut FontCollection,
    viewports_collection: &ViewportsCollection,
) {
    if let NodeType::Element { tag, .. } = &node.get_type() {
        let children = node.children.as_ref();
        let viewports = viewports_collection.get(node.get_id());
        if let Some((_, viewports)) = viewports {
            for viewport_id in viewports {
                let viewport = viewports_collection.get(viewport_id).unwrap().0;
                if let Some(viewport) = viewport {
                    canvas.clip_rect(
                        Rect::new(
                            viewport.x,
                            viewport.y,
                            viewport.x + viewport.width,
                            viewport.y + viewport.height,
                        ),
                        ClipOp::Intersect,
                        true,
                    );
                }
            }
        }

        match tag.as_str() {
            "rect" | "container" => {
                let shadow = &node.get_state().style.shadow;

                #[cfg(not(feature = "wireframe"))]
                if node.get_state().style.background == Color::TRANSPARENT && shadow.intensity == 0
                {
                    return;
                }

                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                paint.set_style(PaintStyle::Fill);
                paint.set_color(node.get_state().style.background);

                let radius = node.get_state().style.radius;
                let radius = if radius < 0.0 { 0.0 } else { radius };

                let ((x, y), (x2, y2)) = node.node_area.get_rect();

                let mut path = Path::new();
                path.add_round_rect(
                    Rect::new(x as f32, y as f32, x2 as f32, y2 as f32),
                    (radius, radius),
                    PathDirection::CW,
                );
                path.close();

                // Shadow effect
                {
                    if shadow.intensity > 0 {
                        let mut blur_paint = paint.clone();

                        blur_paint.set_color(shadow.color);
                        blur_paint.set_alpha(shadow.intensity);
                        blur_paint.set_mask_filter(MaskFilter::blur(
                            BlurStyle::Normal,
                            shadow.size,
                            false,
                        ));
                        canvas.draw_path(&path, &blur_paint);
                    }
                }

                canvas.draw_path(&path, &paint);
            }
            "label" => {
                if let Some(children) = children {
                    let font_size = node.get_state().font_style.font_size;
                    let font_family = &node.get_state().font_style.font_family;
                    let font_color = node.get_state().font_style.color;
                    let align = node.get_state().font_style.align;
                    let font_style = node.get_state().font_style.font_style;

                    let mut paint = Paint::default();

                    paint.set_anti_alias(true);
                    paint.set_style(PaintStyle::StrokeAndFill);
                    paint.set_color(node.get_state().font_style.color);

                    let child_id = children.get(0);

                    let text = if let Some(child_id) = child_id {
                        let dom = dom.lock().unwrap();
                        if let Some(child) = dom.get(*child_id) {
                            if let NodeType::Text { text } = &child.node_data.node_type {
                                Some(text.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if let Some(text) = text {
                        let x = node.node_area.x;
                        let y = node.node_area.y;

                        let mut paragraph_style = ParagraphStyle::default();
                        paragraph_style.set_text_align(align);
                        paragraph_style.set_text_style(
                            TextStyle::new()
                                .set_font_style(font_style)
                                .set_color(font_color)
                                .set_font_size(font_size)
                                .set_font_families(&[font_family]),
                        );
                        let mut paragraph_builder =
                            ParagraphBuilder::new(&paragraph_style, font_collection.clone());

                        paragraph_builder.add_text(text);

                        let mut paragraph = paragraph_builder.build();

                        paragraph.layout(node.node_area.width + 1.0);

                        paragraph.paint(canvas, (x, y));
                    }
                }
            }
            "paragraph" => {
                if let Some(children) = children {
                    let align = node.get_state().font_style.align;
                    let max_lines = node.get_state().font_style.max_lines;

                    let texts = get_inner_texts(children, dom);

                    let (x, y) = node.node_area.get_origin_points();

                    let mut paragraph_style = ParagraphStyle::default();
                    paragraph_style.set_max_lines(max_lines);
                    paragraph_style.set_text_align(align);
                    paragraph_style.set_replace_tab_characters(true);
                    paragraph_style.set_text_height_behavior(TextHeightBehavior::DisableAll);

                    let mut paragraph_builder =
                        ParagraphBuilder::new(&paragraph_style, font_collection.clone());

                    for node_text in &texts {
                        paragraph_builder.push_style(
                            TextStyle::new()
                                .set_font_style(node_text.0.font_style.font_style)
                                .set_height_override(true)
                                .set_height(node_text.0.font_style.line_height)
                                .set_color(node_text.0.font_style.color)
                                .set_font_size(node_text.0.font_style.font_size)
                                .set_font_families(&[node_text.0.font_style.font_family.clone()]),
                        );
                        paragraph_builder.add_text(node_text.1.clone());
                    }

                    if node.get_state().cursor_settings.position.is_some() {
                        // This is very tricky, but it works! It allows freya to render the cursor at the end of a line.
                        paragraph_builder.add_text(" ");
                    }

                    let mut paragraph = paragraph_builder.build();

                    paragraph.layout(node.node_area.width);

                    paragraph.paint(canvas, (x, y));

                    // Draw a cursor if specified
                    draw_cursor(node, paragraph, canvas);
                }
            }
            "svg" => {
                let x = node.node_area.x;
                let y = node.node_area.y;
                if let Some(svg_data) = &node.get_state().style.svg_data {
                    let svg_dom = svg::Dom::from_bytes(svg_data);
                    if let Ok(mut svg_dom) = svg_dom {
                        canvas.save();
                        canvas.translate((x, y));
                        svg_dom.set_container_size((
                            node.node_area.width as i32,
                            node.node_area.height as i32,
                        ));
                        svg_dom.render(canvas);
                        canvas.restore();
                    }
                }
            }
            "image" => {
                let mut draw_img = |bytes: &[u8]| {
                    let pic = Image::from_encoded(unsafe { Data::new_bytes(bytes) });
                    if let Some(pic) = pic {
                        let mut paint = Paint::default();
                        paint.set_anti_alias(true);
                        canvas.draw_image_nine(
                            pic,
                            IRect::new(0, 0, 0, 0),
                            Rect::new(
                                node.node_area.x,
                                node.node_area.y,
                                node.node_area.x + node.node_area.width,
                                node.node_area.y + node.node_area.height,
                            ),
                            skia_safe::FilterMode::Last,
                            Some(&paint),
                        );
                    }
                };

                if let Some(image_ref) = &node.get_state().references.image_ref {
                    let image_data = image_ref.0.lock().unwrap();
                    if let Some(image_data) = image_data.as_ref() {
                        draw_img(image_data)
                    }
                } else if let Some(image_data) = &node.get_state().style.image_data {
                    draw_img(image_data)
                }
            }
            _ => {}
        }

        #[cfg(feature = "wireframe")]
        {
            let mut paint = Paint::default();

            paint.set_anti_alias(true);
            paint.set_style(PaintStyle::Fill);
            paint.set_color(Color::MAGENTA);

            let x = node.node_area.x;
            let y = node.node_area.y;

            let x2 = x + node.node_area.width;
            let y2 = if node.node_area.height < 0.0 {
                y
            } else {
                y + node.node_area.height
            };

            canvas.draw_line((x, y), (x2, y), &paint);
            canvas.draw_line((x2, y), (x2, y2), &paint);
            canvas.draw_line((x2, y2), (x, y2), &paint);
            canvas.draw_line((x, y2), (x, y), &paint);
        }
    }
}

fn get_inner_texts(children: &[NodeId], dom: &SafeDOM) -> Vec<(NodeState, String)> {
    children
        .iter()
        .filter_map(|child_id| {
            let (child, children): (DioxusNode, Vec<NodeId>) = {
                let dom = dom.lock().unwrap();
                let children = dom.tree.children_ids(*child_id).map(|v| v.to_vec());
                (dom.get(*child_id).cloned()?, children?)
            };

            if let NodeType::Element { tag, .. } = child.node_data.node_type {
                if tag != "text" {
                    return None;
                }
                let child_text_id = children.get(0)?;
                let child_text: DioxusNode = {
                    let dom = dom.lock().unwrap();
                    dom.get(*child_text_id).cloned()
                }?;
                if let NodeType::Text { text } = &child_text.node_data.node_type {
                    Some((child.state, text.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<(NodeState, String)>>()
}

fn draw_cursor(node: &RenderData, paragraph: Paragraph, canvas: &mut Canvas) -> Option<()> {
    let cursor = node.get_state().cursor_settings.position?;
    let cursor_color = node.get_state().cursor_settings.color;
    let cursor_position = cursor as usize;

    let cursor_rects = paragraph.get_rects_for_range(
        cursor_position..cursor_position + 1,
        RectHeightStyle::Tight,
        RectWidthStyle::Tight,
    );
    let cursor_rect = cursor_rects.first()?;

    let x = node.node_area.x + cursor_rect.rect.left;
    let y = node.node_area.y + cursor_rect.rect.top;

    let x2 = x + 1.0;
    let y2 = y + (cursor_rect.rect.bottom - cursor_rect.rect.top);

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(cursor_color);

    canvas.draw_rect(Rect::new(x, y, x2, y2), &paint);

    Some(())
}
