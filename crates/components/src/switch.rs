use dioxus::prelude::*;
use freya_elements::{
    elements as dioxus_elements,
    events::{
        KeyboardEvent,
        MouseEvent,
    },
};
use freya_hooks::{
    use_animation_with_dependencies,
    use_applied_theme,
    use_focus,
    use_platform,
    AnimColor,
    AnimNum,
    Ease,
    Function,
    SwitchThemeWith,
};
use winit::window::CursorIcon;

/// Properties for the [`Switch`] component.
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

/// Display whether a state is `true` or `false`.
/// Commonly used for enabled/disabled scenarios.
/// Example: light/dark theme.
///
/// # Styling
///
/// Inherits the [`SwitchTheme`](freya_hooks::SwitchTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut enabled = use_signal(|| false);
///
///     rsx!(Switch {
///         enabled: *enabled.read(),
///         ontoggled: move |_| {
///             enabled.toggle();
///         }
///     })
/// }
/// ```
#[allow(non_snake_case)]
pub fn Switch(props: SwitchProps) -> Element {
    let theme = use_applied_theme!(&props.theme, switch);
    let animation = use_animation_with_dependencies(&theme, |ctx, theme| {
        (
            ctx.with(
                AnimNum::new(2., 22.)
                    .time(300)
                    .function(Function::Expo)
                    .ease(Ease::Out),
            ),
            ctx.with(
                AnimNum::new(14., 18.)
                    .time(300)
                    .function(Function::Expo)
                    .ease(Ease::Out),
            ),
            ctx.with(
                AnimColor::new(&theme.background, &theme.enabled_background)
                    .time(300)
                    .function(Function::Expo)
                    .ease(Ease::Out),
            ),
            ctx.with(
                AnimColor::new(&theme.thumb_background, &theme.enabled_thumb_background)
                    .time(300)
                    .function(Function::Expo)
                    .ease(Ease::Out),
            ),
        )
    });
    let platform = use_platform();
    let mut status = use_signal(SwitchStatus::default);
    let mut focus = use_focus();

    let a11y_id = focus.attribute();

    use_drop(move || {
        if *status.read() == SwitchStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmousedown = |e: MouseEvent| {
        e.stop_propagation();
    };

    let onmouseleave = move |e: MouseEvent| {
        e.stop_propagation();
        *status.write() = SwitchStatus::Idle;
        platform.set_cursor(CursorIcon::default());
    };

    let onmouseenter = move |e: MouseEvent| {
        e.stop_propagation();
        *status.write() = SwitchStatus::Hovering;
        platform.set_cursor(CursorIcon::Pointer);
    };

    let onclick = move |e: MouseEvent| {
        e.stop_propagation();
        focus.focus();
        props.ontoggled.call(());
    };

    let onglobalkeydown = move |e: KeyboardEvent| {
        if focus.validate_globalkeydown(&e) {
            props.ontoggled.call(());
        }
    };

    let offset_x = animation.get().0.read().as_f32();
    let size = animation.get().1.read().as_f32();
    let background = animation.get().2.read().as_string();
    let circle = animation.get().3.read().as_string();

    let border = if focus.is_selected() {
        if props.enabled {
            format!("2 inner {}", theme.enabled_focus_border_fill)
        } else {
            format!("2 inner {}", theme.focus_border_fill)
        }
    } else {
        "none".to_string()
    };

    use_memo(use_reactive(&props.enabled, move |enabled| {
        if enabled {
            animation.start();
        } else if animation.peek_has_run_yet() {
            animation.reverse();
        }
    }));

    rsx!(
        rect {
            margin: "{theme.margin}",
            width: "48",
            height: "25",
            padding: "4",
            corner_radius: "50",
            background: "{background}",
            border: "{border}",
            onmousedown,
            onmouseenter,
            onmouseleave,
            onglobalkeydown,
            onclick,
            a11y_id,
            offset_x: "{offset_x}",
            main_align: "center",
            rect {
                background: "{circle}",
                width: "{size}",
                height: "{size}",
                corner_radius: "50",
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
    pub async fn switch() {
        fn switch_app() -> Element {
            let mut enabled = use_signal(|| false);

            rsx!(
                Switch {
                    enabled: *enabled.read(),
                    ontoggled: move |_| {
                        enabled.toggle();
                    }
                }
                label {
                    "{enabled}"
                }
            )
        }

        let mut utils = launch_test(switch_app);
        let root = utils.root();
        let label = root.get(1);
        utils.wait_for_update().await;

        // Default is false
        assert_eq!(label.get(0).text(), Some("false"));

        utils.click_cursor((15., 15.)).await;

        // Check if after clicking it is now enabled
        assert_eq!(label.get(0).text(), Some("true"));

        utils.click_cursor((15., 15.)).await;

        // Check if after clicking again it is now disabled
        assert_eq!(label.get(0).text(), Some("false"));
    }
}
