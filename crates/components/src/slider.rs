use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{MouseEvent, WheelEvent};
use freya_hooks::{use_get_theme, use_node_ref, use_platform};
use tracing::info;
use winit::window::CursorIcon;

/// [`Slider`] component properties.
#[derive(Props)]
pub struct SliderProps<'a> {
    /// Handler for the `onmoved` event.
    pub onmoved: EventHandler<'a, f64>,
    /// Width of the Slider.
    pub width: f64,
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
///             width: 100.0,
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
    let theme = use_get_theme(cx);
    let theme = &theme.slider;
    let status = use_ref(cx, SliderStatus::default);
    let clicking = use_state(cx, || false);
    let platform = use_platform(cx);

    let value = ensure_correct_slider_range(cx.props.value);
    let (node_reference, size) = use_node_ref(cx);
    let width = cx.props.width + 14.0;

    let progress = (value / 100.0) * cx.props.width + 0.5;

    use_on_unmount(cx, {
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
            let mut x = coordinates.x - 7.5 - size.read().area.min_x() as f64;
            x = x.clamp(0.0, width);

            let mut percentage = x / cx.props.width * 100.0;
            percentage = percentage.clamp(0.0, 100.0);

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
        let progress_x = (value / 100.0) * cx.props.width;

        let mut x = progress_x + (wheel_y / 4.0);
        x = x.clamp(0.0, width);

        let mut percentage = x / cx.props.width * 100.0;
        percentage = percentage.clamp(0.0, 100.0);

        cx.props.onmoved.call(percentage);
    };

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
            padding: "1",
            rect {
                background: "{theme.background}",
                width: "100%",
                height: "6",
                direction: "horizontal",
                corner_radius: "50",
                rect {
                    background: "{theme.thumb_inner_background}",
                    width: "{progress}",
                    height: "100%",
                    corner_radius: "50",
                }
                rect {
                    width: "{progress}",
                    height: "100%",
                    offset_y: "-5",
                    offset_x: "-2",
                    rect {
                        background: "{theme.thumb_background}",
                        width: "17",
                        height: "17",
                        corner_radius: "50",
                        padding: "3",
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
