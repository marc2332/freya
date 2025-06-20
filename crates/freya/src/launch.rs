use dioxus_core::{
    Element,
    VirtualDom,
};
use freya_winit::{
    devtools::{
        DevtoolsReceiver,
        HighlightedNode,
    },
    LaunchConfig,
    WindowConfig,
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
    launch_cfg(
        app,
        LaunchConfig::<()> {
            window_config: WindowConfig {
                size: (700.0, 500.0),
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
    launch_cfg(
        app,
        LaunchConfig::<()> {
            window_config: WindowConfig {
                size: (700.0, 500.0),
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
///     launch_with_props(app, "Whoa!", (700.0, 500.0));
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
///             .with_size(700.0, 500.0)
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

    use freya_core::dom::{
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

    use dioxus::prelude::Props;
    use dioxus_core::{
        fc_to_builder,
        IntoDynNode,
    };
    use dioxus_core_macro::rsx;
    #[cfg(debug_assertions)]
    use dioxus_signals::{
        GlobalSignal,
        Readable,
    };
    use freya_components::NativeContainer;
    use freya_winit::devtools::Devtools;

    #[derive(Props, Clone)]
    struct RootProps {
        app: AppComponent,
        highlighted_node: Option<HighlightedNode>,
        devtools_receiver: Option<DevtoolsReceiver>,
    }
    impl PartialEq for RootProps {
        fn eq(&self, _other: &Self) -> bool {
            true
        }
    }

    #[allow(non_snake_case)]
    fn Root(props: RootProps) -> Element {
        #[allow(non_snake_case)]
        let App = props.app;

        rsx!(
            NativeContainer {
                {
                    #[cfg(all(feature = "devtools", debug_assertions))]
                    rsx!(
                        freya_devtools::DevtoolsView {
                            highlighted_node: props.highlighted_node.unwrap(),
                            devtools_receiver: props.devtools_receiver.unwrap(),
                            App {}
                        }
                    )
                }
                {
                    #[cfg(any(not(feature = "devtools"), not(debug_assertions)))]
                    rsx!(
                        App {}
                    )
                }
            }
        )
    }

    #[cfg(all(feature = "devtools", debug_assertions))]
    let devtools = Some(Devtools::new());

    #[cfg(any(not(feature = "devtools"), not(debug_assertions)))]
    let devtools: Option<(Devtools, DevtoolsReceiver, HighlightedNode)> = None;

    let vdom = VirtualDom::new_with_props(
        Root,
        RootProps {
            app,
            devtools_receiver: devtools.as_ref().map(|d| d.1.clone()),
            highlighted_node: devtools.as_ref().map(|d| d.2.clone()),
        },
    );

    #[cfg(not(feature = "custom-tokio-rt"))]
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let _guard = rt.enter();

        WinitRenderer::launch(vdom, sdom, config, devtools.map(|d| d.0));
    }

    #[cfg(feature = "custom-tokio-rt")]
    WinitRenderer::launch(vdom, sdom, config, devtools.map(|d| d.0), hovered_node);
}

type AppComponent = fn() -> Element;
