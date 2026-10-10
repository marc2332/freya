use freya_core::prelude::WritableUtils;

use crate::{
    Persisted,
    PersistenceError,
    PersistenceFormat,
    PersistenceTransport,
};

pub trait PersistedStateExt<Value: 'static> {
    type TransportError: std::error::Error + 'static;
    type FormatError: std::error::Error + 'static;

    /// Replace and notify state after a successful load, returning whether data existed.
    fn reload_state(
        &self,
        state: &mut impl WritableUtils<Value>,
    ) -> Result<bool, PersistenceError<Self::TransportError, Self::FormatError>>;

    /// Save state without subscribing to its changes.
    fn save_state(
        &self,
        state: &impl WritableUtils<Value>,
    ) -> Result<(), PersistenceError<Self::TransportError, Self::FormatError>>;
}

impl<Value: 'static, Transport, Format> PersistedStateExt<Value> for Persisted<Transport, Format>
where
    Transport: PersistenceTransport,
    Format: PersistenceFormat<Value>,
{
    type TransportError = Transport::Error;
    type FormatError = Format::Error;

    fn reload_state(
        &self,
        state: &mut impl WritableUtils<Value>,
    ) -> Result<bool, PersistenceError<Self::TransportError, Self::FormatError>> {
        let Some(value) = self.load()? else {
            return Ok(false);
        };

        state.set(value);
        Ok(true)
    }

    fn save_state(
        &self,
        state: &impl WritableUtils<Value>,
    ) -> Result<(), PersistenceError<Self::TransportError, Self::FormatError>> {
        self.save(&*state.peek_state())
    }
}
