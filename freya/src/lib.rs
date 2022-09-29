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
    use std::{sync::MutexGuard, thread::sleep, time::Duration};

    use dioxus::core::ElementId;
    use dioxus_native_core::real_dom::{Node, NodeType};

    let wins = wins_config
        .into_iter()
        .map(|(root, win)| {
            #[derive(PartialEq, Eq, Clone)]
            struct CustomNode {
                tag: String,
                id: ElementId,
                height: u16,
            }

            #[derive(Props, Clone)]
            struct NOp {
                node: CustomNode,
                rdom: Arc<Mutex<RealDom<NodeState>>>,
            }
            use dioxus_native_core::traversable::Traversable;
            use std::ops::Index;

            impl PartialEq for NOp {
                fn eq(&self, other: &Self) -> bool {
                    self.node == other.node
                }
            }

            fn N(cx: Scope<NOp>) -> Element {
                let childr = use_state(&cx, || {
                    Vec::<(CustomNode, Arc<Mutex<RealDom<NodeState>>>)>::new()
                });
                let set_childr = childr.setter();
                let rdom = cx.props.rdom.clone();
                let id = cx.props.node.id;
                println!("->{id:?}");

                {
                    let rdom = rdom.clone();
                    let c_rdom = rdom.clone();
                    cx.spawn(async move {
                        loop {
                            sleep(Duration::from_millis(1000));
                            println!(">>{:?}", id);
                            let mut node_children =
                                Vec::<(CustomNode, Arc<Mutex<RealDom<NodeState>>>)>::new();
                            let rdom = &rdom.lock().unwrap();
                            if let Some(node) = rdom.get(id) {
                                if let NodeType::Element { children, .. } = &node.node_type {
                                    println!(">>{:?}", children);
                                    for child_id in children {
                                        let child = rdom.index(*child_id).clone();
                                        let mut n_tag = "text".to_string();
                                        if let NodeType::Element { tag, .. } = &child.node_type {
                                            n_tag = tag.to_string();
                                        }

                                        node_children.push((
                                            CustomNode {
                                                tag: n_tag.to_string(),
                                                height: 1,
                                                id: *child_id,
                                            },
                                            c_rdom.clone(),
                                        ));
                                    }
                                }
                            }
                            println!("___{:?}", node_children.len());
                            set_childr(node_children);
                        }
                    });
                }

                let childr = childr.get();
                println!("??{:?}", childr.len());

                cx.render(rsx! {
                    label {
                        "{cx.props.node.tag}"
                    }
                    childr.iter().map(|(n, rdom)| {
                        rsx! {
                            N {
                                rdom: rdom.clone(),
                                node: n.clone()
                            }
                        }
                    })
                })
            }

            fn app<'a>(cx: Scope<'a, DomProps>) -> Element<'a> {
                let Root = cx.props.root;
                let rdom = cx.props.rdom.clone();

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
                            width: "100%",
                            label {
                                "Devtools!"
                                N {
                                    rdom: rdom.clone(),
                                    node: CustomNode {
                                        height:0,
                                        tag: "root".to_string(),
                                        id: ElementId(0)
                                    }
                                }
                            }
                        }
                    }
                })
            }

            struct DomProps {
                root: fn(cx: Scope) -> Element,
                rdom: Arc<Mutex<RealDom<NodeState>>>,
            }

            let rdom = Arc::new(Mutex::new(RealDom::<NodeState>::new()));
            let event_emitter: Arc<Mutex<Option<UnboundedSender<SchedulerMsg>>>> =
                Arc::new(Mutex::new(None));

            {
                let rdom = rdom.clone();
                let event_emitter = event_emitter.clone();
                std::thread::spawn(move || {
                    let mut dom = VirtualDom::new_with_props(
                        app,
                        DomProps {
                            root: root,
                            rdom: rdom.clone(),
                        },
                    );

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
