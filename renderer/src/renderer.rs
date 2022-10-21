use dioxus_native_core::real_dom::{Node, NodeType};
use dioxus_native_core::traversable::Traversable;
use freya_layers::{NodeArea, NodeData};
use freya_node_state::node::NodeState;
use skia_safe::textlayout::{RectHeightStyle, RectWidthStyle, TextHeightBehavior};
use skia_safe::{
    svg,
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
    BlurStyle, Canvas, ClipOp, Data, IRect, Image, MaskFilter, Paint, PaintStyle, Path,
    PathDirection, Rect,
};

use crate::SkiaDom;

pub fn render_skia(
    dom: &mut &SkiaDom,
    canvas: &mut &mut Canvas,
    node_data: &NodeData,
    area: &NodeArea,
    font_collection: &mut FontCollection,
    viewports: &Vec<NodeArea>,
) {
    let node = &node_data.node;

    let mut must_skip = false;
    for v in viewports {
        if area.x + area.width < v.x || area.y + area.height < v.y || area.x  > v.x + v.width || area.y  > v.y + v.height {
            must_skip = true;
            break;
        }
    }

    if must_skip {
        return;
    }

    for viewport in viewports {
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

    

    match &node.node_type {
        NodeType::Element { tag, children, .. } => {
            match tag.as_str() {
                "rect" | "container" => {
                    let mut paint = Paint::default();

                    paint.set_anti_alias(true);
                    paint.set_style(PaintStyle::Fill);
                    paint.set_color(node.state.style.background);

                    let x = area.x;
                    let y = area.y;

                    let x2 = x + area.width;
                    let y2 = y + area.height;

                    let radius = node.state.style.radius;
                    let radius = if radius < 0.0 { 0.0 } else { radius };

                    let mut path = Path::new();

                    path.add_round_rect(
                        Rect::new(x as f32, y as f32, x2 as f32, y2 as f32),
                        (radius as f32, radius as f32),
                        PathDirection::CW,
                    );

                    path.close();

                    // Shadow effect
                    {
                        let shadow = &node.state.style.shadow;

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
                    let font_size = node.state.font_style.font_size;
                    let font_family = &node.state.font_style.font_family;
                    let font_color = node.state.font_style.color;
                    let align = node.state.font_style.align;

                    let mut paint = Paint::default();

                    paint.set_anti_alias(true);
                    paint.set_style(PaintStyle::StrokeAndFill);
                    paint.set_color(node.state.font_style.color);

                    let child_id = children.get(0);

                    let text = if let Some(child_id) = child_id {
                        let dom = dom.lock().unwrap();
                        if let Some(child) = dom.get(*child_id) {
                            if let NodeType::Text { text } = &child.node_type {
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
                        let x = area.x;
                        let y = area.y;

                        let mut paragraph_style = ParagraphStyle::default();
                        paragraph_style.set_text_align(align);
                        paragraph_style.set_text_style(
                            TextStyle::new()
                                .set_color(font_color)
                                .set_font_size(font_size)
                                .set_font_families(&[font_family]),
                        );
                        let mut paragraph_builder =
                            ParagraphBuilder::new(&paragraph_style, font_collection.clone());

                        paragraph_builder.add_text(text);

                        let mut paragraph = paragraph_builder.build();

                        paragraph.layout(area.width + 1.0);

                        paragraph.paint(canvas, (x, y));
                    }
                }
                "paragraph" => {
                    let align = node.state.font_style.align;
                    let max_lines = node.state.font_style.max_lines;

                    let texts = children
                        .iter()
                        .filter_map(|child_id| {
                            let child: Option<Node<NodeState>> = {
                                let dom = dom.lock().unwrap();
                                dom.get(*child_id).map(|v| v.clone())
                            };

                            if let Some(child) = child {
                                if let NodeType::Element { tag, children, .. } = child.node_type {
                                    if tag != "text" {
                                        return None;
                                    }
                                    if let Some(child_text_id) = children.get(0) {
                                        let child_text: Option<Node<NodeState>> = {
                                            let dom = dom.lock().unwrap();
                                            dom.get(*child_text_id).map(|v| v.clone())
                                        };

                                        if let Some(child_text) = child_text {
                                            if let NodeType::Text { text } = &child_text.node_type {
                                                Some((child.state, text.clone()))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<(NodeState, String)>>();

                    let x = area.x;
                    let y = area.y;

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
                                .set_height_override(true)
                                .set_height(node_text.0.font_style.line_height)
                                .set_color(node_text.0.font_style.color)
                                .set_font_size(node_text.0.font_style.font_size)
                                .set_font_families(&[node_text.0.font_style.font_family.clone()]),
                        );
                        paragraph_builder.add_text(node_text.1.clone());
                    }

                    let mut paragraph = paragraph_builder.build();

                    paragraph.layout(area.width);

                    paragraph.paint(canvas, (x, y));

                    // Draw a cursor if specified
                    if let Some(cursor) = node.state.cursor_settings.position {
                        let cursor_color = node.state.cursor_settings.color;
                        let cursor_position = cursor as usize;

                        let cursor_rects = paragraph.get_rects_for_range(
                            cursor_position..cursor_position + 1,
                            RectHeightStyle::Tight,
                            RectWidthStyle::Tight,
                        );
                        let cursor_rect = cursor_rects.first();

                        if let Some(cursor_rect) = cursor_rect {
                            let x = area.x + cursor_rect.rect.left;
                            let y = area.y + cursor_rect.rect.top;

                            let x2 = x + 1.0;
                            let y2 = y + (cursor_rect.rect.bottom - cursor_rect.rect.top);

                            let mut paint = Paint::default();
                            paint.set_anti_alias(true);
                            paint.set_style(PaintStyle::Fill);
                            paint.set_color(cursor_color);

                            let mut path = Path::new();
                            path.add_rect(
                                Rect::new(x as f32, y as f32, x2 as f32, y2 as f32),
                                None,
                            );
                            path.close();

                            canvas.draw_path(&path, &paint);
                        }
                    }
                }
                "svg" => {
                    let x = area.x;
                    let y = area.y;
                    if let Some(svg_data) = &node.state.style.svg_data {
                        let svg_dom = svg::Dom::from_bytes(svg_data);
                        if let Ok(mut svg_dom) = svg_dom {
                            canvas.save();
                            canvas.translate((x, y));
                            svg_dom.set_container_size((area.width as i32, area.height as i32));
                            svg_dom.render(canvas);
                            canvas.restore();
                        }
                    }
                }
                "image" => {
                    if let Some(image_data) = &node.state.style.image_data {
                        let pic = Image::from_encoded(unsafe { Data::new_bytes(image_data) });
                        if let Some(pic) = pic {
                            let mut paint = Paint::default();
                            paint.set_anti_alias(true);
                            canvas.draw_image_nine(
                                pic,
                                &IRect::new(0, 0, 0, 0),
                                Rect::new(
                                    area.x,
                                    area.y,
                                    area.x + area.width,
                                    area.y + area.height,
                                ),
                                skia_safe::FilterMode::Last,
                                Some(&paint),
                            );
                        }
                    }
                }
                _ => {}
            }

            #[cfg(feature = "wireframe")]
            {
                use skia_safe::Color;
                let mut paint = Paint::default();

                paint.set_anti_alias(true);
                paint.set_style(PaintStyle::Fill);
                paint.set_color(Color::MAGENTA);

                let x = area.x;
                let y = area.y;

                let x2 = x + area.width;
                let y2 = if area.height < 0.0 {
                    y
                } else {
                    y + area.height
                };

                canvas.draw_line((x, y), (x2, y), &paint);
                canvas.draw_line((x2, y), (x2, y2), &paint);
                canvas.draw_line((x2, y2), (x, y2), &paint);
                canvas.draw_line((x, y2), (x, y), &paint);
            }
        }
        _ => {}
    }
}
