use dioxus_core::Component;
use freya_renderer::run;
use freya_renderer::WindowConfig;

#[cfg(not(doctest))]
/// Launch a new Window with the default config.
/// - Width: `400`
/// - Height: `300`
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
            width: 400,
            height: 300,
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
            width: 400,
            height: 300,
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
pub fn launch_with_props(app: Component<()>, title: &'static str, (width, height): (u32, u32)) {
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
///         .with_width(500)
///         .with_height(400)
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
    use dioxus_native_core::real_dom::RealDom;
    use freya_node_state::{CustomAttributeValues, NodeState};

    let rdom = RealDom::<NodeState, CustomAttributeValues>::new();
    let (vdom, mutations_sender, hovered_node) = {
        use dioxus_core::VirtualDom;
        let vdom = VirtualDom::new(root);
        (vdom, None, None)
    };
    run(vdom, rdom, win_config, mutations_sender, hovered_node);
}
