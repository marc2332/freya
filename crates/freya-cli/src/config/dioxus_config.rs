use crate::config::component::ComponentConfig;

use super::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub(crate) struct DioxusConfig {
    #[serde(default)]
    pub(crate) application: ApplicationConfig,

    #[serde(default)]
    pub(crate) bundle: BundleConfig,

    #[serde(default)]
    pub(crate) components: ComponentConfig,

    /// Unified permissions configuration.
    /// Permissions declared here are automatically mapped to platform-specific
    /// identifiers (AndroidManifest.xml, Info.plist, etc.)
    #[serde(default)]
    pub(crate) permissions: PermissionsConfig,

    /// Unified deep linking configuration.
    /// URL schemes and universal links declared here are mapped to platform-specific
    /// configurations. Use `[ios]`, `[android]`, `[macos]` sections for overrides.
    #[serde(default)]
    pub(crate) deep_links: DeepLinkConfig,

    /// Unified background mode configuration.
    /// Background capabilities declared here are mapped to platform-specific
    /// configurations. Use `[ios]`, `[android]` sections for overrides.
    #[serde(default)]
    pub(crate) background: BackgroundConfig,

    /// macOS-specific configuration.
    #[serde(default)]
    pub(crate) macos: MacosConfig,

    /// Windows-specific configuration.
    #[serde(default)]
    pub(crate) windows: WindowsConfig,

    /// Linux-specific configuration.
    #[serde(default)]
    pub(crate) linux: LinuxConfig,

    /// iOS stub - not supported in Freya desktop
    #[serde(default)]
    pub(crate) ios: IosConfigStub,

    /// Android stub - not supported in Freya desktop  
    #[serde(default)]
    pub(crate) android: AndroidConfigStub,
}

/// Platform identifier for bundle resolution.
/// This is separate from the CLI's Platform enum which includes Server and Unknown variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BundlePlatform {
    Ios,
    Android,
    MacOS,
    Windows,
    Linux,
    Web,
}

impl From<crate::BundleFormat> for BundlePlatform {
    fn from(format: crate::BundleFormat) -> Self {
        match format {
            crate::BundleFormat::Ios => BundlePlatform::Ios,
            crate::BundleFormat::Android => BundlePlatform::Android,
            crate::BundleFormat::MacOS => BundlePlatform::MacOS,
            crate::BundleFormat::Windows => BundlePlatform::Windows,
            crate::BundleFormat::Linux => BundlePlatform::Linux,
        }
    }
}

impl DioxusConfig {
    /// Get the resolved bundle identifier for a specific platform.
    /// Platform-specific identifiers override the base bundle identifier.
    pub fn resolved_identifier(&self, platform: BundlePlatform) -> Option<&str> {
        let platform_override = match platform {
            BundlePlatform::Ios => self.ios.identifier.as_deref(),
            BundlePlatform::Android => self.android.identifier.as_deref(),
            BundlePlatform::MacOS => self.macos.identifier.as_deref(),
            BundlePlatform::Windows => self.windows.identifier.as_deref(),
            BundlePlatform::Linux => self.linux.identifier.as_deref(),
            BundlePlatform::Web => None,
        };
        platform_override.or(self.bundle.identifier.as_deref())
    }
}

