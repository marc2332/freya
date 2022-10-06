use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use freya_elements as dioxus_elements;

#[derive(Props)]
pub struct SliderProps<'a> {
    #[props(optional)]
    pub onmoved: Option<EventHandler<'a, f64>>,
    pub width: f64,
    #[props(optional)]
    pub state: Option<&'a UseState<f64>>,
    #[props(optional)]
    pub value: Option<&'a f64>,
}

#[inline]
fn ensure_correct_slider_range(value: f64) -> f64 {
    if value < 0.0 {
        // TODO: Better logging
        println!("Slider value is less than 0.0, setting to 0.0");
        0.0
    } else if value > 1.0 {
        println!("Slider value is greater than 1.0, setting to 1.0");
        1.0
    } else {
        value
    }
}

/// A slider component. It can be uncontrolled, bound to a state or controlled
/// via the value property.
///
/// When it is uncontrolled, there is no way of directly accessing or
/// controlling the state. You can listen to changes with the `onmoved` event.
///
/// The easiest way to manage state is to pass a `UseState<f64>` to the `state`
/// property. This will automatically update with the state of the slider.
///
/// If you need to interface with an outside state management system, you can
/// pass a `&f64` to the `value` property. This will overwrite the state, and
/// the value of the slider will not change unless you change `value`. You can
/// listen for value changes with `onmoved`.
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

    // The slider's state can be managed in three separate ways:
    // - A state can be passed in
    // - A value can be passed in
    // - Neither can be passed in
    //
    // We always need a state instance, although it will be overridden if the
    // `value` prop is passed in.
    let state = cx.props.state.unwrap_or_else(|| use_state(&cx, || -1.0));
    let value = ensure_correct_slider_range(*cx.props.value.unwrap_or_else(|| state.get()));

    // Map the 0 to 1 values to the width of the slider
    let progress = value * cx.props.width;

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

            // Even though we are setting state here, if a value is specified,
            // that value will override the state on the next render, so this
            // would do nothing
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
