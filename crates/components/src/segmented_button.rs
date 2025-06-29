use dioxus::prelude::*;
use freya_core::platform::CursorIcon;
use freya_elements::{
    self as dioxus_elements,
    events::{
        KeyboardEvent,
        PointerEvent,
        PointerType,
    },
    MouseButton,
    TouchPhase,
};
use freya_hooks::{
    use_applied_theme,
    use_focus,
    use_platform,
    ButtonSegmentTheme,
    ButtonSegmentThemeWith,
    SegmentedButtonTheme,
    SegmentedButtonThemeWith,
};

use crate::{
    PressEvent,
    TickIcon,
};

/// Properties for the [`ButtonSegment`].
#[derive(Props, Clone, PartialEq)]
pub struct ButtonSegmentProps {
    /// Inner children for the button.
    pub children: Element,
    /// Event handler for when the button is pressed.
    pub onpress: Option<EventHandler<PressEvent>>,

    #[props(default = true)]
    pub enabled: bool,

    #[props(default = false)]
    pub selected: bool,
}

/// Properties for the [`ButtonSegment`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BaseButtonSegmentProps {
    pub theme: Option<ButtonSegmentThemeWith>,
    /// Inner children for the button.
    pub children: Element,
    /// Event handler for when the button is pressed.
    ///
    /// This will fire upon **mouse click**, **touch** or pressing the **enter key** when it is focused.
    pub onpress: Option<EventHandler<PressEvent>>,

    #[props(default = true)]
    pub enabled: bool,

    #[props(default = false)]
    pub selected: bool,
}

/// Identifies the current status of the [`ButtonSegment`]s.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
enum ButtonSegmentStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the button.
    Hovering,
}

/// [ButtonSegment] are buttons grouped under a [SegmentedButton].
///
/// # Styling
/// Inherits the outline [`ButtonSegmentTheme`](freya_hooks::ButtonSegmentTheme) theme.
#[allow(non_snake_case)]
pub fn ButtonSegment(
    BaseButtonSegmentProps {
        theme,
        onpress,
        children,
        enabled,
        selected,
    }: BaseButtonSegmentProps,
) -> Element {
    BaseButtonSegment(BaseButtonSegmentProps {
        theme,
        children,
        onpress,
        enabled,
        selected,
    })
}

#[allow(non_snake_case)]
pub fn BaseButtonSegment(
    BaseButtonSegmentProps {
        theme,
        onpress,
        children,
        enabled,
        selected,
    }: BaseButtonSegmentProps,
) -> Element {
    let theme = use_applied_theme!(&theme, button_segment);
    let mut focus = use_focus();
    let mut status = use_signal(ButtonSegmentStatus::default);
    let platform = use_platform();

    let a11y_id = focus.attribute();

    let ButtonSegmentTheme {
        background,
        hover_background,
        disabled_background,
        padding,
        width,
        height,
        font_theme,
        shadow,
        selected_padding,
        selected_icon_fill,
    } = theme;

    let onpointerup = move |ev: PointerEvent| {
        if !enabled {
            return;
        }
        focus.request_focus();
        if let Some(onpress) = &onpress {
            let is_valid = match ev.data.pointer_type {
                PointerType::Mouse {
                    trigger_button: Some(MouseButton::Left),
                } => true,
                PointerType::Touch { phase, .. } => phase == TouchPhase::Ended,
                _ => false,
            };
            if is_valid {
                onpress.call(PressEvent::Pointer(ev))
            }
        }
    };

    use_effect(use_reactive!(|enabled| {
        if *status.peek() == ButtonSegmentStatus::Hovering && !enabled {
            platform.set_cursor(CursorIcon::default());
        }
    }));

    use_drop(move || {
        if *status.read() == ButtonSegmentStatus::Hovering && enabled {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onpointerenter = move |_| {
        if enabled {
            platform.set_cursor(CursorIcon::Pointer);
            status.set(ButtonSegmentStatus::Hovering);
        }
    };

    let onpointerleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(ButtonSegmentStatus::default());
    };

    let onkeydown = move |ev: KeyboardEvent| {
        if focus.validate_keydown(&ev) && !enabled {
            if let Some(onpress) = &onpress {
                onpress.call(PressEvent::Key(ev))
            }
        }
    };

    let a11y_focusable = if enabled { "true" } else { "false" };
    let background = match *status.read() {
        _ if !enabled => disabled_background,
        _ if selected => hover_background,
        ButtonSegmentStatus::Hovering => hover_background,
        ButtonSegmentStatus::Idle => background,
    };
    let padding = if selected { selected_padding } else { padding };

    rsx!(
        rect {
            onpointerup,
            onpointerenter,
            onpointerleave,
            onkeydown,
            a11y_id,
            direction: "horizontal",
            width: "{width}",
            height: "{height}",
            padding: "{padding}",
            overflow: "clip",
            a11y_role: "button",
            a11y_focusable,
            color: "{font_theme.color}",
            shadow: "{shadow}",
            background: "{background}",
            text_height: "disable-least-ascent",
            main_align: "center",
            cross_align: "center",
            max_lines: "1",
            spacing: "4",
            if selected {
                TickIcon {
                    fill: "{selected_icon_fill}"
                }
            }
            {&children}
        }
    )
}

/// Properties for the [`SegmentedButton`].
#[derive(Props, Clone, PartialEq)]
pub struct SegmentedButtonProps {
    pub theme: Option<SegmentedButtonThemeWith>,
    /// Inner children for the button.
    pub children: Element,
}

/// [SegmentedButton] is used to group a set of [ButtonSegment]s together.
///
/// # Styling
/// Inherits the outline [`SegmentedButtonTheme`](freya_hooks::SegmentedButtonTheme) theme.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// # use std::collections::HashSet;
/// fn app() -> Element {
///     let mut selected = use_signal(|| HashSet::from([1]));
///     rsx!(
///         rect {
///             padding: "8",
///             spacing: "8",
///             SegmentedButton {
///                 for i in 0..2 {
///                     ButtonSegment {
///                         key: "{i}",
///                         selected: selected.read().contains(&i),
///                         onpress: move |_| {
///                             if selected.read().contains(&i) {
///                                 selected.write().remove(&i);
///                             } else {
///                                 selected.write().insert(i);
///                             }
///                         },
///                         label {
///                             "Option {i}"
///                         }
///                     }
///                 }
///             }
///         }
///     )
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           {app()}
/// #       }
/// #   )
/// # }, (250., 250.).into(), "./images/gallery_segmented_button.png");
/// ```
///
/// # Preview
/// ![SegmentedButton Preview][segmented_button]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("segmented_button", "images/gallery_segmented_button.png")
)]
#[allow(non_snake_case)]
pub fn SegmentedButton(
    BaseSegmentedButtonProps { theme, children }: BaseSegmentedButtonProps,
) -> Element {
    BaseSegmentedButton(BaseSegmentedButtonProps { theme, children })
}

