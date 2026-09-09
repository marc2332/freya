use std::time::Duration;

use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
#[should_panic]
fn detached_task_writes_to_unmounted_state() {
    #[derive(PartialEq)]
    struct Child;

    impl Component for Child {
        fn render(&self) -> impl IntoElement {
            let mut value = use_state(|| 0);

            use_hook(move || {
                spawn_forever(async move {
                    let mut next = 0;
                    loop {
                        futures_lite::future::yield_now().await;
                        next += 1;
                        value.set(next);
                    }
                })
            });

            label().text(value.read().to_string())
        }
    }

    fn app() -> impl IntoElement {
        let mut show = use_state(|| true);

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .on_mouse_up(move |_| show.set(false))
            .maybe(*show.read(), |el| el.child(Child))
    }

    let mut test = launch_test(app);
    test.poll(Duration::from_millis(1), Duration::from_millis(30));

    test.click_cursor((50.0, 50.0));
    test.poll(Duration::from_millis(1), Duration::from_millis(30));
}

#[test]
fn scoped_task_is_cancelled_on_unmount() {
    #[derive(PartialEq)]
    struct Child;

    impl Component for Child {
        fn render(&self) -> impl IntoElement {
            let mut value = use_state(|| 0);

            use_hook(move || {
                spawn(async move {
                    let mut next = 0;
                    loop {
                        futures_lite::future::yield_now().await;
                        next += 1;
                        value.set(next);
                    }
                })
            });

            label().text(value.read().to_string())
        }
    }

    fn app() -> impl IntoElement {
        let mut show = use_state(|| true);

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .on_mouse_up(move |_| show.set(false))
            .maybe(*show.read(), |el| el.child(Child))
    }

    let mut test = launch_test(app);
    test.poll(Duration::from_millis(1), Duration::from_millis(30));

    test.click_cursor((50.0, 50.0));
    test.poll(Duration::from_millis(1), Duration::from_millis(30));
}
