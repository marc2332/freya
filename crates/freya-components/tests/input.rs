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
pub fn input_multiline_test() {
    fn multiline_app() -> impl IntoElement {
        let value = use_state(String::new);
        let mut submitted = use_state(|| false);

        rect()
            .child(
                Input::new(value)
                    .multiline(true)
                    .on_submit(move |_| submitted.set(true)),
            )
            .child(format!(
                "value={:?} submitted={}",
                value.read(),
                submitted.read()
            ))
    }

    let mut test = launch_test(multiline_app);

    // Focus and type two lines
    test.click_cursor((15.0, 15.0));
    test.write_text("hello");
    test.press_key(Key::Named(NamedKey::Enter));
    test.write_text("world");

    // Enter inserted a line break instead of submitting
    let label = test.find(|_, element| {
        Label::try_downcast(element)
            .filter(|label| label.text.as_ref() == "value=\"hello\\nworld\" submitted=false")
    });
    assert!(label.is_some());
}

#[test]
pub fn input_multiline_scrollbar_press_test() {
    fn scrollbar_app() -> impl IntoElement {
        let value = use_state(|| "One\nTwo\nThree\nFour\nFive\nSix\nSeven".to_string());

        rect().child(
            Input::new(value)
                .multiline(true)
                .width(Size::px(300.))
                .height(Size::px(120.)),
        )
    }

    let mut test = launch_test(scrollbar_app);
    test.sync_and_update();

    test.click_cursor((100.0, 40.0));
    test.press_cursor((292.0, 30.0));
    test.move_cursor((100.0, 90.0));
    test.sync_and_update();

    let highlights =
        test.find(|_, element| Some(Paragraph::try_downcast(element)?.highlights.clone()));
    assert_eq!(highlights, Some(vec![]));
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
pub fn input_shift_wheel_scroll_test() {
    fn scroll_app() -> impl IntoElement {
        let value = use_state(String::new);

        rect().child(Input::new(value).width(Size::px(150.)))
    }

    let mut test = launch_test(scroll_app);

    // Focus and fill with text wider than the input
    test.click_cursor((15.0, 15.0));
    test.write_text("this is a very long text that overflows the input width");
    test.sync_and_update();
    test.sync_and_update();

    let paragraph_x = |test: &TestingRunner| {
        test.find(|node, element| {
            Paragraph::try_downcast(element).map(|_| node.layout().area.min_x())
        })
        .unwrap()
    };

    // Move the cursor back to the start so the input is scrolled to the beginning
    test.press_key(Key::Named(NamedKey::Home));
    test.sync_and_update();
    let initial_x = paragraph_x(&test);

    // Hold Shift and wheel over the input to scroll it horizontally while focused
    test.send_event(PlatformEvent::Keyboard {
        name: KeyboardEventName::KeyDown,
        key: Key::Named(NamedKey::Shift),
        code: Code::ShiftLeft,
        modifiers: Modifiers::SHIFT,
    });
    test.sync_and_update();

    test.send_event(PlatformEvent::Wheel {
        name: WheelEventName::Wheel,
        scroll: (0.0, -50.0).into(),
        cursor: (75.0, 15.0).into(),
        source: WheelSource::Device,
    });
    test.sync_and_update();

    let scrolled_x = paragraph_x(&test);
    assert!(scrolled_x < initial_x);
}

#[test]
pub fn input_drag_scrolls_to_cursor_test() {
    fn drag_app() -> impl IntoElement {
        let value =
            use_state(|| "this is a very long text that overflows the input width".to_string());

        rect().child(Input::new(value).width(Size::px(150.)))
    }

    let mut test = launch_test(drag_app);
    test.sync_and_update();

    let paragraph_x = |test: &TestingRunner| {
        test.find(|node, element| {
            Paragraph::try_downcast(element).map(|_| node.layout().area.min_x())
        })
        .unwrap()
    };
    let initial_x = paragraph_x(&test);

    // Start dragging inside the input and move past its right edge
    test.press_cursor((100.0, 15.0));
    test.sync_and_update();
    test.move_cursor((400.0, 15.0));
    test.sync_and_update();

    let dragged_x = paragraph_x(&test);
    assert!(dragged_x < initial_x);

    // Dragging back past the left edge scrolls back to the beginning
    test.move_cursor((-500.0, 15.0));
    test.sync_and_update();

    assert_eq!(paragraph_x(&test), initial_x);

    test.release_cursor((-500.0, 15.0));
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
pub fn input_long_typing_follows_cursor_test() {
    fn typing_app() -> impl IntoElement {
        let value = use_state(String::new);

        rect().child(Input::new(value).width(Size::px(150.)))
    }

    let mut test = launch_test(typing_app);

    let paragraph_metrics = |test: &TestingRunner| {
        test.find(|node, element| {
            Paragraph::try_downcast(element)
                .map(|_| (node.layout().area.min_x(), node.layout().area.width()))
        })
        .unwrap()
    };

    test.click_cursor((15.0, 15.0));
    let (initial_x, _) = paragraph_metrics(&test);

    // Keep typing way beyond the input width, the scroll must follow the cursor every time
    for _ in 0..20 {
        test.write_text("some more text ");
        test.sync_and_update();
        test.sync_and_update();

        let (min_x, width) = paragraph_metrics(&test);
        if width > 150.0 {
            let right_edge = min_x + width;
            assert!(
                (right_edge - 150.0).abs() < 2.0,
                "text end should stay at the right edge, min_x={min_x} width={width}"
            );
        }
    }

    // Moving the cursor back to the start scrolls the input back to the beginning
    test.press_key(Key::Named(NamedKey::Home));
    test.sync_and_update();

    let (home_x, _) = paragraph_metrics(&test);
    assert_eq!(home_x, initial_x);
}
