use dioxus::prelude::*;
use elements_namespace as dioxus_elements;
use trev::launch;

fn main() {
    launch(app);
}

static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");

fn app(cx: Scope) -> Element {
    // TODO(marc2332): Make the image element accept bytes
    let image_data: Vec<String> = RUST_LOGO.to_vec().iter().map(|b| format!("{b}")).collect();
    let image_data = image_data.join(",");

    cx.render(rsx!(image {
        data: "{image_data}"
    }))
}
