use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsArrowIcon() -> Element {
    rsx!(ArrowIcon {
        fill: "black",
        rotate: "0"
    })
}

#[allow(non_snake_case)]
pub fn DsCrossIcon() -> Element {
    rsx!(CrossIcon { fill: "black" })
}

#[allow(non_snake_case)]
pub fn DsTickIcon() -> Element {
    rsx!(TickIcon { fill: "black" })
}
