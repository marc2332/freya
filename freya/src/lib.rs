use anymap::AnyMap;
use dioxus::prelude::*;
use dioxus_core::SchedulerMsg;
use dioxus_native_core::real_dom::RealDom;
use freya_node_state::node::NodeState;
use freya_renderer::run;
use std::sync::Arc;
use std::sync::Mutex;

pub use freya_components::*;
pub use freya_elements as dioxus_elements;
pub use freya_hooks::*;
pub use freya_renderer::*;

#[cfg(not(doctest))]
/// Launch a new Window with the default config:
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
///     cx.render(rsx!(
///         rect {
///             width: "100%",
///             height: "100%",
///             label {
///                 "Hello World!"
///             }
///         }
///     ))
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
/// Launch a new Window with a custom title and the default config:
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
///     cx.render(rsx!(
///         rect {
///             width: "100%",
///             height: "100%",
///             label {
///                 "Hello World!"
///             }
///         }
///     ))
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
///
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
///     cx.render(rsx!(
///         rect {
///             width: "100%",
///             height: "100%",
///             label {
///                 "Hello World!"
///             }
///         }
///     ))
/// }
/// ```
pub fn launch_cfg(wins_config: Vec<(Component<()>, WindowConfig)>) {
    let wins = wins_config
        .into_iter()
        .map(|(root, win)| {
            let rdom = Arc::new(Mutex::new(RealDom::<NodeState>::new()));
            let event_emitter: Arc<Mutex<Option<UnboundedSender<SchedulerMsg>>>> =
                Arc::new(Mutex::new(None));

            {
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
                    let ctx = AnyMap::new();

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
                                let ctx = AnyMap::new();
                                rdom.lock().unwrap().update_state(&dom, to_update, ctx);
                            }
                        });
                });
            }
            (rdom, event_emitter, win.clone())
        })
        .collect();

    run(wins);
}

#[cfg(feature = "devtools")]
fn with_devtools(
    rdom: Arc<Mutex<RealDom<NodeState>>>,
    root: fn(cx: Scope) -> Element,
) -> VirtualDom {
    use dioxus::core::ElementId;
    use dioxus_native_core::real_dom::NodeType;
    use std::time::Duration;
    use tokio::time::sleep;

    #[derive(PartialEq, Eq, Clone)]
    struct TreeNode {
        tag: String,
        id: ElementId,
        height: u16,
        text: Option<String>,
    }

    fn app<'a>(cx: Scope<'a, DomProps>) -> Element<'a> {
        let children = use_state(&cx, || Vec::<TreeNode>::new());
        let setter = children.setter();

        #[allow(non_snake_case)]
        let Root = cx.props.root;

        use_effect(&cx, (), move |_| {
            let rdom = cx.props.rdom.clone();
            async move {
                loop {
                    sleep(Duration::from_millis(25)).await;

                    let rdom = rdom.lock().unwrap();
                    let mut children = Vec::new();

                    let mut root_found = false;
                    let mut devtools_found = false;

                    rdom.traverse_depth_first(|n| {
                        if n.height == 2 {
                            if root_found == false {
                                root_found = true;
                            } else {
                                devtools_found = true;
                            }
                        }

                        if !devtools_found {
                            let mut maybe_text = None;
                            let tag = match &n.node_type {
                                NodeType::Text { text, .. } => {
                                    maybe_text = Some(text.clone());
                                    "text"
                                }
                                NodeType::Element { tag, .. } => tag,
                                NodeType::Placeholder => "placeholder",
                            }
                            .to_string();

                            children.push(TreeNode {
                                height: n.height,
                                id: n.id,
                                tag,
                                text: maybe_text,
                            });
                        }
                    });
                    setter(children);
                }
            }
        });

        let children = children.get().iter().map(|node| {
            let text = node
                .text
                .as_ref()
                .map(|v| format!("({v})"))
                .unwrap_or_default();
            rsx! {
                rect {
                    width: "100%",
                    height: "25",
                    scroll_x: "{node.height * 10}",
                    label {
                        "{node.tag} #{node.id} {text}"
                    }
                }
            }
        });

        cx.render(rsx! {
            rect {
                width: "100%",
                height: "100%",
                direction: "horizontal",
                rect {
                    height: "100%",
                    width: "75%",
                    Root { },
                }
                rect {
                    background: "rgb(40, 40, 40)",
                    height: "100%",
                    width: "25%",
                    label {
                        height: "25",
                        "Devtools!"
                    }
                    ScrollView {
                        height: "calc(100% - 25)",
                        show_scrollbar: true,
                        children
                    }
                }
            }
        })
    }

    struct DomProps {
        root: fn(cx: Scope) -> Element,
        rdom: Arc<Mutex<RealDom<NodeState>>>,
    }

    VirtualDom::new_with_props(
        app,
        DomProps {
            root: root,
            rdom: rdom.clone(),
        },
    )
}
