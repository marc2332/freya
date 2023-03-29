use std::collections::VecDeque;
use std::time::Instant;

use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_elements::{TouchEvent, TouchPhase};

const DOUBLE_TAP_DISTANCE: f64 = 100.0;

// Not sure about these numbers yet
#[cfg(debug_assertions)]
const DOUBLE_TAP_DELAY: u128 = 160; // 160ms
#[cfg(not(debug_assertions))]
const DOUBLE_TAP_DELAY: u128 = 40; // 40

const MAX_EVENTS_QUEUE: usize = 20;

#[derive(Debug, PartialEq, Eq)]
pub enum Gesture {
    Tap,
    DoubleTap,
}

/// [`GestureArea`] component properties.
#[derive(Props)]
pub struct GestureAreaProps<'a> {
    /// Inner children for the Button.
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
            if TouchPhase::Started == event.get_touch_phase() {
                if let Some((last_time, last_event)) = last_event {
                    if TouchPhase::Ended == last_event.get_touch_phase()
                        && event
                            .get_screen_coordinates()
                            .distance_to(last_event.get_screen_coordinates())
                            < DOUBLE_TAP_DISTANCE
                        && last_time.elapsed().as_millis() <= DOUBLE_TAP_DELAY
                    {
                        cx.props.ongesture.call(Gesture::DoubleTap);
                        found_gesture = true;
                        break;
                    }
                }
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
    use freya_elements::TouchPhase;
    use freya_testing::{launch_test, FreyaEvent};

    #[tokio::test]
    pub async fn track_progress() {
        fn use_animation_app(cx: Scope) -> Element {
            let value = use_state(cx, || "EMPTY".to_string());

            let ongesture = |e: Gesture| {
                println!("{e:?}");
                value.set(format!("{e:?}"));
            };

            render!(
                GestureArea {
                    ongesture: ongesture,
                    "{value}"
                }
            )
        }

        let mut utils = launch_test(use_animation_app);

        // Initial state
        utils.wait_for_work((500.0, 500.0)).await;

        assert_eq!(
            utils.root().child(0).unwrap().child(0).unwrap().text(),
            Some("EMPTY")
        );

        utils.send_event(FreyaEvent::Touch {
            name: "touchend",
            location: (1.0, 1.0),
            phase: TouchPhase::Ended,
            finger_id: 0,
            force: None,
        });

        utils.send_event(FreyaEvent::Touch {
            name: "touchstart",
            location: (1.0, 1.0),
            phase: TouchPhase::Started,
            finger_id: 0,
            force: None,
        });

        utils.wait_until_cleanup((500.0, 500.0)).await;
        utils.wait_for_update((500.0, 500.0)).await;

        assert_eq!(
            utils.root().child(0).unwrap().child(0).unwrap().text(),
            Some("DoubleTap")
        );
    }
}
