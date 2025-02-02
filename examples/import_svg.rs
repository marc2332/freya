use freya::prelude::*;

import_svg!(Ferris, "./ferris.svg", {
    width: "70%",
    height: "50%"
});

fn main() {
    launch(app)
}

fn app() -> Element {
    rsx!(Ferris {})
}
