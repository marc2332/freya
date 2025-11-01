use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    rect()
        .expanded()
        .center()
        .child(CircularLoader::new())
        .into()
}
