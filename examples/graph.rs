#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{common::EventMessage, prelude::*};
use freya_node_state::parse_color;
use skia_safe::{
    textlayout::{ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle},
    Color, Paint, PaintStyle,
};
use winit::event_loop::EventLoopProxy;

fn main() {
    launch(app);
}

#[derive(Debug, Props, PartialEq, Clone)]
struct GraphProps {
    labels: Vec<String>,
    data: Vec<Line>,
}

#[allow(non_snake_case)]
fn Graph(cx: Scope<GraphProps>) -> Element {
    let event_loop_proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();
    let state = use_state(cx, || cx.props.clone());
    let state_setter = state.setter();

    use_effect(cx, (cx.props,), move |(data,)| {
        state_setter(data.clone());
        async move {
            if let Some(event_loop_proxy) = &event_loop_proxy {
                event_loop_proxy
                    .send_event(EventMessage::RequestRerender)
                    .unwrap();
            }
        }
    });

    let canvas = use_canvas(cx, || {
        to_owned![state];
        Box::new(move |canvas, font_collection, region| {
            canvas.translate((region.min_x(), region.min_y()));

            let state = state.get();

            let mut paragraph_style = ParagraphStyle::default();
            paragraph_style.set_text_align(TextAlign::Center);
            let mut text_style = TextStyle::new();
            text_style.set_color(Color::BLACK);
            paragraph_style.set_text_style(&text_style);

            let x_labels = &state.labels;

            let x_height: f32 = 50.0;

            let start_x = region.min_x();
            let start_y = region.max_y() - x_height;
            let height = region.height() - x_height;

            let space_x = region.width() / x_labels.len() as f32;

            let (smallest_y, biggest_y) = {
                let mut smallest_y = 0;
                let mut biggest_y = 0;
                for line in state.data.iter() {
                    let max = line.points.iter().max().unwrap();
                    let min = line.points.iter().min().unwrap();

                    if let Some(max) = *max {
                        if max > biggest_y {
                            biggest_y = max;
                        }
                    }
                    if let Some(min) = *min {
                        if min < smallest_y {
                            smallest_y = min;
                        }
                    }
                }

                (smallest_y, biggest_y)
            };

            let y_len = biggest_y - smallest_y;
            let space_y = height / y_len as f32;

            for line in &state.data {
                let mut paint = Paint::default();

                paint.set_anti_alias(true);
                paint.set_style(PaintStyle::Fill);
                paint.set_color(parse_color(&line.color).unwrap());
                paint.set_stroke_width(3.0);

                let mut previous_x = None;
                let mut previous_y = None;

                for (i, y_point) in line.points.iter().enumerate() {
                    let x = (space_x * i as f32) + start_x + (space_x / 2.0);
                    let new_previous_x = previous_x.unwrap_or(x);

                    if let Some(y_point) = y_point {
                        let y = start_y - (space_y * ((y_point - smallest_y) as f32));
                        let new_previous_y = previous_y.unwrap_or(y);

                        canvas.draw_circle((x, y), 5.0, &paint);
                        canvas.draw_line((new_previous_x, new_previous_y), (x, y), &paint);
                        previous_y = Some(y);
                        previous_x = Some(x);
                    } else {
                        previous_y = None;
                        previous_x = None;
                    }
                }
            }

            let space_x = region.width() / x_labels.len() as f32;

            for (i, point) in x_labels.iter().enumerate() {
                let x = (space_x * i as f32) + start_x;

                let mut paragrap_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
                paragrap_builder.add_text(point);
                let mut text = paragrap_builder.build();
                text.layout(space_x);
                text.paint(canvas, (x, start_y + x_height - 30.0));
            }

            canvas.restore();
        })
    });

    render!(Canvas {
        canvas: canvas,
        background: "white",
        width: "100%",
        height: "100%"
    })
}

#[derive(Debug, PartialEq, Clone)]
enum LineType {
    Scatter,
}

#[derive(Debug, PartialEq, Clone)]
struct Line {
    line_type: LineType,
    color: String,
    points: Vec<Option<i32>>,
}

fn app(cx: Scope) -> Element {
    let labels = cx.use_hook(|| {
        vec![
            "15/5/23".to_string(),
            "16/5/23".to_string(),
            "17/5/23".to_string(),
            "18/5/23".to_string(),
            "19/5/23".to_string(),
        ]
    });
    let data = use_state(cx, || {
        vec![
            Line {
                line_type: LineType::Scatter,
                color: "rgb(255, 184, 76)".to_string(),
                points: vec![Some(45), Some(5), Some(182), Some(105), Some(60)],
            },
            Line {
                line_type: LineType::Scatter,
                color: "rgb(44, 211, 225)".to_string(),
                points: vec![Some(80), Some(20), Some(50), Some(90), Some(150)],
            },
            Line {
                line_type: LineType::Scatter,
                color: "rgb(27, 156, 133)".to_string(),
                points: vec![Some(200), Some(150), Some(100), Some(130), Some(40)],
            },
            Line {
                line_type: LineType::Scatter,
                color: "rgb(210, 83, 128)".to_string(),
                points: vec![Some(20), Some(50), Some(80), Some(110), Some(140)],
            },
        ]
    });

    render!(
        rect {
            width: "100%",
            height: "100%",
            padding: "5",
            Graph {
                labels: labels.clone(),
                data: data.get().clone()
            }
        }
    )
}
