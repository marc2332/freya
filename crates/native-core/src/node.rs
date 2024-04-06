//! Items related to Nodes in the RealDom

use rustc_hash::{FxHashMap, FxHashSet};
use shipyard::Component;
use std::{any::Any, fmt::Debug};

use crate::{events::EventName, prelude::AttributeName, tags::TagName};

/// A element node in the RealDom
#[derive(Debug, Clone)]
pub struct ElementNode<V: FromAnyValue = ()> {
    /// The tag name of the element
    pub tag: TagName,
    /// The attributes of the element
    pub attributes: FxHashMap<AttributeName, OwnedAttributeValue<V>>,
    /// The events the element is listening for
    pub listeners: FxHashSet<EventName>,
}

/// A type of node with data specific to the node type.
#[derive(Debug, Clone, Component)]
pub enum NodeType<V: FromAnyValue = ()> {
    /// A text node
    Text(String),
    /// An element node
    Element(ElementNode<V>),
    /// A placeholder node. This can be used as a cheaper placeholder for a node that will be created later
    Placeholder,
}

impl<V: FromAnyValue> NodeType<V> {
    pub fn is_visible_element(&self) -> bool {
        if let NodeType::Element(ElementNode { tag, .. }) = self {
            // No need to consider text spans
            if tag.has_intrinsic_layout() {
                return true;
            }
        }

        false
    }

    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text(..))
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Self::Element(..))
    }

    pub fn is_placeholder(&self) -> bool {
        matches!(self, Self::Placeholder)
    }

    pub fn tag(&self) -> Option<&TagName> {
        match self {
            Self::Element(ElementNode { tag, .. }) => Some(tag),
            _ => None,
        }
    }

    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text.as_str()),
            _ => None,
        }
    }
}

impl<V: FromAnyValue, S: Into<String>> From<S> for NodeType<V> {
    fn from(text: S) -> Self {
        Self::Text(text.into())
    }
}

impl<V: FromAnyValue> From<ElementNode<V>> for NodeType<V> {
    fn from(element: ElementNode<V>) -> Self {
        Self::Element(element)
    }
}

/// An attribute on a DOM node, such as `id="my-thing"` or
/// `href="https://example.com"`.
#[derive(Clone, Copy, Debug)]
pub struct OwnedAttributeView<'a, V: FromAnyValue = ()> {
    /// The discription of the attribute.
    pub attribute: &'a AttributeName,

    /// The value of the attribute.
    pub value: &'a OwnedAttributeValue<V>,
}

/// The value of an attribute on a DOM node. This contains non-text values to allow users to skip parsing attribute values in some cases.
#[derive(Clone)]
pub enum OwnedAttributeValue<V: FromAnyValue = ()> {
    /// A string value. This is the most common type of attribute.
    Text(String),
    /// A floating point value.
    Float(f64),
    /// An integer value.
    Int(i64),
    /// A boolean value.
    Bool(bool),
    /// A custom value specific to the renderer
    Custom(V),
}

impl<V: FromAnyValue> From<String> for OwnedAttributeValue<V> {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl<V: FromAnyValue> From<f64> for OwnedAttributeValue<V> {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl<V: FromAnyValue> From<i64> for OwnedAttributeValue<V> {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl<V: FromAnyValue> From<bool> for OwnedAttributeValue<V> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl<V: FromAnyValue> From<V> for OwnedAttributeValue<V> {
    fn from(value: V) -> Self {
        Self::Custom(value)
    }
}

/// Something that can be converted from a borrowed [Any] value.
pub trait FromAnyValue: Clone + 'static {
    /// Convert from an [Any] value.
    fn from_any_value(value: &dyn Any) -> Self;
}

impl FromAnyValue for () {
    fn from_any_value(_: &dyn Any) -> Self {}
}

impl<V: FromAnyValue> Debug for OwnedAttributeValue<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(arg0) => f.debug_tuple("Text").field(arg0).finish(),
            Self::Float(arg0) => f.debug_tuple("Float").field(arg0).finish(),
            Self::Int(arg0) => f.debug_tuple("Int").field(arg0).finish(),
            Self::Bool(arg0) => f.debug_tuple("Bool").field(arg0).finish(),
            Self::Custom(_) => f.debug_tuple("Any").finish(),
        }
    }
}

impl<V: FromAnyValue> From<&dioxus_core::AttributeValue> for OwnedAttributeValue<V> {
    fn from(value: &dioxus_core::AttributeValue) -> Self {
        match value {
            dioxus_core::AttributeValue::Text(text) => Self::Text(text.clone()),
            dioxus_core::AttributeValue::Float(float) => Self::Float(*float),
            dioxus_core::AttributeValue::Int(int) => Self::Int(*int),
            dioxus_core::AttributeValue::Bool(bool) => Self::Bool(*bool),
            dioxus_core::AttributeValue::Any(any) => Self::Custom(V::from_any_value(any.as_any())),
            dioxus_core::AttributeValue::None => panic!("None attribute values result in removing the attribute, not converting it to a None value."),
            _ => panic!("Unsupported attribute value type"),
        }
    }
}

impl<V: FromAnyValue> OwnedAttributeValue<V> {
    /// Attempt to convert the attribute value to a string.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            OwnedAttributeValue::Text(text) => Some(text),
            _ => None,
        }
    }

    /// Attempt to convert the attribute value to a float.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            OwnedAttributeValue::Float(float) => Some(*float),
            OwnedAttributeValue::Int(int) => Some(*int as f64),
            _ => None,
        }
    }

    /// Attempt to convert the attribute value to an integer.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            OwnedAttributeValue::Float(float) => Some(*float as i64),
            OwnedAttributeValue::Int(int) => Some(*int),
            _ => None,
        }
    }

    /// Attempt to convert the attribute value to a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            OwnedAttributeValue::Bool(bool) => Some(*bool),
            _ => None,
        }
    }

    /// Attempt to convert the attribute value to a custom value.
    pub fn as_custom(&self) -> Option<&V> {
        match self {
            OwnedAttributeValue::Custom(custom) => Some(custom),
            _ => None,
        }
    }
}
