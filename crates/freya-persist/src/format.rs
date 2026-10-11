pub trait PersistenceFormat<T> {
    type Error: std::error::Error + 'static;

    /// Untyped data passed to migration callbacks, such as a JSON value.
    type Raw;

    fn decode_raw(&self, bytes: &[u8]) -> Result<Self::Raw, Self::Error>;
    fn decode_value(&self, value: Self::Raw) -> Result<T, Self::Error>;
    fn encode(&self, value: &T) -> Result<Vec<u8>, Self::Error>;

    fn decode(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        self.decode_value(self.decode_raw(bytes)?)
    }
}
