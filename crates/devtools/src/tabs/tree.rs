use crate::{node::NodeElement, state::DevtoolsChannel, NodeIdSerializer, Route};
use dioxus::prelude::*;
use dioxus_radio::prelude::use_radio;
use dioxus_router::prelude::{router, use_navigator};
use freya_components::*;
use freya_hooks::{theme_with, ScrollViewThemeWith};
use freya_native_core::NodeId;

#[allow(non_snake_case)]
#[component]
pub fn NodesTree(
    height: String,
    selected_node_id: Option<NodeId>,
    onselected: EventHandler<NodeId>,
) -> Element {
    let navigator = use_navigator();
    let radio = use_radio(DevtoolsChannel::UpdatedDOM);

    rsx!(VirtualScrollView {
        show_scrollbar: true,
        length: radio.read().devtools_receiver.borrow().len(),
        item_size: 27.0,
        theme: theme_with!(ScrollViewTheme {
            height: height.to_string().into(),
            padding: "15".into(),
        }),
        builder_args: selected_node_id,
        builder: move |i, selected_node_id: &Option<Option<NodeId>>| {
            let radio = radio.read();
            let node = radio.devtools_receiver.borrow().get(i).cloned().unwrap();
            let node_id = node.id;
            to_owned![onselected];
            rsx! {
                NodeElement {
                    key: "{node_id:?}",
                    is_selected: Some(node_id) == selected_node_id.flatten(),
                    onselected: move |node_id: NodeId| {
                        onselected.call(node_id);
                        match router().current() {
                            Route::NodeInspectorLayout { .. } => {
                                navigator.replace(Route::NodeInspectorLayout { node_id: node_id.serialize() });
                            }
                            _ => {
                                navigator.replace(Route::NodeInspectorStyle { node_id: node_id.serialize() });
                            }
                        }
                    },
                    node_id
                }
            }
        }
    })
}
