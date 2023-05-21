#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::*;
use freya::prelude::*;
use freya_devtools::*;
use freya_devtools::tabs::{style::*, tree::*, computed::*};
use freya_devtools::NodeId;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let a = cx.use_hook(|| NodeId::new_from_index_and_gen(0, 1));
    let b = cx.use_hook(|| Some(*a));

    let children = cx.use_hook(|| vec![
        TreeNode { tag: "aa".to_string(), id: *a , height: 1, text: None, state: NodeState::default(), areas: NodeAreas::default() }
    ]);

    render!(
        Button {
            label {
                "hiiiiii!"
            }
        }
        Router {
            initial_url: "freya://freya/elements".to_string(),
            DevtoolsBar {}
            Route {
                to: "/elements",
                NodesTree {
                    nodes: children,
                    height: "calc(100% - 35)",
                    selected_node_id: &None,
                    onselected: |_node: &TreeNode| {
   
                    }
                }
            }
            Route {
                to: "/elements/style",
                NodesTree {
                    nodes: children,
                    height: "calc(50% - 35)",
                    selected_node_id: b,
                    onselected: |_node: &TreeNode| {
                      
                    }
                }
                NodeInspectorStyle {
                    node: children.get(0).unwrap()
                }
            }
            Route {
                to: "/elements/computed",
                NodesTree {
                    nodes: children,
                    height: "calc(50% - 35)",
                    selected_node_id: b,
                    onselected: |node: &TreeNode| {
                       
                    }
                }
                NodeInspectorComputed {
                    node: children.get(0).unwrap()
                }
            }
        }
    )
}
