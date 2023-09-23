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

type EventsQueue = VecDeque<(Instant, TouchEvent)>;

/// `GestureArea` component.
///
/// # Props
/// See [`GestureAreaProps`].
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///    let gesture = use_state(cx, || "Tap here".to_string());
///    render!(
///        GestureArea {
///            ongesture: move |g| gesture.set(format!("{g:?}")),
///            label {
///                "{gesture}"
///            }
///        }
///    )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn GestureArea<'a>(cx: Scope<'a, GestureAreaProps<'a>>) -> Element {
    let touch_events = use_ref::<EventsQueue>(cx, VecDeque::new);

    use_memo(cx, touch_events, move |_| {
        // Keep the touch events queue under a certain size
        if touch_events.read().len() > MAX_EVENTS_QUEUE {
            touch_events.write_silent().pop_front();
        }

        // Find the first event with the `target_phase` that happened before the `start_time`
        let find_previous_event = |start_time: &Instant,
                                   events: &EventsQueue,
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

        let touch_events = touch_events.read();

        // Process the most recent event
        let event = touch_events.iter().last();

        if let Some((time, event)) = event {
            let phase = event.get_touch_phase();

            match phase {
                TouchPhase::Started => {
                    // TapDown
                    cx.props.ongesture.call(Gesture::TapDown);

                    let last_ended_event =
                        find_previous_event(time, &touch_events, TouchPhase::Ended);
                    let last_started_event =
                        find_previous_event(time, &touch_events, TouchPhase::Started);

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

    /// This test simulates a `DoubleTap` gesture in this order:
    /// 1. Touch start
    /// 2. Touch end
    /// 3. Wait 40ms
    /// 4. Touch start
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
        utils.wait_for_update().await;

        assert_eq!(utils.root().get(0).get(0).text(), Some("EMPTY"));

        utils.push_event(FreyaEvent::Touch {
            name: "touchstart".to_string(),
            location: (1.0, 1.0).into(),
            phase: TouchPhase::Started,
            finger_id: 0,
            force: None,
        });

        utils.push_event(FreyaEvent::Touch {
            name: "touchend".to_string(),
            location: (1.0, 1.0).into(),
            phase: TouchPhase::Ended,
            finger_id: 0,
            force: None,
        });

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        sleep(Duration::from_millis(DOUBLE_TAP_MIN as u64)).await;

        utils.push_event(FreyaEvent::Touch {
            name: "touchstart".to_string(),
            location: (1.0, 1.0).into(),
            phase: TouchPhase::Started,
            finger_id: 0,
            force: None,
        });

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        assert_eq!(utils.root().get(0).get(0).text(), Some("DoubleTap"));
    }

    /// Simulates `TapUp` and `TapDown` gestures.
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
        utils.wait_for_update().await;

        assert_eq!(utils.root().get(0).get(0).text(), Some("EMPTY"));

        utils.push_event(FreyaEvent::Touch {
            name: "touchstart".to_string(),
            location: (1.0, 1.0).into(),
            phase: TouchPhase::Started,
            finger_id: 0,
            force: None,
        });

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        assert_eq!(utils.root().get(0).get(0).text(), Some("TapDown"));

        utils.push_event(FreyaEvent::Touch {
            name: "touchend".to_string(),
            location: (1.0, 1.0).into(),
            phase: TouchPhase::Ended,
            finger_id: 0,
            force: None,
        });

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        assert_eq!(utils.root().get(0).get(0).text(), Some("TapUp"));
    }
}
