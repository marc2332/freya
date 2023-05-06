use freya::prelude::*;

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
fn StatefulSwitch(cx: Scope) -> Element {
    let enabled = use_state(cx, || false);

    render!(Switch {
        enabled: *enabled.get(),
        ontoggled: |_| {
            enabled.set(!enabled.get());
        }
    })
}

fn app(cx: Scope) -> Element {
    let cols = 40;
    let rows = 40;

    render!(
        container {
            width: "100%",
            height: "100%",
            padding: "2.5",
            rect {
                direction: "horizontal",
                width: "100%",
                height: "100%",
                (0..cols).map(|col| {
                    rsx! {
                        rect {
                            key: "{col}",
                            width: "calc(100% / {cols})",
                            height: "100%",
                            (0..rows).map(|row| {
                                rsx! {
                                    StatefulSwitch {
                                        key: "{row}{col}",
                                    }
                                }
                            })
                        }
                    }
                })
            }
        }
    )
}
