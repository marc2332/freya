use std::{
    hash::Hash,
    ops::{
        Deref,
        DerefMut,
    },
};

/// Capture values to use later inside Queries or Mutations, but with a catch, if the capture value changes the mutation will not recapture it because
/// the [PartialEq] implementation always returns `true`.
///
/// So in other words `Capture(1) == Capture(5)` will be `true`.
///
/// **This is intended to use for value types that are not mean to be diffed and that are expected to maintain their value across time.
/// Like "Clients" of external resources such as API Clients.**
///
/// Just so its clear, you might or not need this, use carefully.
#[derive(Clone)]
pub struct Captured<T: Clone>(pub T);

impl<T: Clone> Hash for Captured<T> {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<T: Clone> PartialEq for Captured<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T: Clone> Eq for Captured<T> {}

impl<T: Clone> Deref for Captured<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone> DerefMut for Captured<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
