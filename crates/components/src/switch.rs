use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{KeyboardEvent, MouseEvent};

use freya_hooks::{
    use_animation, use_applied_theme, use_focus, use_platform, Animation, SwitchThemeWith,
};
use winit::window::CursorIcon;

/// [`Switch`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// Theme override.
    pub theme: Option<SwitchThemeWith>,
    /// Whether the `Switch` is enabled or not.
    pub enabled: bool,
    /// Handler for the `ontoggled` event.
    pub ontoggled: EventHandler<()>,
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
/// fn app() -> Element {
///     let mut enabled = use_signal(|| false);
///
///     rsx!(
///         Switch {
///             enabled: *enabled.read(),
///             ontoggled: move |_| {
///                 enabled.toggle();
///             }
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn Switch(props: SwitchProps) -> Element {
    let mut animation = use_animation(|| 0.0);
    let theme = use_applied_theme!(&props.theme, switch);
    let platform = use_platform();
    let mut status = use_signal(SwitchStatus::default);
    let focus = use_focus();

    let focus_id = focus.attribute();

    use_drop({
        to_owned![status, platform];
        move || {
            if *status.read() == SwitchStatus::Hovering {
                platform.set_cursor(CursorIcon::default());
            }
        }
    });

    let onmousedown = |e: MouseEvent| {
        e.stop_propagation();
    };

    let onmouseleave = {
        to_owned![platform];
        move |e: MouseEvent| {
            e.stop_propagation();
            *status.write() = SwitchStatus::Idle;
            platform.set_cursor(CursorIcon::default());
        }
    };

    let onmouseenter = move |e: MouseEvent| {
        e.stop_propagation();
        *status.write() = SwitchStatus::Hovering;
        platform.set_cursor(CursorIcon::Pointer);
    };

    let onclick = {
        let ontoggled = props.ontoggled.clone();
        to_owned![focus];
        move |e: MouseEvent| {
            e.stop_propagation();
            focus.focus();
            ontoggled.call(());
        }
    };

    let onkeydown = {
        to_owned![focus];
        move |e: KeyboardEvent| {
            if focus.validate_keydown(e) {
                props.ontoggled.call(());
            }
        }
    };

    let (offset_x, background, circle) = {
        if props.enabled {
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
        if props.enabled {
            format!("2 solid {}", theme.enabled_focus_border_fill)
        } else {
            format!("2 solid {}", theme.focus_border_fill)
        }
    } else {
        "none".to_string()
    };

    let _ = use_memo_with_dependencies(&props.enabled, move |enabled| {
        if enabled {
            animation.start(Animation::new_sine_in_out(0.0..=25.0, 200));
        } else if animation.peek_value() > 0.0 {
            animation.start(Animation::new_sine_in_out(25.0..=0.0, 200));
        }
    });

    rsx!(
        rect {
            margin: "1.5",
            width: "50",
            height: "25",
            padding: "1",
            corner_radius: "50",
            background: "{background}",
            border: "{border}",
            onmousedown,
            onmouseenter,
            onmouseleave,
            onkeydown,
            onclick,
            focus_id,
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
