use freya::prelude::*;

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
fn StatefulSwitch() -> Element {
    let mut enabled = use_signal(|| false);

    rsx!(Switch {
        enabled: *enabled.read(),
        ontoggled: move |_| {
            enabled.toggle();
        }
    })
}

fn app() -> Element {
    let cols = 100;
    let rows = 100;

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            padding: "2.5",
            rect {
                direction: "horizontal",
                width: "100%",
                height: "100%",
                {(0..cols).map(|col| {
                    rsx! {
                        rect {
                            key: "{col}",
                            width: "calc(100% / {cols})",
                            height: "100%",
                            {(0..rows).map(|row| {
                                rsx! {
                                    StatefulSwitch {
                                        key: "{row}{col}",
                                    }
                                }
                            })}
                        }
                    }
                })}
            }
        }
    )
}
