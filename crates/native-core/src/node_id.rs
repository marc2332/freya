use std::{
    ops::Deref,
    str::FromStr,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Debug, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NodeId(shipyard::EntityId);

impl Deref for NodeId {
    type Target = shipyard::EntityId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<NodeId> for shipyard::EntityId {
    fn from(value: NodeId) -> Self {
        value.0
    }
}

impl From<shipyard::EntityId> for NodeId {
    fn from(value: shipyard::EntityId) -> Self {
        NodeId(value)
    }
}

impl FromStr for NodeId {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((index, gen)) = s.split_once('-') else {
            return Err(std::fmt::Error);
        };
        let index = index.parse().map_err(|_| std::fmt::Error)?;
        let gen = gen.parse().map_err(|_| std::fmt::Error)?;
        Ok(Self(shipyard::EntityId::new_from_index_and_gen(index, gen)))
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}-{}", self.index(), self.gen()))
    }
}

impl torin::prelude::NodeKey for NodeId {}

impl ragnarok::NodeKey for NodeId {}
