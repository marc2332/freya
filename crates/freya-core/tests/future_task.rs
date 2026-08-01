use std::time::Duration;

use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn future_restarts_when_subscribed_state_changes() {
    fn app() -> impl IntoElement {
        let mut value = use_state(|| 1);
        let future = use_future(move || {
            let value = value();
            async move { value * 2 }
        });

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .on_mouse_up(move |_| value.set(value() + 1))
            .child(match &*future.state() {
                FutureState::Fulfilled(doubled) => doubled.to_string(),
                _ => "loading".to_string(),
            })
    }

    let mut test = launch_test(app);
    test.poll(Duration::from_millis(1), Duration::from_millis(30));

    assert!(
        test.find(|_, element| Label::try_downcast(element).filter(|l| l.text.as_ref() == "2"))
            .is_some()
    );

    test.click_cursor((50.0, 50.0));
    test.poll(Duration::from_millis(1), Duration::from_millis(30));

    assert!(
        test.find(|_, element| Label::try_downcast(element).filter(|l| l.text.as_ref() == "4"))
            .is_some()
    );
}
