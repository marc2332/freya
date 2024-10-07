use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsLink() -> Element {
    rsx! {
        Link {
            to: "https://crates.io/crates/freya",
            label { "Freya crates.io" }
        }
    }
}
