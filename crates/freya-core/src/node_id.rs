use std::{
    num::NonZeroU64,
    ops::AddAssign,
    str::FromStr,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, PartialOrd, Ord)]
pub struct NodeId(pub NonZeroU64);

impl NodeId {
    pub const ROOT: NodeId = NodeId(NonZeroU64::MIN);
    pub const PLACEHOLDER: NodeId = NodeId(NonZeroU64::MAX);
}

impl AddAssign<u64> for NodeId {
    fn add_assign(&mut self, other: u64) {
        self.0 = self.0.saturating_add(other);
    }
}

impl torin::prelude::NodeKey for NodeId {}
impl ragnarok::NodeKey for NodeId {}

impl From<NodeId> for u64 {
    fn from(value: NodeId) -> Self {
        value.0.get()
    }
}
impl From<u64> for NodeId {
    fn from(value: u64) -> Self {
        Self(NonZeroU64::new(value).unwrap())
    }
}

impl FromStr for NodeId {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index = s.parse().map_err(|_| std::fmt::Error)?;
        Ok(Self(index))
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
