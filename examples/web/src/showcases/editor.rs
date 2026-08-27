use freya::{
    code_editor::*,
    prelude::*,
    text_edit::Rope,
};

const SAMPLE: &str = r#"use freya::prelude::*;

fn app() -> impl IntoElement {
    let mut count = use_state(|| 0);

    rect()
        .expanded()
        .center()
        .spacing(12.)
        .child(format!("Clicked {} times", count()))
        .child(
            Button::new()
                .on_press(move |_| *count.write() += 1)
                .child("Press me"),
        )
}
"#;

#[derive(PartialEq)]
pub struct EditorShowcase;

impl Component for EditorShowcase {
    fn render(&self) -> impl IntoElement {
        let a11y_id = use_a11y();
        let editor = use_state(move || {
            let mut editor = CodeEditorData::new(Rope::from_str(SAMPLE), None);
            editor.measure(14., "Noto Sans");
            editor
        });

        rect()
            .spacing(20.)
            .expanded()
            .child(super::heading(
                "Code Editor",
                "An editable, measured text buffer",
            ))
            .child(
                rect()
                    .expanded()
                    .width(Size::fill())
                    .corner_radius(12.)
                    .child(CodeEditor::new(editor, a11y_id).background((24, 24, 27))),
            )
    }
}
