use crate::event_handler::Callback;

/// Element mappers stored by a component.
#[derive(Clone, Default)]
pub struct Mappers<T>(Vec<Callback<T, T>>);

impl<T> Mappers<T> {
    /// Adds a mapper.
    pub fn push(&mut self, mapper: impl Into<Callback<T, T>>) {
        self.0.push(mapper.into());
    }

    /// Applies all mappers.
    pub fn apply(&self, mut value: T) -> T {
        for mapper in &self.0 {
            value = mapper.call(value);
        }
        value
    }
}

impl<T> PartialEq for Mappers<T> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
