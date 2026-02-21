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
    let editor = use_state(|| {
        let path = PathBuf::from("./crates/freya-code-editor/src/editor_ui.rs");
        let rope = Rope::from_str(&std::fs::read_to_string(&path).unwrap());
        let mut editor = CodeEditorData::new(CodeEditorMetadata { path, title: None }, rope);
        editor.parse();
        editor.measure(14.);
        editor
    });

    CodeEditor::new(editor, focus.a11y_id()).font_size(16.)
}
