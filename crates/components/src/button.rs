use dioxus::prelude::*;
use freya_elements::{
    elements as dioxus_elements,
    events::{
        KeyboardEvent,
        PointerEvent,
        PointerType,
    },
};
use freya_hooks::{
    use_applied_theme,
    use_focus,
    use_platform,
    ButtonTheme,
    ButtonThemeWith,
};
use winit::{
    event::{
        MouseButton,
        TouchPhase,
    },
    window::CursorIcon,
};

/// Properties for the [`Button`], [`FilledButton`] and [`OutlineButton`] components.
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Theme override.
    pub theme: Option<ButtonThemeWith>,
    /// Inner children for the button.
    pub children: Element,
    /// Event handler for when the button is pressed.
    pub onpress: Option<EventHandler<PressEvent>>,
    /// Event handler for when the button is clicked. Not recommended, use `onpress` instead.
    pub onclick: Option<EventHandler<()>>,
}

/// Clickable button.
///
/// # Styling
/// Inherits the [`ButtonTheme`](freya_hooks::ButtonTheme) theme.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         Button {
///             onpress: |_| println!("clicked"),
///             label {
///                 "Click this"
///             }
///         }
///     )
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           {app()}
/// #       }
/// #   )
/// # }, (185., 185.).into(), "./images/gallery_button.png");
/// ```
///
/// # Preview
/// ![Button Preview][button]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("button", "images/gallery_button.png")
)]
#[allow(non_snake_case)]
pub fn Button(props: ButtonProps) -> Element {
    let theme = use_applied_theme!(&props.theme, button);
    ButtonBase(BaseButtonProps {
        theme,
        children: props.children,
        onpress: props.onpress,
        onclick: props.onclick,
    })
}

/// Clickable button with a solid fill color.
///
/// # Styling
/// Inherits the filled [`ButtonTheme`](freya_hooks::ButtonTheme) theme.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         FilledButton {
///             onpress: |_| println!("clicked"),
///             label {
///                 "Click this"
///             }
///         }
///     )
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           {app()}
/// #       }
/// #   )
/// # }, (185., 185.).into(), "./images/gallery_filled_button.png");
/// ```
///
/// # Preview
/// ![FilledButton Preview][filled_button]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("filled_button", "images/gallery_filled_button.png")
)]
#[allow(non_snake_case)]
pub fn FilledButton(props: ButtonProps) -> Element {
    let theme = use_applied_theme!(&props.theme, filled_button);
    ButtonBase(BaseButtonProps {
        theme,
        children: props.children,
        onpress: props.onpress,
        onclick: props.onclick,
    })
}

/// Clickable button with an outline style.
///
/// # Styling
/// Inherits the outline [`ButtonTheme`](freya_hooks::ButtonTheme) theme.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         OutlineButton {
///             onpress: |_| println!("clicked"),
///             label {
///                 "Click this"
///             }
///         }
///     )
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           {app()}
/// #       }
/// #   )
/// # }, (185., 185.).into(), "./images/gallery_outline_button.png");
/// ```
///
/// # Preview
/// ![OutlineButton Preview][outline_button]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("outline_button", "images/gallery_outline_button.png")
)]
#[allow(non_snake_case)]
pub fn OutlineButton(props: ButtonProps) -> Element {
    let theme = use_applied_theme!(&props.theme, outline_button);
    ButtonBase(BaseButtonProps {
        theme,
        children: props.children,
        onpress: props.onpress,
        onclick: props.onclick,
    })
}

pub enum PressEvent {
    Pointer(PointerEvent),
    Key(KeyboardEvent),
}

impl PressEvent {
    pub fn stop_propagation(&self) {
        match &self {
            Self::Pointer(ev) => ev.stop_propagation(),
            Self::Key(ev) => ev.stop_propagation(),
        }
    }
}

/// Properties for the [`Button`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BaseButtonProps {
    /// Theme.
    pub theme: ButtonTheme,
    /// Inner children for the button.
    pub children: Element,
    /// Event handler for when the button is pressed.
    pub onpress: Option<EventHandler<PressEvent>>,
    /// Event handler for when the button is clicked. Not recommended, use `onpress` instead.
    pub onclick: Option<EventHandler<()>>,
}

/// Identifies the current status of the Button.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ButtonStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the button.
    Hovering,
}

#[allow(non_snake_case)]
pub fn ButtonBase(
    BaseButtonProps {
        onpress,
        children,
        theme,
        onclick,
    }: BaseButtonProps,
) -> Element {
    let mut focus = use_focus();
    let mut status = use_signal(ButtonStatus::default);
    let platform = use_platform();

    let a11y_id = focus.attribute();

    let ButtonTheme {
        background,
        hover_background,
        border_fill,
        focus_border_fill,
        padding,
        margin,
        corner_radius,
        width,
        height,
        font_theme,
        shadow,
    } = theme;

    let onpointerup = {
        to_owned![onpress, onclick];
        move |ev: PointerEvent| {
            focus.focus();
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
            } else if let Some(onclick) = onclick {
                if let PointerType::Mouse {
                    trigger_button: Some(MouseButton::Left),
                    ..
                } = ev.data.pointer_type
                {
                    onclick.call(())
                }
            }
        }
    };

    use_drop(move || {
        if *status.read() == ButtonStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(ButtonStatus::Hovering);
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(ButtonStatus::default());
    };

    let onglobalkeydown = move |ev: KeyboardEvent| {
        if focus.validate_globalkeydown(&ev) {
            if let Some(onpress) = &onpress {
                onpress.call(PressEvent::Key(ev))
            }
        }
    };

    let background = match *status.read() {
        ButtonStatus::Hovering => hover_background,
        ButtonStatus::Idle => background,
    };
    let border = if focus.is_selected() {
        format!("2 inner {focus_border_fill}")
    } else {
        format!("1 inner {border_fill}")
    };

    rsx!(
        rect {
            onpointerup,
            onmouseenter,
            onmouseleave,
            onglobalkeydown,
            a11y_id,
            width: "{width}",
            height: "{height}",
            padding: "{padding}",
            margin: "{margin}",
            overflow: "clip",
            a11y_role:"button",
            color: "{font_theme.color}",
            shadow: "{shadow}",
            border: "{border}",
            corner_radius: "{corner_radius}",
            background: "{background}",
            text_height: "disable-least-ascent",
            main_align: "center",
            cross_align: "center",
            {&children}
        }
    )
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn button() {
        fn button_app() -> Element {
            let mut state = use_signal(|| false);

            rsx!(
                Button {
                    onpress: move |_| state.toggle(),
                    label {
                        "{state}"
                    }
                }
            )
        }

        let mut utils = launch_test(button_app);
        let root = utils.root();
        let label = root.get(0).get(0);
        utils.wait_for_update().await;

        assert_eq!(label.get(0).text(), Some("false"));

        utils.click_cursor((15.0, 15.0)).await;

        assert_eq!(label.get(0).text(), Some("true"));

        utils.push_event(PlatformEvent::Touch {
            name: EventName::TouchStart,
            location: (15.0, 15.0).into(),
            finger_id: 1,
            phase: TouchPhase::Started,
            force: None,
        });
        utils.wait_for_update().await;

        utils.push_event(PlatformEvent::Touch {
            name: EventName::TouchEnd,
            location: (15.0, 15.0).into(),
            finger_id: 1,
            phase: TouchPhase::Ended,
            force: None,
        });
        utils.wait_for_update().await;

        assert_eq!(label.get(0).text(), Some("false"));
    }
}
