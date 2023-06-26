#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render!(rect {
        width: "100%",
        height: "100%",
        display: "center",
        direction: "both",
        background: "linear-gradient(20deg, red 0%, blue 100%)" // rect {
                                                                //     padding: "12",
                                                                //     width: "200",
                                                                //     radius: "20",
                                                                //     background: "linear-gradient(-90deg, rgb(207, 119, 243) 0%, rgb(0, 155, 255) 47%, rgb(42, 201, 219) 100%)",
                                                                //     shadow: "0 4 12 linear-gradient(-90deg, rgb(207, 119, 243) 0%, rgb(0, 155, 255) 47%, rgb(42, 201, 219) 100%)",
                                                                //     color: "black",
                                                                //     padding: "12",
                                                                //     display: "center",
                                                                //     direction: "horizontal",
                                                                //     onclick: move |_| count += 1,
                                                                //     label {
                                                                //         font_weight: "600",
                                                                //         "Click to increase!"
                                                                //     }
                                                                // }
    })
}
