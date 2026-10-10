pub trait PersistenceTransport {
    type Error: std::error::Error + 'static;

    /// Return `None` when no persisted data exists.
    fn load(&self) -> Result<Option<Vec<u8>>, Self::Error>;

    /// Replace the stored data.
    fn save(&self, bytes: &[u8]) -> Result<(), Self::Error>;
}
