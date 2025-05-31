#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

static VISIBLE_COMPONENT: GlobalSignal<Components> = GlobalSignal::new(|| Components::Component1);

fn main() {
    launch_with_title(app, "Conditional Rendering");
}

// this example is useful if a router is overkill, e.g. when you want to render specific components conditionally instead of whole pages
fn app() -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "white",
            rect {
                direction: "horizontal",
                Button {
                    onpress: |_| *VISIBLE_COMPONENT.write() = Components::Component1,
                    label {
                        "Component 1"
                    }
                }
                Button {
                    onpress: |_| *VISIBLE_COMPONENT.write() = Components::Component2,
                    label {
                        "Component 2"
                    }
                }
            }
            ConditionalComponent {  }
        }
    )
}

#[derive(Copy, Clone)]
enum Components {
    Component1,
    Component2,
}

#[component]
fn ConditionalComponent() -> Element {
    let visible_component = *VISIBLE_COMPONENT.read();
    match visible_component {
        Components::Component1 => rsx!(Component1 {}),
        Components::Component2 => rsx!(Component2 {}),
    }
}

#[component]
fn Component1() -> Element {
    rsx! {
        rect {
            width: "100%",
            height: "100%",
            background: "blue",
            label {
                color: "green",
                "Component 1"
            }
        }
    }
}

#[component]
fn Component2() -> Element {
    rsx! {
        rect {
            width: "100%",
            height: "100%",
            background: "red",
            label {
                color: "yellow",
                "Component 2"
            }
        }
    }
}
