use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{MouseEvent, WheelEvent};

use freya_hooks::{use_applied_theme, use_focus, use_node, use_platform, SliderThemeWith};
use tracing::info;
use winit::window::CursorIcon;

/// Properties for the [`Slider`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// Theme override.
    pub theme: Option<SliderThemeWith>,
    /// Handler for the `onmoved` event.
    pub onmoved: EventHandler<f64>,
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
/// # Styling
/// Inherits a [`SliderTheme`](freya_hooks::SliderTheme) theme.
///
/// # Example
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut percentage = use_signal(|| 20.0);
///
///     rsx!(
///         label {
///             "Value: {percentage}"
///         }
///         Slider {
///             width: "50%",
///             value: *percentage.read(),
///             onmoved: move |p| {
///                 percentage.set(p);
///             }
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn Slider(
    SliderProps {
        value,
        onmoved,
        theme,
        width,
    }: SliderProps,
) -> Element {
    let theme = use_applied_theme!(&theme, slider);
    let mut focus = use_focus();
    let mut status = use_signal(SliderStatus::default);
    let mut clicking = use_signal(|| false);
    let platform = use_platform();
    let (node_reference, size) = use_node();

    let value = ensure_correct_slider_range(value);
    let focus_id = focus.attribute();

    use_drop(move || {
        if *status.peek() == SliderStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseleave = move |e: MouseEvent| {
        e.stop_propagation();
        *status.write() = SliderStatus::Idle;
        platform.set_cursor(CursorIcon::default());
    };

    let onmouseenter = move |e: MouseEvent| {
        e.stop_propagation();
        *status.write() = SliderStatus::Hovering;
        platform.set_cursor(CursorIcon::Pointer);
    };

    let onmouseover = {
        to_owned![onmoved];
        move |e: MouseEvent| {
            e.stop_propagation();
            if *clicking.peek() {
                let coordinates = e.get_element_coordinates();
                let x = coordinates.x - size.area.min_x() as f64 - 6.0;
                let percentage = x / (size.area.width() as f64 - 15.0) * 100.0;
                let percentage = percentage.clamp(0.0, 100.0);

                onmoved.call(percentage);
            }
        }
    };

    let onmousedown = {
        to_owned![onmoved];
        move |e: MouseEvent| {
            e.stop_propagation();
            focus.focus();
            clicking.set(true);
            let coordinates = e.get_element_coordinates();
            let x = coordinates.x - 6.0;
            let percentage = x / (size.area.width() as f64 - 15.0) * 100.0;
            let percentage = percentage.clamp(0.0, 100.0);

            onmoved.call(percentage);
        }
    };

    let onclick = move |_: MouseEvent| {
        clicking.set(false);
    };

    let onwheel = move |e: WheelEvent| {
        e.stop_propagation();
        let wheel_y = e.get_delta_y().clamp(-1.0, 1.0);
        let percentage = value + (wheel_y * 2.0);
        let percentage = percentage.clamp(0.0, 100.0);

        onmoved.call(percentage);
    };

    let inner_width = (size.area.width() - 15.0) * (value / 100.0) as f32;
    let border = if focus.is_selected() {
        format!("2 solid {}", theme.border_fill)
    } else {
        "none".to_string()
    };

    rsx!(
        rect {
            reference: node_reference,
            width: "{width}",
            height: "20",
            onmousedown,
            onglobalclick: onclick,
            focus_id,
            onmouseenter,
            onglobalmouseover: onmouseover,
            onmouseleave,
            onwheel: onwheel,
            main_align: "center",
            cross_align: "center",
            border: "{border}",
            corner_radius: "8",
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

#[cfg(test)]
mod test {
    use dioxus::prelude::use_signal;
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn slider() {
        fn slider_app() -> Element {
            let mut value = use_signal(|| 50.);

            rsx!(
                Slider {
                    value: *value.read(),
                    onmoved: move |p| {
                        value.set(p);
                    }
                }
                label {
                    "{value}"
                }
            )
        }

        let mut utils = launch_test(slider_app);
        let root = utils.root();
        let label = root.get(1);
        utils.wait_for_update().await;

        assert_eq!(label.get(0).text(), Some("50"));

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (250.0, 7.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseDown,
            cursor: (250.0, 7.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (500.0, 7.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (500.0, 7.0).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;

        assert_eq!(label.get(0).text(), Some("100"));
    }
}
