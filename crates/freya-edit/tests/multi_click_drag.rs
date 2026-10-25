use std::{
    collections::HashMap,
    time::Duration,
};

use freya::prelude::*;
use freya_edit::*;
use freya_testing::prelude::*;

fn editor_runner(select_all_on_double_click: bool) -> TestingRunner {
    let mut runner = launch_test(move || {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello World".to_string(),
            move || {
                EditableConfig::new().with_select_all_on_double_click(select_all_on_double_click)
            },
        );
        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();

        paragraph()
            .holder(holder.read().clone())
            .font_family("NotoSans")
            .width(Size::fill())
            .height(Size::fill())
            .highlights(
                editor
                    .get_visible_selection(EditorLine::SingleParagraph)
                    .map(|selection| vec![selection]),
            )
            .on_mouse_down(move |event: Event<MouseEventData>| {
                editable.process_event(EditableEvent::Down {
                    location: event.element_location,
                    editor_line: EditorLine::SingleParagraph,
                    holder: &holder.read(),
                });
            })
            .on_mouse_move(move |event: Event<MouseEventData>| {
                editable.process_event(EditableEvent::Move {
                    location: event.element_location,
                    editor_line: EditorLine::SingleParagraph,
                    holder: &holder.read(),
                });
            })
            .on_global_pointer_up(move |_: Event<PointerEventData>| {
                editable.process_event(EditableEvent::Release);
            })
            .span(Span::new(editor.to_string()))
    });
    runner.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    runner.set_default_fonts(&["NotoSans".into()]);
    runner
}

fn highlights(runner: &TestingRunner) -> Vec<(usize, usize)> {
    runner
        .find(|_, element| Some(Paragraph::try_downcast(element)?.highlights.clone()))
        .unwrap()
}

#[test]
fn double_click_drag_selects_words_in_both_directions() {
    let mut runner = editor_runner(false);
    runner.click_cursor((50.0, 3.0));
    runner.press_cursor((50.0, 3.0));
    for location in [(50.0, 3.0), (51.0, 3.0), (80.0, 3.0)] {
        runner.move_cursor(location);
        assert_eq!(highlights(&runner), vec![(6, 16)]);
    }

    runner.move_cursor((10.0, 3.0));
    assert_eq!(highlights(&runner), vec![(16, 0)]);

    runner.move_cursor((50.0, 3.0));
    assert_eq!(highlights(&runner), vec![(6, 16)]);

    runner.move_cursor((50.0, 25.0));
    assert_eq!(highlights(&runner), vec![(6, 28)]);

    runner.release_cursor((50.0, 25.0));
    runner.move_cursor((80.0, 3.0));
    assert_eq!(highlights(&runner), vec![(6, 28)]);

    runner.press_cursor((50.0, 3.0));
    assert!(highlights(&runner).is_empty());
}

#[test]
fn triple_click_drag_selects_lines_in_both_directions() {
    for (location, destination, anchor, extended) in [
        ((50.0, 3.0), (50.0, 25.0), (0, 17), (0, 28)),
        ((50.0, 25.0), (50.0, 3.0), (17, 28), (28, 0)),
    ] {
        let mut runner = editor_runner(false);
        runner.click_cursor(location);
        runner.click_cursor(location);
        runner.press_cursor(location);
        for pointer_location in [location, (location.0 + 1.0, location.1)] {
            runner.move_cursor(pointer_location);
            assert_eq!(highlights(&runner), vec![anchor]);
        }

        runner.move_cursor(destination);
        assert_eq!(highlights(&runner), vec![extended]);

        runner.move_cursor(location);
        assert_eq!(highlights(&runner), vec![anchor]);
    }
}

#[test]
fn select_all_drag_keeps_all_text_selected() {
    for (press_count, select_all) in [(4, false), (2, true)] {
        let mut runner = editor_runner(select_all);
        for _ in 1..press_count {
            runner.click_cursor((50.0, 3.0));
        }
        runner.press_cursor((50.0, 3.0));
        for location in [(50.0, 3.0), (10.0, 3.0), (200.0, 25.0)] {
            runner.move_cursor(location);
            assert_eq!(highlights(&runner), vec![(0, 28)]);
        }
    }
}

#[test]
fn multi_click_drag_uses_utf16_offsets_across_paragraphs() {
    let editor = RopeEditor::new(
        "😀 alpha\n😀 beta\n😀 gamma".to_string(),
        TextSelection::new_range((12, 16)),
        4,
        EditorHistory::new(Duration::from_millis(500)),
    );
    let mut dragging = TextDragging::default();
    dragging.start_selection(PressEventType::Double, editor.selection().clone());
    assert_eq!(
        dragging.measure_selection(&editor, 5, EditorLine::Paragraph(2)),
        TextSelection::new_range((12, 25)),
    );
    assert_eq!(
        dragging.measure_selection(&editor, 5, EditorLine::Paragraph(0)),
        TextSelection::new_range((16, 3)),
    );
    assert_eq!(
        dragging.measure_selection(&editor, 5, EditorLine::Paragraph(1)),
        TextSelection::new_range((12, 16)),
    );

    let anchor = TextSelection::new_range(editor.find_line_boundaries(13));
    assert_eq!(anchor, TextSelection::new_range((9, 17)));
    dragging.start_selection(PressEventType::Triple, anchor.clone());
    assert_eq!(
        dragging.measure_selection(&editor, 5, EditorLine::Paragraph(2)),
        TextSelection::new_range((9, 25)),
    );
    assert_eq!(
        dragging.measure_selection(&editor, 5, EditorLine::Paragraph(0)),
        TextSelection::new_range((17, 0)),
    );
    assert_eq!(
        dragging.measure_selection(&editor, 5, EditorLine::Paragraph(1)),
        anchor,
    );
}
