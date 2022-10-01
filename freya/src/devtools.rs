use dioxus::core::GlobalNodeId;
use dioxus::prelude::*;
use dioxus_core::{ElementId, Scope};
use dioxus_native_core::real_dom::{NodeType, RealDom};
use freya_components::*;
use freya_elements as dioxus_elements;
use freya_node_state::node::NodeState;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::sleep;

#[derive(PartialEq, Eq, Clone)]
struct TreeNode {
    tag: String,
    id: ElementId,
    height: u16,
    text: Option<String>,
}

#[derive(Props)]
pub struct DevToolsProps {
    rdom: Arc<Mutex<RealDom<NodeState>>>,
}

// Hacky stuff over here
impl PartialEq for DevToolsProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[allow(non_snake_case)]
pub fn DevTools(cx: Scope<DevToolsProps>) -> Element {
    let children = use_state(&cx, || Vec::<TreeNode>::new());
    let setter = children.setter();

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
                    if n.node_data.height == 2 {
                        if root_found == false {
                            root_found = true;
                        } else {
                            devtools_found = true;
                        }
                    }

                    if !devtools_found {
                        let mut maybe_text = None;
                        let tag = match &n.node_data.node_type {
                            NodeType::Text { text, .. } => {
                                maybe_text = Some(text.clone());
                                "text"
                            }
                            NodeType::Element { tag, .. } => tag,
                            NodeType::Placeholder => "placeholder",
                        }
                        .to_string();

                        let id = match n.node_data.id {
                            GlobalNodeId::VNodeId(id) => id,
                            GlobalNodeId::TemplateId {
                                template_ref_id, ..
                            } => template_ref_id,
                        };

                        children.push(TreeNode {
                            height: n.node_data.height,
                            id,
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
        ScrollView {
            width: "100%",
            height: "100%",
            padding: "30",
            show_scrollbar: true,
            children
        }
    })
}
