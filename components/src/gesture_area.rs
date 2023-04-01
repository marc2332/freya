use std::collections::VecDeque;
use std::time::Instant;

use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{touch::TouchPhase, TouchEvent};

/// Distance between the first tap and the second tap in `DoubleTap` gesture.
const DOUBLE_TAP_DISTANCE: f64 = 100.0;
/// Maximum time between the start of the first tap and the start of the second tap in a `DoubleTap` gesture.
const DOUBLE_TAP_TIMEOUT: u128 = 300; // 300ms
/// Minimum time between the end of the first time to the start of the second tap in a `DoubleTap` gesture.
const DOUBLE_TAP_MIN: u128 = 40; // 40ms

/// In-memory events queue maximum size.
const MAX_EVENTS_QUEUE: usize = 20;

/// Gesture emitted by the `GestureArea` component.
#[derive(Debug, PartialEq, Eq)]
pub enum Gesture {
    TapUp,
    TapDown,
    DoubleTap,
}

/// [`GestureArea`] component properties.
#[derive(Props)]
pub struct GestureAreaProps<'a> {
    /// Inner children for the GestureArea.
    pub children: Element<'a>,

    /// Handler for the `ongesture` event.
    pub ongesture: EventHandler<'a, Gesture>,
}

/// `GestureArea` component.
///
/// # Props
/// See [`GestureAreaProps`].
///
/// # Example
///
///
#[allow(non_snake_case)]
pub fn GestureArea<'a>(cx: Scope<'a, GestureAreaProps<'a>>) -> Element {
    let touch_events = use_ref::<VecDeque<(Instant, TouchEvent)>>(cx, VecDeque::new);

    use_effect(cx, touch_events, move |_| {
        // Keep the touch events queue under a certain size
        if touch_events.read().len() > MAX_EVENTS_QUEUE {
            touch_events.write_silent().pop_front();
        }

        let find_previous_event = |start_time: &Instant,
                                   events: &VecDeque<(Instant, TouchEvent)>,
                                   target_phase: TouchPhase|
         -> Option<(Instant, TouchEvent)> {
            let mut start = false;
            for (time, event) in events.iter().rev() {
                if time == start_time {
                    start = true;
                    continue;
                }
                if event.phase == target_phase && start {
                    return Some((*time, event.clone()));
                }
            }
            None
        };

        for (i, (time, event)) in touch_events.read().iter().enumerate() {
            let phase = event.get_touch_phase();
            let is_from_past = i != touch_events.read().len() - 1;

            if is_from_past {
                continue;
            }

            #[allow(clippy::single_match)]
            match phase {
                TouchPhase::Started => {
                    // TapDown
                    cx.props.ongesture.call(Gesture::TapDown);

                    let last_ended_event =
                        find_previous_event(time, &touch_events.read(), TouchPhase::Ended);
                    let last_started_event =
                        find_previous_event(time, &touch_events.read(), TouchPhase::Started);

                    // DoubleTap
                    if let Some(((ended_time, ended_event), (started_time, _))) =
                        last_ended_event.zip(last_started_event)
                    {
                        // Has the latest `touchend` event went too far?
                        let is_ended_close = event
                            .get_screen_coordinates()
                            .distance_to(ended_event.get_screen_coordinates())
                            < DOUBLE_TAP_DISTANCE;
                        // Is the latest `touchend` mature enough?
                        println!("-> {}", ended_time.elapsed().as_millis());
                        let is_ended_mature = ended_time.elapsed().as_millis() >= DOUBLE_TAP_MIN;

                        // Hast the latest `touchstart` event expired?
                        let is_started_recent =
                            started_time.elapsed().as_millis() <= DOUBLE_TAP_TIMEOUT;

                        if is_ended_close && is_ended_mature && is_started_recent {
                            cx.props.ongesture.call(Gesture::DoubleTap);
                        }
                    }
                }
                TouchPhase::Ended => {
                    // TapUp
                    cx.props.ongesture.call(Gesture::TapUp);
                }
                _ => {}
            }
        }

        async move {}
    });

    let ontouchcancel = |e: TouchEvent| {
        touch_events.write().push_back((Instant::now(), e));
    };

    let ontouchend = |e: TouchEvent| {
        touch_events.write().push_back((Instant::now(), e));
    };

    let ontouchmove = |e: TouchEvent| {
        touch_events.write().push_back((Instant::now(), e));
    };

    let ontouchstart = |e: TouchEvent| {
        touch_events.write().push_back((Instant::now(), e));
    };

    render!(
        rect {
            ontouchcancel: ontouchcancel,
            ontouchend: ontouchend,
            ontouchmove: ontouchmove,
            ontouchstart: ontouchstart,
            &cx.props.children
        }
    )
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use freya::prelude::*;
    use freya_elements::events::touch::TouchPhase;
    use freya_testing::{launch_test, FreyaEvent};
    use tokio::time::sleep;

    use crate::gesture_area::DOUBLE_TAP_MIN;

    #[tokio::test]
    pub async fn double_tap() {
        fn dobule_tap_app(cx: Scope) -> Element {
            let value = use_state(cx, || "EMPTY".to_string());

            let ongesture = |e: Gesture| {
                value.set(format!("{e:?}"));
            };

            render!(
                GestureArea {
                    ongesture: ongesture,
                    "{value}"
                }
            )
        }

        let mut utils = launch_test(dobule_tap_app);

        // Initial state
        utils.wait_for_work((500.0, 500.0));

        assert_eq!(
            utils.root().child(0).unwrap().child(0).unwrap().text(),
            Some("EMPTY")
        );

        utils.push_event(FreyaEvent::Touch {
            name: "touchstart",
            location: (1.0, 1.0),
            phase: TouchPhase::Started,
            finger_id: 0,
            force: None,
        });

        utils.push_event(FreyaEvent::Touch {
            name: "touchend",
            location: (1.0, 1.0),
            phase: TouchPhase::Ended,
            finger_id: 0,
            force: None,
        });

        utils.wait_for_update((500.0, 500.0)).await;
        utils.wait_for_update((500.0, 500.0)).await;

        sleep(Duration::from_millis(DOUBLE_TAP_MIN as u64)).await;

        utils.push_event(FreyaEvent::Touch {
            name: "touchstart",
            location: (1.0, 1.0),
            phase: TouchPhase::Started,
            finger_id: 0,
            force: None,
        });

        utils.wait_for_update((500.0, 500.0)).await;
        utils.wait_for_update((500.0, 500.0)).await;

        assert_eq!(
            utils.root().child(0).unwrap().child(0).unwrap().text(),
            Some("DoubleTap")
        );
    }

    #[tokio::test]
    pub async fn tap_up_down() {
        fn tap_up_down_app(cx: Scope) -> Element {
            let value = use_state(cx, || "EMPTY".to_string());

            let ongesture = |e: Gesture| {
                value.set(format!("{e:?}"));
            };

            render!(
                GestureArea {
                    ongesture: ongesture,
                    "{value}"
                }
            )
        }

        let mut utils = launch_test(tap_up_down_app);

        // Initial state
        utils.wait_for_work((500.0, 500.0));

        assert_eq!(
            utils.root().child(0).unwrap().child(0).unwrap().text(),
            Some("EMPTY")
        );

        utils.push_event(FreyaEvent::Touch {
            name: "touchstart",
            location: (1.0, 1.0),
            phase: TouchPhase::Started,
            finger_id: 0,
            force: None,
        });

        utils.wait_for_update((500.0, 500.0)).await;
        utils.wait_for_update((500.0, 500.0)).await;

        assert_eq!(
            utils.root().child(0).unwrap().child(0).unwrap().text(),
            Some("TapDown")
        );

        utils.push_event(FreyaEvent::Touch {
            name: "touchend",
            location: (1.0, 1.0),
            phase: TouchPhase::Ended,
            finger_id: 0,
            force: None,
        });

        utils.wait_for_update((500.0, 500.0)).await;
        utils.wait_for_update((500.0, 500.0)).await;

        assert_eq!(
            utils.root().child(0).unwrap().child(0).unwrap().text(),
            Some("TapUp")
        );
    }
}
