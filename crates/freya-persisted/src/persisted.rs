use crate::{
    PersistenceError,
    PersistenceFormat,
    PersistenceMigrationError,
    PersistenceTransport,
};

#[derive(Clone)]
pub struct Persisted<Transport, Format> {
    transport: Transport,
    format: Format,
}

impl<Transport: PersistenceTransport, Format> Persisted<Transport, Format> {
    pub fn new(transport: Transport, format: Format) -> Self {
        Self { transport, format }
    }

    /// Load a value, returning `None` only when the transport has no persisted data.
    pub fn load<T>(&self) -> Result<Option<T>, PersistenceError<Transport::Error, Format::Error>>
    where
        Format: PersistenceFormat<T>,
    {
        self.transport
            .load()
            .map_err(PersistenceError::Transport)?
            .map(|bytes| self.format.decode(&bytes).map_err(PersistenceError::Format))
            .transpose()
    }

    /// Transform the format's raw value before typed decoding without overwriting storage.
    pub fn load_migrated<T, MigrationError: std::error::Error + 'static>(
        &self,
        migrate: impl FnOnce(Format::Raw) -> Result<Format::Raw, MigrationError>,
    ) -> Result<Option<T>, PersistenceMigrationError<Transport::Error, Format::Error, MigrationError>>
    where
        Format: PersistenceFormat<T>,
    {
        let Some(bytes) = self.transport.load().map_err(PersistenceError::Transport)? else {
            return Ok(None);
        };

        let raw = self
            .format
            .decode_raw(&bytes)
            .map_err(PersistenceError::Format)?;
        let raw = migrate(raw).map_err(PersistenceMigrationError::Migration)?;

        self.format
            .decode_value(raw)
            .map(Some)
            .map_err(PersistenceError::Format)
            .map_err(PersistenceMigrationError::Persisted)
    }

    /// Encode and save a value without requiring reactive state.
    pub fn save<T>(
        &self,
        value: &T,
    ) -> Result<(), PersistenceError<Transport::Error, Format::Error>>
    where
        Format: PersistenceFormat<T>,
    {
        let bytes = self
            .format
            .encode(value)
            .map_err(PersistenceError::Format)?;

        self.transport
            .save(&bytes)
            .map_err(PersistenceError::Transport)
    }
}
