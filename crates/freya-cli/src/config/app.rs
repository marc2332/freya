use std::path::PathBuf;

use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub(crate) struct ApplicationConfig {
    /// The path where global assets will be added when components are added with `dx components add`
    #[serde(default)]
    pub(crate) asset_dir: Option<PathBuf>,

    #[serde(default)]
    pub(crate) out_dir: Option<PathBuf>,

    /// Use this file for the info.plist associated with the macOS app.
    /// `dx` will merge any required settings into this file required to build the app
    #[serde(default)]
    pub(crate) macos_info_plist: Option<PathBuf>,

    /// Use this file for the entitlements.plist associated with the macOS app.
    #[serde(default)]
    pub(crate) macos_entitlements: Option<PathBuf>,
}
