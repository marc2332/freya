use std::borrow::Cow;

use freya_code_editor::prelude::{
    CodeEditor,
    CodeEditorData,
    LanguageId,
    Rope,
};
use freya_core::prelude::*;
use torin::prelude::*;

pub(crate) fn render_code_block(
    idx: usize,
    code: String,
    language: Option<String>,
    _background_code: Color,
    _color_code: Color,
    code_font_size: f32,
    font_family: Cow<'static, str>,
) -> Element {
    CodeBlockEditor::new(
        move || Cow::Owned(code.clone()),
        language,
        code_font_size,
        font_family,
    )
    .key(idx)
    .into()
}

/// Resolves a fenced code block info string into a [`LanguageId`].
///
/// Markdown fences use language names (`rust`, `python`) rather than the file
/// extensions [`LanguageId::parse`] expects, and may carry extra attributes after
/// the language (e.g. ` ```rust,ignore `), so both forms are handled here.
fn language_from_fence(fence: &str) -> LanguageId {
    // Fences may carry attributes after the language, e.g. ```rust,ignore
    let name = fence
        .split([',', ' '])
        .next()
        .unwrap_or(fence)
        .trim()
        .to_ascii_lowercase();

    LanguageId::parse(&name)
}

#[derive(PartialEq)]
struct CodeBlockEditor {
    code: NoArgCallback<Cow<'static, str>>,
    language: Option<String>,
    font_size: f32,
    font_family: Cow<'static, str>,
    key: DiffKey,
}

impl CodeBlockEditor {
    fn new(
        code: impl Into<NoArgCallback<Cow<'static, str>>>,
        language: Option<String>,
        font_size: f32,
        font_family: Cow<'static, str>,
    ) -> Self {
        Self {
            code: code.into(),
            language,
            font_size,
            font_family,
            key: DiffKey::None,
        }
    }
}

impl KeyExt for CodeBlockEditor {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for CodeBlockEditor {
    fn render(&self) -> impl IntoElement {
        let code = self.code.clone();
        let language = self.language.clone();
        let font_size = self.font_size;
        let font_family = self.font_family.clone();

        let a11y_id = use_a11y();

        let editor = use_state({
            let font_family = font_family.clone();
            move || {
                let code = code.call();
                let language_id = language
                    .as_deref()
                    .map(language_from_fence)
                    .unwrap_or_default();
                let mut editor = CodeEditorData::new(Rope::from_str(&code), language_id);
                editor.parse();
                editor.measure(font_size, &font_family);
                editor
            }
        });

        let line_height = (font_size * 1.4).floor();
        let lines = editor.read().rope.len_lines().max(1);

        rect()
            .width(Size::fill())
            .height(Size::px(lines as f32 * line_height))
            .corner_radius(6.)
            .overflow(Overflow::Clip)
            .child(
                CodeEditor::new(editor, a11y_id)
                    .read_only(true)
                    .gutter(false)
                    .show_whitespace(false)
                    .font_size(font_size)
                    .line_height(1.4)
                    .font_family(font_family),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