impl Default for DioxusConfig {
    fn default() -> Self {
        Self {
            application: ApplicationConfig {
                asset_dir: None,
                out_dir: None,
                public_dir: Some("public".into()),
                tailwind_input: None,
                tailwind_output: None,
                ios_info_plist: None,
                android_manifest: None,
                android_main_activity: None,
                android_min_sdk_version: None,
                macos_info_plist: None,
                ios_entitlements: None,
                macos_entitlements: None,
            },
            bundle: BundleConfig::default(),
            components: ComponentConfig::default(),
            permissions: PermissionsConfig::default(),
            deep_links: DeepLinkConfig::default(),
            background: BackgroundConfig::default(),
            macos: MacosConfig::default(),
            windows: WindowsConfig::default(),
            linux: LinuxConfig::default(),
            ios: IosConfigStub::default(),
            android: AndroidConfigStub::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_dir_defaults_to_public() {
        let config = DioxusConfig::default();
        assert_eq!(
            config.application.public_dir,
            Some(std::path::PathBuf::from("public"))
        );
    }

    #[test]
    fn static_dir_can_be_overridden() {
        let source = r#"
            [application]
            public_dir = "public2"
        "#;

        let config: DioxusConfig = toml::from_str(source).expect("parse config");
        assert_eq!(
            config.application.public_dir.as_deref(),
            Some(std::path::Path::new("public2"))
        );
    }

    #[test]
    fn static_dir_can_be_disabled() {
        let source = r#"
            [application]
            public_dir = ""
        "#;

        let config: DioxusConfig = toml::from_str(source).expect("parse config");
        assert_eq!(config.application.public_dir.as_deref(), None);
    }
}

/// iOS configuration stub (not supported in Freya desktop)
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub(crate) struct IosConfigStub {
    #[serde(default)]
    pub(crate) identifier: Option<String>,
    #[serde(default)]
    pub(crate) plist: PlistStub,
    #[serde(default)]
    pub(crate) raw: IosRawStub,
    #[serde(default)]
    pub(crate) entitlements: EntitlementsStub,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub(crate) struct PlistStub {
    #[serde(flatten)]
    pub(crate) entries: std::collections::HashMap<String, serde_json::Value>,
}

impl std::ops::Deref for PlistStub {
    type Target = std::collections::HashMap<String, serde_json::Value>;
    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub(crate) struct IosRawStub {
    #[serde(default)]
    pub(crate) info_plist: Option<String>,
    #[serde(default)]
    pub(crate) entitlements: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub(crate) struct EntitlementsStub {
    #[serde(default)]
    pub(crate) associated_domains: Vec<String>,
    #[serde(default)]
    pub(crate) app_groups: Vec<String>,
    #[serde(default)]
    pub(crate) aps_environment: Option<String>,
    #[serde(default)]
    pub(crate) icloud: bool,
    #[serde(default)]
    pub(crate) keychain_access_groups: Vec<String>,
    #[serde(default)]
    pub(crate) apple_pay: bool,
    #[serde(default)]
    pub(crate) healthkit: bool,
    #[serde(default)]
    pub(crate) homekit: bool,
    #[serde(default)]
    pub(crate) additional: std::collections::HashMap<String, serde_json::Value>,
}

/// Android configuration stub (not supported in Freya desktop)
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub(crate) struct AndroidConfigStub {
    #[serde(default)]
    pub(crate) identifier: Option<String>,
    #[serde(default)]
    pub(crate) min_sdk: Option<u32>,
    #[serde(default)]
    pub(crate) target_sdk: Option<u32>,
    #[serde(default)]
    pub(crate) compile_sdk: Option<u32>,
    #[serde(default)]
    pub(crate) features: Vec<String>,
    #[serde(default)]
    pub(crate) gradle_dependencies: Vec<String>,
    #[serde(default)]
    pub(crate) gradle_plugins: Vec<String>,
    #[serde(default)]
    pub(crate) application: AndroidApplicationStub,
    #[serde(default)]
    pub(crate) raw: AndroidRawStub,
    #[serde(default)]
    pub(crate) proguard_rules: Vec<String>,
    #[serde(default)]
    pub(crate) lib_name: Option<String>,
}

/// Android application stub
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub(crate) struct AndroidApplicationStub {
    #[serde(default)]
    pub(crate) uses_cleartext_traffic: Option<bool>,
    #[serde(default)]
    pub(crate) theme: Option<String>,
    #[serde(default)]
    pub(crate) supports_rtl: Option<bool>,
    #[serde(default)]
    pub(crate) large_heap: Option<bool>,
}

/// Android raw configuration stub
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub(crate) struct AndroidRawStub {
    #[serde(default)]
    pub(crate) manifest: Option<String>,
}
