use dioxus_core::Element;
use freya_core::window_config::WindowConfig;
use freya_winit::{
    LaunchConfig,
    WinitRenderer,
};

/// Launch a new window with the default config.
///
/// - Width: `700.0`
/// - Height: `500.0`
/// - Decorations enabled
/// - Transparency disabled
/// - Window title: `Freya`
/// - Window background: white
///
/// # Example
///
/// ```rust,no_run
/// # use freya::prelude::*;
///
/// fn main() {
///     launch(app);
/// }
///
/// fn app() -> Element {
///    rsx!(
///         rect {
///             width: "100%",
///             height: "100%",
///             label {
///                 "Hello World!"
///             }
///         }
///     )
/// }
/// ```
pub fn launch(app: AppComponent) {
    launch_cfg(LaunchConfig::default().with_window(WindowConfig::new(app)))
}

/// Launch a new window with a custom title and the default config.
///
/// - Width: `700`
/// - Height: `500`
/// - Decorations enabled
/// - Transparency disabled
/// - Window background: white
///
/// # Example
///
/// ```rust,no_run
/// # use freya::prelude::*;
///
/// fn main() {
///     launch_with_title(app, "Whoa!");
/// }
///
/// fn app() -> Element {
///    rsx!(
///         rect {
///             width: "100%",
///             height: "100%",
///             label {
///                 "Hello World!"
///             }
///         }
///     )
/// }
/// ```
pub fn launch_with_title(app: AppComponent, title: &'static str) {
    launch_cfg(LaunchConfig::default().with_window(WindowConfig::new(app).with_title(title)))
}

/// Launch a new window with a custom title, width and height and the default config.
///
/// - Decorations enabled
/// - Transparency disabled
/// - Window background: white
///
/// # Example
///
/// ```rust,no_run
/// # use freya::prelude::*;
///
/// fn main() {
///     launch_with_params(app, "Whoa!", (700.0, 500.0));
/// }
///
/// fn app() -> Element {
///    rsx!(
///         rect {
///             width: "100%",
///             height: "100%",
///             label {
///                 "Hello World!"
///             }
///         }
///     )
/// }
/// ```
pub fn launch_with_params(app: AppComponent, title: &'static str, (width, height): (f64, f64)) {
    launch_cfg(
        LaunchConfig::default().with_window(
            WindowConfig::new(app)
                .with_title(title)
                .with_size(width, height),
        ),
    )
}

/// Launch a new window with a custom config.
/// You can use a builder if you wish.
///
/// # Example
/// ```rust,no_run
/// # use freya::prelude::*;
///
/// fn main() {
///     launch_cfg(
///         LaunchConfig::new()
///             .with_window(WindowConfig::new(app)
///                 .with_size(700.0, 500.0)
///                 .with_decorations(true)
///                 .with_transparency(false)
///                 .with_title("Freya App")
///                 .with_background("rgb(150, 100, 200)"))
///     );
/// }
///
/// fn app() -> Element {
///    rsx!(
///         rect {
///             width: "100%",
///             height: "100%",
///             label {
///                 "Hello World!"
///             }
///         }
///     )
/// }
/// ```
pub fn launch_cfg(config: LaunchConfig) {
    #[cfg(feature = "performance-overlay")]
    let config = config.with_plugin(crate::plugins::PerformanceOverlayPlugin::default());

    #[cfg(feature = "tracing-subscriber")]
    {
        use tracing_subscriber::{
            fmt,
            prelude::__tracing_subscriber_SubscriberExt,
            util::SubscriberInitExt,
            EnvFilter,
        };

        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();
    }

    #[cfg(not(feature = "custom-tokio-rt"))]
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let _guard = rt.enter();

        WinitRenderer::launch(config);
    }

    #[cfg(feature = "custom-tokio-rt")]
    WinitRenderer::launch(config);
}

type AppComponent = fn() -> Element;
