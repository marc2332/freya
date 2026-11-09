use std::time::Duration;

use freya::prelude::*;
use freya_testing::prelude::*;

#[derive(Default, Clone)]
struct DragState {
    generated: usize,
    previews: usize,
    finished: Option<Result<NativeDragResult, DragError>>,
}

fn app(enabled: bool) -> impl IntoElement {
    let mut state = use_consume::<State<DragState>>();

    NativeDrag::new(move || {
        state.write().generated += 1;
        TransferData::text("Dragged text")
    })
    .width(Size::px(100.))
    .height(Size::px(100.))
    .enabled(enabled)
    .preview(move || {
        state.write().previews += 1;
        "Preview".into_element()
    })
    .on_finished(move |result| state.write().finished = Some(result))
    .child("Drag me")
}

#[test]
fn drag_starts_once_after_the_threshold_and_reports_unsupported_backends() {
    let (mut test, state) = TestingRunner::new(
        || app(true),
        (200., 200.).into(),
        |runner| runner.provide_root_context(|| State::create(DragState::default())),
        1.,
    );
    test.sync_and_update();
    test.move_cursor((40., 40.));
    test.press_cursor((40., 40.));
    test.move_cursor((42., 40.));
    assert_eq!(state.peek().generated, 0);
    test.move_cursor((44., 40.));
    assert_eq!(state.peek().generated, 0);

    test.move_cursor((45., 40.));
    test.poll_n(Duration::ZERO, 4);
    assert_eq!(state.peek().generated, 1);
    assert_eq!(state.peek().previews, 1);
    assert_eq!(state.peek().finished, Some(Err(DragError::Unsupported)));

    test.move_cursor((60., 40.));
    test.poll_n(Duration::ZERO, 2);
    assert_eq!(state.peek().generated, 1);
}

#[test]
fn disabled_or_released_gestures_do_not_start_drags() {
    for enabled in [false, true] {
        let (mut test, state) = TestingRunner::new(
            move || app(enabled),
            (200., 200.).into(),
            |runner| runner.provide_root_context(|| State::create(DragState::default())),
            1.,
        );
        test.sync_and_update();
        test.press_cursor((20., 20.));
        if enabled {
            test.release_cursor((20., 20.));
        }
        test.move_cursor((50., 20.));
        test.release_cursor((50., 20.));
        test.poll_n(Duration::ZERO, 3);

        assert_eq!(state.peek().generated, 0);
    }
}
