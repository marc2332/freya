use std::borrow::Cow;

use freya_code_editor::prelude::{
    CodeEditor,
    CodeEditorData,
    EditorLanguage,
    Rope,
};
use freya_core::prelude::*;
use torin::prelude::*;

/// Maps a code block's language string to an [`EditorLanguage`], or `None` to skip highlighting.
pub type LanguageResolver = Callback<String, Option<EditorLanguage>>;

#[derive(PartialEq)]
pub(crate) struct CodeBlockEditor {
    code: NoArgCallback<Cow<'static, str>>,
    language: Option<String>,
    resolver: Option<LanguageResolver>,
    font_size: f32,
    font_family: Cow<'static, str>,
    key: DiffKey,
}

impl CodeBlockEditor {
    pub(crate) fn new(
        code: impl Into<NoArgCallback<Cow<'static, str>>>,
        language: Option<String>,
        resolver: Option<LanguageResolver>,
        font_size: f32,
        font_family: Cow<'static, str>,
    ) -> Self {
        Self {
            code: code.into(),
            language,
            resolver,
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
        let resolver = self.resolver.clone();
        let font_size = self.font_size;
        let font_family = self.font_family.clone();

        let a11y_id = use_a11y();

        let editor = use_state({
            let font_family = font_family.clone();
            move || {
                let code = code.call();
                let language = language
                    .zip(resolver)
                    .and_then(|(language, resolve)| resolve.call(language));
                let mut editor = CodeEditorData::new(Rope::from_str(&code), language);
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
