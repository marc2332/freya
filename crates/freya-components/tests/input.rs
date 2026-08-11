use std::collections::HashMap;

use freya::prelude::*;
use freya_testing::prelude::*;

/// An input over a known font, so a press at a given x lands on a known character.
fn measured_input(value: &'static str) -> TestingRunner {
    let mut utils = launch_test(move || {
        let value = use_state(|| value.to_string());
        Input::new(value).width(Size::px(300.))
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("../../freya-edit/tests/NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);
    utils.sync_and_update();
    utils
}

fn highlights(utils: &mut TestingRunner) -> Option<Vec<(usize, usize)>> {
    utils.find(|_, e| Some(Paragraph::try_downcast(e)?.highlights.clone()))
}

#[test]
pub fn input_double_click_selects_the_word_under_the_pointer() {
    let mut utils = measured_input("hello world");

    // Two presses in the same spot inside the first word.
    utils.click_cursor((20.0, 15.0));
    utils.click_cursor((20.0, 15.0));
    // The press that focuses the input renders before the focus lands, and an unfocused
    // input paints no highlight at all: without this frame the selection is right and
    // the assertion still reads an empty one.
    utils.sync_and_update();
    utils.sync_and_update();

    assert_eq!(highlights(&mut utils), Some(vec![(0, 5)]));

    // A third press widens to the line.
    utils.click_cursor((20.0, 15.0));
    assert_eq!(highlights(&mut utils), Some(vec![(0, 11)]));
}

#[test]
pub fn input_double_click_survives_the_pointer_moving_under_it() {
    let mut utils = measured_input("hello world");

    utils.click_cursor((20.0, 15.0));
    utils.press_cursor((20.0, 15.0));
    utils.sync_and_update();
    utils.sync_and_update();
    assert_eq!(highlights(&mut utils), Some(vec![(0, 5)]));

    // A real double click is never perfectly still. The word the press selected must
    // survive the pointer twitching inside it, rather than collapsing to the pointer.
    utils.move_cursor((21.0, 15.0));
    utils.sync_and_update();
    assert_eq!(highlights(&mut utils), Some(vec![(0, 5)]));

    // Dragging on past the word extends by whole words, never mid-word.
    utils.move_cursor((60.0, 15.0));
    utils.sync_and_update();
    assert_eq!(highlights(&mut utils), Some(vec![(0, 11)]));

    utils.release_cursor((60.0, 15.0));
}

/// The line jump: Cmd on macOS, Home/End everywhere.
#[cfg(target_os = "macos")]
const LINE_JUMP: (Key, Key, Modifiers) = (
    Key::Named(NamedKey::ArrowLeft),
    Key::Named(NamedKey::ArrowRight),
    Modifiers::META,
);
#[cfg(not(target_os = "macos"))]
const LINE_JUMP: (Key, Key, Modifiers) = (
    Key::Named(NamedKey::Home),
    Key::Named(NamedKey::End),
    Modifiers::empty(),
);

#[test]
pub fn input_jumps_and_selects_to_the_line_bounds() {
    let (to_start, to_end, jump) = LINE_JUMP;
    let mut utils = measured_input("hello world");

    // Focus with the caret somewhere in the middle.
    utils.click_cursor((20.0, 15.0));
    utils.sync_and_update();

    utils.press_key_with_modifiers(to_start.clone(), jump | Modifiers::SHIFT);
    assert_eq!(highlights(&mut utils), Some(vec![(1, 0)]));

    utils.press_key_with_modifiers(to_end.clone(), jump | Modifiers::SHIFT);
    assert_eq!(highlights(&mut utils), Some(vec![(1, 11)]));

    // Unshifted, the same chord moves the caret and drops the selection.
    utils.press_key_with_modifiers(to_start, jump);
    assert_eq!(highlights(&mut utils), Some(vec![]));

    // Typing now lands at the line start, which is what the jump was for.
    utils.write_text("X");
    let text = utils.find(|_, e| {
        Paragraph::try_downcast(e).filter(|p| p.spans.iter().any(|s| s.text == "Xhello world"))
    });
    assert!(text.is_some());
}

#[test]
pub fn input_test() {
    fn input_app() -> impl IntoElement {
        let value = use_state(String::new);

        rect()
            .spacing(6.)
            .child(Input::new(value).placeholder("Type your name"))
            .child(format!("Your name is {}", value.read()))
    }

    let mut test = launch_test(input_app);

    let placeholder = test.find(|_, element| {
        Paragraph::try_downcast(element)
            .filter(|paragraph| paragraph.spans.iter().any(|s| s.text == "Type your name"))
    });
    assert!(placeholder.is_some());
    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "Your name is ")
    });
    assert!(label.is_some());

    // Focus
    test.click_cursor((15.0, 15.0));
    // Type
    test.write_text("Rust");

    let text = test.find(|_, element| {
        Paragraph::try_downcast(element)
            .filter(|paragraph| paragraph.spans.iter().any(|s| s.text == "Rust"))
    });
    assert!(text.is_some());
    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "Your name is Rust")
    });
    assert!(label.is_some());
}

