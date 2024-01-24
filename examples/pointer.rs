#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Pointer events", (200.0, 200.0));
}

fn app() -> Element {
    let onpointerdown = move |ev: PointerEvent| {
        println!("Down -> {:?}", ev.data.get_pointer_type());
    };

    let onpointerup = move |ev: PointerEvent| {
        println!("Up -> {:?}", ev.data.get_pointer_type());
    };

    let onpointerover = move |ev: PointerEvent| {
        println!("Over -> {:?}", ev.data.get_pointer_type());
    };

    let onpointerenter = move |ev: PointerEvent| {
        println!("Enter -> {:?}", ev.data.get_pointer_type());
    };

    let onpointerleave = move |ev: PointerEvent| {
        println!("Leave -> {:?}", ev.data.get_pointer_type());
    };

    rsx!(
        rect {
            overflow: "clip",
            height: "100%",
            width: "100%",
            background: "rgb(100, 100, 100)",
            padding: "12",
            rect {
                overflow: "clip",
                height: "100%",
                width: "100%",
                background: "rgb(168, 218, 220)",
                color: "black",
                padding: "12",
                onpointerdown: onpointerdown,
                onpointerup: onpointerup,
                onpointerover: onpointerover,
                onpointerenter: onpointerenter,
                onpointerleave: onpointerleave,
                label { "Click me" }
            }
        }
    )
}
