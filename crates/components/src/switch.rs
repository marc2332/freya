use dioxus::prelude::*;
use freya_core::platform::CursorIcon;
use freya_elements::{
    self as dioxus_elements,
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
    OnDepsChange,
    SwitchThemeWith,
};

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
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut enabled = use_signal(|| false);
///
///     rsx!(Switch {
///         enabled: enabled(),
///         ontoggled: move |_| {
///             enabled.toggle();
///         }
///     })
/// }
/// # use freya_testing::prelude::*;
/// # // ENABLED
/// # use freya_testing::prelude::*;
/// # launch_doc_with_utils(|| {
/// #   rsx!(
/// #       Preview {
/// #           Switch {
/// #               enabled: true,
/// #               ontoggled: move |_| { }
/// #           }
/// #       }
/// #   )
/// # }, (185., 185.).into(), |mut utils| async move {
/// #   utils.wait_for_update().await;
/// #   tokio::time::sleep(std::time::Duration::from_millis(50)).await;
/// #   utils.wait_for_update().await;
/// #   utils.save_snapshot("./images/gallery_enabled_switch.png");
/// # });
/// #
/// # // DISABLED
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           Switch {
/// #               enabled: false,
/// #               ontoggled: move |_| { }
/// #           }
/// #       }
/// #   )
/// # }, (185., 185.).into(), "./images/gallery_not_enabled_switch.png");
/// ```
/// # Preview
///
/// | Enabled       | Not Enabled   |
/// | ------------- | ------------- |
/// | ![Switch Enabled Demo][gallery_enabled_switch] | ![Switch Not Enabled Demo][gallery_not_enabled_switch] |
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!(
        "gallery_not_enabled_switch",
        "images/gallery_not_enabled_switch.png"
    )
)]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("gallery_enabled_switch", "images/gallery_enabled_switch.png")
)]
#[allow(non_snake_case)]
pub fn Switch(props: SwitchProps) -> Element {
    let theme = use_applied_theme!(&props.theme, switch);
    let animation = use_animation_with_dependencies(&theme, |conf, theme| {
        conf.on_deps_change(OnDepsChange::Finish);
        (
            AnimNum::new(2., 22.)
                .time(300)
                .function(Function::Expo)
                .ease(Ease::Out),
            AnimNum::new(14., 18.)
                .time(300)
                .function(Function::Expo)
                .ease(Ease::Out),
            AnimColor::new(&theme.background, &theme.enabled_background)
                .time(300)
                .function(Function::Expo)
                .ease(Ease::Out),
            AnimColor::new(&theme.thumb_background, &theme.enabled_thumb_background)
                .time(300)
                .function(Function::Expo)
                .ease(Ease::Out),
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

    let onkeydown = move |e: KeyboardEvent| {
        if focus.validate_keydown(&e) {
            props.ontoggled.call(());
        }
    };

    let (offset_x, size, background, circle) = &*animation.get().read_unchecked();
    let offset_x = offset_x.read();
    let size = size.read();
    let background = background.read();
    let circle = circle.read();

    let border = if focus.is_focused_with_keyboard() {
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
            onkeydown,
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
