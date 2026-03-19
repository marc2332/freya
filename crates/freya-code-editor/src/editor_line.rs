use std::borrow::Cow;

use freya_core::prelude::*;
use freya_edit::{
    EditableEvent,
    EditorLine,
    TextEditor,
};
use torin::{
    gaps::Gaps,
    prelude::Alignment,
    size::Size,
};

use crate::{
    editor_data::CodeEditorData,
    editor_theme::EditorTheme,
    syntax::TextNode,
};

#[derive(Clone, PartialEq)]
pub struct EditorLineUI {
    pub(crate) editor: Writable<CodeEditorData>,
    pub(crate) font_size: f32,
    pub(crate) line_height: f32,
    pub(crate) line_index: usize,
    pub(crate) read_only: bool,
    pub(crate) gutter: bool,
    pub(crate) show_whitespace: bool,
    pub(crate) font_family: Cow<'static, str>,
    pub(crate) theme: Readable<EditorTheme>,
}

impl Component for EditorLineUI {
    fn render_key(&self) -> DiffKey {
        DiffKey::from(&self.line_index)
    }
    fn render(&self) -> impl IntoElement {
        let EditorLineUI {
            mut editor,
            font_size,
            line_height,
            line_index,
            read_only,
            gutter,
            show_whitespace,
            font_family,
            theme,
        } = self.clone();

        let holder = use_state(ParagraphHolder::default);

        let editor_data = editor.read();
        let theme = theme.read();

        let longest_width = editor_data.metrics.longest_width;
        let line = editor_data.metrics.syntax_blocks.get_line(line_index);
        let highlights = editor_data.get_visible_selection(EditorLine::Paragraph(line_index));
        let gutter_width = font_size * 5.0;
        let is_line_selected = editor_data.cursor_row() == line_index;

        let on_mouse_down = {
            let mut editor = editor.clone();
            let font_family = font_family.clone();
            move |e: Event<MouseEventData>| {
                editor.write_if(|mut editor_editor| {
                    editor_editor.process(
                        font_size,
                        &font_family,
                        EditableEvent::Down {
                            location: e.element_location,
                            editor_line: EditorLine::Paragraph(line_index),
                            holder: &holder.read(),
                        },
                    )
                });
            }
        };

        let on_mouse_move = {
            let font_family = font_family.clone();
            move |e: Event<MouseEventData>| {
                editor.write_if(|mut editor_editor| {
                    editor_editor.process(
                        font_size,
                        &font_family,
                        EditableEvent::Move {
                            location: e.element_location,
                            editor_line: EditorLine::Paragraph(line_index),
                            holder: &holder.read(),
                        },
                    )
                });
            }
        };

        let cursor_index = if read_only {
            None
        } else {
            is_line_selected.then(|| editor_data.cursor_col())
        };
        let gutter_color = if is_line_selected {
            theme.gutter_selected
        } else {
            theme.gutter_unselected
        };
        let visible_selection = match editor_data.get_selection() {
            None => false,
            Some((s, e)) if s != e => true,
            _ => false,
        };
        let line_background = if is_line_selected && !visible_selection {
            theme.line_selected_background
        } else {
            Color::TRANSPARENT
        };

        rect()
            .horizontal()
            .height(Size::px(line_height))
            .background(line_background)
            .font_size(font_size)
            .maybe(gutter, |el| {
                el.child(
                    rect()
                        .width(Size::px(gutter_width))
                        .height(Size::fill())
                        .padding(Gaps::new(0., 0., 0., 20.))
                        .main_align(Alignment::Center)
                        .child(
                            label()
                                .color(gutter_color)
                                .text(format!("{} ", line_index + 1)),
                        ),
                )
            })
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .on_mouse_down(on_mouse_down)
                    .on_mouse_move(on_mouse_move)
                    .cursor_color(theme.cursor)
                    .cursor_style(CursorStyle::Block)
                    .cursor_index(cursor_index)
                    .cursor_mode(CursorMode::Expanded)
                    .vertical_align(VerticalAlign::Center)
                    .highlights(highlights.map(|h| vec![h]))
                    .highlight_color(theme.highlight)
                    .width(Size::px(longest_width))
                    .min_width(Size::fill())
                    .height(Size::fill())
                    .font_family(font_family)
                    .max_lines(1)
                    .color(theme.text)
                    .spans_iter(line.iter().map(|span| {
                        let text: Cow<str> = match &span.1 {
                            TextNode::Range(word_pos) => {
                                editor_data.rope.slice(word_pos.clone()).into()
                            }
                            TextNode::LineOfChars { len, char } => {
                                if show_whitespace {
                                    Cow::Owned(char.to_string().repeat(*len))
                                } else {
                                    Cow::Owned(" ".repeat(*len))
                                }
                            }
                        };
                        Span::new(Cow::Owned(text.to_string())).color(span.0)
                    })),
            )
    }
}
