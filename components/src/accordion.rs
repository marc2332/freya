use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_animation, use_get_theme, use_node, Animation};

/// `Accordion` component.
#[inline_props]
#[allow(non_snake_case)]
pub fn Accordion<'a>(cx: Scope<'a>, children: Element<'a>, summary: Element<'a>) -> Element<'a> {
    let theme = use_get_theme(cx);
    let accordion_theme = &theme.accordion;
    let animation = use_animation(cx, 0.0);
    let open = use_state(cx, || false);
    let (node_ref, size) = use_node(cx);

    let animation_value = animation.value();

    // Adapt the accordion if the body size changes
    use_effect(cx, &(size.width, size.height, animation.is_animating()), {
        to_owned![animation];
        move |(_, height, animating)| {
            if (height as f64) < animation.value() && !animating {
                animation.set_value(size.height as f64);
            }
            async move {}
        }
    });

    let onclick = move |_: MouseEvent| {
        let bodyHeight = size.height as f64;
        if *open.get() {
            animation.start(Animation::new_sine_in_out(bodyHeight..=0.0, 200));
        } else {
            animation.start(Animation::new_sine_in_out(0.0..=bodyHeight, 200));
        }
        open.set(!*open.get());
    };

    render!(
        container {
            color: "{accordion_theme.color}",
            padding: "10",
            radius: "3",
            width: "100%",
            height: "auto",
            background: "{accordion_theme.background}",
            onclick: onclick,
            summary
            container {
                width: "100%",
                height: "{animation_value}",
                rect {
                    reference: node_ref,
                    height: "auto",
                    width: "100%",
                    children
                }
            }
        }
    )
}

/// `AccordionSummary` component.
#[inline_props]
#[allow(non_snake_case)]
pub fn AccordionSummary<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    render!(children)
}

/// `AccordionBody` component.
#[inline_props]
#[allow(non_snake_case)]
pub fn AccordionBody<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    render!(rect {
        width: "100%",
        padding: "15 0 0 0",
        children
    })
}
