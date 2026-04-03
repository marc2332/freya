use std::time::Duration;

use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn owned_task_handle_cancels_on_drop() {
    fn app() -> impl IntoElement {
        let mut counter = use_consume::<State<i32>>();

        let mut handle: State<Option<OwnedTaskHandle>> = use_hook(|| {
            let owned = spawn(async move {
                loop {
                    *counter.write() += 1;
                    futures_lite::future::yield_now().await;
                }
            })
            .owned();

            State::create(Some(owned))
        });

        Button::new().child("Drop").on_press(move |_| {
            handle.write().take();
        })
    }

    let (mut test, counter) = TestingRunner::new(
        app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(0i32)),
        1.,
    );

    test.poll(Duration::from_millis(1), Duration::from_millis(30));

    let count_before = *counter.peek();
    assert!(count_before > 0);

    test.click_cursor((50.0, 20.0));
    test.poll(Duration::from_millis(1), Duration::from_millis(30));

    let count_after_drop = *counter.peek();

    test.poll(Duration::from_millis(1), Duration::from_millis(30));

    let count_after_more_polling = *counter.peek();
    assert_eq!(count_after_drop, count_after_more_polling);
}
