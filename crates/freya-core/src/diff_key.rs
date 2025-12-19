use std::{
    hash::Hash,
    rc::Rc,
};
#[derive(PartialEq, Hash, Eq, Clone, Debug, Default)]
pub enum DiffKey {
    Root,
    U64(u64),
    #[default]
    None,
}

impl DiffKey {
    pub fn or(self, other: DiffKey) -> Self {
        match self {
            Self::None => other,
            _ => self,
        }
    }

    pub fn new_rc<T: ?Sized>(rc: &Rc<T>) -> Self {
        use std::hash::Hasher;
        let mut hasher = std::hash::DefaultHasher::default();
        Rc::as_ptr(rc).hash(&mut hasher);
        DiffKey::U64(hasher.finish())
    }
}

impl<T: std::hash::Hash> From<&T> for DiffKey {
    fn from(value: &T) -> Self {
        use std::hash::Hasher;
        let mut hasher = std::hash::DefaultHasher::default();
        value.hash(&mut hasher);
        DiffKey::U64(hasher.finish())
    }
}

impl<A, R> From<fn(&A) -> R> for DiffKey {
    fn from(value: fn(&A) -> R) -> Self {
        Self::U64(value as *const () as u64)
    }
}

#[allow(coherence_leak_check)]
impl<A, R> From<fn(A) -> R> for DiffKey {
    fn from(value: fn(A) -> R) -> Self {
        Self::U64(value as *const () as u64)
    }
}

impl<R> From<fn() -> R> for DiffKey {
    fn from(value: fn() -> R) -> Self {
        Self::U64(value as *const () as u64)
    }
}
