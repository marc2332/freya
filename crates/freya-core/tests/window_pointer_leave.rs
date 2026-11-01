use freya_core::{
    integration::*,
    prelude::*,
};
use freya_testing::TestingRunner;
use torin::size::Size;

#[test]
fn pointer_exit_clears_hover_without_global_move() {
    #[derive(Clone, Copy)]
    struct Counters(State<(usize, usize, usize)>);

    fn app() -> impl IntoElement {
        let mut counters = use_consume::<Counters>().0;
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .on_pointer_enter(move |_| counters.write().0 += 1)
            .on_pointer_leave(move |event: Event<PointerEventData>| {
                assert_eq!(event.global_location(), (10., 10.).into());
                counters.write().1 += 1;
            })
            .on_global_pointer_move(move |_| counters.write().2 += 1)
    }

    let (mut test, counters) = TestingRunner::new(
        app,
        (100., 100.).into(),
        |runner| runner.provide_root_context(|| Counters(State::create((0, 0, 0)))),
        1.,
    );
    test.sync_and_update();

    test.move_cursor((10., 10.));
    assert_eq!(*counters.0.peek(), (1, 0, 1));

    test.send_event(PlatformEvent::PointerExit {
        cursor: (10., 10.).into(),
    });
    test.sync_and_update();
    assert_eq!(*counters.0.peek(), (1, 1, 1));

    test.move_cursor((-1., -1.));
    assert_eq!(*counters.0.peek(), (1, 1, 2));

    test.move_cursor((10., 10.));
    assert_eq!(*counters.0.peek(), (2, 1, 3));
}
