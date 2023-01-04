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
    launch_cfg((
        app,
        WindowConfig::<()> {
            width: 400,
            height: 300,
            decorations: true,
            transparent: false,
            title: "Freya",
            state: None,
        },
    ))
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
    launch_cfg((
        app,
        WindowConfig::<()> {
            width: 400,
            height: 300,
            decorations: true,
            transparent: false,
            title,
            state: None,
        },
    ))
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
    launch_cfg((
        app,
        WindowConfig::<()> {
            width,
            height,
            decorations: true,
            transparent: false,
            title,
            state: None,
        },
    ))
}

#[cfg(not(doctest))]
/// Launch a new Window with custom options:
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
/// launch_cfg((
///     app,
///     WindowConfig::<()>::builder()
///         .with_width(500)
///         .with_height(400)
///         .with_decorations(true)
///         .with_transparency(false)
///         .with_title("Freya App")
///         .build()
/// ]);
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
pub fn launch_cfg<T: 'static + Clone + Send>(win_config: (Component<()>, WindowConfig<T>)) {
    run(win_config);
}

#[cfg(feature = "devtools")]
use dioxus::prelude::{fc_to_builder, format_args_f, render, Element, LazyNodes, Scope, VNode};
#[cfg(feature = "devtools")]
use freya_node_state::CustomAttributeValues;

#[cfg(feature = "devtools")]
fn with_devtools(
    rdom: Arc<Mutex<RealDom<NodeState, CustomAttributeValues>>>,
    root: fn(cx: Scope) -> Element,
) -> VirtualDom {
    use crate::devtools::DevTools;
    use freya_components::ThemeProvider;
    use freya_elements as dioxus_elements;

    fn app(cx: Scope<DomProps>) -> Element {
        #[allow(non_snake_case)]
        let Root = cx.props.root;

        render!(
            rect {
                width: "100%",
                height: "100%",
                direction: "horizontal",
                container {
                    height: "100%",
                    width: "calc(100% - 350)",
                    Root { },
                }
                rect {
                    background: "rgb(40, 40, 40)",
                    height: "100%",
                    width: "350",
                    ThemeProvider {
                        DevTools {
                            rdom: cx.props.rdom.clone()
                        }
                    }
                }
            }
        )
    }

    struct DomProps {
        root: fn(cx: Scope) -> Element,
        rdom: Arc<Mutex<RealDom<NodeState, CustomAttributeValues>>>,
    }

    VirtualDom::new_with_props(app, DomProps { root, rdom })
}
