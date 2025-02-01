#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use skia_safe::{
    textlayout::{
        FontCollection,
        ParagraphBuilder,
        ParagraphStyle,
        TextAlign,
        TextStyle,
    },
    Color,
    Paint,
    Point,
};

fn main() {
    launch_with_props(app, "Speedometer", (400., 400.));
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    let animation = use_animation(|_conf| {
        AnimNum::new(0., 255.)
            .time(1600)
            .ease(Ease::Out)
            .function(Function::Expo)
    });

    let speed = animation.get().read().read() as u8;

    let min = move |_| {
        animation.run(AnimDirection::Reverse);
    };

    let max = move |_| {
        animation.run(AnimDirection::Forward);
    };

    rsx!(
        rect {
            background: "black",
            width: "100%",
            height: "100%",
            main_align: "center",
            cross_align: "center",
            Speedometer {
                width: 200.,
                height: 200.,
                speed
            }
            rect {
                direction: "horizontal",
                main_align: "center",
                cross_align: "center",
                spacing: "4",
                Button {
                    onpress: min,
                    label {
                        "🛑"
                    }
                }
                Button {
                    onpress: max,
                    label {
                        "🏎️"
                    }
                }
            }
        }
    )
}

#[component]
fn Speedometer(speed: ReadOnlySignal<u8>, width: f32, height: f32) -> Element {
    let platform = use_platform();
    let (reference, size) = use_node_signal();

    let canvas = use_canvas(move || {
        platform.invalidate_drawing_area(size.peek().area);
        platform.request_animation_frame();
        let speed = speed();
        Box::new(move |ctx| {
            ctx.canvas.translate((ctx.area.min_x(), ctx.area.min_y()));

            draw_speedometer(ctx.canvas, &ctx.area, ctx.font_collection, speed);

            ctx.canvas.restore();
        })
    });

    rsx!(rect {
        canvas_reference: canvas.attribute(),
        reference,
        width: "{width}",
        height: "{height}",
    })
}

fn draw_speedometer(
    canvas: &skia_safe::Canvas,
    area: &Area,
    font_collection: &FontCollection,
    speed: u8,
) {
    let center = Point::new(area.width() * 0.5, area.height() * 0.5);
    let radius = 80.0;

    // Draw the outer circle
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(skia_safe::paint::Style::Stroke);
    paint.set_stroke_width(2.0);
    paint.set_color(Color::from_rgb(245, 245, 245));

    canvas.draw_circle(center, radius, &paint);

    // Draw the tick marks
    paint.set_stroke_width(2.);
    for i in (0..=100).step_by(2) {
        let angle = std::f32::consts::PI * (1.0 + (i as f32 / 100.0));
        let outer = Point::new(
            center.x + radius * angle.cos(),
            center.y + radius * angle.sin(),
        );
        let inner = Point::new(
            center.x + (radius - 20.0) * angle.cos(),
            center.y + (radius - 20.0) * angle.sin(),
        );

        canvas.draw_line(inner, outer, &paint);
    }

    // Draw the label
    draw_text(
        &speed.to_string(),
        canvas,
        font_collection,
        40.,
        center.x - 20.,
        center.y + 25.,
    );

    // Draw the needle
    paint.set_color(Color::RED);
    paint.set_stroke_width(3.0);
    let needle_angle = std::f32::consts::PI * ((speed as f32 / 255.) + 1.); // Pointing straight up
    let needle_end = Point::new(
        center.x + (radius - 25.0) * needle_angle.cos(),
        center.y + (radius - 25.0) * needle_angle.sin(),
    );
    canvas.draw_line(center, needle_end, &paint);
}

fn draw_text(
    text: &str,
    canvas: &skia_safe::Canvas,
    font_collection: &FontCollection,
    width: f32,
    x: f32,
    y: f32,
) {
    let mut style = ParagraphStyle::default();
    style.set_text_align(TextAlign::Center);
    let mut text_style = TextStyle::new();
    text_style.set_font_families(&["Inter"]);
    text_style.set_color(Color::from_rgb(245, 245, 245));
    text_style.set_font_size(20.0);
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection.clone());
    paragraph_builder.push_style(&text_style);
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(width);

    paragraph.paint(canvas, (x, y));
}
