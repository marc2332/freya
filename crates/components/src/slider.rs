use dioxus::prelude::*;
use freya_elements::{
    elements as dioxus_elements,
    events::{
        KeyboardEvent,
        MouseEvent,
        WheelEvent,
    },
};
use freya_hooks::{
    use_applied_theme,
    use_focus,
    use_node,
    use_platform,
    SliderThemeWith,
};
use winit::window::CursorIcon;

/// Properties for the [`Slider`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// Theme override.
    pub theme: Option<SliderThemeWith>,
    /// Handler for the `onmoved` event.
    pub onmoved: EventHandler<f64>,
    /// Size of the Slider.
    #[props(into, default = "100%".to_string())]
    pub size: String,
    /// Height of the Slider.
    pub value: f64,
    #[props(default = "horizontal".to_string())]
    pub direction: String,
}

#[inline]
fn ensure_correct_slider_range(value: f64) -> f64 {
    if value < 0.0 {
        #[cfg(debug_assertions)]
        tracing::info!("Slider value is less than 0.0, setting to 0.0");
        0.0
    } else if value > 100.0 {
        #[cfg(debug_assertions)]
        tracing::info!("Slider value is greater than 100.0, setting to 100.0");
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
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut percentage = use_signal(|| 20.0);
///
///     rsx!(
///         label {
///             "Value: {percentage}"
///         }
///         Slider {
///             size: "50%",
///             value: *percentage.read(),
///             onmoved: move |p| {
///                 percentage.set(p);
///             }
///         }
///     )
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           Slider {
/// #               size: "50%",
/// #               value: 50.0,
/// #               onmoved: move |p| { }
/// #           }
/// #       }
/// #   )
/// # }, (200., 150.).into(), "./images/gallery_slider.png");
/// ```
/// # Preview
/// ![Slider Preview][slider]
#[embed_doc_image::embed_doc_image("slider", "images/gallery_slider.png")]
#[allow(non_snake_case)]
pub fn Slider(
    SliderProps {
        value,
        onmoved,
        theme,
        size,
        direction,
    }: SliderProps,
) -> Element {
    let theme = use_applied_theme!(&theme, slider);
    let mut focus = use_focus();
    let mut status = use_signal(SliderStatus::default);
    let mut clicking = use_signal(|| false);
    let platform = use_platform();
    let (node_reference, node_size) = use_node();

    let direction_is_vertical = direction == "vertical";
    let value = ensure_correct_slider_range(value);
    let a11y_id = focus.attribute();

    use_drop(move || {
        if *status.peek() == SliderStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onkeydown = move |e: KeyboardEvent| match e.key {
        Key::ArrowLeft if !direction_is_vertical => {
            e.stop_propagation();
            let percentage = (value - 4.).clamp(0.0, 100.0);
            onmoved.call(percentage);
        }
        Key::ArrowRight if !direction_is_vertical => {
            e.stop_propagation();
            let percentage = (value + 4.).clamp(0.0, 100.0);
            onmoved.call(percentage);
        }
        Key::ArrowUp if direction_is_vertical => {
            e.stop_propagation();
            let percentage = (value + 4.).clamp(0.0, 100.0);
            onmoved.call(percentage);
        }
        Key::ArrowDown if direction_is_vertical => {
            e.stop_propagation();
            let percentage = (value - 4.).clamp(0.0, 100.0);
            onmoved.call(percentage);
        }
        _ => {}
    };

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

    let onmousemove = {
        to_owned![onmoved];
        move |e: MouseEvent| {
            e.stop_propagation();
            if *clicking.peek() {
                let coordinates = e.get_element_coordinates();
                let percentage = if direction_is_vertical {
                    let y = coordinates.y - node_size.area.min_y() as f64 - 6.0;
                    100. - (y / (node_size.area.height() as f64 - 15.0) * 100.0)
                } else {
                    let x = coordinates.x - node_size.area.min_x() as f64 - 6.0;
                    x / (node_size.area.width() as f64 - 15.0) * 100.0
                };
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
            let percentage = if direction_is_vertical {
                let y = coordinates.y - 6.0;
                100. - (y / (node_size.area.height() as f64 - 15.0) * 100.0)
            } else {
                let x = coordinates.x - 6.0;
                x / (node_size.area.width() as f64 - 15.0) * 100.0
            };
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

    let border = if focus.is_selected() {
        format!("2 inner {}", theme.border_fill)
    } else {
        "none".to_string()
    };

    let (
        width,
        height,
        container_width,
        container_height,
        inner_width,
        inner_height,
        main_align,
        offset_x,
        offset_y,
    ) = if direction_is_vertical {
        let inner_height = (node_size.area.height() - 15.0) * (value / 100.0) as f32;
        (
            "20",
            size.as_str(),
            "6",
            "100%",
            "100%".to_string(),
            inner_height.to_string(),
            "end",
            -6,
            3,
        )
    } else {
        let inner_width = (node_size.area.width() - 15.0) * (value / 100.0) as f32;
        (
            size.as_str(),
            "20",
            "100%",
            "6",
            inner_width.to_string(),
            "100%".to_string(),
            "start",
            -3,
            -6,
        )
    };

    let inner_fill = rsx!(rect {
        background: "{theme.thumb_inner_background}",
        width: "{inner_width}",
        height: "{inner_height}",
        corner_radius: "50"
    });

    let thumb = rsx!(
        rect {
            width: "fill",
            offset_x: "{offset_x}",
            offset_y: "{offset_y}",
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
    );

    rsx!(
        rect {
            reference: node_reference,
            width: "{width}",
            height: "{height}",
            onmousedown,
            onglobalclick: onclick,
            a11y_id,
            onmouseenter,
            onglobalmousemove: onmousemove,
            onmouseleave,
            onwheel: onwheel,
            onkeydown,
            main_align: "center",
            cross_align: "center",
            border: "{border}",
            corner_radius: "8",
            rect {
                background: "{theme.background}",
                width: "{container_width}",
                height: "{container_height}",
                main_align: "{main_align}",
                direction: "{direction}",
                corner_radius: "50",
                if direction_is_vertical {
                    {thumb}
                    {inner_fill}
                } else {
                    {inner_fill}
                    {thumb}
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
            name: EventName::MouseMove,
            cursor: (250.0, 7.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseDown,
            cursor: (250.0, 7.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseMove,
            cursor: (500.0, 7.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        assert_eq!(label.get(0).text(), Some("100"));
    }
}