/// Properties for the [`BaseSegmentedButton`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BaseSegmentedButtonProps {
    pub theme: Option<SegmentedButtonThemeWith>,
    /// Inner children for the button.
    pub children: Element,
}

#[allow(non_snake_case)]
pub fn BaseSegmentedButton(
    BaseSegmentedButtonProps { theme, children }: BaseSegmentedButtonProps,
) -> Element {
    let theme = use_applied_theme!(&theme, segmented_button);

    let SegmentedButtonTheme {
        background,
        shadow,
        border_fill,
        corner_radius,
    } = theme;

    rsx!(
        rect {
            overflow: "clip",
            background: "{background}",
            shadow: "{shadow}",
            border: "2 outer {border_fill}",
            corner_radius: "{corner_radius}",
            direction: "horizontal",
            {&children}
        }
    )
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn segmented_button() {
        fn segmented_button_app() -> Element {
            let mut selected = use_signal(HashSet::new);
            rsx!(
                SegmentedButton {
                    for i in 0..5 {
                        ButtonSegment {
                            key: "{i}",
                            selected: selected.read().contains(&i),
                            onpress: move |_| {
                                if selected.read().contains(&i) {
                                    selected.write().remove(&i);
                                } else {
                                    selected.write().insert(i);
                                }
                            },
                            label {
                                "Option {i}"
                            }
                        }
                    }
                }
            )
        }

        let mut utils = launch_test(segmented_button_app);
        let root = utils.root();
        utils.wait_for_update().await;

        assert!(root.get(0).get(0).get(0).is_placeholder());
        assert!(root.get(0).get(1).get(0).is_placeholder());
        assert!(root.get(0).get(2).get(0).is_placeholder());

        utils.click_cursor((115.0, 15.0)).await;

        assert!(root.get(0).get(0).get(0).is_placeholder());
        assert!(root.get(0).get(1).get(0).is_element());
        assert!(root.get(0).get(2).get(0).is_placeholder());

        utils.click_cursor((115.0, 15.0)).await;

        assert!(root.get(0).get(0).get(0).is_placeholder());
        assert!(root.get(0).get(1).get(0).is_placeholder());
        assert!(root.get(0).get(2).get(0).is_placeholder());

        utils.click_cursor((15.0, 15.0)).await;
        utils.click_cursor((215.0, 15.0)).await;

        assert!(root.get(0).get(0).get(0).is_element());
        assert!(root.get(0).get(1).get(0).is_placeholder());
        assert!(root.get(0).get(2).get(0).is_element());
    }
}