#[test]
pub fn input_password_mode_test() {
    fn password_app() -> impl IntoElement {
        let value = use_state(String::new);

        rect()
            .child(Input::new(value).mode(InputMode::new_password()))
            .child(format!("value={}", value.read()))
    }

    let mut test = launch_test(password_app);

    // Focus and type
    test.click_cursor((15.0, 15.0));
    test.write_text("secret");

    // The rendered paragraph should show masked characters
    let masked = test.find(|_, element| {
        Paragraph::try_downcast(element)
            .filter(|paragraph| paragraph.spans.iter().any(|s| s.text == "******"))
    });
    assert!(masked.is_some());

    // But the underlying state should hold the real value
    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "value=secret")
    });
    assert!(label.is_some());
}

#[test]
pub fn input_validator_test() {
    fn validator_app() -> impl IntoElement {
        let value = use_state(String::new);

        rect()
            .child(
                Input::new(value).on_validate(move |validator: InputValidator| {
                    // Only allow numeric input
                    if !validator.text().chars().all(|c| c.is_ascii_digit()) {
                        validator.set_valid(false);
                    }
                }),
            )
            .child(format!("value={}", value.read()))
    }

    let mut test = launch_test(validator_app);

    // Focus
    test.click_cursor((15.0, 15.0));

    // Type invalid text (letters), should be rejected
    test.write_text("abc");

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "value=")
    });
    assert!(label.is_some());

    // Type valid text (digits), should be accepted
    test.write_text("123");

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "value=123")
    });
    assert!(label.is_some());
}

#[test]
pub fn input_submit_test() {
    fn submit_app() -> impl IntoElement {
        let value = use_state(String::new);
        let mut submitted = use_state(String::new);

        rect()
            .child(Input::new(value).on_submit(move |text: String| {
                submitted.set(text);
            }))
            .child(format!("submitted={}", submitted.read()))
    }

    let mut test = launch_test(submit_app);

    // Focus and type
    test.click_cursor((15.0, 15.0));
    test.write_text("hello");

    // Not yet submitted
    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "submitted=")
    });
    assert!(label.is_some());

    // Press Enter to submit
    test.press_key(Key::Named(NamedKey::Enter));

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "submitted=hello")
    });
    assert!(label.is_some());
}

#[test]
pub fn input_disabled_test() {
    fn disabled_app() -> impl IntoElement {
        let value = use_state(String::new);

        rect()
            .child(Input::new(value).enabled(false))
            .child(format!("value={}", value.read()))
    }

    let mut test = launch_test(disabled_app);

    // Try to focus and type
    test.click_cursor((15.0, 15.0));
    test.write_text("hello");

    // Value should remain empty since input is disabled
    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "value=")
    });
    assert!(label.is_some());
}

#[test]
pub fn input_escape_unfocus_test() {
    fn escape_app() -> impl IntoElement {
        let value = use_state(String::new);

        rect()
            .child(Input::new(value))
            .child(format!("value={}", value.read()))
    }

    let mut test = launch_test(escape_app);

    // Focus and type
    test.click_cursor((15.0, 15.0));
    test.write_text("hello");

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "value=hello")
    });
    assert!(label.is_some());

    // Press Escape to unfocus.
    test.press_key(Key::Named(NamedKey::Escape));
    test.sync_and_update();

    // Type more text, should not be captured since the input lost focus
    test.write_text("world");

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "value=hello")
    });
    assert!(label.is_some());
}

#[test]
pub fn input_auto_focus_test() {
    fn auto_focus_app() -> impl IntoElement {
        let value = use_state(String::new);

        rect()
            .child(Input::new(value).auto_focus(true))
            .child(format!("value={}", value.read()))
    }

    let mut test = launch_test(auto_focus_app);

    test.sync_and_update();

    // Type without clicking, auto_focus should have focused the input
    test.write_text("typed");

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "value=typed")
    });
    assert!(label.is_some());
}

#[test]
pub fn input_select_all_on_init_test() {
    fn rename_app() -> impl IntoElement {
        // Seeded before the input mounts, the way a rename affordance opens over the name it is
        // replacing.
        let value = use_state(|| "old name".to_string());

        rect()
            .child(Input::new(value).auto_focus(true).select_all_on_init(true))
            .child(format!("value={}", value.read()))
    }

    let mut test = launch_test(rename_app);

    test.sync_and_update();

    // No click needed: the input auto-focuses, and the seeded value arrives selected.
    test.write_text("new");

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "value=new")
    });
    assert!(
        label.is_some(),
        "typing replaced the selected value instead of landing in front of it"
    );
}

#[test]
pub fn input_without_select_all_on_init_keeps_the_cursor_at_the_start() {
    fn seeded_app() -> impl IntoElement {
        let value = use_state(|| "old name".to_string());

        rect()
            .child(Input::new(value).auto_focus(true))
            .child(format!("value={}", value.read()))
    }

    let mut test = launch_test(seeded_app);

    test.sync_and_update();

    test.write_text("new");

    // The default is unchanged: an input mounts with its cursor at position 0, so this inserts
    // there. `select_all_on_init` is opt-in precisely because that is right for a field the user
    // is editing rather than replacing.
    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "value=newold name")
    });
    assert!(label.is_some());
}
