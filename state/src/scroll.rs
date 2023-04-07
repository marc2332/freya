use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::NodeDepState;
use dioxus_native_core_macro::sorted_str_slice;
use freya_common::LayoutNotifier;

use crate::CustomAttributeValues;

#[derive(Default, Clone, Debug)]
pub struct Scroll {
    pub scroll_y: f32,
    pub scroll_x: f32,
}

impl NodeDepState<CustomAttributeValues> for Scroll {
    type Ctx = LayoutNotifier;
    type DepState = ();

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "scroll_y", "scroll_x",
        ])))
        .with_text()
        .with_tag();

    fn reduce<'a>(
        &mut self,
        node: NodeView<CustomAttributeValues>,
        _sibling: (),
        ctx: &Self::Ctx,
    ) -> bool {
        let mut scroll_y = 0.0;
        let mut scroll_x = 0.0;

        if let Some(attributes) = node.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "scroll_y" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            let scroll: f32 = attr.parse().unwrap();
                            scroll_y = scroll * 2.0;
                        }
                    }
                    "scroll_x" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            let scroll: f32 = attr.parse().unwrap();
                            scroll_x = scroll * 2.0;
                        }
                    }
                    _ => {
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
                }
            }
        }

        let changed = (scroll_x != self.scroll_x) || (scroll_y != self.scroll_y);

        if changed {
            *ctx.lock().unwrap() = true;
        }

        *self = Self { scroll_y, scroll_x };
        changed
    }
}
