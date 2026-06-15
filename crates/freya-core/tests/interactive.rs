use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn interactive_false_passes_events_through() {
    fn app() -> impl IntoElement {
        let mut behind = use_state(|| 0);
        let mut overlay = use_state(|| 0);

        rect()
            .width(Size::percent(100.))
            .height(Size::percent(100.))
            .child(
                rect()
                    .expanded()
                    .background(Color::RED)
                    .on_mouse_up(move |_| behind.set(behind() + 1)),
            )
            .child(
                rect()
                    .position(Position::new_absolute())
                    .layer(Layer::Overlay)
                    .interactive(false)
                    .expanded()
                    .child(
                        rect()
                            .expanded()
                            .opacity(0.99)
                            .on_mouse_up(move |_| overlay.set(overlay() + 1)),
                    ),
            )
            .child(label().text(format!("behind:{} overlay:{}", behind(), overlay())))
    }

    let mut test = launch_test(app);
    test.click_cursor((50.0, 50.0));

    assert!(
        test.find(|_, e| Label::try_downcast(e).filter(|l| l.text.as_ref() == "behind:1 overlay:0"))
            .is_some()
    );
}
