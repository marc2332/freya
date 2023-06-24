use dioxus_core::Component;
use freya_renderer::run_app;
use freya_renderer::{LaunchConfig, WindowConfig};

#[cfg(not(doctest))]
/// Launch a new Window with the default config.
/// - Width: `600.0`
/// - Height: `600.0`
/// - Decorations enabled
/// - Transparency disabled
/// - Window title: `Freya`
/// - Window background: white
///
/// # Example
/// ```rust
/// # use dioxus::prelude::*;
/// # use freya::{dioxus_elements, *};
/// launch(app);
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

#[cfg(not(doctest))]
/// Launch a new Window with a custom title and the default config.
/// - Width: `400`
/// - Height: `300`
/// - Decorations enabled
/// - Transparency disabled
/// - Window background: white
///
/// # Example
/// ```rust
/// # use dioxus::prelude::*;
/// # use freya::{dioxus_elements, *};
/// launch_with_title(app, "Whoah!");
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

#[cfg(not(doctest))]
/// Launch a new Window with a custom title, width and height and the default config.
/// - Decorations enabled
/// - Transparency disabled
/// - Window background: white
///
/// # Example
/// ```rust
/// # use dioxus::prelude::*;
/// # use freya::{dioxus_elements, *};
/// launch_with_props(app, "Whoah!", (400, 600));
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

#[cfg(not(doctest))]
/// Launch a new Window with custom config.
/// - Width
/// - Height
/// - Decorations
/// - Transparency
/// - Window title
/// - Window background color
///
/// # Example
/// ```rust
/// # use dioxus::prelude::*;
/// # use freya::{dioxus_elements, *};
/// launch_cfg(
///     app,
///     WindowConfig::<()>::builder()
///         .with_width(500.0)
///         .with_height(400.0)
///         .with_decorations(true)
///         .with_transparency(false)
///         .with_title("Freya App")
///         .with_background("rgb(150, 100, 200")
///         .build()
/// );
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
pub fn launch_cfg<T: 'static + Clone + Send>(root: Component, config: LaunchConfig<T>) {
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
                root,
                mutations_notifier.clone(),
                hovered_node.clone(),
            );
            (vdom, Some(mutations_notifier), hovered_node)
        }

        #[cfg(any(not(feature = "devtools"), not(debug_assertions)))]
        {
            use dioxus_core::VirtualDom;
            let vdom = VirtualDom::new(root);
            (vdom, None, None)
        }
    };
    run_app(vdom, sdom, config, mutations_notifier, hovered_node);
}
