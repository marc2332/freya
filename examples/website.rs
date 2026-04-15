#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    code_editor::*,
    prelude::*,
    text_edit::Rope,
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
    use_init_theme(dark_theme);

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
impl Component for Home {
    fn render(&self) -> impl IntoElement {
        let editor_focus = use_focus();
        let editor_theme = use_state(|| EditorTheme {
            background: Color::TRANSPARENT,
            line_selected_background: Color::TRANSPARENT,
            ..Default::default()
        });
        let editor = use_state(move || {
            let rope = Rope::from_str(CODE);
            let mut editor = CodeEditorData::new(rope, LanguageId::Rust);
            editor.set_theme(SyntaxTheme {
                comment: (230, 230, 230).into(),
                ..Default::default()
            });
            editor.parse();
            editor.measure(14., "Jetbrains Mono");
            editor
        });

        rect()
            .cross_align(Alignment::Center)
            .width(Size::fill())
            .spacing(40.0)
            .child(
                rect()
                    .direction(Direction::Horizontal)
                    .cross_align(Alignment::Center)
                    .spacing(12.0)
                    .child(svg(Bytes::from_static(include_bytes!("./website/freya_icon.svg"))).width(Size::px(150.)).height(Size::px(150.)))
                    .child(
                        rect()
                            .spacing(16.0)
                            .child(
                                paragraph()
                                    .width(Size::px(500.0))
                                    .span("Build cross-platform GUI applications using 🦀 Rust.\nPowered by 🎨 Skia.")
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
                                    .child(
                                        CodeEditor::new(editor, editor_focus.a11y_id())
                                            .theme(editor_theme)
                                            .read_only(true)
                                            .gutter(false)
                                            .line_height(1.2)
                                            .show_whitespace(false)
                                        ),
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
impl Component for Navigation {
    fn render(&self) -> impl IntoElement {
        rect()
            .direction(Direction::Horizontal)
            .spacing(24.0)
            .cross_align(Alignment::Center)
            .color((214, 211, 209))
            .child(
                svg(Bytes::from_static(include_bytes!(
                    "./website/freya_icon.svg"
                )))
                .width(Size::px(50.))
                .height(Size::px(50.)),
            )
            .child(
                svg(Bytes::from_static(include_bytes!(
                    "./website/freya_logo.svg"
                )))
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
impl Component for Counter {
    fn render(&self) -> impl IntoElement {
        use_init_theme(light_theme);
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

const CODE: &str = r#"fn app() -> impl IntoElement {
    let mut count = use_state(|| 4);

    let counter = rect()
        .width(Size::fill())
        .height(Size::percent(50.))
        .center()
        .color((255, 255, 255))
        .background((15, 163, 242))
        .font_weight(FontWeight::BOLD)
        .font_size(75.)
        .shadow((0., 4., 20., 4., (0, 0, 0, 80)))
        .child(count.read().to_string());

    let actions = rect()
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
        );

    rect().child(counter).child(actions)
}"#;
