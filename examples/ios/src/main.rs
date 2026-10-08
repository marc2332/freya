use freya::prelude::{
    LaunchConfig,
    WindowConfig,
    launch,
};

#[path = "../../android/src/app/mod.rs"]
mod app;

fn main() {
    env_logger::init();

    #[cfg(target_os = "ios")]
    let window = WindowConfig::new(app::app);

    #[cfg(not(target_os = "ios"))]
    let window = WindowConfig::new(app::app)
        .with_size(400., 700.)
        .with_resizable(false);

    let config = LaunchConfig::new().with_window(window);

    #[cfg(target_os = "ios")]
    let config = config.with_plugin(freya::ios::IosPlugin::default());

    launch(config)
}
