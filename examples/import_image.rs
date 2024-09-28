use freya::prelude::*;

import_image!(Ferris, "./rust_logo.png", "70%", "50%");
import_image!(FerrisWithRequiredSize, "./rust_logo.png");

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(
        Ferris { }
        FerrisWithRequiredSize {
            width: "50%",
            height: "50%",
        }
    )
}
