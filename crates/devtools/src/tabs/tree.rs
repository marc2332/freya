use crate::{node::NodeElement, NodeIdSerializer, Route, TreeNode};
use dioxus::prelude::*;
use dioxus_native_core::NodeId;
use dioxus_router::prelude::use_navigator;
use freya_components::*;
use freya_hooks::{theme_with, ScrollViewThemeWith};

#[allow(non_snake_case)]
#[component]
pub fn NodesTree(
    height: String,
    selected_node_id: Option<NodeId>,
    onselected: EventHandler<TreeNode>,
) -> Element {
    let router = use_navigator();
    let nodes = use_context::<Signal<Vec<TreeNode>>>();

    rsx!(VirtualScrollView {
        show_scrollbar: true,
        length: nodes.read().len(),
        item_size: 27.0,
        theme: theme_with!(ScrollViewTheme {
            height: height.to_string().into(),
            padding: "15".into(),
        }),
        builder_args: selected_node_id,
        builder: move |i, selected_node_id: &Option<Option<NodeId>>| {
            let nodes = nodes.read();
            let node = nodes.get(i).cloned().unwrap();
            to_owned![onselected];
            rsx! {
                NodeElement {
                    key: "{node.id:?}",
                    is_selected: Some(node.id) == selected_node_id.flatten(),
                    onselected: move |node: TreeNode| {
                        onselected.call(node.clone());
                        router.replace(Route::TreeStyleTab { node_id: node.id.serialize() });
                    },
                    node: node
                }
            }
        }
    })
}
