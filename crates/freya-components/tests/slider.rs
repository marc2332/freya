use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
pub fn slider_mouse_drag_horizontal() {
    fn slider_app() -> impl IntoElement {
        let mut value = use_state(|| 0.0);

        rect()
            .child(label().text(format!("Value: {}", value() as i32)))
            .child(
                Slider::new(move |v| value.set(v))
                    .value(value())
                    .size(Size::px(200.)),
            )
    }

    let mut test = launch_test(slider_app);
    test.sync_and_update();

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Value:"))
                .map(|_| node)
        })
        .unwrap();

    assert_eq!(
        Label::try_downcast(&*label.element()).unwrap().text,
        "Value: 0"
    );

    // Click at the middle of the slider (should be around 50%)
    test.click_cursor((100.0, 20.0));

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Value:"))
                .map(|_| node)
        })
        .unwrap();

    let value_text = Label::try_downcast(&*label.element()).unwrap().text;
    let value: f64 = value_text.replace("Value: ", "").parse().unwrap();

    // Should be around 50% (middle of 200px slider)
    assert!((value - 50.0).abs() < 10.0);

    // Drag to the right
    test.move_cursor((150.0, 20.0));
    test.sync_and_update();
    test.press_cursor((150.0, 20.0));
    test.sync_and_update();
    test.move_cursor((175.0, 20.0));
    test.sync_and_update();
    test.release_cursor((175.0, 20.0));
    test.sync_and_update();

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Value:"))
                .map(|_| node)
        })
        .unwrap();

    let value_text = Label::try_downcast(&*label.element()).unwrap().text;
    let value: f64 = value_text.replace("Value: ", "").parse().unwrap();

    // Should be around 87% (175 out of 200px)
    assert!((value - 87.0).abs() < 10.0);
}

#[test]
pub fn slider_keyboard_horizontal() {
    fn slider_app() -> impl IntoElement {
        let mut value = use_state(|| 50.0);

        rect()
            .child(label().text(format!("Value: {}", value() as i32)))
            .child(
                Slider::new(move |v| value.set(v))
                    .value(value())
                    .size(Size::px(200.)),
            )
    }

    let mut test = launch_test(slider_app);
    test.sync_and_update();

    // Focus the slider by clicking on it
    test.click_cursor((100.0, 20.0));

    // Press ArrowRight to increase value
    for _ in 0..5 {
        test.press_key(Key::ArrowRight);
    }

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Value:"))
                .map(|_| node)
        })
        .unwrap();

    let value_text = Label::try_downcast(&*label.element()).unwrap().text;
    let value: f64 = value_text.replace("Value: ", "").parse().unwrap();

    // Should have increased by 5 * 4 = 20 (from ~50 to ~70)
    assert!((value - 75.0).abs() < 10.0);

    // Press ArrowLeft to decrease value
    for _ in 0..3 {
        test.press_key(Key::ArrowLeft);
    }

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Value:"))
                .map(|_| node)
        })
        .unwrap();

    let value_text = Label::try_downcast(&*label.element()).unwrap().text;
    let value: f64 = value_text.replace("Value: ", "").parse().unwrap();

    // Should have decreased by 3 * 4 = 12 (from ~70 to ~58)
    assert!((value - 58.0).abs() < 10.0);
}

#[test]
pub fn slider_vertical() {
    fn slider_app() -> impl IntoElement {
        let mut value = use_state(|| 0.0);

        rect()
            .child(label().text(format!("Value: {}", value() as i32)))
            .child(
                Slider::new(move |v| value.set(v))
                    .value(value())
                    .direction(Direction::Vertical)
                    .size(Size::px(200.)),
            )
    }

    let mut test = launch_test(slider_app);
    test.sync_and_update();

    // Click at the middle of the vertical slider
    // For vertical, coordinates are inverted (top = 100%, bottom = 0%)
    test.click_cursor((20.0, 105.0));

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Value:"))
                .map(|_| node)
        })
        .unwrap();

    let value_text = Label::try_downcast(&*label.element()).unwrap().text;
    let value: f64 = value_text.replace("Value: ", "").parse().unwrap();

    // Should be around 50%
    assert!((value - 50.0).abs() < 10.0);
}

