use dioxus_native_core::{node_ref::NodeView, real_dom::Node};
use skia_safe::{Paint, PaintStyle, Path};

use crate::{
    node::{NodeState, SizeMode},
    run::RenderContext,
};

pub fn container(
    node: &Node<NodeState>,
    context: &RenderContext,
) -> ((Path, Paint), (f32, f32), (i32, i32), (f32, f32)) {
    let mut path = Path::new();
    let mut paint = Paint::default();

    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(node.state.style.background);

    let mut x = context.x;
    let mut y = context.y;
    let mut width = match node.state.size.width {
        SizeMode::AUTO => 0.0,
        SizeMode::STRETCH => context.width as f32,
        SizeMode::Manual(w) => w,
    };
    let mut height = match node.state.size.height {
        SizeMode::AUTO => 0.0,
        SizeMode::STRETCH => context.height as f32,
        SizeMode::Manual(h) => h,
    };

    let padding = node.state.size.padding;
    let horizontal_padding = padding.1 + padding.3;
    let vertical_padding = padding.0 + padding.2;

    path.move_to((x, y));
    path.line_to((width as i32, y));
    path.line_to((width, height));
    path.line_to((x, height as i32));

    (
        (path, paint),
        (horizontal_padding, vertical_padding),
        (x, y),
        (width, height),
    )
}
