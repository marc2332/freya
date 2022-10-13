use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use fermi::use_atom_ref;
use freya_elements as dioxus_elements;
use freya_hooks::{use_animation, AnimationMode};

use crate::THEME;

#[derive(Props)]
pub struct SwitchProps<'a> {
    pub enabled: bool,
    pub ontoggled: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Switch<'a>(cx: Scope<'a, SwitchProps<'a>>) -> Element<'a> {
    let (start_enabled, restart_enabled, progress_enabled) =
        use_animation(&cx, || AnimationMode::new_sine_in_out(0.0..=25.0, 200));
    let (start_disabled, restart_disabled, progress_disabled) =
        use_animation(&cx, || AnimationMode::new_sine_in_out(25.0..=0.0, 200));
    let theme = use_atom_ref(&cx, THEME);
    let theme = theme.read();
    let hovering = use_state(&cx, || false);
    let clicking = use_state(&cx, || false);

    let onmouseleave = |_: UiEvent<MouseData>| {
        if *clicking.get() == false {
            hovering.set(false);
        }
    };

    let onmouseover = |_: UiEvent<MouseData>| {
        hovering.set(true);
    };

    let onmousedown = |_: UiEvent<MouseData>| {
        clicking.set(true);
    };

    let onclick = |_: UiEvent<MouseData>| {
        clicking.set(false);
        cx.props.ontoggled.call(());
    };

    use_effect(&cx, &cx.props.enabled, move |enabled| async move {
        if enabled {
            start_enabled();
            restart_disabled();
        } else {
            start_disabled();
            restart_enabled();
        }
    });

    let (scroll_x, border, circle) = {
        if cx.props.enabled {
            (
                progress_enabled,
                theme.switch.enabled_background,
                theme.switch.enabled_thumb_background,
            )
        } else {
            (
                progress_disabled,
                theme.switch.background,
                theme.switch.thumb_background,
            )
        }
    };

    render!(
        container {
            width: "auto",
            height: "auto",
            direction: "both",
            padding: "3",
            rect {
                width: "50",
                height: "25",
                padding: "2",
                radius: "50",
                background: "{border}",
                onmousedown: onmousedown,
                onmouseover: onmouseover,
                onmouseleave: onmouseleave,
                onclick: onclick,
                rect {
                    width: "100%",
                    height: "100%",
                    scroll_x: "{scroll_x}",
                    padding: "5",
                    radius: "50",
                    rect {
                        background: "{circle}",
                        direction: "both",
                        width: "18",
                        height: "18",
                        radius: "50",
                    }
                }
            }
        }
    )
}
