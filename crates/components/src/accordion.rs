use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;

use freya_hooks::{
    use_animation_with_dependencies, use_applied_theme, use_node, use_platform, AccordionTheme,
    AccordionThemeWith, AnimNum,
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

/// Properties for the [`Accordion`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// Theme override.
    pub theme: Option<AccordionThemeWith>,
    /// Inner children for the Accordion.
    pub children: Element,
    /// Summary element.
    pub summary: Element,
}

/// Show other elements under a collapsable box.
///
/// # Styling
/// Inherits the [`AccordionTheme`](freya_hooks::AccordionTheme)
///
#[allow(non_snake_case)]
pub fn Accordion(props: AccordionProps) -> Element {
    let theme = use_applied_theme!(&props.theme, accordion);
    let mut open = use_signal(|| false);
    let (node_ref, size) = use_node();

    let animation = use_animation_with_dependencies(&size.area.height(), move |ctx, height| {
        ctx.with(AnimNum::new(0., height).time(200))
    });
    let mut status = use_signal(AccordionStatus::default);
    let platform = use_platform();

    let animation_value = animation.get().read().as_f32();
    let AccordionTheme {
        background,
        color,
        border_fill,
    } = theme;

    let onclick = move |_: MouseEvent| {
        open.toggle();
        if *open.read() {
            animation.start();
        } else {
            animation.reverse();
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

/// Properties for the [`AccordionSummary`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionSummaryProps {
    /// Inner children for the AccordionSummary.
    children: Element,
}

/// Intended to use as summary for an [`Accordion`].
#[allow(non_snake_case)]
pub fn AccordionSummary(props: AccordionSummaryProps) -> Element {
    rsx!({ props.children })
}

/// Properties for the [`AccordionBody`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionBodyProps {
    /// Inner children for the AccordionBody.
    children: Element,
}

/// Intended to wrap the body of an [`Accordion`].
#[allow(non_snake_case)]
pub fn AccordionBody(props: AccordionBodyProps) -> Element {
    rsx!(rect {
        width: "100%",
        padding: "15 0 0 0",
        {props.children}
    })
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use freya::prelude::*;
    use freya_testing::prelude::*;
    use tokio::time::sleep;
    use winit::event::MouseButton;

    #[tokio::test]
    pub async fn accordion() {
        fn accordion_app() -> Element {
            rsx!(
                Accordion {
                    summary: rsx!(AccordionSummary {
                        label {
                            "Accordion Summary"
                        }
                    }),
                    AccordionBody {
                        label {
                            "Accordion Body"
                        }
                    }
                }
            )
        }

        let mut utils = launch_test(accordion_app);

        let root = utils.root();
        let content = root.get(0).get(1).get(0);
        let label = content.get(0);
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        // Accordion is closed, therefore label is hidden.
        assert!(!label.is_visible());

        // Click on the accordion
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (5., 5.).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;

        // State somewhere in the middle
        sleep(Duration::from_millis(70)).await;
        utils.wait_for_update().await;

        // Accordion is open, therefore label is visible.
        assert!(label.is_visible());
    }
}
