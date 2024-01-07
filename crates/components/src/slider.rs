use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{MouseEvent, WheelEvent};

use freya_hooks::{use_applied_theme, use_node, use_platform, SliderThemeWith};
use tracing::info;
use winit::window::CursorIcon;

/// [`Slider`] component properties.
#[derive(Props)]
pub struct SliderProps<'a> {
    /// Theme override.
    pub theme: Option<SliderThemeWith>,
    /// Handler for the `onmoved` event.
    pub onmoved: EventHandler<'a, f64>,
    /// Width of the Slider.
    #[props(into, default = "100%".to_string())]
    pub width: String,
    /// Height of the Slider.
    pub value: f64,
}

#[inline]
fn ensure_correct_slider_range(value: f64) -> f64 {
    if value < 0.0 {
        info!("Slider value is less than 0.0, setting to 0.0");
        0.0
    } else if value > 100.0 {
        info!("Slider value is greater than 100.0, setting to 100.0");
        100.0
    } else {
        value
    }
}

/// Describes the current status of the Slider.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SliderStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the slider.
    Hovering,
}

/// Controlled `Slider` component.
///
/// You must pass a percentage from 0.0 to 100.0 and listen for value changes with `onmoved` and then decide if this changes are applicable,
/// and if so, apply them.
///
/// # Props
/// See [`SliderProps`].
///
/// # Styling
/// Inherits a [`SliderTheme`](freya_hooks::SliderTheme) theme.
///
/// # Example
/// ```no_run
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     let percentage = use_state(cx, || 20.0);
///
///     render!(
///         label {
///             "Value: {percentage}"
///         }
///         Slider {
///             width: "50%",
///             value: *percentage.get(),
///             onmoved: |p| {
///                 percentage.set(p);
///             }
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn Slider<'a>(cx: Scope<'a, SliderProps>) -> Element<'a> {
    let theme = use_applied_theme!(cx, &cx.props.theme, slider);
    let status = use_ref(cx, SliderStatus::default);
    let clicking = use_state(cx, || false);
    let platform = use_platform(cx);

    let value = ensure_correct_slider_range(cx.props.value);
    let (node_reference, size) = use_node(cx);
    let width = &cx.props.width;

    use_on_destroy(cx, {
        to_owned![status, platform];
        move || {
            if *status.read() == SliderStatus::Hovering {
                platform.set_cursor(CursorIcon::default());
            }
        }
    });

    let onmouseleave = {
        to_owned![platform, status];
        move |_: MouseEvent| {
            *status.write_silent() = SliderStatus::Idle;
            platform.set_cursor(CursorIcon::default());
        }
    };

    let onmouseenter = move |_: MouseEvent| {
        *status.write_silent() = SliderStatus::Hovering;
        platform.set_cursor(CursorIcon::Hand);
    };

    let onmouseover = move |e: MouseEvent| {
        if *clicking.get() {
            let coordinates = e.get_element_coordinates();
            let x = coordinates.x - size.area.min_x() as f64 - 6.0;
            let percentage = x / (size.area.width() as f64 - 15.0) * 100.0;
            let percentage = percentage.clamp(0.0, 100.0);

            cx.props.onmoved.call(percentage);
        }
    };

    let onmousedown = move |e: MouseEvent| {
        clicking.set(true);
        let coordinates = e.get_element_coordinates();
        let x = coordinates.x - 6.0;
        let percentage = x / (size.area.width() as f64 - 15.0) * 100.0;
        let percentage = percentage.clamp(0.0, 100.0);

        cx.props.onmoved.call(percentage);
    };

    let onclick = |_: MouseEvent| {
        clicking.set(false);
    };

    let onwheel = move |e: WheelEvent| {
        let wheel_y = e.get_delta_y().clamp(-1.0, 1.0);
        let percentage = value + (wheel_y * 2.0);
        let percentage = percentage.clamp(0.0, 100.0);

        cx.props.onmoved.call(percentage);
    };

    let inner_width = (size.area.width() - 15.0) * (value / 100.0) as f32;

    render!(
        rect {
            reference: node_reference,
            width: "{width}",
            height: "20",
            onmousedown: onmousedown,
            onglobalclick: onclick,
            onmouseenter: onmouseenter,
            onglobalmouseover: onmouseover,
            onmouseleave: onmouseleave,
            onwheel: onwheel,
            main_align: "center",
            cross_align: "center",
            rect {
                background: "{theme.background}",
                width: "100%",
                height: "6",
                direction: "horizontal",
                corner_radius: "50",
                rect {
                    background: "{theme.thumb_inner_background}",
                    width: "{inner_width}",
                    height: "100%",
                    corner_radius: "50"
                }
                rect {
                    width: "fill",
                    height: "100%",
                    offset_y: "-6",
                    offset_x: "-3",
                    rect {
                        background: "{theme.thumb_background}",
                        width: "18",
                        height: "18",
                        corner_radius: "50",
                        padding: "4",
                        rect {
                            height: "100%",
                            width: "100%",
                            background: "{theme.thumb_inner_background}",
                            corner_radius: "50"
                        }
                    }
                }
            }
        }
    )
}
