use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use freya_elements as dioxus_elements;

#[derive(Props)]
pub struct SliderProps<'a> {
    pub onmoved: EventHandler<'a, f64>,
    pub width: f64,
    /// The value that the slider will use when first initialized.
    #[props(optional)]
    pub starting_value: Option<f64>,
}

/// Provides a slider that goes between 0 and the width of the slider. An
/// initial value can be provided with the `starting_value` prop, but this will
/// not change the slider value if updated.
///
/// # Example
/// ```rs
/// use dioxus::prelude::*;
/// use freya::{dioxus_elements, *};
///
/// const INITIAL_SLIDER_OFFSET: f64 = 30.0;
///
/// fn main() {
///     launch(app);
/// }
///
/// fn app(cx: Scope) -> Element {
///     let font_size = use_state(&cx, || 20.0 + INITIAL_SLIDER_OFFSET);
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
///             Slider {
///                 width: 100.0,
///                 starting_value: INITIAL_SLIDER_OFFSET,
///                 onmoved: |e| {
///                     font_size.set(e + 20.0); // Minimum is 20
///                 }
///             }
///         }
///     ))
/// }
/// ```
#[allow(non_snake_case)]
pub fn Slider<'a>(cx: Scope<'a, SliderProps>) -> Element<'a> {
    let hovering = use_state(&cx, || false);
    let progress = use_state(&cx, || cx.props.starting_value.unwrap_or(0.0));
    let clicking = use_state(&cx, || false);

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
            progress.set(x);
            cx.props.onmoved.call(x);
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
