use std::ops::Deref;

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

impl NodeId {
    pub fn serialize(&self) -> String {
        format!("{}-{}", self.index(), self.gen())
    }

    pub fn deserialize(node_id: &str) -> Self {
        let (index, gen) = node_id.split_once('-').unwrap();
        Self(shipyard::EntityId::new_from_index_and_gen(
            index.parse().unwrap(),
            gen.parse().unwrap(),
        ))
    }
}

impl torin::prelude::NodeKey for NodeId {}

impl ragnarok::NodeKey for NodeId {}
