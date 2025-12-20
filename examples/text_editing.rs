use freya::{
    helpers::from_fn,
    prelude::*,
    text_edit::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut editable = use_editable(
        || "Hello Rustaceans\nHello World".to_string(),
        EditableConfig::new,
        EditableMode::SingleLineMultipleEditors,
    );

    let editor = editable.editor().read();

    let on_global_key_down = move |e: Event<KeyboardEventData>| {
        editable.process_event(EditableEvent::KeyDown {
            key: &e.key,
            modifiers: e.modifiers,
        });
    };

    rect()
        .font_family("NotoSans")
        .width(Size::fill())
        .height(Size::fill())
        .background((255, 255, 255))
        .on_global_key_down(on_global_key_down)
        .children_iter(editor.lines().enumerate().map(move |(i, line)| {
            let line = line.text.to_string();
            from_fn((), line, move |line| {
                let holder = use_state(ParagraphHolder::default);
                let editor = editable.editor().read();

                let is_selected = editor.cursor_row() == i;
                let cursor_index = if is_selected {
                    Some(editor.cursor_col())
                } else {
                    None
                };

                let on_mouse_down = move |e: Event<MouseEventData>| {
                    editable.process_event(EditableEvent::Down {
                        location: e.element_location,
                        editor_id: i,
                        holder: &holder.read(),
                    });
                };

                paragraph()
                    .holder(holder.read().clone())
                    .width(Size::fill())
                    .height(Size::px(30.0))
                    .max_lines(1)
                    .cursor_index(cursor_index)
                    .cursor_color((0, 0, 0))
                    .on_mouse_down(on_mouse_down)
                    .highlights(editor.get_visible_selection(i).map(|h| vec![h]))
                    .span(Span::new(line.clone()))
                    .into()
            })
        }))
        .child(
            label()
                .color((0, 0, 0))
                .height(Size::percent(50.0))
                .text(format!("{}:{}", editor.cursor_row(), editor.cursor_col())),
        )
}
