use std::collections::HashMap;

use freya::prelude::*;
use freya_edit::*;
use freya_testing::prelude::*;

/// Clicking past the end of a wrapped line must not land on the next line.
#[test]
fn click_past_a_wrapped_line_end() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(
            || "dddddddddddddd asdad dddddddddddddd asdad dddddddddddddd".to_string(),
            EditableConfig::new,
        );
        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();
        let cursor_pos = editor.cursor_pos();

        let on_mouse_down = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };

        rect()
            .font_family("NotoSans")
            .width(Size::px(200.))
            .height(Size::fill())
            .background((255, 255, 255))
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .width(Size::fill())
                    .cursor_index(cursor_pos)
                    .cursor_color((0, 0, 0))
                    .on_mouse_down(on_mouse_down)
                    .span(Span::new(editor.to_string())),
            )
            .child(label().color((0, 0, 0)).text(format!(
                "{}:{}",
                editor.cursor_row(),
                editor.cursor_col()
            )))
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);
    utils.sync_and_update();

    utils.click_cursor((199., 8.));
    let end_of_first_line = utils.find(|_, e| Some(Label::try_downcast(e)?.text.to_string()));
    assert_eq!(end_of_first_line.as_deref(), Some("0:20"));

    utils.click_cursor((2., 25.));
    let start_of_second_line = utils.find(|_, e| Some(Label::try_downcast(e)?.text.to_string()));
    assert_eq!(start_of_second_line.as_deref(), Some("0:21"));
}
