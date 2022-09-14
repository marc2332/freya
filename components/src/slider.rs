use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use freya_elements as dioxus_elements;

#[derive(Props)]
pub struct SliderProps<'a> {
    onmoved: EventHandler<'a, f64>,
    width: f64,
}

#[allow(non_snake_case)]
pub fn Slider<'a>(cx: Scope<'a, SliderProps>) -> Element<'a> {
    let hovering = use_state(&cx, || false);
    let progress = use_state(&cx, || 0.0f64);
    let clicking = use_state(&cx, || false);

    let onmouseleave = |_: UiEvent<MouseData>| {
        if *clicking.get() == false {
            hovering.set(false);
        }
    };

    let onmouseover = |e: UiEvent<MouseData>| {
        hovering.set(true);
        if *clicking.get() {
            let coordinates = e.coordinates().element();
            let mut x = coordinates.x - 11.0;
            if x < 0.0 {
                x = 0.0;
            }
            if x > cx.props.width - 20.0 {
                x = cx.props.width - 20.0;
            }
            progress.set(x);
            cx.props.onmoved.call(x);
        }
    };

    let onmousedown = |_: UiEvent<MouseData>| {
        clicking.set(true);
    };

    let onclick = |_: UiEvent<MouseData>| {
        clicking.set(false);
    };

    cx.render(rsx!(
        container {
            background: "white",
            width: "{cx.props.width}",
            height: "20",
            scroll_x: "{progress}",
            padding: "5",
            onmousedown: onmousedown,
            onclick: onclick,
            shadow: "0 0 60 35 white",
            radius: "25",
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            rect {
                background: "rgb(255, 166, 0)",
                width: "15",
                height: "15",
                radius: "15",
            }
        }
    ))
}
