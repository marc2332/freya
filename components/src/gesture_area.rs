use std::collections::VecDeque;
use std::time::Instant;

use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{touch::TouchPhase, TouchEvent};

const DOUBLE_TAP_DISTANCE: f64 = 100.0;

// Not sure about these numbers yet
#[cfg(debug_assertions)]
const DOUBLE_TAP_DELAY: u128 = 160; // 160ms
#[cfg(not(debug_assertions))]
const DOUBLE_TAP_DELAY: u128 = 40; // 40

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

        let mut last_event: Option<(Instant, TouchEvent)> = None;

        let mut found_gesture = false;

        for (time, event) in touch_events.read().iter() {
            let phase = event.get_touch_phase();

            #[allow(clippy::single_match)]
            match phase {
                TouchPhase::Started => {
                    found_gesture = true;

                    // TapDown
                    cx.props.ongesture.call(Gesture::TapDown);

                    // DoubleTap
                    if let Some((last_time, last_event)) = last_event {
                        let is_ended = TouchPhase::Ended == last_event.get_touch_phase();
                        let is_close = event
                            .get_screen_coordinates()
                            .distance_to(last_event.get_screen_coordinates())
                            < DOUBLE_TAP_DISTANCE;
                        let is_recent = last_time.elapsed().as_millis() <= DOUBLE_TAP_DELAY;

                        if is_ended && is_close && is_recent {
                            cx.props.ongesture.call(Gesture::DoubleTap);
                        }
                    }
                }
                TouchPhase::Ended => {
                    // TapUp
                    found_gesture = true;
                    cx.props.ongesture.call(Gesture::TapUp);
                }
                _ => {}
            }

            last_event = Some((*time, event.clone()))
        }

        if found_gesture {
            touch_events.write_silent().clear();
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
            width: "100%",
            height: "100%",
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
    use freya::prelude::*;
    use freya_elements::events::touch::TouchPhase;
    use freya_testing::{launch_test, FreyaEvent};

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
            name: "touchend",
            location: (1.0, 1.0),
            phase: TouchPhase::Ended,
            finger_id: 0,
            force: None,
        });

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
