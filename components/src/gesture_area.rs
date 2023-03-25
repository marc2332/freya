use std::collections::VecDeque;
use std::time::Instant;

use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_elements::{TouchEvent, TouchPhase};

const DOUBLE_TAP_DISTANCE: f64 = 100.0;

const DOUBLE_TAP_DELAY: u128 = 40; // 40ms

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
            touch_events.write_silent().pop_back();
        }

        let mut last_event: Option<(Instant, TouchEvent)> = None;

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
                        cx.props.ongesture.call(Gesture::DoubleTap)
                    }
                }
            }

            last_event = Some((*time, event.clone()))
        }
        async move {}
    });

    let ontouchcancel = |e: TouchEvent| {
        touch_events.write().push_front((Instant::now(), e));
    };

    let ontouchend = |e: TouchEvent| {
        touch_events.write().push_front((Instant::now(), e));
    };

    let ontouchmove = |e: TouchEvent| {
        touch_events.write().push_front((Instant::now(), e));
    };

    let ontouchstart = |e: TouchEvent| {
        touch_events.write().push_front((Instant::now(), e));
    };

    render!(
        rect {
            width: "auto",
            height: "auto",
            ontouchcancel: ontouchcancel,
            ontouchend: ontouchend,
            ontouchmove: ontouchmove,
            ontouchstart: ontouchstart,
            &cx.props.children
        }
    )
}
