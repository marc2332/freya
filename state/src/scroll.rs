use std::sync::{Arc, Mutex};

use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::ParentDepState;
use dioxus_native_core_macro::sorted_str_slice;
use freya_layout_common::LayoutMemorizer;

#[derive(Default, Clone)]
pub struct Scroll {
    pub scroll_y: f32,
    pub scroll_x: f32,
    pub id: usize,
}

// TODO(marc2332) Why use ParentDepState? NodeDepState might make more sense
impl ParentDepState for Scroll {
    type Ctx = Arc<Mutex<LayoutMemorizer>>;
    type DepState = Self;

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "scroll_y", "scroll_x",
        ])))
        .with_text()
        .with_tag();

    fn reduce<'a>(
        &mut self,
        node: NodeView,
        _parent: Option<&'a Self::DepState>,
        ctx: &Self::Ctx,
    ) -> bool {
        let mut scroll_y = 0.0;
        let mut scroll_x = 0.0;

        for attr in node.attributes() {
            match attr.name {
                "scroll_y" => {
                    let scroll: f32 = attr.value.to_string().parse().unwrap();
                    scroll_y = scroll;
                }
                "scroll_x" => {
                    let scroll: f32 = attr.value.to_string().parse().unwrap();
                    scroll_x = scroll;
                }
                _ => {
                    println!("Unsupported attribute <{}>", attr.name);
                }
            }
        }

        let changed = (scroll_x != self.scroll_x) || (scroll_y != self.scroll_y);

        if changed {
            ctx.lock().unwrap().mark_as_dirty(node.id());
        }

        *self = Self {
            scroll_y,
            scroll_x,
            id: node.id().0,
        };
        changed
    }
}
