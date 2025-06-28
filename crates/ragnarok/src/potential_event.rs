use crate::{
    NameOfEvent,
    NodeKey,
    SourceEvent,
};

/// Potential events are events that might get emitted or not.
#[derive(Clone, Debug, PartialEq)]
pub struct PotentialEvent<Key: NodeKey, Name: NameOfEvent, Source: SourceEvent> {
    pub node_key: Key,
    pub name: Name,
    pub source_event: Source,
    pub layer: i16,
}

impl<Key: NodeKey, Name: NameOfEvent, Source: SourceEvent> Eq
    for PotentialEvent<Key, Name, Source>
{
}

impl<Key: NodeKey, Name: NameOfEvent, Source: SourceEvent> PartialOrd
    for PotentialEvent<Key, Name, Source>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Key: NodeKey, Name: NameOfEvent, Source: SourceEvent> Ord
    for PotentialEvent<Key, Name, Source>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .layer
            .cmp(&self.layer)
            .then_with(|| self.name.cmp(&other.name))
    }
}
