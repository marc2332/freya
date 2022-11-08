use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_elements::{MouseEvent, WheelEvent};
use freya_hooks::use_get_theme;

/// Properties for the Slider component.
#[derive(Props)]
pub struct SliderProps<'a> {
    pub onmoved: EventHandler<'a, f64>,
    pub width: f64,
    pub value: f64,
}

#[inline]
fn ensure_correct_slider_range(value: f64) -> f64 {
    if value < 0.0 {
        // TODO: Better logging
        println!("Slider value is less than 0.0, setting to 0.0");
        0.0
    } else if value > 100.0 {
        println!("Slider value is greater than 100.0, setting to 100.0");
        100.0
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
///                 onclick: move |_| {
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
    let theme = use_get_theme(&cx);
    let theme = &theme.slider;
    let hovering = use_state(&cx, || false);
    let clicking = use_state(&cx, || false);
    let value = ensure_correct_slider_range(cx.props.value);
    let width = cx.props.width + 14.0;

    let progress = (value / 100.0) as f64 * cx.props.width + 0.5;

    let onmouseleave = |_: MouseEvent| {
        if !(*clicking.get()) {
            hovering.set(false);
        }
    };

    let onmouseover = move |e: MouseEvent| {
        hovering.set(true);
        if *clicking.get() {
            let coordinates = e.get_element_coordinates();
            let mut x = coordinates.x - 7.5;
            if x < 0.0 {
                x = 0.0;
            }
            if x > width {
                x = width;
            }
            let percentage = x / cx.props.width * 100.0;

            let percentage = if percentage < 0.0 {
                0.0
            } else if percentage > 100.0 {
                100.0
            } else {
                percentage
            };

            cx.props.onmoved.call(percentage);
        }
    };

    let onmousedown = |_: MouseEvent| {
        clicking.set(true);
    };

    let onclick = |_: MouseEvent| {
        clicking.set(false);
    };

    let onwheel = move |e: WheelEvent| {
        let wheel_y = e.get_delta_y();
        let progress_x = (value / 100.0) as f64 * cx.props.width;

        let mut x = progress_x + (wheel_y * 7.5);
        if x < 0.0 {
            x = 0.0;
        }
        if x > width {
            x = width;
        }
        let percentage = x / cx.props.width * 100.0;

        let percentage = if percentage < 0.0 {
            0.0
        } else if percentage > 100.0 {
            100.0
        } else {
            percentage
        };

        cx.props.onmoved.call(percentage);
    };

    render!(
        container {
            width: "{width}",
            height: "20",
            onmousedown: onmousedown,
            onclick: onclick,
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            onwheel: onwheel,
            display: "center",
            direction: "both",
            padding: "2",
            rect {
                background: "{theme.background}",
                width: "100%",
                height: "6",
                direction: "horizontal",
                radius: "50",
                rect {
                    background: "{theme.thumb_inner_background}",
                    width: "{progress}",
                    height: "100%",
                    radius: "50",
                }
                rect {
                    width: "{progress}",
                    height: "100%",
                    scroll_y: "-5",
                    scroll_x: "-2",
                    rect {
                        background: "{theme.thumb_background}",
                        width: "17",
                        height: "17",
                        radius: "50",
                        padding: "6",
                        rect {
                            height: "100%",
                            width: "100%",
                            background: "{theme.thumb_inner_background}",
                            radius: "50"
                        }
                    }
                }
            }
        }
    )
}
