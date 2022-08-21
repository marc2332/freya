use dioxus::prelude::*;
use elements_namespace as dioxus_elements;
use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let sizes = use_state(&cx, || (50, 50, 50, 50));

    cx.render(rsx! {
        view {
            height: "stretch",
            width: "stretch",
            direction: "horizontal",
            padding: "30",
            view {
                width: "{sizes.0}%",
                height: "stretch",
                layer: "1",
                view {
                    background: "red",
                    height: "{sizes.2}%",
                    width: "stretch",
                    padding: "30",
                    layer: "1",
                    onclick: |_| sizes.with_mut(|v| {
                        v.0 += 5;
                        v.1 -= 5;
                        v.2 += 5;
                        v.3 -= 5;
                    }),
                    text {
                        layer: "1",
                        "Click to increase",
                    }
                }
                view {
                    background: "green",
                    height: "{sizes.3}%",
                    width: "stretch",
                    padding: "30",
                    layer: "1",
                    onclick: |_| sizes.with_mut(|v| {
                        v.0 += 5;
                        v.1 -= 5;
                        v.2 -= 5;
                        v.3 += 5;
                    }),
                    text {
                        layer: "1",
                        "Click to increase",
                    }
                }
            }
            view {
                width: "{sizes.1}%",
                height: "stretch",
                layer: "1",
                view {
                    background: "blue",
                    height: "{sizes.2}%",
                    width: "stretch",
                    padding: "30",
                    layer: "1",
                    onclick: |_| sizes.with_mut(|v| {
                        v.0 -= 5;
                        v.1 += 5;
                        v.2 += 5;
                        v.3 -= 5;
                    }),
                    text {
                        layer: "1",
                        "Click to increase",
                    }
                }
                view {
                    background: "black",
                    height: "{sizes.3}%",
                    width: "stretch",
                    padding: "30",
                    layer: "1",
                    onclick: |_| sizes.with_mut(|v| {
                        v.0 -= 5;
                        v.1 += 5;
                        v.2 -= 5;
                        v.3 += 5;
                    }),
                    text {
                        layer: "1",
                        "Click to increase",
                    }
                }
            }
        }
    })
}
