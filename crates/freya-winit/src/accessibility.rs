use std::{
    mem,
    ops::BitOrAssign,
};

use freya_core::prelude::NavigationMode;

#[derive(Hash, PartialEq, Eq)]
pub enum AccessibilityTask {
    Init,
    ProcessUpdate { mode: Option<NavigationMode> },
    None,
}

impl AccessibilityTask {
    pub fn take(&mut self) -> Self {
        mem::replace(self, Self::None)
    }
}

impl BitOrAssign for AccessibilityTask {
    fn bitor_assign(&mut self, rhs: Self) {
        if self == &Self::None {
            *self = rhs
        }
    }
}
