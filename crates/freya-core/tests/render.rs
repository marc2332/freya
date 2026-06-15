use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
pub fn basic_render() {
    fn app() -> impl IntoElement {
        let mut show_popup = use_state(|| true);

        rect()
            .child(
                Popup::new()
                    .on_close_request(move |_| show_popup.set(false))
                    .maybe(show_popup(), |popup| {
                        popup
                            .child(PopupTitle::new("Title".to_string()))
                            .child(PopupContent::new().child("Hello, World!"))
                    }),
            )
            .child(
                Button::new()
                    .child("Open")
                    .on_press(move |_| show_popup.toggle()),
            )
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    let data = test.render();

    assert!(!data.is_empty());
}
