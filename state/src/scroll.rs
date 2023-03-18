use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State};
use dioxus_native_core::SendAnyMap;
use dioxus_native_core_macro::partial_derive_state;

use crate::CustomAttributeValues;

#[derive(Default, Clone, Debug, Component)]
pub struct Scroll {
    pub scroll_y: f32,
    pub scroll_x: f32,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for Scroll {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&["scroll_y", "scroll_x"]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let mut scroll_y = 0.0;
        let mut scroll_x = 0.0;

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "scroll_y" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            let scroll: f32 = attr.parse().unwrap();
                            scroll_y = scroll;
                        }
                    }
                    "scroll_x" => {
                        let attr = attr.value.as_text();
                        if let Some(attr) = attr {
                            let scroll: f32 = attr.parse().unwrap();
                            scroll_x = scroll;
                        }
                    }
                    _ => {
                        println!("Unsupported attribute <{}>", attr.attribute.name);
                    }
                }
            }
        }

        let changed = (scroll_x != self.scroll_x) || (scroll_y != self.scroll_y);

        *self = Self { scroll_y, scroll_x };
        changed
    }
}
