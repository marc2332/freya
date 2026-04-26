use freya::prelude::{
    LaunchConfig,
    WindowConfig,
    launch,
};

mod app;

fn main() {
    env_logger::init();
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app::app)
                .with_size(400., 700.)
                .with_resizable(false),
        ),
    )
}
