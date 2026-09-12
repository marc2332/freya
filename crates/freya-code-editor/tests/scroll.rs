use freya_code_editor::prelude::*;
use freya_core::prelude::*;
use freya_testing::{
    TestingNode,
    prelude::*,
};

fn editor_app() -> impl IntoElement {
    let a11y_id = use_a11y();
    let editor = use_state(move || {
        let rope = Rope::from_str(
            &(0..200)
                .map(|line| format!("line {line}"))
                .collect::<Vec<_>>()
                .join("\n"),
        );
        let mut editor = CodeEditorData::new(rope, None);
        editor.parse();
        editor.measure(14., "Jetbrains Mono");
        editor
    });

    CodeEditor::new(editor, a11y_id).a11y_auto_focus(true)
}

fn line_text(line: &TestingNode) -> String {
    let element = line.children()[1].element();
    let paragraph = Paragraph::try_downcast(&*element).unwrap();
    paragraph
        .spans
        .iter()
        .map(|span| span.text.trim().to_string())
        .collect()
}

#[test]
pub fn code_editor_scrolls_to_cursor() {
    let mut test = launch_test(editor_app);
    test.sync_and_update();

    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let lines = || scrollview.children()[0].children()[0].children();

    assert_eq!(line_text(&lines()[0]), "line 0");

    for _ in 0..60 {
        test.press_key(Key::Named(NamedKey::ArrowDown));
    }

    assert_eq!(line_text(&lines()[0]), "line 34");

    for _ in 0..40 {
        test.press_key(Key::Named(NamedKey::ArrowUp));
    }

    assert_eq!(line_text(&lines()[0]), "line 20");
}
