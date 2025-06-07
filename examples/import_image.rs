use freya::prelude::*;

import_image!(RustLogo, "./rust_logo.png", {
    width: "auto",
    height: "auto",
    sampling: "trilinear",
    aspect_ratio: "min",
});

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(RustLogo {})
}
