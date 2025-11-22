use std::{
    num::NonZeroU64,
    ops::AddAssign,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, PartialOrd, Ord)]
pub struct ScopeId(pub NonZeroU64);

impl ScopeId {
    pub const ROOT: ScopeId = ScopeId(NonZeroU64::MIN);
}

impl From<u64> for ScopeId {
    fn from(value: u64) -> Self {
        Self(NonZeroU64::new(value).unwrap())
    }
}

impl AddAssign<u64> for ScopeId {
    fn add_assign(&mut self, other: u64) {
        self.0 = self.0.saturating_add(other);
    }
}
