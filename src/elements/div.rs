use dioxus_native_core::real_dom::Node;
use skia_safe::{Paint, PaintStyle, Path};

use crate::{node::NodeState, run::RenderContext};

pub fn container(
    node: &Node<NodeState>,
    context: &RenderContext,
    (width, height): (i32, i32),
) -> ((Path, Paint), (i32, i32)) {
    let mut path = Path::new();
    let mut paint = Paint::default();

    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(node.state.style.background);

    let x = context.x;
    let y = context.y;

    let x2 = x + width;
    let y2 = y + height;

    path.move_to((x, y));
    path.line_to((x2, y));
    path.line_to((x2, y2));
    path.line_to((x, y2));

    ((path, paint), (x, y))
}
