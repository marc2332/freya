use dioxus_native_core::real_dom::{Node, NodeType};
use layers_engine::{NodeArea, NodeData};
use skia_safe::{
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
    utils::text_utils::Align,
    BlurStyle, Canvas, ClipOp, Color, Data, Font, FontMgr, IRect, Image, MaskFilter, Paint,
    PaintStyle, Path, PathDirection, Rect,
};
use state::node::NodeState;
use std::ops::Index;

use crate::SkiaDom;

pub fn render_skia(
    dom: &mut &SkiaDom,
    canvas: &mut &mut Canvas,
    node_data: &NodeData,
    area: &NodeArea,
    font: &Font,
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
                    paint.set_color(node.state.style.color);

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
                    let y = area.y + 12.0; /* Line height, wip */

                    canvas.draw_str_align(text, (x, y), &font, &paint, Align::Left);
                }
                "paragraph" => {
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
                    let y = area.y + 12.0; /* Line height, wip */

                    let mut font_collection = FontCollection::new();
                    font_collection.set_default_font_manager(FontMgr::default(), "Fira Sans");

                    let mut paragraph_builder =
                        ParagraphBuilder::new(&ParagraphStyle::default(), &font_collection);

                    paragraph_builder.add_text(text);
                    paragraph_builder.push_style(
                        TextStyle::new()
                            .set_color(Color::WHITE)
                            .set_font_families(&["Fira Sans"]),
                    );

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
