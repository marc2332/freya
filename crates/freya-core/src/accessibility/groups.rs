use std::ops::{
    Deref,
    DerefMut,
};

use accesskit::NodeId as AccessibilityId;
use rustc_hash::FxHashMap;

#[derive(Default)]
pub struct AccessibilityGroups {
    groups: FxHashMap<AccessibilityId, Vec<AccessibilityId>>,
}

impl Deref for AccessibilityGroups {
    type Target = FxHashMap<AccessibilityId, Vec<AccessibilityId>>;
    fn deref(&self) -> &Self::Target {
        &self.groups
    }
}

impl DerefMut for AccessibilityGroups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.groups
    }
}
