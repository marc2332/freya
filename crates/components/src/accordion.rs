use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_animation, use_get_theme, use_node, AccordionTheme, Animation};

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

    let animation_value = animation.value();
    let AccordionTheme { background, color } = theme.accordion;

    // Adapt the accordion if the body size changes
    let _ = use_memo(
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

    render!(
        rect {
            overflow: "clip",
            color: "{color}",
            padding: "10",
            corner_radius: "3",
            width: "100%",
            height: "auto",
            background: "{background}",
            onclick: onclick,
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
