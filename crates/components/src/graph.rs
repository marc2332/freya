use dioxus::prelude::*;
use freya_common::EventMessage;
use freya_elements::elements as dioxus_elements;
use freya_engine::prelude::*;

use freya_hooks::{use_applied_theme, use_canvas, use_platform, GraphTheme, GraphThemeWith};
use freya_node_state::Parse;

/// Data line for the [`Graph`] component.
#[derive(Debug, PartialEq, Clone)]
pub struct GraphLine {
    color: String,
    points: Vec<Option<i32>>,
}

impl GraphLine {
    pub fn new(color: &str, points: Vec<Option<i32>>) -> Self {
        Self {
            color: color.to_string(),
            points,
        }
    }
}

/// Properties for the [`Graph`] component.
#[derive(Debug, Props, PartialEq, Clone)]
pub struct GraphProps {
    /// Theme override.
    pub theme: Option<GraphThemeWith>,
    /// X axis labels.
    labels: Vec<String>,
    /// Y axis data.
    data: Vec<GraphLine>,
}

/// Graph component.
#[allow(non_snake_case)]
pub fn Graph(props: GraphProps) -> Element {
    let platform = use_platform();
    let GraphTheme { width, height } = use_applied_theme!(&props.theme, graph);

    use_effect(use_reactive(&props, move |_| {
        platform.send(EventMessage::RequestRerender).ok();
    }));

    let canvas = use_canvas(&props, |state| {
        Box::new(move |canvas, font_collection, region| {
            canvas.translate((region.min_x(), region.min_y()));

            let mut paragraph_style = ParagraphStyle::default();
            paragraph_style.set_text_align(TextAlign::Center);

            let mut text_style = TextStyle::new();
            text_style.set_color(Color::BLACK);
            paragraph_style.set_text_style(&text_style);

            let x_labels = &state.labels;
            let x_height: f32 = 50.0;

            let start_x = region.min_x();
            let start_y = region.height() - x_height;
            let height = region.height() - x_height;

            let space_x = region.width() / x_labels.len() as f32;

            // Calculate the smallest and biggest items
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

            // Difference between the smalles and biggest Y Axis item
            let y_axis_len = biggest_y - smallest_y;
            // Space between items in the Y axis
            let space_y = height / y_axis_len as f32;

            // Draw the lines
            for line in &state.data {
                let mut paint = Paint::default();

                paint.set_anti_alias(true);
                paint.set_style(PaintStyle::Fill);
                paint.set_color(Color::parse(&line.color).unwrap());
                paint.set_stroke_width(3.0);

                let mut previous_x = None;
                let mut previous_y = None;

                for (i, y_point) in line.points.iter().enumerate() {
                    let line_x = (space_x * i as f32) + start_x + (space_x / 2.0);
                    // Save the position where the last point drawed
                    let new_previous_x = previous_x.unwrap_or(line_x);

                    if let Some(y_point) = y_point {
                        let line_y = start_y - (space_y * ((y_point - smallest_y) as f32));
                        let new_previous_y = previous_y.unwrap_or(line_y);

                        // Draw the line and circle
                        canvas.draw_circle((line_x, line_y), 5.0, &paint);
                        canvas.draw_line(
                            (new_previous_x, new_previous_y),
                            (line_x, line_y),
                            &paint,
                        );

                        previous_y = Some(line_y);
                        previous_x = Some(line_x);
                    } else {
                        previous_y = None;
                        previous_x = None;
                    }
                }
            }

            // Space between labels
            let space_x = region.width() / x_labels.len() as f32;

            // Draw the labels
            for (i, point) in x_labels.iter().enumerate() {
                let x = (space_x * i as f32) + start_x;

                let mut paragrap_builder =
                    ParagraphBuilder::new(&paragraph_style, font_collection.clone());
                paragrap_builder.add_text(point);
                let mut text = paragrap_builder.build();

                text.layout(space_x);
                text.paint(canvas, (x, start_y + x_height - 30.0));
            }

            canvas.restore();
        })
    });

    rsx!(
        rect {
            width: "{width}",
            height: "{height}",
            padding: "15 5",
            background: "white",
            rect {
                canvas_reference: canvas.attribute(),
                width: "100%",
                height: "100%",
            }
        }
    )
}
