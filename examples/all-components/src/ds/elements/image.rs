use freya::prelude::*;

static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");

#[allow(non_snake_case)]
pub fn DsImage() -> Element {
    let image_data = static_bytes(RUST_LOGO);

    rsx!(image {
        image_data: image_data,
        width: "250",
        height: "250",
    })
}
