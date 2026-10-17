#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;
use skia_safe::Paint;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut alternate = use_state(|| false);
    let is_alternate = alternate();

    let callback = RenderCallback::new(move |context| {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(if is_alternate {
            Color::RED
        } else {
            Color::BLUE
        });

        match &mut context.target {
            RenderTarget::Background { canvas, .. } => {
                let center_x = context.size.width * if is_alternate { 0.7 } else { 0.3 };
                let center_y = context.size.height / 4.0;
                canvas.draw_circle((center_x, center_y), 50.0, &paint);
            }
            RenderTarget::TextStyle(text_style) => {
                text_style.set_foreground_paint(&paint);
            }
        }
    });

    rect()
        .background(callback.clone())
        .expanded()
        .center()
        .on_press(move |_| alternate.toggle())
        .child(
            label()
                .font_size(32.)
                .color(callback)
                .text("Click to change color"),
        )
}
