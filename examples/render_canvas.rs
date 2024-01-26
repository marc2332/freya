#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{common::EventMessage, prelude::*};
use skia_safe::{Color, Font, FontStyle, Paint, Typeface};

fn main() {
    launch(app);
}

fn app() -> Element {
    let platform = use_platform();
    let mut state = use_signal(|| 0);

    use_effect(move || {
        platform.send(EventMessage::RequestRerender).unwrap();
    });

    let canvas = use_canvas(&*state.read(), |state| {
        Box::new(move |canvas, _, region| {
            canvas.translate((region.min_x(), region.min_y()));

            let mut text_paint = Paint::default();
            text_paint.set_anti_alias(true);
            text_paint.set_color(Color::WHITE);
            let font = Font::new(
                Typeface::from_name("Inter", FontStyle::default()).unwrap(),
                50.0,
            );

            canvas.draw_str(
                format!("value is {}", state),
                ((region.max_x() / 2.0 - 120.0), region.max_y() / 2.0),
                &font,
                &text_paint,
            );

            canvas.restore();
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
