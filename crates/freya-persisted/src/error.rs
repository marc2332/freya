#[derive(Debug, thiserror::Error)]
pub enum PersistenceError<TransportError, FormatError> {
    #[error("persistence transport failed: {0}")]
    Transport(#[source] TransportError),
    #[error("persistence format failed: {0}")]
    Format(#[source] FormatError),
}

#[derive(Debug, thiserror::Error)]
pub enum PersistenceMigrationError<TransportError, FormatError, MigrationError> {
    #[error(transparent)]
    Persisted(#[from] PersistenceError<TransportError, FormatError>),
    #[error("persistence migration failed: {0}")]
    Migration(#[source] MigrationError),
}
