use freya::prelude::*;

static FERRIS: &[u8] = include_bytes!("./ferris.svg");

#[allow(non_snake_case)]
pub fn DsSvg() -> Element {
    let image_data = static_bytes(FERRIS);

    rsx!(svg {
        width: "100%",
        height: "50%",
        svg_data: image_data,
    })
}
