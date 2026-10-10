use serde::{
    Serialize,
    de::DeserializeOwned,
};

use crate::PersistenceFormat;

#[derive(Clone, Copy, Debug, Default)]
pub struct PersistedJson;

impl<T: Serialize + DeserializeOwned> PersistenceFormat<T> for PersistedJson {
    type Error = serde_json::Error;
    type Raw = serde_json::Value;

    fn decode_raw(&self, bytes: &[u8]) -> Result<Self::Raw, Self::Error> {
        serde_json::from_slice(bytes)
    }

    fn decode_value(&self, value: Self::Raw) -> Result<T, Self::Error> {
        serde_json::from_value(value)
    }

    fn encode(&self, value: &T) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec_pretty(value)
    }
}
