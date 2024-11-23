#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use tree_sitter_highlight::{
    HighlightEvent,
    Highlighter,
};

fn main() {
    launch_with_props(app, "like freyaui.dev but in freya", (1500.0, 900.0));
}

fn app() -> Element {
    rsx!(
        rect {
            background: "rgb(24, 24, 27)",
            color: "white",
            font_family: "Inter",
            ThemeProvider {
                theme: DARK_THEME,
                ScrollView {
                    rect {
                        cross_align: "center",
                        width: "fill",
                        rect {
                            width: "60%",
                            spacing: "40",
                            padding: "40 0",
                            Navigation { }
                            Home { }
                        }
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn Home() -> Element {
    rsx!(
        rect {
            cross_align: "center",
            width: "fill",
            spacing: "40",
            rect {
                direction: "horizontal",
                cross_align: "center",
                spacing: "12",
                BigFreyaLogo {  }
                rect {
                    spacing: "16",
                    paragraph {
                        width: "500",
                        text {
                            "Build native & cross-platform GUI applications using 🦀 Rust. Powered by 🧬 "
                        }
                        text {
                            decoration: "underline",
                            "Dioxus "
                        }
                        text {
                            "and 🎨 "
                        }
                        text {
                            decoration: "underline",
                            "Skia."
                        }
                    }
                    rect {
                        direction: "horizontal",
                        spacing: "10",
                        Link {
                            to: "https://book.freyaui.dev/getting_started.html",
                            Button {
                                theme: theme_with!(ButtonTheme {
                                    padding: "10 24".into(),
                                    border_fill: "none".into(),
                                    background: "rgb(14, 165, 233)".into(),
                                    hover_background: "rgb(2, 132, 199)".into(),
                                    font_theme: theme_with!(FontTheme {
                                        color: "black".into()
                                    }),
                                }),
                                label {
                                    "Get Started"
                                }
                            }
                        }
                        Link {
                            to: "https://github.com/marc2332/freya",
                            Button {
                                theme: theme_with!(ButtonTheme {
                                    padding: "10 24".into(),
                                    border_fill: "none".into(),
                                    background: "rgb(253, 186, 116)".into(),
                                    hover_background: "rgb(251, 146, 60)".into(),
                                    font_theme: theme_with!(FontTheme {
                                        color: "black".into()
                                    }),
                                }),
                                label {
                                    "Source Code"
                                }
                            }
                        }
                        Link {
                            to: "https://github.com/sponsors/marc2332",
                            Button {
                                theme: theme_with!(ButtonTheme {
                                    padding: "10 24".into(),
                                    border_fill: "none".into(),
                                    background: "rgb(249, 168, 212)".into(),
                                    hover_background: "rgb(244, 114, 182)".into(),
                                    font_theme: theme_with!(FontTheme {
                                        color: "black".into()
                                    }),
                                }),
                                label {
                                    "Sponsor"
                                }
                            }
                        }
                    }
                }
            }
            rect {
                width: "fill",
                cross_align: "center",
                rect {
                    background: "rgb(19, 19, 21)",
                    border: "1 inner rgb(41, 37, 36)",
                    corner_radius: "16",
                    width: "960",
                    height: "612",
                    spacing: "20",
                    padding: "32 24",
                    direction: "horizontal",
                    rect {
                        width: "calc(50% - 20)",
                        height: "fill",
                        cross_align: "center",
                        Code { }
                    }
                    rect {
                        width: "calc(50% - 20)",
                        height: "fill",
                        padding: "20",
                        spacing: "20",
                        rect {
                            height: "90%",
                            Counter {}
                        }
                        rect {
                            height: "fill",
                            width: "fill",
                            cross_align: "center",
                            Link {
                                to: "https://github.com/marc2332/freya#want-to-try-it-",
                                Button {
                                    theme: theme_with!(ButtonTheme {
                                        padding: "10 24".into(),
                                        border_fill: "none".into(),
                                        background: "rgb(109, 78, 233)".into(),
                                        hover_background: "rgb(87, 62, 186)".into(),
                                    }),
                                    label {
                                        "Run Locally"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}

import_svg!(IconLogo, "./freya_icon.svg", "50", "50");
import_svg!(FreyaLogo, "./freya_logo.svg", "50", "50");
import_svg!(BigFreyaLogo, "./freya_icon.svg", "150", "150");

#[allow(non_snake_case)]
fn Navigation() -> Element {
    rsx!(
        rect {
            direction: "horizontal",
            spacing: "24",
            cross_align: "center",
            color: "rgb(214, 211, 209)",
            IconLogo { }
            FreyaLogo { }
            Link {
                to: "https://freyaui.dev/blog",
                label {
                    "Book"
                }
            }
            Link {
                to: "https://book.freyaui.dev/",
                label {
                    "Book"
                }
            }
            Link {
                to: "https://docs.rs/freya/latest/freya/",
                label {
                    "Docs"
                }
            }
            Link {
                to: "https://discord.gg/sYejxCdewG",
                label {
                    "Discord"
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        ThemeProvider {
            theme: LIGHT_THEME,
            rect {
                corner_radius: "16",
                overflow: "clip",
                shadow: "0 0 10 0 rgb(0, 0, 0, 0.3)",
                rect {
                    height: "50%",
                    width: "100%",
                    main_align: "center",
                    cross_align: "center",
                    background: "rgb(0, 119, 182)",
                    color: "white",
                    shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
                    label {
                        font_size: "75",
                        font_weight: "bold",
                        "{count}"
                    }
                }
                rect {
                    height: "50%",
                    width: "100%",
                    main_align: "center",
                    cross_align: "center",
                    background: "white",
                    direction: "horizontal",
                    spacing: "8",
                    Button {
                        onclick: move |_| count += 1,
                        label { "Increase" }
                    }
                    Button {
                        onclick: move |_| count -= 1,
                        label { "Decrease" }
                    }
                }
            }
        }
    )
}

const CODE: &str = r#"fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
            label {
                font_size: "75",
                font_weight: "bold",
                "{count}"
            }
        }
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            Button {
                onclick: move |_| count += 1,
                label { "Increase" }
            }
            Button {
                onclick: move |_| count -= 1,
                label { "Decrease" }
            }
        }
    )
}"#;

const HIGHLIGH_TAGS: [&str; 23] = [
    "constructor",
    "attribute",
    "constant",
    "constant.builtin",
    "function.builtin",
    "function",
    "function.method",
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
    "number",
    "comment",
];

#[component]
fn Code() -> Element {
    let code = use_hook(move || {
        use tree_sitter_highlight::HighlightConfiguration;

        let mut rust_config = HighlightConfiguration::new(
            tree_sitter_rust::LANGUAGE.into(),
            "rust",
            tree_sitter_rust::HIGHLIGHTS_QUERY,
            tree_sitter_rust::INJECTIONS_QUERY,
            tree_sitter_rust::TAGS_QUERY,
        )
        .unwrap();

        rust_config.configure(&HIGHLIGH_TAGS);

        let mut highlighter = Highlighter::new();

        let highlights = highlighter
            .highlight(&rust_config, CODE.as_bytes(), None, |_| None)
            .unwrap();

        let rope = Rope::from_str(CODE);

        let mut syntax_blocks = SyntaxBlocks::default();

        let mut prepared_block: (SyntaxType, Vec<(usize, String)>) =
            (SyntaxType::Unknown, Vec::new());

        for event in highlights {
            match event.unwrap() {
                HighlightEvent::Source { start, end } => {
                    // Prepare the whole block even if it's splitted across multiple lines.
                    let data_begining = rope.byte_slice(start..end);
                    let starting_line = rope.char_to_line(start);

                    let mut back = String::new();
                    let mut line = starting_line;

                    for (i, d) in data_begining.chars().enumerate() {
                        if d != '\n' {
                            back.push(d);
                        }

                        if start + i == end - 1 || d == '\n' {
                            prepared_block.1.push((line, back.clone()));
                            line += 1;
                            back.clear();
                        }
                    }
                }
                HighlightEvent::HighlightStart(s) => {
                    // Specify the type of the block
                    prepared_block.0 = SyntaxType::from(HIGHLIGH_TAGS[s.0]);
                }
                HighlightEvent::HighlightEnd => {
                    // Push all the block chunks to their specified line
                    for (i, d) in prepared_block.1 {
                        if syntax_blocks.get(i).is_none() {
                            syntax_blocks.push(Vec::new());
                        }
                        let line = syntax_blocks.last_mut().unwrap();
                        line.push((prepared_block.0.clone(), d));
                    }
                    // Clear the prepared block
                    prepared_block = (SyntaxType::Unknown, Vec::new());
                }
            }
        }

        // Mark all the remaining text as not readable
        if !prepared_block.1.is_empty() {
            for (i, d) in prepared_block.1 {
                if syntax_blocks.get(i).is_none() {
                    syntax_blocks.push(Vec::new());
                }
                let line = syntax_blocks.last_mut().unwrap();
                line.push((SyntaxType::Unknown, d));
            }
        }

        syntax_blocks
    });

    rsx!(
        rect {
            for (line_n, line) in code.iter().enumerate() {
                paragraph {
                    key: "{line_n}",
                    font_size: "12",
                    font_family: "Jetbrains Mono",
                    line_height: "1.3",
                    for (node_n, (syntax_type, text)) in line.iter().enumerate() {
                        text {
                            key: "{node_n}",
                            color: "{syntax_type.color()}",
                            "{text}"
                        }
                    }
                }
            }
        }
    )
}

#[derive(Clone, Debug)]
pub enum SyntaxType {
    Number,
    String,
    Keyword,
    Operator,
    Variable,
    Function,
    Comment,
    Punctuation,
    Unknown,
}

impl SyntaxType {
    pub fn color(&self) -> &str {
        match self {
            SyntaxType::Number => "#9ECBFF",
            SyntaxType::String => "#9ECBFF",
            SyntaxType::Keyword => "#F97583",
            SyntaxType::Operator => "#F97583",
            SyntaxType::Variable => "white",
            SyntaxType::Function => "#B392F0",
            SyntaxType::Comment => "green",
            SyntaxType::Punctuation => "white",
            SyntaxType::Unknown => "white",
        }
    }
}

impl From<&str> for SyntaxType {
    fn from(s: &str) -> Self {
        match s {
            "keyword" => SyntaxType::Keyword,
            "variable" => SyntaxType::Variable,
            "operator" => SyntaxType::Operator,
            "string" => SyntaxType::String,
            "number" => SyntaxType::Number,
            "function" => SyntaxType::Function,
            "constructor" => SyntaxType::Function,
            "comment" => SyntaxType::Comment,
            "punctuation.bracket" => SyntaxType::Punctuation,
            _ => SyntaxType::Unknown,
        }
    }
}

pub type SyntaxBlocks = Vec<Vec<(SyntaxType, String)>>;
