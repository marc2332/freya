use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn switch_writable_setter_test() {
    fn app() -> impl IntoElement {
        let toggled = use_state(|| false);
        let setter = toggled.into_writable().into_writable_setter();

        rect()
            .child(Switch::new(setter))
            .child(format!("toggled={}", toggled.read()))
    }

    let mut test = launch_test(app);
    test.click_cursor((15., 15.));

    assert!(
        test.find(|_, element| {
            Label::try_downcast(element).filter(|label| label.text.as_ref() == "toggled=true")
        })
        .is_some()
    );

    test.click_cursor((15., 15.));

    assert!(
        test.find(|_, element| {
            Label::try_downcast(element).filter(|label| label.text.as_ref() == "toggled=false")
        })
        .is_some()
    );
}

#[test]
fn switch_callback_setter_test() {
    fn app() -> impl IntoElement {
        let mut toggled = use_state(|| false);
        let mut updates = use_state(|| 0);
        let setter = WritableSetter::new(toggled, move |value| {
            *updates.write() += 1;

            if *updates.peek() > 1 {
                toggled.set(value);
            }
        });

        rect()
            .child(Switch::new(setter))
            .child(format!("{}:{}", toggled.read(), updates.read()))
    }

    let mut test = launch_test(app);
    test.click_cursor((15., 15.));

    assert!(
        test.find(|_, element| {
            Label::try_downcast(element).filter(|label| label.text.as_ref() == "false:1")
        })
        .is_some()
    );

    test.click_cursor((15., 15.));

    assert!(
        test.find(|_, element| {
            Label::try_downcast(element).filter(|label| label.text.as_ref() == "true:2")
        })
        .is_some()
    );
}
