use freya::prelude::Button;
use freya_core::prelude::*;
use freya_testing::prelude::*;

#[test]
pub fn button_test() {
    fn button_app() -> impl IntoElement {
        let mut state = use_state(|| false);

        Button::new()
            .on_press(move |_| {
                state.toggle();
            })
            .child(format!("{}", state.read()))
    }

    let mut test = launch_test(button_app);

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "false")
    });
    assert!(label.is_some());

    test.click_cursor((15.0, 15.0));

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "true")
    });
    assert!(label.is_some());
}

#[test]
pub fn button_keyboard_press_test() {
    fn button_app() -> impl IntoElement {
        let mut state = use_state(|| false);

        Button::new()
            .on_press(move |_| {
                state.toggle();
            })
            .child(format!("{}", state.read()))
    }

    let mut test = launch_test(button_app);

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "false")
    });
    assert!(label.is_some());

    // Focus the button by clicking it
    test.press_key(Key::Named(NamedKey::Tab));
    test.sync_and_update();
    test.sync_and_update();
    test.sync_and_update();
    test.sync_and_update();

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "false")
    });
    assert!(label.is_some());

    // Press Enter key while button is focused
    test.press_key(Key::Named(NamedKey::Enter));

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "true")
    });
    assert!(label.is_some());
}
