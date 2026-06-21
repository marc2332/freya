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
        let center_x = context.size.width / 2.0;
        let center_y = context.size.height / 2.0;

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
