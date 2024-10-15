use freya::prelude::*;

import_svg!(Ferris, "./ferris.svg", "70%", "50%");
import_svg!(FerrisWithRequiredSize, "./ferris.svg");

fn main() {
    launch(app)
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