#[test]
pub fn slider_keyboard_vertical() {
    fn slider_app() -> impl IntoElement {
        let mut value = use_state(|| 50.0);

        rect()
            .child(label().text(format!("Value: {}", value() as i32)))
            .child(
                Slider::new(move |v| value.set(v))
                    .value(value())
                    .direction(Direction::Vertical)
                    .size(Size::px(200.)),
            )
    }

    let mut test = launch_test(slider_app);
    test.sync_and_update();

    // Focus the slider by clicking on it
    test.click_cursor((20.0, 105.0));

    // Press ArrowUp to increase value (vertical slider)
    for _ in 0..5 {
        test.press_key(Key::ArrowUp);
    }

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Value:"))
                .map(|_| node)
        })
        .unwrap();

    let value_text = Label::try_downcast(&*label.element()).unwrap().text;
    let value: f64 = value_text.replace("Value: ", "").parse().unwrap();

    // Should have increased by 5 * 4 = 20 (from ~50 to ~70)
    assert!((value - 70.0).abs() < 10.0);

    // Press ArrowDown to decrease value
    for _ in 0..3 {
        test.press_key(Key::ArrowDown);
    }

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Value:"))
                .map(|_| node)
        })
        .unwrap();

    let value_text = Label::try_downcast(&*label.element()).unwrap().text;
    let value: f64 = value_text.replace("Value: ", "").parse().unwrap();

    // Should have decreased by 3 * 4 = 12 (from ~70 to ~58)
    assert!((value - 58.0).abs() < 10.0);
}

#[test]
pub fn slider_clamping() {
    fn slider_app() -> impl IntoElement {
        let mut value = use_state(|| 95.0);

        rect()
            .child(label().text(format!("Value: {}", value() as i32)))
            .child(
                Slider::new(move |v| value.set(v))
                    .value(value())
                    .size(Size::px(200.)),
            )
    }

    let mut test = launch_test(slider_app);
    test.sync_and_update();

    // Focus the slider
    test.click_cursor((100.0, 20.0));

    // Try to go beyond 100%
    for _ in 0..25 {
        test.press_key(Key::ArrowRight);
    }

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Value:"))
                .map(|_| node)
        })
        .unwrap();

    let value_text = Label::try_downcast(&*label.element()).unwrap().text;
    let value: f64 = value_text.replace("Value: ", "").parse().unwrap();

    // Should be clamped at 100
    assert_eq!(value, 100.0);

    // Try to go below 0%
    for _ in 0..30 {
        test.press_key(Key::ArrowLeft);
    }

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Value:"))
                .map(|_| node)
        })
        .unwrap();

    let value_text = Label::try_downcast(&*label.element()).unwrap().text;
    let value: f64 = value_text.replace("Value: ", "").parse().unwrap();

    // Should be clamped at 0
    assert_eq!(value, 0.0);
}

#[test]
pub fn slider_disabled() {
    fn slider_app() -> impl IntoElement {
        let mut value = use_state(|| 50.0);

        rect()
            .child(label().text(format!("Value: {}", value() as i32)))
            .child(
                Slider::new(move |v| value.set(v))
                    .value(value())
                    .enabled(false)
                    .size(Size::px(200.)),
            )
    }

    let mut test = launch_test(slider_app);
    test.sync_and_update();

    // Try to click the slider
    test.click_cursor((150.0, 20.0));

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Value:"))
                .map(|_| node)
        })
        .unwrap();

    let value_text = Label::try_downcast(&*label.element()).unwrap().text;
    let value: f64 = value_text.replace("Value: ", "").parse().unwrap();

    // Value should remain unchanged at 50
    assert_eq!(value, 50.0);

    // Try keyboard input
    test.press_key(Key::ArrowRight);

    let label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Value:"))
                .map(|_| node)
        })
        .unwrap();

    let value_text = Label::try_downcast(&*label.element()).unwrap().text;
    let value: f64 = value_text.replace("Value: ", "").parse().unwrap();

    // Value should still be 50
    assert_eq!(value, 50.0);
}
