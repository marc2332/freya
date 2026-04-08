use std::fs;

use serde::{
    Deserialize,
    Serialize,
};
use tracing::{
    trace,
    warn,
};

/// Describes cli settings from project or global level.
/// The order of priority goes:
/// 1. CLI Flags/Arguments
/// 2. Project-level Settings
/// 3. Global-level settings.
///
/// This allows users to control the cli settings with ease.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct CliSettings {
    /// Describes whether hot reload should always be on.
    pub(crate) always_hot_reload: Option<bool>,
    /// Describes whether the CLI should always open the browser for Web targets.
    pub(crate) always_open_browser: Option<bool>,
    /// Describes whether desktop apps in development will be pinned always-on-top.
    pub(crate) always_on_top: Option<bool>,
    /// Describes the interval in seconds that the CLI should poll for file changes on WSL.
    #[serde(default = "default_wsl_file_poll_interval")]
    pub(crate) wsl_file_poll_interval: Option<u16>,
    /// Use tooling from path rather than downloading them.
    pub(crate) no_downloads: Option<bool>,
    /// Ignore updates for this version
    pub(crate) ignore_version_update: Option<String>,
    /// Disable telemetry
    pub(crate) disable_telemetry: Option<bool>,
}

impl CliSettings {
    pub fn global_or_default() -> Self {
        CliSettings::from_global().unwrap_or_default()
    }

    /// Get the current settings structure from global.
    pub(crate) fn from_global() -> Option<Self> {
        let settings = crate::Workspace::global_settings_file();

        if !settings.exists() {
            trace!("global settings file does not exist, returning None");
            return None;
        }

        let Some(data) = fs::read_to_string(&settings).ok() else {
            warn!("failed to read global settings file");
            return None;
        };

        let Some(data) = toml::from_str::<CliSettings>(&data).ok() else {
            warn!("failed to parse global settings file");
            return None;
        };

        Some(data)
    }
}

fn default_wsl_file_poll_interval() -> Option<u16> {
    Some(2)
}
