#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    prelude::*,
    text_edit::Rope,
};
use tree_sitter_highlight::{
    HighlightEvent,
    Highlighter,
};

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_title("like freyaui.dev but in freya")
                .with_size(1500.0, 900.0),
        ),
    )
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);

    rect()
        .background((24, 24, 27))
        .color((255, 255, 255))
        .child(
            ScrollView::new().child(
                rect()
                    .cross_align(Alignment::Center)
                    .width(Size::fill())
                    .child(
                        rect()
                            .width(Size::percent(60.))
                            .spacing(40.0)
                            .padding((40.0, 0.0))
                            .child(Navigation)
                            .child(Home),
                    ),
            ),
        )
        .into()
}

#[derive(PartialEq)]
struct Home;
impl Render for Home {
    fn render(&self) -> impl IntoElement {
        rect()
            .cross_align(Alignment::Center)
            .width(Size::fill())
            .spacing(40.0)
            .child(
                rect()
                    .direction(Direction::Horizontal)
                    .cross_align(Alignment::Center)
                    .spacing(12.0)
                    .child(svg(Bytes::from_static(include_bytes!("./freya_icon.svg"))).width(Size::px(150.)).height(Size::px(150.)))
                    .child(
                        rect()
                            .spacing(16.0)
                            .child(
                                paragraph()
                                    .width(Size::px(500.0))
                                    .span(Span::new("Build native & cross-platform GUI applications using 🦀 Rust. Powered by 🧬 "))
                                    .span(Span::new("Dioxus "))
                                    .span(Span::new(" and 🎨 "))
                                    .span(Span::new("Skia.")),
                            )
                            .child(
                                rect()
                                    .direction(Direction::Horizontal)
                                    .spacing(10.0)
                                    .child(Link::new("https://book.freyaui.dev/getting_started.html")
                                        .child(Button::new()
                                            .padding((10., 24.))
                                            .background((14, 165, 233))
                                            .hover_background((2, 132, 199))
                                            .border_fill(Color::TRANSPARENT)
                                            .color(Color::BLACK)
                                            .child("Get Started")
                                        )
                                    )
                                    .child(Link::new("https://github.com/marc2332/freya")
                                        .child(Button::new()
                                            .padding((10., 24.))
                                            .background((253, 186, 116))
                                            .hover_background((251, 146, 60))
                                            .border_fill(Color::TRANSPARENT)
                                            .color(Color::BLACK)
                                            .child("Source Code")
                                        )
                                    )
                                    .child(Link::new("https://github.com/sponsors/marc2332")
                                        .child(Button::new()
                                            .padding((10., 24.))
                                            .background((249, 168, 212))
                                            .hover_background((244, 114, 182))
                                            .border_fill(Color::TRANSPARENT)
                                            .color(Color::BLACK)
                                            .child("Sponsor")
                                        )
                                    )
                            ),
                    ),
            )
            .child(
                rect()
                    .width(Size::fill())
                    .cross_align(Alignment::Center)
                    .child(
                        rect()
                            .background((19, 19, 21))
                            .border(Border::new().alignment(BorderAlignment::Inner).fill((41, 37, 36)).width(1.0))
                            .corner_radius(16.0)
                            .width(Size::px(960.0))
                            .height(Size::px(700.0))
                            .spacing(20.0)
                            .padding((32.0, 24.0))
                            .direction(Direction::Horizontal)
                            .child(
                                rect()
                                    .width(Size::func(|c| Some(c.parent / 2. - 20.)))
                                    .height(Size::fill())
                                    .cross_align(Alignment::Center)
                                    .child(Code),
                            )
                            .child(
                                rect()
                                    .width(Size::func(|c| Some(c.parent / 2. - 20.)))
                                    .height(Size::fill())
                                    .padding((20.0, 20.0))
                                    .spacing(20.0)
                                    .child(rect().height(Size::percent(90.)).child(Counter))
                                    .child(
                                        rect()
                                            .height(Size::fill())
                                            .width(Size::fill())
                                            .cross_align(Alignment::Center)
                                            .child(
                                                Link::new("https://github.com/marc2332/freya#want-to-try-it-")
                                                    .child(Button::new()
                                                        .padding((10., 24.))
                                                        .background((109, 78, 233))
                                                        .hover_background((87, 62, 186))
                                                        .border_fill(Color::TRANSPARENT)
                                                        .color(Color::WHITE)
                                                        .child("Run Locally")
                                                    )
                                            ),
                                    ),
                            ),
                    ),
            )
    }
}

