use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    let platform_state = PlatformState::get();

    format!("{:?}", platform_state.root_size.read()).into()
}
