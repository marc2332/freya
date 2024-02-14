use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;

use freya_hooks::{
    use_animation_with_dependencies, use_applied_theme, use_node_signal, use_platform,
    AccordionTheme, AccordionThemeWith, AnimNum,
};
use winit::window::CursorIcon;

/// Indicates the current status of the accordion.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum AccordionStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the accordion.
    Hovering,
}

/// [`Accordion`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// Theme override.
    pub theme: Option<AccordionThemeWith>,
    /// Inner children for the Accordion.
    pub children: Element,
    /// Summary element.
    pub summary: Element,
}

/// `Accordion` component.
///
/// # Props
/// See [`AccordionProps`].
///
/// # Styling
/// Inherits the [`AccordionTheme`](freya_hooks::AccordionTheme)
///
#[allow(non_snake_case)]
pub fn Accordion(props: AccordionProps) -> Element {
    let theme = use_applied_theme!(&props.theme, accordion);
    let mut open = use_signal(|| false);
    let (node_ref, size) = use_node_signal();
    let area = size().area;
    let animation = use_animation_with_dependencies(&area.height(), move |ctx, height| {
        ctx.with(AnimNum::new(0., height).time(200))
    });
    let mut status = use_signal(AccordionStatus::default);
    let platform = use_platform();

    let animation_value = animation.read().get().read().as_f32();
    let AccordionTheme {
        background,
        color,
        border_fill,
    } = theme;

    let onclick = move |_: MouseEvent| {
        open.toggle();
        if *open.read() {
            animation.read().start();
        } else {
            animation.read().reverse();
        }
    };

    use_drop(move || {
        if *status.read() == AccordionStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(AccordionStatus::Hovering);
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(AccordionStatus::default());
    };

    rsx!(
        rect {
            onmouseenter,
            onmouseleave,
            overflow: "clip",
            color: "{color}",
            padding: "10",
            margin: "2 4",
            corner_radius: "6",
            width: "100%",
            height: "auto",
            background: "{background}",
            onclick,
            border: "1 solid {border_fill}",
            {&props.summary}
            rect {
                overflow: "clip",
                width: "100%",
                height: "{animation_value}",
                rect {
                    reference: node_ref,
                    height: "auto",
                    width: "100%",
                    {&props.children}
                }
            }
        }
    )
}

/// [`AccordionSummary`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionSummaryProps {
    /// Inner children for the AccordionSummary.
    children: Element,
}

/// `AccordionSummary` component.
///
/// # Props
/// See [`AccordionSummaryProps`].
///
#[allow(non_snake_case)]
pub fn AccordionSummary(props: AccordionSummaryProps) -> Element {
    rsx!({ props.children })
}

/// [`AccordionBody`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionBodyProps {
    /// Inner children for the AccordionBody.
    children: Element,
}

/// `AccordionBody` component.
///
/// # Props
/// See [`AccordionBodyProps`].
///
#[allow(non_snake_case)]
pub fn AccordionBody(props: AccordionBodyProps) -> Element {
    rsx!(rect {
        width: "100%",
        padding: "15 0 0 0",
        {props.children}
    })
}
