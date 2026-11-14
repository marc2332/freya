use std::{
    collections::HashMap,
    time::Duration,
};

use freya::prelude::*;
use freya_edit::*;
use freya_testing::prelude::*;

/// A paragraph wired to touch editing. Pressing `L` stands for the long
/// press timer firing, `H` for grabbing the cursor handle and `E` for the selection end handle.
fn touch_runner() -> TestingRunner {
    let mut runner = launch_test(move || {
        let mut editable = use_editable(|| "Hello Rustaceans".to_string(), EditableConfig::new);
        let holder = use_state(ParagraphHolder::default);
        let mut last_touch = use_state(CursorPoint::default);
        let editor = editable.editor().read();
        let touch = editable.touch().read();

        let controls = format!(
            "handles={} toolbar={}",
            touch.handles_visible(),
            touch.toolbar_visible()
        );

        rect()
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .font_family("NotoSans")
                    .width(Size::fill())
                    .height(Size::px(100.))
                    .cursor_index(editor.cursor_pos())
                    .highlights(
                        editor
                            .get_visible_selection(EditorLine::SingleParagraph)
                            .map(|selection| vec![selection]),
                    )
                    .on_pointer_down(move |event: Event<PointerEventData>| {
                        if let PointerEventData::Touch(_) = event.data() {
                            last_touch.set(event.global_location());
                            editable.process_touch(TouchEditEvent::Down {
                                location: event.global_location(),
                            });
                        }
                    })
                    .on_global_pointer_move(move |event: Event<PointerEventData>| {
                        if let PointerEventData::Touch(_) = event.data() {
                            last_touch.set(event.global_location());
                            editable.process_touch(TouchEditEvent::Move {
                                location: event.global_location(),
                                editor_line: EditorLine::SingleParagraph,
                                holder: &holder.read(),
                            });
                        }
                    })
                    .on_global_pointer_up(move |event: Event<PointerEventData>| {
                        if let PointerEventData::Touch(_) = event.data() {
                            editable.process_touch(TouchEditEvent::Up {
                                location: event.global_location(),
                                editor_line: EditorLine::SingleParagraph,
                                holder: &holder.read(),
                            });
                        }
                    })
                    .on_global_key_down(move |event: Event<KeyboardEventData>| {
                        if event.key == Key::Character("L".into()) {
                            editable.process_touch(TouchEditEvent::LongPress {
                                location: last_touch(),
                                editor_line: EditorLine::SingleParagraph,
                                holder: &holder.read(),
                            });
                        } else if event.key == Key::Character("H".into()) {
                            editable
                                .process_touch(TouchEditEvent::HandleDown(SelectionHandle::Cursor));
                        } else if event.key == Key::Character("E".into()) {
                            editable
                                .process_touch(TouchEditEvent::HandleDown(SelectionHandle::End));
                        } else {
                            editable.process_event(EditableEvent::KeyDown {
                                key: &event.key,
                                modifiers: event.modifiers,
                                editor_line: Some(EditorLine::SingleParagraph),
                                holder: Some(&holder.read()),
                            });
                        }
                    })
                    .span(Span::new(editor.to_string())),
            )
            .child(label().text(controls))
    });
    runner.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    runner.set_default_fonts(&["NotoSans".into()]);
    runner.sync_and_update();
    runner
}

struct Snapshot {
    cursor: Option<usize>,
    highlights: Vec<(usize, usize)>,
    controls: String,
}

fn snapshot(runner: &TestingRunner) -> Snapshot {
    let (cursor, highlights) = runner
        .find(|_, element| {
            let paragraph = Paragraph::try_downcast(element)?;
            Some((paragraph.cursor_index, paragraph.highlights.clone()))
        })
        .unwrap();
    let controls = runner
        .find(|_, element| Some(Label::try_downcast(element)?.text.to_string()))
        .unwrap();
    Snapshot {
        cursor,
        highlights,
        controls,
    }
}

fn tap(runner: &mut TestingRunner, location: (f64, f64)) {
    runner.press_touch(location);
    runner.release_touch(location);
}

#[test]
fn a_tap_places_the_cursor_when_the_finger_lifts() {
    let mut runner = touch_runner();
    runner.press_touch((120.0, 5.0));
    assert_eq!(snapshot(&runner).cursor, Some(0));

    runner.release_touch((120.0, 5.0));
    let after = snapshot(&runner);
    assert!(after.cursor.unwrap() > 0);
    assert!(after.highlights.is_empty());
    assert_eq!(after.controls, "handles=true toolbar=false");
}

