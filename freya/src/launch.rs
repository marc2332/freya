use anymap::AnyMap;
use dioxus::prelude::{Component, UnboundedSender, VirtualDom};
use dioxus_core::SchedulerMsg;
use dioxus_native_core::real_dom::RealDom;
use freya_node_state::NodeState;
use freya_renderer::run;
use freya_renderer::WindowConfig;
use std::sync::Arc;
use std::sync::Mutex;

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
    launch_cfg(vec![(
        app,
        WindowConfig {
            width: 400,
            height: 300,
            decorations: true,
            transparent: false,
            title: "Freya",
        },
    )])
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
    launch_cfg(vec![(
        app,
        WindowConfig {
            width: 400,
            height: 300,
            decorations: true,
            transparent: false,
            title,
        },
    )])
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
    launch_cfg(vec![(
        app,
        WindowConfig {
            width,
            height,
            decorations: true,
            transparent: false,
            title,
        },
    )])
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
/// launch_cfg(vec![(
///     app,
///     WindowConfig {
///         width: 500,
///         height: 400,
///         decorations: true,
///         transparent: false,
///         title: "Freya Window"
///     }
/// )]);
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
pub fn launch_cfg(wins_config: Vec<(Component<()>, WindowConfig)>) {
    use freya_common::LayoutMemorizer;

    let wins = wins_config
        .into_iter()
        .map(|(root, win)| {
            let rdom = Arc::new(Mutex::new(RealDom::<NodeState>::new()));
            let event_emitter: Arc<Mutex<Option<UnboundedSender<SchedulerMsg>>>> =
                Arc::new(Mutex::new(None));

            let layout_memorizer = Arc::new(Mutex::new(LayoutMemorizer::new()));

            {
                let layout_memorizer = layout_memorizer.clone();
                let rdom = rdom.clone();
                let event_emitter = event_emitter.clone();
                std::thread::spawn(move || {
                    let mut dom = {
                        #[cfg(feature = "devtools")]
                        {
                            with_devtools(rdom.clone(), root)
                        }

                        #[cfg(not(feature = "devtools"))]
                        {
                            VirtualDom::new(root)
                        }
                    };

                    let muts = dom.rebuild();
                    let to_update = rdom.lock().unwrap().apply_mutations(vec![muts]);
                    let mut ctx = AnyMap::new();

                    ctx.insert(layout_memorizer.clone());

                    rdom.lock().unwrap().update_state(&dom, to_update, ctx);

                    event_emitter
                        .lock()
                        .unwrap()
                        .replace(dom.get_scheduler_channel());

                    tokio::runtime::Builder::new_multi_thread()
                        .enable_all()
                        .build()
                        .unwrap()
                        .block_on(async move {
                            loop {
                                dom.wait_for_work().await;
                                let mutations = dom.work_with_deadline(|| false);

                                let to_update = rdom.lock().unwrap().apply_mutations(mutations);

                                let mut ctx = AnyMap::new();
                                ctx.insert(layout_memorizer.clone());
                                if !to_update.is_empty() {
                                    rdom.lock().unwrap().update_state(&dom, to_update, ctx);
                                }
                            }
                        });
                });
            }
            (rdom, event_emitter, layout_memorizer, win)
        })
        .collect();

    run(wins);
}
#[cfg(feature = "devtools")]
use dioxus::prelude::{
    fc_to_builder, format_args_f, render, Element, LazyNodes, NodeFactory, Scope, VNode,
};

#[cfg(feature = "devtools")]
fn with_devtools(
    rdom: Arc<Mutex<RealDom<NodeState>>>,
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
        rdom: Arc<Mutex<RealDom<NodeState>>>,
    }

    VirtualDom::new_with_props(app, DomProps { root, rdom })
}
