use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{KeyboardEvent, MouseEvent};

use freya_hooks::{
    use_animation, use_applied_theme, use_focus, use_platform, Animation, SwitchThemeWith,
};
use winit::window::CursorIcon;

/// [`Switch`] component properties.
#[derive(Props)]
pub struct SwitchProps<'a> {
    /// Theme override.
    pub theme: Option<SwitchThemeWith>,
    /// Whether the `Switch` is enabled or not.
    pub enabled: bool,
    /// Handler for the `ontoggled` event.
    pub ontoggled: EventHandler<'a, ()>,
}

/// Describes the current status of the Switch.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SwitchStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the switch.
    Hovering,
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
///             enabled: *enabled.get(),
///             ontoggled: |_| {
///                 enabled.set(!enabled.get());
///             }
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn Switch<'a>(cx: Scope<'a, SwitchProps<'a>>) -> Element<'a> {
    let animation = use_animation(cx, || 0.0);
    let theme = use_applied_theme!(cx, &cx.props.theme, switch);
    let platform = use_platform(cx);
    let status = use_ref(cx, SwitchStatus::default);
    let focus = use_focus(cx);

    let focus_id = focus.attribute(cx);

    use_on_destroy(cx, {
        to_owned![status, platform];
        move || {
            if *status.read() == SwitchStatus::Hovering {
                platform.set_cursor(CursorIcon::default());
            }
        }
    });

    let onmouseleave = {
        to_owned![platform];
        move |_: MouseEvent| {
            *status.write_silent() = SwitchStatus::Idle;
            platform.set_cursor(CursorIcon::default());
        }
    };

    let onmouseenter = move |_: MouseEvent| {
        *status.write_silent() = SwitchStatus::Hovering;
        platform.set_cursor(CursorIcon::Pointer);
    };

    let onclick = |_: MouseEvent| {
        focus.focus();
        cx.props.ontoggled.call(());
    };

    let onkeydown = |e: KeyboardEvent| {
        if focus.validate_keydown(e) {
            cx.props.ontoggled.call(());
        }
    };

    let (offset_x, background, circle) = {
        if cx.props.enabled {
            (
                animation.value(),
                theme.enabled_background,
                theme.enabled_thumb_background,
            )
        } else {
            (animation.value(), theme.background, theme.thumb_background)
        }
    };
    let border = if focus.is_selected() {
        if cx.props.enabled {
            format!("2 solid {}", theme.enabled_focus_border_fill)
        } else {
            format!("2 solid {}", theme.focus_border_fill)
        }
    } else {
        "none".to_string()
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
            background: "{background}",
            border: "{border}",
            onmousedown: |_| {},
            onmouseenter: onmouseenter,
            onmouseleave: onmouseleave,
            onkeydown: onkeydown,
            onclick: onclick,
            focus_id: focus_id,
            rect {
                width: "100%",
                height: "100%",
                offset_x: "{offset_x}",
                padding: "2.5",
                corner_radius: "50",
                rect {
                    background: "{circle}",
                    width: "18",
                    height: "18",
                    corner_radius: "50",
                }
            }
        }
    )
}