#[derive(PartialEq)]
struct Navigation;
impl Render for Navigation {
    fn render(&self) -> impl IntoElement {
        rect()
            .direction(Direction::Horizontal)
            .spacing(24.0)
            .cross_align(Alignment::Center)
            .color((214, 211, 209))
            .child(
                svg(Bytes::from_static(include_bytes!("./freya_icon.svg")))
                    .width(Size::px(50.))
                    .height(Size::px(50.)),
            )
            .child(
                svg(Bytes::from_static(include_bytes!("./freya_logo.svg")))
                    .width(Size::px(50.))
                    .height(Size::px(50.)),
            )
            .child(Link::new("https://freyaui.dev/blog").child("Blog"))
            .child(Link::new("https://book.freyaui.dev/").child("Book"))
            .child(Link::new("https://docs.rs/freya/latest/freya/").child("Docs"))
            .child(Link::new("https://discord.gg/sYejxCdewG").child("Discord"))
    }
}

#[derive(PartialEq)]
struct Counter;
impl Render for Counter {
    fn render(&self) -> impl IntoElement {
        use_init_default_theme();
        let mut count = use_state(|| 4);

        rect()
            .corner_radius(16.)
            .overflow(Overflow::Clip)
            .shadow(Shadow::new().blur(10.).color((0, 0, 0, 0.3)))
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::percent(50.))
                    .center()
                    .color((255, 255, 255))
                    .background((15, 163, 242))
                    .font_size(75.)
                    .shadow((0., 4., 20., 4., (0, 0, 0, 80)))
                    .child(count.read().to_string()),
            )
            .child(
                rect()
                    .horizontal()
                    .background((255, 255, 255))
                    .width(Size::fill())
                    .height(Size::percent(50.))
                    .center()
                    .spacing(8.0)
                    .child(
                        Button::new()
                            .on_press(move |_| {
                                *count.write() += 1;
                            })
                            .child("Increase"),
                    )
                    .child(
                        Button::new()
                            .on_press(move |_| {
                                *count.write() -= 1;
                            })
                            .child("Decrease"),
                    ),
            )
    }
}

#[derive(PartialEq)]
struct Code;
impl Render for Code {
    fn render(&self) -> impl IntoElement {
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

        let mut container = rect();

        for (line_n, line) in code.iter().enumerate() {
            let mut p = paragraph()
                .key(line_n)
                .font_size(12.0)
                .font_family("Jetbrains Mono");
            // .line_height(1.3);

            for (syntax_type, text) in line.iter() {
                p = p.span(Span::new(text.clone()).color(syntax_type.color()));
            }

            container = container.child(p);
        }

        container
    }
}

const CODE: &str = r#"fn app() -> Element {
    let mut count = use_state(|| 4);

    rect()
        .child(
            rect()
                .width(Size::fill())
                .height(Size::percent(50.))
                .center()
                .color((255, 255, 255))
                .background((15, 163, 242))
                .font_size(75.)
                .shadow((0., 4., 20., 4., (0, 0, 0, 80)))
                .child(count.read().to_string()),
        )
        .child(
            rect()
                .horizontal()
                .width(Size::fill())
                .height(Size::percent(50.))
                .center()
                .spacing(8.0)
                .child(
                    Button::new()
                        .on_press(move |_| {
                            *count.write() += 1;
                        })
                        .child("Increase"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| {
                            *count.write() -= 1;
                        })
                        .child("Decrease"),
                ),
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
    pub fn color(&self) -> Color {
        match self {
            SyntaxType::Number => Color::from_hex("#9ECBFF").unwrap(),
            SyntaxType::String => Color::from_hex("#9ECBFF").unwrap(),
            SyntaxType::Keyword => Color::from_hex("#F97583").unwrap(),
            SyntaxType::Operator => Color::from_hex("#F97583").unwrap(),
            SyntaxType::Variable => Color::WHITE,
            SyntaxType::Function => Color::from_hex("#B392F0").unwrap(),
            SyntaxType::Comment => Color::GREEN,
            SyntaxType::Punctuation => Color::WHITE,
            SyntaxType::Unknown => Color::WHITE,
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
