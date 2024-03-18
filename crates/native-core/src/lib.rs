use std::any::Any;
use std::hash::BuildHasherDefault;

use node_ref::NodeMask;
use rustc_hash::FxHasher;

pub mod attributes;
pub mod dioxus;
pub mod events;
pub mod node;
pub mod node_ref;
mod passes;
pub mod real_dom;
pub mod tags;
pub mod tree;

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
    pub use crate::attributes::*;
    pub use crate::dioxus::*;
    pub use crate::events::*;
    pub use crate::node::{ElementNode, FromAnyValue, NodeType, OwnedAttributeView};
    pub use crate::node_ref::{AttributeMaskBuilder, NodeMaskBuilder, NodeView};
    pub use crate::passes::{run_pass, PassDirection, RunPassView, TypeErasedState};
    pub use crate::passes::{Dependancy, DependancyView, Dependants, State};
    pub use crate::real_dom::{NodeImmutable, NodeMut, NodeRef, RealDom};
    pub use crate::NodeId;
    pub use crate::SendAnyMap;
}

/// A map that can be sent between threads
pub type FxDashMap<K, V> = dashmap::DashMap<K, V, BuildHasherDefault<FxHasher>>;
/// A set that can be sent between threads
pub type FxDashSet<K> = dashmap::DashSet<K, BuildHasherDefault<FxHasher>>;
/// A map of types that can be sent between threads
pub type SendAnyMap = anymap::Map<dyn Any + Send + Sync + 'static>;
