use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_animation, use_get_theme, Animation};

/// [`Switch`] component properties.
#[derive(Props)]
pub struct SwitchProps<'a> {
    /// Whether the `Switch` is enabled or not.
    pub enabled: bool,
    /// Handler for the `ontoggled` event.
    pub ontoggled: EventHandler<'a, ()>,
}

/// Controlled `Switch` component.
///
/// # Props
/// See [`SwitchProps`].
///
/// # Styling
/// Inherits the [`SwitchTheme`](freya_hooks::SwitchTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     let enabled = use_state(&cx, || false);
///
///     render!(
///         Switch {
///         enabled: *enabled.get(),
///         ontoggled: |_| {
///             enabled.set(!enabled.get());
///         }
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn Switch<'a>(cx: Scope<'a, SwitchProps<'a>>) -> Element<'a> {
    let animation = use_animation(cx, || 0.0);
    let theme = use_get_theme(cx);
    let hovering = use_state(cx, || false);
    let clicking = use_state(cx, || false);

    let onmouseleave = |_: MouseEvent| {
        if !(*clicking.get()) {
            hovering.set(false);
        }
    };

    let onmouseover = |_: MouseEvent| {
        hovering.set(true);
    };

    let onmousedown = |_: MouseEvent| {
        clicking.set(true);
    };

    let onclick = |_: MouseEvent| {
        clicking.set(false);
        cx.props.ontoggled.call(());
    };

    let (offset_x, border, circle) = {
        if cx.props.enabled {
            (
                animation.value(),
                theme.switch.enabled_background,
                theme.switch.enabled_thumb_background,
            )
        } else {
            (
                animation.value(),
                theme.switch.background,
                theme.switch.thumb_background,
            )
        }
    };

    let _ = use_memo(cx, &cx.props.enabled, move |enabled| {
        if enabled {
            animation.start(Animation::new_sine_in_out(0.0..=25.0, 200));
        } else if animation.value() > 0.0 {
            animation.start(Animation::new_sine_in_out(25.0..=0.0, 200));
        }
    });

    render!(
        rect {
            margin: "1.5",
            width: "50",
            height: "25",
            padding: "1",
            corner_radius: "50",
            background: "{border}",
            onmousedown: onmousedown,
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            onclick: onclick,
            rect {
                width: "100%",
                height: "100%",
                offset_x: "{offset_x}",
                padding: "2.5",
                corner_radius: "50",
                rect {
                    background: "{circle}",
                    direction: "both",
                    width: "18",
                    height: "18",
                    corner_radius: "50",
                }
            }
        }
    )
}
