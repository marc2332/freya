use freya::prelude::*;

import_svg!(Ferris, "./ferris.svg", "100%", "100%");

fn main() {
    launch(|| rsx!(Ferris {}))
}
