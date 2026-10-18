#![no_std]

//! Keyed tree diffing with FIFO matching for repeated sibling keys.
//!
//! Paths identify nodes by child index from the root. Movement parents use new-tree
//! paths, while movement pairs contain old and new child indices. Removals use old
//! paths, additions and modifications use new paths. Removal order is unspecified.
//! The caller applies the resulting diff and owns node lifetimes.
//!
//! Requires `alloc`. The optional `std` feature selects standard-library maps for
//! interoperability. Maps use Fx hashing, which is not intended for adversarial keys.

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod collections;

use core::hash::Hash;

use crate::collections::{
    Box,
    HashMap,
    Vec,
    VecDeque,
};

/// A tree node with stable sibling keys and index-based paths.
/// Children must expose paths extending their parent's path by their child index.
pub trait DiffNode: Sized {
    type Key: Eq + Hash;
    type Modifications;

    fn key(&self) -> &Self::Key;
    fn path(&self) -> &[u32];
    fn children(&self) -> &[Self];

    /// Whether these nodes have compatible kinds, independently of their keys.
    fn compatible(&self, previous: &Self) -> bool;

    /// Returns changes for compatible nodes with matching keys, or `None` if unchanged.
    fn modifications(&self, previous: &Self) -> Option<Self::Modifications>;

    /// Checks application-specific sibling key rules before matching children in debug builds.
    #[cfg(debug_assertions)]
    fn validate_children(&self) {}
}

/// Changes between two trees, without applying them.
#[derive(Debug)]
pub struct Diff<Modifications> {
    pub added: Vec<Box<[u32]>>,
    pub modified: Vec<(Box<[u32]>, Modifications)>,
    pub removed: Vec<Box<[u32]>>,
    pub moved: HashMap<Box<[u32]>, Vec<(u32, u32)>>,
}

impl<Modifications> Default for Diff<Modifications> {
    fn default() -> Self {
        Self {
            added: Vec::new(),
            modified: Vec::new(),
            removed: Vec::new(),
            moved: HashMap::default(),
        }
    }
}

/// Appends changes from `previous` to `current` into `diff`.
/// Matching roots must have paths of equal depth and corresponding parents.
pub fn diff<Node: DiffNode>(
    current: &Node,
    previous: Option<&Node>,
    diff: &mut Diff<Node::Modifications>,
) {
    diff_inner(current, previous, diff, false);
}

fn diff_inner<Node: DiffNode>(
    current: &Node,
    previous: Option<&Node>,
    diff: &mut Diff<Node::Modifications>,
    parent_removed: bool,
) {
    // Add new nodes and their descendants in preorder.
    let Some(previous) = previous else {
        diff.added.push(current.path().into());
        for child in current.children() {
            diff_inner(child, None, diff, false);
        }
        return;
    };

    // Replace incompatible node kinds without matching their descendants.
    if !current.compatible(previous) {
        diff.removed.push(previous.path().into());
        diff_inner(current, None, diff, false);
        return;
    }

    // Propagate replacement to matched descendants when a key or ancestor changes.
    let removed_here = current.key() != previous.key() || parent_removed;
    if removed_here {
        diff.added.push(current.path().into());
        diff.removed.push(previous.path().into());
    } else {
        if let Some(modifications) = current.modifications(previous) {
            diff.modified.push((current.path().into(), modifications));
        }
        // Record index changes under the new parent path, independently of modifications.
        if let (Some((&new_index, parent)), Some(&old_index)) =
            (current.path().split_last(), previous.path().last())
            && new_index != old_index
        {
            diff.moved
                .entry(parent.into())
                .or_default()
                .push((old_index, new_index));
        }
    }

    #[cfg(debug_assertions)]
    current.validate_children();
    let current_children = current.children();
    let previous_children = previous.children();
    // Match the common prefix positionally without building a key map.
    let mut start = 0;
    while start < current_children.len() && start < previous_children.len() {
        if current_children[start].key() != previous_children[start].key() {
            break;
        }
        diff_inner(
            &current_children[start],
            Some(&previous_children[start]),
            diff,
            removed_here,
        );
        start += 1;
    }

    if start < current_children.len() || start < previous_children.len() {
        // Queue remaining old indices by key to preserve FIFO matching for duplicates.
        let mut previous_keys = HashMap::<&Node::Key, VecDeque<usize>>::default();
        for (index, child) in previous_children.iter().enumerate().skip(start) {
            previous_keys
                .entry(child.key())
                .or_default()
                .push_back(index);
        }
        // Reuse the earliest unmatched old sibling or add the new node and its descendants.
        for child in &current_children[start..] {
            if let Some(old_index) = previous_keys
                .get_mut(child.key())
                .and_then(VecDeque::pop_front)
            {
                diff_inner(
                    child,
                    Some(&previous_children[old_index]),
                    diff,
                    removed_here,
                );
            } else {
                diff_inner(child, None, diff, false);
            }
        }
        // Remove unmatched branch roots, leaving descendant cleanup to the caller.
        for indices in previous_keys.values() {
            for index in indices {
                diff.removed.push(previous_children[*index].path().into());
            }
        }
    }
}
