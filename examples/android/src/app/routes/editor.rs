use freya::{
    code_editor::*,
    prelude::*,
    text_edit::Rope,
};

const SAMPLE_CODE: &str = r#"use freya::prelude::*;

fn app() -> impl IntoElement {
    let mut count = use_state(|| 0);
    let is_positive = *count.read() >= 0;

    rect()
        .expanded()
        .center()
        .spacing(12.)
        .child(
            rect()
                .width(Size::px(250.))
                .height(Size::px(120.))
                .center()
                .background(if is_positive {
                    (15, 163, 242)
                } else {
                    (220, 50, 50)
                })
                .corner_radius(16.)
                .color(Color::WHITE)
                .font_size(56.)
                .font_weight(FontWeight::BOLD)
                .shadow((0., 4., 20., 4., (0, 0, 0, 80)))
                .child(count.read().to_string()),
        )
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .child(
                    Button::new()
                        .filled()
                        .on_press(move |_| {
                            *count.write() -= 1;
                        })
                        .child("Decrease"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| {
                            count.set(0);
                        })
                        .child("Reset"),
                )
                .child(
                    Button::new()
                        .filled()
                        .on_press(move |_| {
                            *count.write() += 1;
                        })
                        .child("Increase"),
                ),
        )
}

#[derive(PartialEq)]
struct TodoItem {
    label: String,
    done: bool,
}

#[derive(PartialEq)]
struct TodoList;

impl Component for TodoList {
    fn render(&self) -> impl IntoElement {
        let mut items = use_state::<Vec<TodoItem>>(Vec::new);
        let mut input = use_state(String::new);

        let on_submit = move |_| {
            let text = input.read().trim().to_string();
            if !text.is_empty() {
                items.write().push(TodoItem {
                    label: text,
                    done: false,
                });
                input.set(String::new());
            }
        };

        rect()
            .width(Size::px(400.))
            .spacing(8.)
            .padding(16.)
            .child(
                rect()
                    .horizontal()
                    .spacing(8.)
                    .child(
                        Input::new()
                            .value(input.read().clone())
                            .on_change(move |txt| input.set(txt))
                            .placeholder("Add a task..."),
                    )
                    .child(
                        Button::new()
                            .filled()
                            .on_press(on_submit)
                            .child("Add"),
                    ),
            )
            .children(
                items
                    .read()
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| {
                        let label = if item.done {
                            format!("[x] {}", item.label)
                        } else {
                            format!("[ ] {}", item.label)
                        };

                        Button::new()
                            .flat()
                            .on_press(move |_| {
                                items.write()[idx].done =
                                    !items.read()[idx].done;
                            })
                            .child(label)
                            .into()
                    })
                    .collect::<Vec<_>>(),
            )
    }
}"#;

#[derive(PartialEq)]
pub struct EditorDemo;

impl Component for EditorDemo {
    fn render(&self) -> impl IntoElement {
        let focus = use_focus();

        let editor = use_state(move || {
            let rope = Rope::from_str(SAMPLE_CODE);
            let mut editor = CodeEditorData::new(rope, LanguageId::Rust);
            editor.set_theme(SyntaxTheme::default());
            editor.parse();
            editor.measure(14., "Jetbrains Mono");
            editor
        });

        rect()
            .expanded()
            .padding(8.)
            .child(CodeEditor::new(editor, focus.a11y_id()).line_height(1.3))
    }
}
