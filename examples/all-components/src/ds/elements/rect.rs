use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsRect() -> Element {
    rsx!(rect {
        background: "green",
        width: "150",
        height: "150"
    })
}
