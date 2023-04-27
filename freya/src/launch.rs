use dioxus_core::Component;
use freya_renderer::run;
use freya_renderer::WindowConfig;

#[cfg(not(doctest))]
/// Launch a new Window with the default config.
/// - Width: `400.0`
/// - Height: `300.0`
/// - Decorations enabled
/// - Transparency disabled
/// - Window title: `Freya`
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
        WindowConfig::<()> {
            width: 400.0,
            height: 300.0,
            decorations: true,
            transparent: false,
            title: "Freya",
            state: None,
        },
    )
}

#[cfg(not(doctest))]
/// Launch a new Window with a custom title and the default config.
/// - Width: `400`
/// - Height: `300`
/// - Decorations enabled
/// - Transparency disabled
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
        WindowConfig::<()> {
            width: 400.0,
            height: 300.0,
            decorations: true,
            transparent: false,
            title,
            state: None,
        },
    )
}

#[cfg(not(doctest))]
/// Launch a new Window with a custom title, width and height and the default config.
/// - Decorations enabled
/// - Transparency disabled
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
        WindowConfig::<()> {
            width,
            height,
            decorations: true,
            transparent: false,
            title,
            state: None,
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
pub fn launch_cfg<T: 'static + Clone + Send>(root: Component, win_config: WindowConfig<T>) {
    use freya_dom::{FreyaDOM, SafeDOM};

    let fdom = FreyaDOM::default();
    let sdom = SafeDOM::new(fdom);

    let (vdom, mutations_sender, hovered_node) = {
        #[cfg(feature = "devtools")]
        #[cfg(debug_assertions)]
        {
            use freya_devtools::with_devtools;
            use std::sync::{Arc, Mutex};
            use tokio::sync::mpsc::unbounded_channel;

            let hovered_node = Some(Arc::new(Mutex::new(None)));
            let (mutations_sender, mutations_receiver) = unbounded_channel::<()>();
            let vdom = with_devtools(sdom.clone(), root, mutations_receiver, hovered_node.clone());
            (vdom, Some(mutations_sender), hovered_node)
        }

        #[cfg(any(not(feature = "devtools"), not(debug_assertions)))]
        {
            use dioxus_core::VirtualDom;
            let vdom = VirtualDom::new(root);
            (vdom, None, None)
        }
    };
    run(vdom, sdom, win_config, mutations_sender, hovered_node);
}
