#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use freya::{
    code_editor::*,
    prelude::*,
};
use ropey::Rope;

fn main() {
    launch(LaunchConfig::default().with_window(WindowConfig::new(app)));
}

fn app() -> impl IntoElement {
    use_init_theme(|| DARK_THEME);
    let focus = use_focus();
    let custom_theme = use_state(|| EditorTheme {
        background: (20, 20, 20).into(),
        ..Default::default()
    });
    let editor = use_state(move || {
        let path = PathBuf::from("./crates/freya-code-editor/src/editor_ui.rs");
        let rope = Rope::from_str(&std::fs::read_to_string(&path).unwrap());
        let mut editor = CodeEditorData::new(rope, LanguageId::Rust);
        editor.set_theme(SyntaxTheme {
            comment: (230, 230, 230).into(),
            ..Default::default()
        });
        editor.parse();
        editor.measure(14., "Jetbrains Mono");
        editor
    });

    CodeEditor::new(editor, focus.a11y_id()).theme(custom_theme)
}
