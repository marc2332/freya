use std::{
    collections::HashMap,
    ops::{
        Deref,
        DerefMut,
    },
};

use accesskit::NodeId as AccessibilityId;

#[derive(Default)]
pub struct AccessibilityGroups {
    groups: HashMap<AccessibilityId, Vec<AccessibilityId>>,
}

impl Deref for AccessibilityGroups {
    type Target = HashMap<AccessibilityId, Vec<AccessibilityId>>;

    fn deref(&self) -> &Self::Target {
        &self.groups
    }
}

impl DerefMut for AccessibilityGroups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.groups
    }
}
