pub trait NodeKey: Clone + PartialEq + Eq + std::hash::Hash + Copy + std::fmt::Debug + Ord {}

impl NodeKey for usize {}
