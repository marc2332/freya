#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;
use skia_safe::{
    Paint,
    PaintStyle,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    canvas(RenderCallback::new(|context| {
        let area = context.layout_node.visible_area();
        let center_x = area.center().x;
        let center_y = area.center().y;

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_color(Color::BLUE);

        context
            .canvas
            .draw_circle((center_x, center_y), 50.0, &paint);
    }))
    .width(Size::percent(100.))
    .height(Size::percent(100.))
}
