use std::time::Duration;

use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn scoped_task_survives_the_unmount_of_the_element_component() {
    #[derive(PartialEq)]
    struct Child {
        on_mouse_up: EventHandler<Event<MouseEventData>>,
    }

    impl Component for Child {
        fn render(&self) -> impl IntoElement {
            rect()
                .width(Size::fill())
                .height(Size::px(50.))
                .on_mouse_up(self.on_mouse_up.clone())
        }
    }

    fn app() -> impl IntoElement {
        let mut counter = use_consume::<State<i32>>();
        let mut show = use_state(|| true);

        let on_mouse_up = EventHandler::new_current(move |_| {
            spawn(async move {
                loop {
                    *counter.write() += 1;
                    futures_lite::future::yield_now().await;
                }
            });
        });

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .maybe(*show.read(), |el| el.child(Child { on_mouse_up }))
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::px(50.))
                    .on_mouse_up(move |_| show.set(false)),
            )
    }

    let (mut test, counter) = TestingRunner::new(
        app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(0i32)),
        1.,
    );
    test.poll(Duration::from_millis(1), Duration::from_millis(30));

    test.click_cursor((10.0, 10.0));
    test.poll(Duration::from_millis(1), Duration::from_millis(30));

    let count_before_unmount = *counter.peek();
    assert!(count_before_unmount > 0);

    test.click_cursor((10.0, 60.0));
    test.poll(Duration::from_millis(1), Duration::from_millis(30));

    assert!(*counter.peek() > count_before_unmount);
}
