use freya::{
    performance::PerformanceOverlayPlugin,
    prelude::{LaunchConfig, WindowConfig, launch},
};

mod app;

fn main() {
    env_logger::init();
    launch(
        LaunchConfig::new()
            .with_plugin(PerformanceOverlayPlugin::default().with_visible(true))
            .with_window(
                WindowConfig::new(app::app)
                    .with_size(500., 450.)
                    .with_resizable(false),
            ),
    )
}
