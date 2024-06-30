use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsProgressBar() -> Element {
    rsx!(ProgressBar { progress: 75.0 })
}

#[allow(non_snake_case)]
pub fn DsLoader() -> Element {
    rsx!(Loader {})
}
