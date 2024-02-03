use dioxus_core::Component;
use freya_renderer::DesktopRenderer;
use freya_renderer::{LaunchConfig, WindowConfig};

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
/// fn app(cx: Scope) -> Element {
///    render!(
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
pub fn launch(app: Component<()>) {
    launch_cfg(
        app,
        LaunchConfig {
            window: WindowConfig::<()> {
                width: 600.0,
                height: 600.0,
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
/// fn app(cx: Scope) -> Element {
///    render!(
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
pub fn launch_with_title(app: Component<()>, title: &'static str) {
    launch_cfg(
        app,
        LaunchConfig {
            window: WindowConfig::<()> {
                width: 400.0,
                height: 300.0,
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
/// fn app(cx: Scope) -> Element {
///    render!(
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
pub fn launch_with_props(app: Component<()>, title: &'static str, (width, height): (f64, f64)) {
    launch_cfg(
        app,
        LaunchConfig {
            window: WindowConfig::<()> {
                width,
                height,
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
///         LaunchConfig::<()>::builder()
///             .with_width(500.0)
///             .with_height(400.0)
///             .with_decorations(true)
///             .with_transparency(false)
///             .with_title("Freya App")
///             .with_background("rgb(150, 100, 200")
///             .build()
///     );
/// }
///
/// fn app(cx: Scope) -> Element {
///    render!(
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
pub fn launch_cfg<T: 'static + Clone + Send>(app: Component, config: LaunchConfig<T>) {
    use freya_dom::prelude::{FreyaDOM, SafeDOM};

    let fdom = FreyaDOM::default();
    let sdom = SafeDOM::new(fdom);

    #[cfg(feature = "log")]
    {
        use tracing::Level;
        use tracing_subscriber::FmtSubscriber;

        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Setting default subscriber failed");
    }

    let (vdom, mutations_notifier, hovered_node) = {
        #[cfg(feature = "devtools")]
        #[cfg(debug_assertions)]
        {
            use freya_devtools::with_devtools;
            use std::sync::{Arc, Mutex};
            use tokio::sync::Notify;

            let hovered_node = Some(Arc::new(Mutex::new(None)));
            let mutations_notifier = Arc::new(Notify::new());
            let vdom = with_devtools(
                sdom.clone(),
                app,
                mutations_notifier.clone(),
                hovered_node.clone(),
            );
            (vdom, Some(mutations_notifier), hovered_node)
        }

        #[cfg(any(not(feature = "devtools"), not(debug_assertions)))]
        {
            let vdom = with_accessibility(app);
            (vdom, None, None)
        }
    };
    DesktopRenderer::launch(vdom, sdom, config, mutations_notifier, hovered_node);
}

#[cfg(any(not(feature = "devtools"), not(debug_assertions)))]
use dioxus_core::VirtualDom;
#[cfg(any(not(feature = "devtools"), not(debug_assertions)))]
fn with_accessibility(app: Component) -> VirtualDom {
    use dioxus_core::fc_to_builder;
    use dioxus_core::{Element, Scope};
    use dioxus_core_macro::render;
    use freya_hooks::use_init_accessibility;

    struct RootProps {
        app: Component,
    }

    #[allow(non_snake_case)]
    fn Root(cx: Scope<RootProps>) -> Element {
        use_init_accessibility(cx);

        #[allow(non_snake_case)]
        let App = cx.props.app;

        render!(App {})
    }

    VirtualDom::new_with_props(Root, RootProps { app })
}
