#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    core::prelude::EventMessage,
    prelude::*,
};
use skia_safe::{
    Color,
    Font,
    FontStyle,
    Paint,
};

fn main() {
    launch(app);
}

fn app() -> Element {
    let platform = use_platform();
    let mut state = use_signal(|| 0);

    use_effect(move || {
        platform.send(EventMessage::RequestRerender).unwrap();
    });

    let canvas = use_canvas(move || {
        let state = *state.read();
        Box::new(move |ctx| {
            ctx.canvas.translate((ctx.area.min_x(), ctx.area.min_y()));

            let mut text_paint = Paint::default();
            text_paint.set_anti_alias(true);
            text_paint.set_color(Color::WHITE);
            let typefaces = ctx
                .font_collection
                .find_typefaces(&["Times New Roman"], FontStyle::default());
            let font = Font::new(
                typefaces
                    .first()
                    .expect("'Times New Roman' font not found."),
                50.0,
            );

            ctx.canvas.draw_str(
                format!("value is {}", state),
                ((ctx.area.max_x() / 2.0 - 120.0), ctx.area.max_y() / 2.0),
                &font,
                &text_paint,
            );

            ctx.canvas.restore();
        })
    });

    rsx!(
        rect {
            onclick: move |_| {
                state += 1;
            },
            Canvas {
                canvas,
                theme: theme_with!(CanvasTheme {
                    background: "black".into(),
                    width: "100%".into(),
                    height: "100%".into(),
                })
            }
        }
    )
}
