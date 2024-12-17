use std::any::{
    Any,
    TypeId,
};

use node_ref::NodeMask;

pub mod attributes;
pub mod dioxus;
pub mod events;
pub mod node;
pub mod node_ref;
mod passes;
pub mod real_dom;
pub mod tags;
pub mod tree;

use rustc_hash::FxHashMap;
pub use shipyard::EntityId as NodeId;

pub mod exports {
    //! Important dependencies that are used by the rest of the library
    //! Feel free to just add the dependencies in your own Crates.toml
    // exported for the macro
    #[doc(hidden)]
    pub use rustc_hash::FxHashSet;
    pub use shipyard;
}

/// A prelude of commonly used items
pub mod prelude {
    pub use crate::{
        attributes::*,
        dioxus::*,
        events::*,
        node::{
            ElementNode,
            FromAnyValue,
            NodeType,
            OwnedAttributeView,
        },
        node_ref::{
            AttributeMaskBuilder,
            NodeMaskBuilder,
            NodeView,
        },
        passes::{
            run_pass,
            Dependancy,
            DependancyView,
            Dependants,
            PassDirection,
            RunPassView,
            State,
            TypeErasedState,
        },
        real_dom::{
            NodeImmutable,
            NodeMut,
            NodeRef,
            RealDom,
        },
        NodeId,
        SendAnyMap,
    };
}

/// A map of types that can be sent between threads
#[derive(Debug)]
pub struct SendAnyMap {
    map: FxHashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>,
}

impl Default for SendAnyMap {
    fn default() -> Self {
        Self::new()
    }
}

impl SendAnyMap {
    pub fn new() -> Self {
        Self {
            map: FxHashMap::default(),
        }
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|any| any.downcast_ref::<T>())
    }

    pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }
}
