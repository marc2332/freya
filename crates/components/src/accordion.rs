use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{
    use_animation, use_get_theme, use_node, use_platform, AccordionTheme, Animation,
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
#[derive(Props)]
pub struct AccordionProps<'a> {
    /// Inner children for the Accordion.
    children: Element<'a>,
    /// Summary element.
    summary: Element<'a>,
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
pub fn Accordion<'a>(cx: Scope<'a, AccordionProps<'a>>) -> Element<'a> {
    let theme = use_get_theme(cx);
    let animation = use_animation(cx, || 0.0);
    let open = use_state(cx, || false);
    let (node_ref, size) = use_node(cx);
    let status = use_state(cx, AccordionStatus::default);
    let platform = use_platform(cx);

    let animation_value = animation.value();
    let AccordionTheme {
        background,
        color,
        border_fill,
    } = theme.accordion;

    // Adapt the accordion if the body size changes
    use_memo(
        cx,
        &(
            size.area.width(),
            size.area.height(),
            animation.is_animating(),
        ),
        {
            to_owned![animation];
            move |(_, height, animating)| {
                if (height as f64) < animation.value() && !animating {
                    animation.set_value(size.area.height() as f64);
                }
            }
        },
    );

    let onclick = move |_: MouseEvent| {
        let bodyHeight = size.area.height() as f64;
        if *open.get() {
            animation.start(Animation::new_sine_in_out(bodyHeight..=0.0, 200));
        } else {
            animation.start(Animation::new_sine_in_out(0.0..=bodyHeight, 200));
        }
        open.set(!*open.get());
    };

    use_on_unmount(cx, {
        to_owned![status, platform];
        move || {
            if *status.current() == AccordionStatus::Hovering {
                platform.set_cursor(CursorIcon::default());
            }
        }
    });

    let onmouseenter = {
        to_owned![status, platform];
        move |_| {
            platform.set_cursor(CursorIcon::Hand);
            status.set(AccordionStatus::Hovering);
        }
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(AccordionStatus::default());
    };

    render!(
        rect {
            onmouseenter: onmouseenter,
            onmouseleave: onmouseleave,
            overflow: "clip",
            color: "{color}",
            padding: "10",
            margin: "2 4",
            corner_radius: "6",
            width: "100%",
            height: "auto",
            background: "{background}",
            onclick: onclick,
            border: "1 solid {border_fill}",
            &cx.props.summary
            rect {
                overflow: "clip",
                width: "100%",
                height: "{animation_value}",
                rect {
                    reference: node_ref,
                    height: "auto",
                    width: "100%",
                    &cx.props.children
                }
            }
        }
    )
}

/// [`AccordionSummary`] component properties.
#[derive(Props)]
pub struct AccordionSummaryProps<'a> {
    /// Inner children for the AccordionSummary.
    children: Element<'a>,
}

/// `AccordionSummary` component.
///
/// # Props
/// See [`AccordionSummaryProps`].
///
#[allow(non_snake_case)]
pub fn AccordionSummary<'a>(cx: Scope<'a, AccordionSummaryProps<'a>>) -> Element<'a> {
    render!(&cx.props.children)
}

/// [`AccordionBody`] component properties.
#[derive(Props)]
pub struct AccordionBodyProps<'a> {
    /// Inner children for the AccordionBody.
    children: Element<'a>,
}

/// `AccordionBody` component.
///
/// # Props
/// See [`AccordionBodyProps`].
///
#[allow(non_snake_case)]
pub fn AccordionBody<'a>(cx: Scope<'a, AccordionBodyProps<'a>>) -> Element<'a> {
    render!(rect {
        width: "100%",
        padding: "15 0 0 0",
        &cx.props.children
    })
}
