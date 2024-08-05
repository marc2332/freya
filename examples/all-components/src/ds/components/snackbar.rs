use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsSnackBar() -> Element {
    let mut show = use_signal(|| false);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            Button {
                onclick: move |_| show.toggle(),
                label { "Open" }
            }
            SnackBar {
                show,
                label {
                    "Hello, World!"
                }
            }
        }
    )
}
