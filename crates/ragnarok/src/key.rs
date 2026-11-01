pub trait NodeKey: Clone + PartialEq + Eq + core::hash::Hash + Copy + core::fmt::Debug {}

impl NodeKey for usize {}
