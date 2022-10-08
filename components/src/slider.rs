use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use freya_elements as dioxus_elements;

#[derive(Props)]
pub struct SliderProps<'a> {
    pub onmoved: EventHandler<'a, f64>,
    pub width: f64,
    pub value: f64
}

#[inline]
fn ensure_correct_slider_range(value: f64) -> f64 {
    if value < 0.0 {
        // TODO: Better logging
        println!("Slider value is less than 0.0, setting to 0.0");
        0.0
    } else if value > 100.0 {
        println!("Slider value is greater than 1.0, setting to 1.0");
        1.0
    } else {
        value
    }
}

/// A controlled Slider component.
/// 
/// You must pass a percentage from 0.0 to 100.0 and listen for value changes with `onmoved` and then decide if this changes are applicable,
/// and if so, apply them.
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
///     let percentage = use_state(&cx, || 20.0);
///     let font_size = percentage.get() * MAX_FONT_SIZE + 20.0;
///
///     render!(
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
///                     percentage.set(20);
///                 },
///                  label {
///                     width: "80", 
///                     "Reset size"
///                  }
///             }
///             Slider {
///                 width: 100.0,
///                 value: *percentage.get(),
///                 onmoved: |p| {
///                     percentage.set(p);
///                 }
///             }
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn Slider<'a>(cx: Scope<'a, SliderProps>) -> Element<'a> {
    let hovering = use_state(&cx, || false);
    let clicking = use_state(&cx, || false);
    let value = ensure_correct_slider_range(cx.props.value);

    let progress = (value / 100.0) as f64 * cx.props.width;

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
            let percentage = x / cx.props.width * 100.0;

            cx.props.onmoved.call(percentage);
        }
    };

    let onmousedown = |_: UiEvent<MouseData>| {
        clicking.set(true);
    };

    let onclick = |_: UiEvent<MouseData>| {
        clicking.set(false);
    };

    render!(
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
    )
}
