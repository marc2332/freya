use freya::prelude::*;
use tree_sitter_highlight::*;
use tree_sitter_javascript::*;

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
fn Body(cx: Scope) -> Element {
    let theme = use_theme(&cx);
    let theme = theme.read();
    let (content, cursor, process_keyevent, process_clickevent, cursor_ref) = use_editable(
        &cx,
        || "const test = false; \n hello_world(true); \n hello_world(false); \n hello_world(false);",
        EditableMode::SingleLineMultipleEditors,
    );
    let font_size_percentage = use_state(&cx, || 15.0);
    let line_height_percentage = use_state(&cx, || 0.0);
    let is_bold = use_state(&cx, || false);
    let is_italic = use_state(&cx, || false);

    // minimum font size is 5
    let font_size = font_size_percentage + 5.0;
    let line_height = (line_height_percentage / 25.0) + 1.2;

    let highlight_names = &mut [
        "attribute",
        "constant",
        "function.builtin",
        "function",
        "keyword",
        "operator",
        "property",
        "punctuation",
        "punctuation.bracket",
        "punctuation.delimiter",
        "string",
        "string.special",
        "tag",
        "type",
        "type.builtin",
        "variable",
        "variable.builtin",
        "variable.parameter",
    ];

    let mut highlighter = Highlighter::new();

    let mut javascript_config = HighlightConfiguration::new(
        tree_sitter_javascript::language(),
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_javascript::INJECTION_QUERY,
        tree_sitter_javascript::LOCALS_QUERY,
    )
    .unwrap();
    javascript_config.configure(highlight_names);

    let data = content.to_string();
    let highlights = highlighter
        .highlight(&javascript_config, data.as_bytes(), None, |_| None)
        .unwrap();

    let mut words: Vec<Vec<(&str, String)>> = vec![];
    let mut last_finished: (Option<&str>, Vec<(usize, String)>) = (None, vec![]);


    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                let data = content.get().lines(start..end);
                let starting_line = content.get().line_of_offset(start);

                for (i, d) in data.enumerate() {
                    last_finished.1.push((starting_line + i, d.to_string()));
                }
            }
            HighlightEvent::HighlightStart(s) => {
                last_finished.0 = Some(highlight_names[s.0]);
                eprintln!("highlight style started: {:?}", highlight_names[s.0]);
            }
            HighlightEvent::HighlightEnd => {
                for (i, d) in last_finished.1 {
                    if words.get(i).is_none() {
                        words.push(vec![]);
                    }
                    let mut line = words.last_mut().unwrap();
                    line.push((last_finished.0.unwrap(), d));
                }
                last_finished = (None, vec![]);
            }
        }
    }

   

    render!(
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
            onkeydown: process_keyevent,
            cursor_reference: cursor_ref,
            direction: "horizontal",
            background: "{theme.body.background}",
            rect {
                width: "100%",
                height: "100%",
                padding: "30",
                ScrollView {
                    width: "100%",
                    height: "100%",
                    show_scrollbar: true,
                    words.iter().enumerate().map(move |(line_index, line)| {
                        let process_clickevent = process_clickevent.clone();

                        let is_line_selected = cursor.1 == line_index;

                        // Only show the cursor in the active line
                        let character_index = if is_line_selected {
                            cursor.0.to_string()
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
                            process_clickevent.send((e, line_index)).ok();
                        };

                        let manual_line_height = font_size * line_height;

                        let cursor_id = line_index;


                        rsx! {
                            rect {
                                key: "{line_index}",
                                width: "100%",
                                height: "{manual_line_height}",
                                direction: "horizontal",
                                background: "{line_background}",
                                radius: "7",
                                rect {
                                    width: "{font_size * 2.0}",
                                    height: "100%",
                                    display: "center",
                                    direction: "horizontal",
                                    label {
                                        font_size: "{font_size}",
                                        color: "rgb(200, 200, 200)",
                                        "{line_index + 1} "
                                    }
                                }
                                paragraph {
                                    width: "100%",
                                    cursor_index: "{character_index}",
                                    cursor_color: "white",
                                    max_lines: "1",
                                    cursor_mode: "editable",
                                    cursor_id: "{cursor_id}",
                                    onmousedown: onmousedown,
                                    height: "{manual_line_height}",
                                    line.iter().map(|(t, word)| {
                                        println!("{line_index:?}{:?}", word);
                                        rsx!(
                                            text {
                                                width: "100%",
                                                color: "{get_color_from_type(t)}",
                                                font_size: "{font_size}",
                                                "{word}"
                                            }
                                        )
                                    })
                                }
                            }
                        }
                    })
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
    )
}

fn app(cx: Scope) -> Element {
    use_init_theme(&cx, DARK_THEME);
    render!(Body {})
}

fn get_color_from_type(t: &str) -> &str {
    match t {
        "keyword" => "red",
        "variable" => "blue",
        "operator" => "green",
        "string" => "yellow",
        _ => "white",
    }
}
