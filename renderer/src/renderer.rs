use dioxus_native_core::real_dom::{Node, NodeType};
use layers_engine::{NodeArea, NodeData};
use skia_safe::{
    utils::text_utils::Align, BlurStyle, Canvas, ClipOp, Color, Font, MaskFilter, Paint,
    PaintStyle, Path, PathDirection, Rect,
};
use state::node::NodeState;
use std::ops::Index;

use crate::SkiaDom;

pub fn render_skia(
    dom: &mut &SkiaDom,
    canvas: &mut &mut Canvas,
    node: &NodeData,
    area: &NodeArea,
    font: &Font,
    viewports: &Vec<NodeArea>,
) {
    let node = node.node.as_ref().unwrap();

    for viewport in viewports {
        canvas.clip_rect(
            Rect::new(
                viewport.x as f32,
                viewport.y as f32,
                (viewport.x + viewport.width) as f32,
                (viewport.y + viewport.height) as f32,
            ),
            ClipOp::Intersect,
            true,
        );
    }

    match &node.node_type {
        NodeType::Element { tag, children, .. } => {
            match tag.as_str() {
                "view" | "container" => {
                    let mut paint = Paint::default();

                    paint.set_anti_alias(true);
                    paint.set_style(PaintStyle::Fill);
                    paint.set_color(node.state.style.background);

                    let x = area.x;
                    let y = area.y;

                    let x2 = x + area.width;
                    let y2 = y + area.height;

                    //

                    let radius = node.state.style.radius;
                    let radius = if radius < 0 { 0 } else { radius };

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
                "text" => {
                    let mut paint = Paint::default();

                    paint.set_anti_alias(true);
                    paint.set_style(PaintStyle::StrokeAndFill);
                    paint.set_color(Color::WHITE);

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
                    let y = area.y + 12; /* Line height, wip */

                    canvas.draw_str_align(text, (x, y), &font, &paint, Align::Left);
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
                let y2 = if area.height < 0 { y } else { y + area.height };

                canvas.draw_line((x as f32, y as f32), (x2 as f32, y as f32), &paint);
                canvas.draw_line((x2 as f32, y as f32), (x2 as f32, y2 as f32), &paint);
                canvas.draw_line((x2 as f32, y2 as f32), (x as f32, y2 as f32), &paint);
                canvas.draw_line((x as f32, y2 as f32), (x as f32, y as f32), &paint);

                path.close();
            }
        }
        _ => {}
    }
}
