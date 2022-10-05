use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use freya_elements as dioxus_elements;

#[derive(Props)]
pub struct SliderProps<'a> {
    #[props(optional)]
    pub onmoved: Option<EventHandler<'a, f64>>,
    pub width: f64,
    pub value: &'a UseState<f64>,
}

/// A slider which takes in a state (`value`), which is set to any value between
/// 0 and 1. When the slider is moved, both the optional `onmoved` event is
/// called and the state is updated with the new value
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
///                 value: percentage,
///             }
///         }
///     ))
/// }
/// ```
#[allow(non_snake_case)]
pub fn Slider<'a>(cx: Scope<'a, SliderProps>) -> Element<'a> {
    let hovering = use_state(&cx, || false);
    let clicking = use_state(&cx, || false);
    let progress = cx.props.value * cx.props.width;

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

            cx.props.value.set(percentage);
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
