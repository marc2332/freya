use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use freya_elements as dioxus_elements;

#[derive(Props)]
pub struct SliderProps<'a> {
    #[props(optional)]
    pub onmoved: Option<EventHandler<'a, f64>>,
    pub width: f64,
    #[props(optional)]
    pub state: Option<&'a UseState<f64>>,
}

/// A slider component. It can either be uncontrolled or bound to a state.
///
/// When it is uncontrolled, there is no way of directly accessing or
/// controlling the state. You can listen to changes with the `onmoved` event.
///
/// If you need to provide a default value, change the value, or sync the value
/// with other parts of the UI, you should pass a state to the `state` prop.
/// This state will be updated when the slider is moved and the slider will
/// update when the state is changed.
///
/// # Example
/// ```rs
/// use dioxus::prelude::*;
/// use freya::{dioxus_elements, *};
///
/// const MAX_FONT_SIZE: f64 = 100.0;
///
/// fn main() {
///     launch(app);
/// }
///
/// fn app(cx: Scope) -> Element {
///     let percentage = use_state(&cx, || 0.2);
///     let font_size = percentage.get() * MAX_FONT_SIZE + 20.0;
///
///     cx.render(rsx!(
///         rect {
///             width: "100%",
///             height: "100%",
///             background: "black",
///             padding: "20",
///             label {
///                 font_size: "{font_size}",
///                 font_family: "Inter",
///                 height: "150",
///                 "Hello World"
///             }
///             Button {
///                 on_click: move |_| {
///                     percentage.set(0.2);
///                 },
///                 label { "Reset size" }
///             }
///             Slider {
///                 width: 100.0,
///                 state: percentage,
///             }
///         }
///     ))
/// }
/// ```
#[allow(non_snake_case)]
pub fn Slider<'a>(cx: Scope<'a, SliderProps>) -> Element<'a> {
    let hovering = use_state(&cx, || false);
    let clicking = use_state(&cx, || false);
    let state = cx.props.state.unwrap_or_else(|| use_state(&cx, || 0.0));
    let progress = state * cx.props.width;

    // The slider's input value should *never*, be outside of the range 0-1.
    // Panic if this happens
    assert!(
        state.get().clone() >= 0.0 && state.get().clone() <= 1.0,
        "The value passed into a slider must be between 0 and 1. The value passed in was: {}",
        state.get().clone()
    );

    let onmouseleave = |_: UiEvent<MouseData>| {
        if *clicking.get() == false {
            hovering.set(false);
        }
    };

    let onmouseover = |e: UiEvent<MouseData>| {
        hovering.set(true);
        if *clicking.get() {
            let coordinates = e.coordinates().element();
            let mut x = coordinates.x - 11.0;
            if x < 0.0 {
                x = 0.0;
            }
            if x > cx.props.width - 20.0 {
                x = cx.props.width - 20.0;
            }
            let percentage = x / cx.props.width;

            state.set(percentage);
            if let Some(onmoved) = &cx.props.onmoved {
                onmoved.call(percentage);
            }
        }
    };

    let onmousedown = |_: UiEvent<MouseData>| {
        clicking.set(true);
    };

    let onclick = |_: UiEvent<MouseData>| {
        clicking.set(false);
    };

    cx.render(rsx!(
        container {
            background: "white",
            width: "{cx.props.width}",
            height: "20",
            scroll_x: "{progress}",
            padding: "5",
            onmousedown: onmousedown,
            onclick: onclick,
            shadow: "0 0 60 35 white",
            radius: "25",
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            rect {
                background: "rgb(255, 166, 0)",
                width: "15",
                height: "15",
                radius: "15",
            }
        }
    ))
}
