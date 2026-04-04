use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for component-related settings in the project
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub(crate) struct ComponentConfig {
    /// The path where components are stored
    #[serde(default)]
    pub(crate) components_dir: Option<PathBuf>,
}
