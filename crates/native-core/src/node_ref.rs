//! Utilities that provide limited access to nodes

use rustc_hash::FxHashSet;

use crate::{
    node::{ElementNode, FromAnyValue, NodeType, OwnedAttributeView},
    prelude::AttributeName,
    tags::TagName,
    NodeId,
};

/// A view into a [NodeType] with a mask that determines what is visible.
#[derive(Debug)]
pub struct NodeView<'a, V: FromAnyValue = ()> {
    id: NodeId,
    inner: &'a NodeType<V>,
    mask: &'a NodeMask,
    height: u16,
}

impl<'a, V: FromAnyValue> NodeView<'a, V> {
    /// Create a new NodeView from a VNode, and mask.
    pub fn new(id: NodeId, node: &'a NodeType<V>, view: &'a NodeMask, height: u16) -> Self {
        Self {
            inner: node,
            mask: view,
            id,
            height,
        }
    }

    pub fn node_type(&self) -> &'a NodeType<V> {
        self.inner
    }

    /// Get the node id of the node
    pub fn node_id(&self) -> NodeId {
        self.id
    }

    /// Get the node height
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Get the tag of the node if the tag is enabled in the mask
    pub fn tag(&self) -> Option<&'a TagName> {
        self.mask
            .tag
            .then_some(match &self.inner {
                NodeType::Element(ElementNode { tag, .. }) => Some(tag),
                _ => None,
            })
            .flatten()
    }

    /// Get any attributes that are enabled in the mask
    pub fn attributes<'b>(
        &'b self,
    ) -> Option<impl Iterator<Item = OwnedAttributeView<'a, V>> + 'b> {
        match &self.inner {
            NodeType::Element(ElementNode { attributes, .. }) => Some(
                attributes
                    .iter()
                    .filter(move |(attr, _)| self.mask.attritutes.contains(attr))
                    .map(|(attr, val)| OwnedAttributeView {
                        attribute: attr,
                        value: val,
                    }),
            ),
            _ => None,
        }
    }

    /// Get the text if it is enabled in the mask
    pub fn text(&self) -> Option<&str> {
        self.mask.text.then_some(self.inner.text()).flatten()
    }
}

/// A mask that contains a list of attributes that are visible.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AttributeMask {
    /// All attributes are visible
    All,
    /// Only the given attributes are visible
    Some(FxHashSet<AttributeName>),
}

impl AttributeMask {
    /// Check if the mask contains the given attribute
    pub fn contains(&self, attr: &AttributeName) -> bool {
        match self {
            AttributeMask::All => true,
            AttributeMask::Some(attrs) => attrs.contains(attr),
        }
    }

    /// Combine two attribute masks
    pub fn union(&self, other: &Self) -> Self {
        match (self, other) {
            (AttributeMask::Some(s), AttributeMask::Some(o)) => {
                AttributeMask::Some(s.union(o).cloned().collect())
            }
            _ => AttributeMask::All,
        }
    }

    /// Check if two attribute masks overlap
    fn overlaps(&self, other: &Self) -> bool {
        match (self, other) {
            (AttributeMask::All, AttributeMask::Some(attrs)) => !attrs.is_empty(),
            (AttributeMask::Some(attrs), AttributeMask::All) => !attrs.is_empty(),
            (AttributeMask::Some(attrs1), AttributeMask::Some(attrs2)) => {
                !attrs1.is_disjoint(attrs2)
            }
            _ => true,
        }
    }
}

impl Default for AttributeMask {
    fn default() -> Self {
        AttributeMask::Some(FxHashSet::default())
    }
}

/// A mask that limits what parts of a node a dependency can see.
#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct NodeMask {
    attritutes: AttributeMask,
    tag: bool,
    text: bool,
    listeners: bool,
}

impl NodeMask {
    /// Check if two masks overlap
    pub fn overlaps(&self, other: &Self) -> bool {
        (self.tag && other.tag)
            || self.attritutes.overlaps(&other.attritutes)
            || (self.text && other.text)
            || (self.listeners && other.listeners)
    }

    /// Combine two node masks
    pub fn union(&self, other: &Self) -> Self {
        Self {
            attritutes: self.attritutes.union(&other.attritutes),
            tag: self.tag | other.tag,
            text: self.text | other.text,
            listeners: self.listeners | other.listeners,
        }
    }

    /// Allow the mask to view the given attributes
    pub fn add_attributes(&mut self, attributes: AttributeMask) {
        self.attritutes = self.attritutes.union(&attributes);
    }

    /// Get the mask for the attributes
    pub fn attributes(&self) -> &AttributeMask {
        &self.attritutes
    }

    /// Set the mask to view the tag
    pub fn set_tag(&mut self) {
        self.tag = true;
    }

    /// Get the mask for the tag
    pub fn tag(&self) -> bool {
        self.tag
    }

    /// Set the mask to view the text
    pub fn set_text(&mut self) {
        self.text = true;
    }

    /// Get the mask for the text
    pub fn text(&self) -> bool {
        self.text
    }

    /// Set the mask to view the listeners
    pub fn set_listeners(&mut self) {
        self.listeners = true;
    }

    /// Get the mask for the listeners
    pub fn listeners(&self) -> bool {
        self.listeners
    }
}

/// A builder for a mask that controls what attributes are visible.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AttributeMaskBuilder<'a> {
    /// All attributes are visible
    All,
    /// Only the given attributes are visible
    Some(&'a [AttributeName]),
}

impl Default for AttributeMaskBuilder<'_> {
    fn default() -> Self {
        AttributeMaskBuilder::Some(&[])
    }
}

/// A mask that limits what parts of a node a dependency can see.
#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct NodeMaskBuilder<'a> {
    attritutes: AttributeMaskBuilder<'a>,
    tag: bool,
    text: bool,
    listeners: bool,
}

impl<'a> NodeMaskBuilder<'a> {
    /// A node mask with no parts visible.
    pub const NONE: Self = Self::new();
    /// A node mask with every part visible.
    pub const ALL: Self = Self::new()
        .with_attrs(AttributeMaskBuilder::All)
        .with_text()
        .with_tag()
        .with_listeners();

    /// Create a empty node mask
    pub const fn new() -> Self {
        Self {
            attritutes: AttributeMaskBuilder::Some(&[]),
            tag: false,
            text: false,
            listeners: false,
        }
    }

    /// Allow the mask to view the given attributes
    pub const fn with_attrs(mut self, attritutes: AttributeMaskBuilder<'a>) -> Self {
        self.attritutes = attritutes;
        self
    }

    /// Allow the mask to view the tag
    pub const fn with_tag(mut self) -> Self {
        self.tag = true;
        self
    }

    /// Allow the mask to view the text
    pub const fn with_text(mut self) -> Self {
        self.text = true;
        self
    }

    /// Allow the mask to view the listeners
    pub const fn with_listeners(mut self) -> Self {
        self.listeners = true;
        self
    }

    /// Build the mask
    pub fn build(self) -> NodeMask {
        NodeMask {
            attritutes: match self.attritutes {
                AttributeMaskBuilder::All => AttributeMask::All,
                AttributeMaskBuilder::Some(attrs) => {
                    AttributeMask::Some(FxHashSet::from_iter(attrs.iter().copied()))
                }
            },
            tag: self.tag,
            text: self.text,
            listeners: self.listeners,
        }
    }
}
