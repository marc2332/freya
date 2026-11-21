use freya_code_editor::prelude::*;
use freya_edit::{
    TextSelection,
    TextEditor,
};

// `TextSelection` stores UTF-16 code units while `Rope::get_slice` takes char
// indices, so a selection that contains a surrogate pair used to slice the
// wrong characters.
#[test]
fn selected_text_is_taken_by_char_index() {
    // The emoji is one char but two UTF-16 code units.
    let mut editor = CodeEditorData::new(Rope::from_str("ab 中文 😀 结尾"), None);

    *editor.selection_mut() = TextSelection::Range { from: 6, to: 8 };

    assert_eq!(editor.get_selected_text().as_deref(), Some("😀"));
}

#[test]
fn selected_text_keeps_bmp_characters() {
    let mut editor = CodeEditorData::new(Rope::from_str("ab 中文 结尾"), None);

    *editor.selection_mut() = TextSelection::Range { from: 3, to: 5 };

    assert_eq!(editor.get_selected_text().as_deref(), Some("中文"));
}