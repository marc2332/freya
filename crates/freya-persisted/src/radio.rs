use freya_radio::prelude::{
    RadioChannel,
    RadioStation,
};

use crate::{
    Persisted,
    PersistenceError,
    PersistenceFormat,
    PersistenceMigrationError,
    PersistenceTransport,
};

pub trait PersistedRadioExt<Stored> {
    type TransportError: std::error::Error + 'static;
    type FormatError: std::error::Error + 'static;
    type Raw;

    /// Save selected state without subscribing or notifying channels.
    fn save_radio<Value, Channel>(
        &self,
        station: RadioStation<Value, Channel>,
        to: impl FnOnce(&Value) -> &Stored,
    ) -> Result<(), PersistenceError<Self::TransportError, Self::FormatError>>
    where
        Channel: RadioChannel<Value>;

    /// Merge stored data and return the channel selected by `from`.
    /// Returns `None` when storage is missing or `from` chooses silence.
    fn reload_radio<Value, Channel>(
        &self,
        station: RadioStation<Value, Channel>,
        from: impl FnOnce(&mut Value, Stored) -> Option<Channel>,
    ) -> Result<Option<Channel>, PersistenceError<Self::TransportError, Self::FormatError>>
    where
        Channel: RadioChannel<Value>;

    /// Migrate before decoding and merging, leaving storage unchanged until save.
    fn reload_radio_migrated<Value, Channel, MigrationError>(
        &self,
        station: RadioStation<Value, Channel>,
        from: impl FnOnce(&mut Value, Stored) -> Option<Channel>,
        migrate: impl FnOnce(Self::Raw) -> Result<Self::Raw, MigrationError>,
    ) -> Result<
        Option<Channel>,
        PersistenceMigrationError<Self::TransportError, Self::FormatError, MigrationError>,
    >
    where
        Channel: RadioChannel<Value>,
        MigrationError: std::error::Error + 'static;
}

impl<Stored, Transport, Format> PersistedRadioExt<Stored> for Persisted<Transport, Format>
where
    Transport: PersistenceTransport,
    Format: PersistenceFormat<Stored>,
{
    type TransportError = Transport::Error;
    type FormatError = Format::Error;
    type Raw = Format::Raw;

    fn save_radio<Value, Channel>(
        &self,
        station: RadioStation<Value, Channel>,
        to: impl FnOnce(&Value) -> &Stored,
    ) -> Result<(), PersistenceError<Self::TransportError, Self::FormatError>>
    where
        Channel: RadioChannel<Value>,
    {
        let value = station.peek();
        self.save(to(&value))
    }

    fn reload_radio<Value, Channel>(
        &self,
        mut station: RadioStation<Value, Channel>,
        from: impl FnOnce(&mut Value, Stored) -> Option<Channel>,
    ) -> Result<Option<Channel>, PersistenceError<Self::TransportError, Self::FormatError>>
    where
        Channel: RadioChannel<Value>,
    {
        let Some(stored) = self.load()? else {
            return Ok(None);
        };

        Ok(station.write_with_channel(|value| from(value, stored)))
    }

    fn reload_radio_migrated<Value, Channel, MigrationError>(
        &self,
        mut station: RadioStation<Value, Channel>,
        from: impl FnOnce(&mut Value, Stored) -> Option<Channel>,
        migrate: impl FnOnce(Self::Raw) -> Result<Self::Raw, MigrationError>,
    ) -> Result<
        Option<Channel>,
        PersistenceMigrationError<Self::TransportError, Self::FormatError, MigrationError>,
    >
    where
        Channel: RadioChannel<Value>,
        MigrationError: std::error::Error + 'static,
    {
        let Some(stored) = self.load_migrated(migrate)? else {
            return Ok(None);
        };

        Ok(station.write_with_channel(|value| from(value, stored)))
    }
}
