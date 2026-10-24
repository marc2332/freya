use std::time::Duration;

use freya::prelude::*;
use freya_testing::prelude::*;

fn setup(virtualized: bool, platform: TargetPlatform) -> (TestingRunner, ScrollController) {
    let (mut test, controller) = TestingRunner::new(
        move || {
            let controller = use_consume::<ScrollController>();
            if virtualized {
                VirtualScrollView::new_controlled(
                    |item, _| rect().key(item.index).height(Size::px(50.)).into(),
                    controller,
                )
                .length(100usize)
                .item_size(50.)
                .into_element()
            } else {
                ScrollView::new_controlled(controller)
                    .child(rect().height(Size::px(5000.)))
                    .into_element()
            }
        },
        (500., 500.).into(),
        |runner| {
            runner.provide_root_context(|| platform);
            runner.provide_root_context(|| ScrollController::new(0, 0))
        },
        1.,
    );
    test.sync_and_update();
    (test, controller)
}

fn wheel(test: &mut TestingRunner, phase: TouchPhase, delta: f64) {
    test.send_event(PlatformEvent::Wheel {
        name: WheelEventName::Wheel,
        scroll: (0., delta).into(),
        cursor: (5., 5.).into(),
        source: WheelSource::Pixel,
        phase,
    });
    test.sync_and_update();
}

fn position(test: &TestingRunner, controller: ScrollController) -> (f32, i32) {
    test.run_in(|| {
        let (horizontal, vertical): (i32, i32) = controller.into();
        let rendered = controller.animated_position((horizontal as f32, vertical as f32).into());
        (rendered.y, vertical)
    })
}

#[test]
fn linux_scroll_gesture_smoothing_momentum_and_interruption() {
    for virtualized in [false, true] {
        let (mut test, controller) = setup(virtualized, TargetPlatform::Linux);
        wheel(&mut test, TouchPhase::Started, -40.);
        let (rendered, target) = position(&test, controller);
        assert_eq!(rendered, 0.);
        assert_eq!(target, -40);

        test.poll(Duration::from_millis(8), Duration::from_millis(32));
        wheel(&mut test, TouchPhase::Moved, -40.);
        test.poll(Duration::from_millis(8), Duration::from_millis(32));
        wheel(&mut test, TouchPhase::Moved, -40.);
        let (rendered, target) = position(&test, controller);
        assert!(rendered < 0. && rendered > target as f32);
        wheel(&mut test, TouchPhase::Ended, 0.);
        assert!(position(&test, controller).1 < target);

        test.poll(Duration::from_millis(8), Duration::from_millis(32));
        wheel(&mut test, TouchPhase::Started, 0.);
        wheel(&mut test, TouchPhase::Cancelled, 0.);
        let stopped = position(&test, controller);
        test.poll(Duration::from_millis(8), Duration::from_millis(150));
        assert_eq!(position(&test, controller), stopped);
    }
}

#[test]
fn linux_scroll_gesture_pause_does_not_fling() {
    for virtualized in [false, true] {
        let (mut test, controller) = setup(virtualized, TargetPlatform::Linux);
        wheel(&mut test, TouchPhase::Started, -40.);
        test.poll(Duration::from_millis(8), Duration::from_millis(32));
        wheel(&mut test, TouchPhase::Moved, -40.);
        test.poll(Duration::from_millis(8), Duration::from_millis(150));
        let target = position(&test, controller).1;
        wheel(&mut test, TouchPhase::Ended, 0.);
        assert_eq!(position(&test, controller).1, target);
    }
}

#[test]
fn other_platforms_keep_direct_pixel_scrolling() {
    for platform in [TargetPlatform::MacOs, TargetPlatform::Windows] {
        for virtualized in [false, true] {
            let (mut test, controller) = setup(virtualized, platform);
            wheel(&mut test, TouchPhase::Started, -40.);
            assert_eq!(position(&test, controller), (-40., -40));
            wheel(&mut test, TouchPhase::Moved, -40.);
            wheel(&mut test, TouchPhase::Ended, 0.);
            assert_eq!(position(&test, controller), (-80., -80));
        }
    }
}
