use freya::prelude::*;
use freya::dioxus_elements::MouseEvent;

fn main() {
    launch_cfg(vec![(
        app,
        WindowConfig::<()>::builder()
            .with_width(900)
            .with_height(500)
            .with_decorations(true)
            .with_transparency(false)
            .with_title("Editor")
            .build(),
    )]);
}



#[allow(non_snake_case)]
fn Test(cx: Scope) -> Element {
    render!(
        rect {
            ScrollView {
                width: "100%",
                height: "100%",
                show_scrollbar: true,
                (vec![0,1,2,3]).iter().enumerate().map(move |(i, l)| {
                    rsx! {
                        rect {
                            key:"{l}"
                        }
                    }
                })
            }
        }       
    )
}


fn app(cx: Scope) -> Element {
    use_init_default_theme(&cx);
    render!(
        Body {}
    )
}

#[allow(non_snake_case)]
fn Body(cx: Scope) -> Element {
    let theme = use_theme(&cx);
    println!("CALLED");
    let (content, cursor, process_keyevent, process_clickevent, cursor_ref) = use_editable(
        &cx,
        || {
            "AALorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet"
        },
        EditableMode::SingleLineMultipleEditors,
    );
    let font_size_percentage = use_state(&cx, || 15.0);
    let line_height_percentage = use_state(&cx, || 0.0);
    let is_bold = use_state(&cx, || false);
    let is_italic = use_state(&cx, || false);

    // minimum font size is 5
    let font_size = font_size_percentage + 5.0;
    let line_height = (line_height_percentage / 25.0) + 1.2;
    let mut line_index = 0;

    let cursor_char = content.offset_of_line(cursor.1) + cursor.0;

    let font_style = {
        if *is_bold.get() && *is_italic.get() {
            "bold-italic"
        } else if *is_italic.get() {
            "italic"
        } else if *is_bold.get() {
            "bold"
        } else {
            "normal"
        }
    };

    content.lines(0..);

    render!(
        rect {
            width: "100%",
            height: "100%",
            rect {
                width: "100%",
                height: "60",
                padding: "20",
                direction: "horizontal",
                background: "rgb(20, 20, 20)",
                rect {
                    height: "100%",
                    width: "100%",
                    direction: "horizontal",
                    padding: "10",
                    label {
                        font_size: "30",
                        "Editor"
                    }
                    rect {
                        width: "20",
                    }
                    rect {
                        height: "40%",
                        display: "center",
                        width: "130",
                        Slider {
                            width: 100.0,
                            value: *font_size_percentage.get(),
                            onmoved: |p| {
                                font_size_percentage.set(p);
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            display: "center",
                            direction: "horizontal",
                            label {
                                "Font size"
                            }
                        }
    
                    }
                    rect {
                        height: "40%",
                        display: "center",
                        direction: "vertical",
                        width: "130",
                        Slider {
                            width: 100.0,
                            value: *line_height_percentage.get(),
                            onmoved: |p| {
                                line_height_percentage.set(p);
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            display: "center",
                            direction: "horizontal",
                            label {
                                "Line height"
                            }
                        }
                    }
                    rect {
                        height: "40%",
                        display: "center",
                        direction: "vertical",
                        width: "60",
                        Switch {
                            enabled: *is_bold.get(),
                            ontoggled: |_| {
                                is_bold.set(!is_bold.get());
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            display: "center",
                            direction: "horizontal",
                            label {
                                "Bold"
                            }
                        }
                    }
                    rect {
                        height: "40%",
                        display: "center",
                        direction: "vertical",
                        width: "60",
                        Switch {
                            enabled: *is_italic.get(),
                            ontoggled: |_| {
                                is_italic.set(!is_italic.get());
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            display: "center",
                            direction: "horizontal",
                            label {
                                "Italic"
                            }
                        }
                    }
                }
            }
            rect {
                width: "100%",
                height: "calc(100% - 90)",
                padding: "20",
                onkeydown: move |e| {
                   process_keyevent.send(e.data).unwrap();
                },
                cursor_reference: cursor_ref.read().clone(),
                direction: "horizontal",
                background: "{theme.body.background}",
                rect {
                    width: "50%",
                    height: "100%",
                    padding: "30",
                    vec![0,0,0,0,0,0,0,0].iter().enumerate().map(move |(i, _)| {
                        println!("{}", i);                       
                        rsx! {
                            rect {
                                key: "{i}",
                                label {
                                    "{i}"
                                }
                            }
                        }
                    })
                }
                rect {
                    background: "{theme.body.background}",
                    radius: "15",
                    width: "50%",
                    height: "100%",
                    padding: "30",
                    shadow: "0 10 30 7 white",
                    ScrollView {
                        width: "100%",
                        height: "100%",
                        show_scrollbar: true,
                        paragraph {
                            width: "100%",
                            cursor_index: "{cursor_char}",
                            cursor_color: "white",
                            line_height: "{line_height}",
                            text {
                                color: "white",
                                font_size: "{font_size}",
                                "{content}"
                            }
                        }
                    }
                }
            }
            rect {
                width: "100%",
                height: "30",
                background: "rgb(20, 20, 20)",
                direction: "horizontal",
                padding: "10",
                label {
                    color: "rgb(200, 200, 200)",
                    "Ln {cursor.1 + 1}, Col {cursor.0 + 1}"
                }
            }
        }
    )
}