use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let platform = Platform::get();

    format!("{:?}", platform.root_size.read())
}
