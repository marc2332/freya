use dioxus_core::Element;
use freya_renderer::{
    DesktopRenderer,
    LaunchConfig,
    WindowConfig,
};

/// Launch a new window with the default config.
///
/// - Width: `600.0`
/// - Height: `600.0`
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
    launch_cfg(
        app,
        LaunchConfig::<()> {
            window_config: WindowConfig {
                size: (600.0, 600.0),
                decorations: true,
                transparent: false,
                title: "Freya",
                ..Default::default()
            },
            ..Default::default()
        },
    )
}

/// Launch a new window with a custom title and the default config.
///
/// - Width: `400`
/// - Height: `300`
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
    launch_cfg(
        app,
        LaunchConfig::<()> {
            window_config: WindowConfig {
                size: (400.0, 300.0),
                decorations: true,
                transparent: false,
                title,
                ..Default::default()
            },
            ..Default::default()
        },
    )
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
///     launch_with_props(app, "Whoa!", (400.0, 600.0));
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
pub fn launch_with_props(app: AppComponent, title: &'static str, (width, height): (f64, f64)) {
    launch_cfg(
        app,
        LaunchConfig::<()> {
            window_config: WindowConfig {
                size: (width, height),
                decorations: true,
                transparent: false,
                title,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}

/// Launch a new window with a custom config.
/// You can use a builder if you wish.
///
/// - Width
/// - Height
/// - Decorations
/// - Transparency
/// - Window title
/// - Window background color
///
/// # Example
/// ```rust,no_run
/// # use freya::prelude::*;
///
/// fn main() {
///     launch_cfg(
///         app,
///         LaunchConfig::<()>::new()
///             .with_size(500.0, 400.0)
///             .with_decorations(true)
///             .with_transparency(false)
///             .with_title("Freya App")
///             .with_background("rgb(150, 100, 200)")
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
pub fn launch_cfg<T: 'static + Clone>(app: AppComponent, config: LaunchConfig<T>) {
    #[cfg(feature = "performance-overlay")]
    let config = config.with_plugin(crate::plugins::PerformanceOverlayPlugin::default());

    use freya_core::prelude::{
        FreyaDOM,
        SafeDOM,
    };

    let fdom = FreyaDOM::default();
    let sdom = SafeDOM::new(fdom);

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

    let (vdom, devtools, hovered_node) = {
        #[cfg(feature = "devtools")]
        #[cfg(debug_assertions)]
        {
            use std::sync::{
                Arc,
                Mutex,
            };

            use freya_devtools::with_devtools;
            use freya_renderer::devtools::Devtools;

            let hovered_node = Some(Arc::new(Mutex::new(None)));
            let (devtools, devtools_receiver) = Devtools::new();
            let vdom = with_devtools(app, devtools_receiver.clone(), hovered_node.clone());
            (vdom, Some(devtools), hovered_node)
        }

        #[cfg(any(not(feature = "devtools"), not(debug_assertions)))]
        {
            let vdom = with_accessibility(app);
            (vdom, None, None)
        }
    };
    #[cfg(not(feature = "custom-tokio-rt"))]
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let _guard = rt.enter();

        DesktopRenderer::launch(vdom, sdom, config, devtools, hovered_node);
    }

    #[cfg(feature = "custom-tokio-rt")]
    DesktopRenderer::launch(vdom, sdom, config, devtools, hovered_node);
}

#[cfg(any(not(feature = "devtools"), not(debug_assertions)))]
use dioxus_core::VirtualDom;
#[cfg(any(not(feature = "devtools"), not(debug_assertions)))]
fn with_accessibility(app: AppComponent) -> VirtualDom {
    use dioxus::prelude::Props;
    use dioxus_core::fc_to_builder;
    use dioxus_core_macro::rsx;
    use freya_components::NativeContainer;

    #[derive(Props, Clone, PartialEq)]
    struct RootProps {
        app: AppComponent,
    }

    #[allow(non_snake_case)]
    fn Root(props: RootProps) -> Element {
        #[allow(non_snake_case)]
        let App = props.app;

        rsx!(NativeContainer {
            App {}
        })
    }

    VirtualDom::new_with_props(Root, RootProps { app })
}

type AppComponent = fn() -> Element;
