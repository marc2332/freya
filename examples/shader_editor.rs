use std::{
    sync::Arc,
    time::Instant,
};

use freya::{
    events::MouseEvent,
    prelude::*,
};
use skia_safe::{
    textlayout::{
        ParagraphBuilder,
        ParagraphStyle,
    },
    Color,
    Data,
    Paint,
    Rect,
    RuntimeEffect,
};

const SHADER: &str = "
 uniform vec2 u_resolution;
 uniform float u_time;

 vec4 main(vec2 cords) {
     vec2 U = cords / 55.;

     float t = .8* u_time;
     float r = ceil(U.x + t) + ceil(U.y + t);
     float v = mod(r, 4.) > 1. ? U.x : U.y;
     float b = step(fract(v+.2), .5);

     vec4 C = vec4(.9*b, 0. + abs(sin(t) * 0.5), .6-b, 1.);
     return C;
 }
 ";

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_width(900.0)
            .with_height(500.0)
            .with_title("Shader Editor"),
    );
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    let editable = use_editable(
        || EditableConfig::new(SHADER.trim().to_string()),
        EditableMode::SingleLineMultipleEditors,
    );

    rsx!(
        Body {
            rect {
                direction: "horizontal",
                ShaderEditor {
                    editable
                },
                ShaderView {
                    editable
                }
            }
        }
    )
}

#[component]
fn ShaderEditor(editable: UseEditable) -> Element {
    let cursor_reference = editable.cursor_attr();
    let editor = editable.editor().read();

    let onglobalclick = move |_: MouseEvent| {
        editable.process_event(&EditableEvent::Click);
    };

    let onkeydown = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyDown(e.data));
    };

    let onkeyup = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyUp(e.data));
    };

    rsx!(
        rect {
            onkeydown,
            onkeyup,
            onglobalclick,
            cursor_reference,
            width: "50%",
            height: "fill",
            VirtualScrollView {
                length: editor.len_lines(),
                item_size: 27.0,
                scroll_with_arrows: false,
                cache_elements: false,
                builder: move |line_index, _: &Option<()>| {
                    let editor = editable.editor().read();
                    let line = editor.line(line_index).unwrap();

                    let is_line_selected = editor.cursor_row() == line_index;

                    // Only show the cursor in the active line
                    let character_index = if is_line_selected {
                        editor.visible_cursor_col().to_string()
                    } else {
                        "none".to_string()
                    };

                    // Only highlight the active line
                    let line_background = if is_line_selected {
                        "rgb(37, 37, 37)"
                    } else {
                        ""
                    };

                    let onmousedown = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseDown(e.data, line_index));
                    };

                    let onmousemove = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseMove(e.data, line_index));
                    };

                    let highlights = editable.highlights_attr(line_index);

                    rsx! {
                        rect {
                            key: "{line_index}",
                            width: "100%",
                            height: "27",
                            direction: "horizontal",
                            background: "{line_background}",
                            label {
                                main_align: "center",
                                width: "30",
                                height: "100%",
                                text_align: "center",
                                font_size: "15",
                                color: "rgb(200, 200, 200)",
                                "{line_index + 1} "
                            }
                            paragraph {
                                main_align: "center",
                                height: "100%",
                                width: "100%",
                                cursor_index: "{character_index}",
                                cursor_color: "white",
                                max_lines: "1",
                                cursor_mode: "editable",
                                cursor_id: "{line_index}",
                                onmousedown,
                                onmousemove,
                                highlights,
                                highlight_mode: "expanded",
                                text {
                                    color: "rgb(240, 240, 240)",
                                    font_size: "15",
                                    "{line}"
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn ShaderView(editable: UseEditable) -> Element {
    let platform = use_platform();

    use_hook(|| {
        let mut ticker = platform.new_ticker();

        spawn(async move {
            loop {
                ticker.tick().await;
                platform.request_animation_frame();
            }
        });
    });

    let canvas = use_canvas(move || {
        let editor = editable.editor().read();
        let runtime_effect = RuntimeEffect::make_for_shader(editor.to_string(), None);
        let shared_runtime_effect = Arc::new(RuntimeEffectWrapper(runtime_effect));
        let instant = Instant::now();

        Box::new(move |canvas, font_collection, region, _| {
            canvas.save();

            let runtime_effect = &shared_runtime_effect.0;

            if let Ok(runtime_effect) = runtime_effect {
                let mut builder = UniformsBuilder::default();
                builder.set(
                    "u_resolution",
                    UniformValue::FloatVec(vec![region.width(), region.height()]),
                );
                builder.set(
                    "u_time",
                    UniformValue::Float(instant.elapsed().as_secs_f32()),
                );

                let uniforms = Data::new_copy(&builder.build(&runtime_effect));

                let shader = runtime_effect.make_shader(uniforms, &[], None).unwrap();

                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                paint.set_color(Color::WHITE);
                paint.set_shader(shader);

                canvas.draw_rect(
                    Rect::new(
                        region.min_x(),
                        region.min_y(),
                        region.max_x(),
                        region.max_y(),
                    ),
                    &paint,
                );
            } else if let Err(err) = runtime_effect {
                let mut text_paint = Paint::default();
                text_paint.set_anti_alias(true);
                text_paint.set_color(Color::WHITE);
                let mut paragraph_builder =
                    ParagraphBuilder::new(&ParagraphStyle::default(), font_collection.clone());
                paragraph_builder.add_text(err);
                let mut paragraph = paragraph_builder.build();
                paragraph.layout(region.width());

                paragraph.paint(canvas, (region.min_x(), region.min_y()));
            }

            canvas.restore();
        })
    });

    rsx!(rect {
        width: "fill",
        height: "fill",
        background: "black",
        canvas_reference: canvas.attribute()
    })
}

struct RuntimeEffectWrapper(Result<RuntimeEffect, String>);

unsafe impl Sync for RuntimeEffectWrapper {}
unsafe impl Send for RuntimeEffectWrapper {}
