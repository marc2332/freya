use dioxus_native_core::real_dom::{Node, NodeType};
use layers::{NodeArea, NodeData};
use skia_safe::{
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
    utils::text_utils::Align,
    BlurStyle, Canvas, ClipOp, Data, Font, FontStyle, IRect, Image, MaskFilter, Paint, PaintStyle,
    Path, PathDirection, Rect,
};
use state::node::NodeState;
use std::ops::Index;

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
                    let mut paint = Paint::default();

                    paint.set_anti_alias(true);
                    paint.set_style(PaintStyle::StrokeAndFill);
                    paint.set_color(node.state.font_style.color);

                    let child_id = children.get(0);

                    let text = if let Some(child_id) = child_id {
                        let child: Node<NodeState> = {
                            let dom = dom.lock().unwrap();
                            dom.index(*child_id).clone()
                        };

                        if let NodeType::Text { text } = child.node_type {
                            text
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    let x = area.x;
                    let y = area.y + node.state.font_style.font_size - 4.0; // TODO: Fix this, it's TOO MAGIC

                    let type_faces = font_collection.find_typefaces(
                        &[&node.state.font_style.font_family],
                        FontStyle::default(),
                    );

                    let type_face = type_faces.get(0);

                    let font = if let Some(type_face) = type_face {
                        Font::new(type_face, node.state.font_style.font_size)
                    } else {
                        let mut font = Font::default();
                        font.set_size(node.state.font_style.font_size);
                        font
                    };

                    canvas.draw_str_align(text, (x, y), &font, &paint, Align::Left);
                }
                "paragraph" => {
                    let texts = children
                        .iter()
                        .filter_map(|child_id| {
                            let child: Node<NodeState> = {
                                let dom = dom.lock().unwrap();
                                dom.index(*child_id).clone()
                            };

                            if let NodeType::Element { tag, children, .. } = child.node_type {
                                if tag != "text" {
                                    return None;
                                }
                                if let Some(child_text_id) = children.get(0) {
                                    let child_text: Node<NodeState> = {
                                        let dom = dom.lock().unwrap();
                                        dom.index(*child_text_id).clone()
                                    };
                                    if let NodeType::Text { text } = child_text.node_type {
                                        Some((child.state, text))
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

                    let paragraph_style = ParagraphStyle::default();

                    let mut paragraph_builder =
                        ParagraphBuilder::new(&paragraph_style, font_collection.clone());

                    for node_text in texts {
                        paragraph_builder.push_style(
                            TextStyle::new()
                                .set_color(node_text.0.font_style.color)
                                .set_font_size(node_text.0.font_style.font_size)
                                .set_font_families(&[node_text.0.font_style.font_family]),
                        );
                        paragraph_builder.add_text(node_text.1);
                    }

                    let mut paragraph = paragraph_builder.build();

                    paragraph.layout(area.width);

                    paragraph.paint(canvas, (x, y));
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
                                Rect::new(area.x, area.y, area.width, area.height),
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

                let mut path = Path::new();
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

                path.close();
            }
        }
        _ => {}
    }
}
