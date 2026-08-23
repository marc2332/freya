use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
pub fn on_visible_with_scrollview() {
    fn on_visible_app() -> impl IntoElement {
        let mut count = use_consume::<State<i32>>();

        ScrollView::new()
            .child(rect().height(Size::px(600.)).width(Size::px(200.)))
            .child(
                rect()
                    .height(Size::px(200.))
                    .width(Size::px(200.))
                    .on_visible(move |_| *count.write() += 1),
            )
    }

    let (mut test, count) = TestingRunner::new(
        on_visible_app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(0i32)),
        1.,
    );
    test.sync_and_update();

    // The target sits at 600..800, below the 500px tall viewport
    assert_eq!(*count.peek(), 0);

    // Scrolling it into view fires the event
    test.scroll((250., 250.), (0., -300.));
    test.sync_and_update();
    assert_eq!(*count.peek(), 1);

    // Scrolling further while it stays visible does not fire again
    test.scroll((250., 250.), (0., -50.));
    test.sync_and_update();
    assert_eq!(*count.peek(), 1);

    // Scrolling it fully out and back in fires again
    test.scroll((250., 250.), (0., 350.));
    test.sync_and_update();
    assert_eq!(*count.peek(), 1);
    test.scroll((250., 250.), (0., -300.));
    test.sync_and_update();
    assert_eq!(*count.peek(), 2);
}
