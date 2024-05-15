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
    let cols = 30;
    let rows = 30;

    rsx!(
        for row in 0..rows {
            rect {
                key: "{row}",
                width: "100%",
                height: "{(100.0 / rows as f32)}%",
                direction: "horizontal",
                for col in 0..cols {
                    rect {
                        width: "{(100.0 / cols as f32)}%",
                        key: "{row}{col}",
                        StatefulSwitch { }
                    }
                }
            }
        }
    )
}
