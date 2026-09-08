use std::time::Duration;

use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn task_handles_report_finished_tasks() {
    fn app() -> impl IntoElement {
        let mut states = use_consume::<State<Vec<bool>>>();

        let completed_task = use_hook(|| spawn(async { futures_lite::future::yield_now().await }));

        let forever_task = use_hook(|| {
            spawn(async {
                loop {
                    futures_lite::future::yield_now().await;
                }
            })
            .owned()
        });

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .on_mouse_up(move |_| {
                states.write().push(completed_task.is_finished());
                states.write().push(forever_task.is_finished());
                forever_task.cancel();
                states.write().push(forever_task.is_finished());
            })
    }

    let (mut test, states) = TestingRunner::new(
        app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(Vec::<bool>::new())),
        1.,
    );

    test.poll(Duration::from_millis(1), Duration::from_millis(30));

    test.click_cursor((50.0, 50.0));
    test.poll(Duration::from_millis(1), Duration::from_millis(30));

    assert_eq!(*states.peek(), vec![true, false, true]);
}
