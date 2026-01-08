use freya::prelude::Input;
use freya_core::prelude::*;
use freya_testing::prelude::*;

#[test]
pub fn input_test() {
    fn input_app() -> impl IntoElement {
        let mut value = use_state(String::new);

        rect()
            .spacing(6.)
            .child(
                Input::new()
                    .placeholder("Type your name")
                    .value(value.read().clone())
                    .on_change(move |v| value.set(v)),
            )
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
