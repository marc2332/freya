use freya::prelude::*;
use freya_testing::prelude::*;

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

    // Press Escape to unfocus
    test.press_key(Key::Named(NamedKey::Escape));

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

    // Type without clicking, auto_focus should have focused the input
    test.write_text("typed");

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "value=typed")
    });
    assert!(label.is_some());
}