#[test]
fn dragging_scrolls_instead_of_selecting() {
    let mut runner = touch_runner();
    runner.press_touch((10.0, 5.0));
    runner.move_touch((60.0, 5.0));
    runner.move_touch((120.0, 5.0));
    runner.release_touch((120.0, 5.0));

    let after = snapshot(&runner);
    assert_eq!(after.cursor, Some(0));
    assert!(after.highlights.is_empty());
}

#[test]
fn a_double_tap_selects_a_word() {
    let mut runner = touch_runner();
    tap(&mut runner, (90.0, 5.0));
    tap(&mut runner, (92.0, 5.0));

    let after = snapshot(&runner);
    assert_eq!(after.highlights, vec![(6, 16)]);
    assert_eq!(after.controls, "handles=true toolbar=true");
}

#[test]
fn slow_taps_only_move_the_cursor() {
    let mut runner = touch_runner();
    tap(&mut runner, (90.0, 5.0));
    std::thread::sleep(Duration::from_millis(350));
    tap(&mut runner, (92.0, 5.0));

    assert!(snapshot(&runner).highlights.is_empty());
}

#[test]
fn a_long_press_selects_a_word_and_dragging_extends_it_by_words() {
    let mut runner = touch_runner();
    runner.press_touch((10.0, 5.0));
    runner.press_key(Key::Character("L".into()));
    let pressed = snapshot(&runner);
    assert_eq!(pressed.highlights, vec![(0, 5)]);
    // The toolbar waits for the finger to lift.
    assert_eq!(pressed.controls, "handles=true toolbar=false");

    runner.move_touch((90.0, 5.0));
    assert_eq!(snapshot(&runner).highlights, vec![(0, 16)]);

    runner.release_touch((90.0, 5.0));
    assert_eq!(snapshot(&runner).controls, "handles=true toolbar=true");
}

#[test]
fn a_long_press_after_moving_is_ignored() {
    let mut runner = touch_runner();
    runner.press_touch((10.0, 5.0));
    runner.move_touch((80.0, 5.0));
    runner.press_key(Key::Character("L".into()));

    assert!(snapshot(&runner).highlights.is_empty());
}

#[test]
fn typing_hides_the_touch_controls() {
    let mut runner = touch_runner();
    tap(&mut runner, (90.0, 5.0));
    runner.press_key(Key::Character("x".into()));

    assert_eq!(snapshot(&runner).controls, "handles=false toolbar=false");
}

#[test]
fn tapping_the_cursor_again_toggles_the_toolbar() {
    let mut runner = touch_runner();
    tap(&mut runner, (90.0, 5.0));
    std::thread::sleep(Duration::from_millis(350));
    tap(&mut runner, (90.0, 5.0));
    assert_eq!(snapshot(&runner).controls, "handles=true toolbar=true");

    std::thread::sleep(Duration::from_millis(350));
    tap(&mut runner, (90.0, 5.0));
    assert_eq!(snapshot(&runner).controls, "handles=true toolbar=false");
}

#[test]
fn dragging_the_cursor_handle_moves_the_cursor() {
    let mut runner = touch_runner();
    tap(&mut runner, (10.0, 5.0));
    let before = snapshot(&runner).cursor.unwrap();

    runner.press_key(Key::Character("H".into()));
    runner.move_touch((120.0, 5.0));
    let dragged = snapshot(&runner);
    assert!(dragged.cursor.unwrap() > before);
    assert!(dragged.highlights.is_empty());

    runner.release_touch((120.0, 5.0));
    assert_eq!(snapshot(&runner).controls, "handles=true toolbar=true");
}

#[test]
fn the_end_handle_of_a_backwards_selection_moves_its_end() {
    let mut runner = touch_runner();
    // Long press on "Rustaceans" and drag back over "Hello": the selection is made backwards.
    runner.press_touch((90.0, 5.0));
    runner.press_key(Key::Character("L".into()));
    runner.move_touch((10.0, 5.0));
    runner.release_touch((10.0, 5.0));
    assert_eq!(snapshot(&runner).highlights, vec![(16, 0)]);

    // Dragging the end handle back keeps "Hello" selected.
    runner.press_key(Key::Character("E".into()));
    runner.move_touch((20.0, 5.0));
    let (from, to) = snapshot(&runner).highlights[0];
    assert_eq!(from.min(to), 0);
    assert!(from.max(to) < 16);
}
