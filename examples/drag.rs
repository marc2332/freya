use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use elements_namespace as dioxus_elements;

use freya::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let hovering = use_state(&cx, || false);
    let positions = use_state(&cx, || (0.0f64, 0.0f64));
    let clicking = use_state(&cx, || false);

    let onmouseleave = |_: UiEvent<MouseData>| {
        if *clicking.get() == false {
            hovering.set(false);
        }
    };

    let onmouseover = |e: UiEvent<MouseData>| {
        hovering.set(true);
        if *clicking.get() {
            let coordinates = e.coordinates().client();
            positions.set((coordinates.x - 50.0, coordinates.y - 50.0));
        }
    };

    let onmousedown = |_: UiEvent<MouseData>| {
        clicking.set(true);
    };

    let onclick = |_: UiEvent<MouseData>| {
        clicking.set(false);
    };

    cx.render(rsx!(
        rect {
            background: "rgb(35, 35, 35)",
            width: "100%",
            height: "100%",
            scroll_x: "{positions.0}",
            scroll_y: "{positions.1}",
            onmousedown: onmousedown,
            onclick: onclick,
            label {
                color: "white",
                "Drag me"
            }
            rect {
                background: "rgb(255, 166, 0)",
                direction: "both",
                width: "100",
                height: "100",
                radius: "15",
                shadow: "0 0 60 35 white",
                onmouseover: onmouseover,
                onmouseleave: onmouseleave
            }
        }

    ))
}
