use std::path::PathBuf;

/// Indicates the state of the item.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpandableItemState<I, V> {
    Open(Vec<TreeItem<I, V>>),
    Closed,
}

/// Abstract the path matching.
pub trait ItemPath: PartialEq {
    fn item_starts_with(&self, other: &Self) -> bool;
}

// Implement it for PathBuf to make it easier for end users
impl ItemPath for PathBuf {
    fn item_starts_with(&self, other: &Self) -> bool {
        self.starts_with(other)
    }
}

/// Item part of a larget Tree.
///
/// `Expandable` can be expanded/open, e.g folders
/// `Standalone` cannot be expanded/opened, e.g files
#[derive(Debug, Clone, PartialEq)]
pub enum TreeItem<I, V> {
    Expandable {
        id: I,
        value: V,
        state: ExpandableItemState<I, V>,
    },
    Standalone {
        id: I,
        value: V,
    },
}

impl<I, V> TreeItem<I, V>
where
    I: ItemPath + Clone,
    V: Clone + PartialEq,
{
    /// Get the ID of the item, e.g its path.
    pub fn id(&self) -> &I {
        match self {
            Self::Expandable { id, .. } => id,
            Self::Standalone { id, .. } => id,
        }
    }

    /// Update the state of the given Expandable Item, e.g to open with more items or to simply close.
    pub fn set_state(&mut self, item_id: &I, item_state: &ExpandableItemState<I, V>) {
        if let TreeItem::Expandable { id, state, .. } = self {
            if id == item_id {
                *state = item_state.clone();
            } else if item_id.item_starts_with(id) {
                if let ExpandableItemState::Open(items) = state {
                    for item in items {
                        item.set_state(item_id, item_state);
                    }
                }
            }
        }
    }

    /// Turn all the inner items and this item itself into a flat list.
    /// This can be useful for virtualization.
    pub fn flat(&self, depth: usize, root_id: &I) -> Vec<FlatItem<I>> {
        let mut flat_items = vec![self.clone().into_flat(depth, root_id.clone())];
        if let TreeItem::Expandable {
            state: ExpandableItemState::Open(items),
            ..
        } = self
        {
            for item in items {
                let inner_items = item.flat(depth + 1, root_id);
                flat_items.extend(inner_items);
            }
        }
        flat_items
    }

    fn into_flat(self, depth: usize, root_id: I) -> FlatItem<I> {
        match self {
            TreeItem::Standalone { id, .. } => FlatItem {
                id,
                is_standalone: true,
                is_open: false,
                depth,
                root_id,
            },
            TreeItem::Expandable { id, state, .. } => FlatItem {
                id,
                is_standalone: false,
                is_open: state != ExpandableItemState::Closed,
                depth,
                root_id,
            },
        }
    }
}

/// Just like a TreeItem for flattened.
/// Use this when rendering the items.
#[derive(Clone, Debug, PartialEq)]
pub struct FlatItem<I> {
    pub id: I,
    pub is_open: bool,
    pub is_standalone: bool,
    pub depth: usize,
    pub root_id: I,
}
