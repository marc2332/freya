//! Maps unified Dioxus.toml config to platform-specific manifest data.
//!
//! This module converts cross-platform declarations (permissions, deep links,
//! background modes) into platform-specific identifiers:
//! - Android: `<uses-permission>` entries, intent filters, foreground service types
//! - iOS/macOS: Info.plist keys, URL schemes, UIBackgroundModes

use crate::config::{
    DeepLinkConfig,
    MacosConfig,
    PermissionsConfig,
};

/// iOS/macOS plist entry for Info.plist
#[derive(Debug, Clone)]
pub struct PlistEntry {
    /// Plist key (e.g., "NSCameraUsageDescription")
    pub key: String,
    /// User-facing description shown in permission dialogs
    pub value: String,
}

/// Maps unified permissions, deep links, and background modes to platform-specific identifiers
#[derive(Debug, Default)]
pub struct ManifestMapper {
    pub macos_plist_entries: Vec<PlistEntry>,

    /// URL schemes for macOS CFBundleURLTypes (merged from deep_links.schemes + macos.url_schemes)
    pub macos_url_schemes: Vec<String>,
}

impl ManifestMapper {
    /// Create a new permission mapper from the unified config
    pub fn from_config(
        permissions: &PermissionsConfig,
        deep_links: &DeepLinkConfig,
        macos: &MacosConfig,
    ) -> Self {
        let mut mapper = Self::default();

        // Map unified permissions
        mapper.map_location(permissions);
        mapper.map_camera(permissions);
        mapper.map_microphone(permissions);
        mapper.map_photos(permissions);
        mapper.map_bluetooth(permissions);
        mapper.map_contacts(permissions);
        mapper.map_calendar(permissions);

        // Map deep links
        mapper.map_deep_links(deep_links, macos);

        mapper
    }

    fn map_location(&mut self, permissions: &PermissionsConfig) {
        if let Some(loc) = &permissions.location {
            self.macos_plist_entries.push(PlistEntry {
                key: "NSLocationUsageDescription".to_string(),
                value: loc.description.clone(),
            });
        }
    }

    fn map_camera(&mut self, permissions: &PermissionsConfig) {
        if let Some(cam) = &permissions.camera {
            self.macos_plist_entries.push(PlistEntry {
                key: "NSCameraUsageDescription".to_string(),
                value: cam.description.clone(),
            });
        }
    }

    fn map_microphone(&mut self, permissions: &PermissionsConfig) {
        if let Some(mic) = &permissions.microphone {
            self.macos_plist_entries.push(PlistEntry {
                key: "NSMicrophoneUsageDescription".to_string(),
                value: mic.description.clone(),
            });
        }
    }

    fn map_photos(&mut self, permissions: &PermissionsConfig) {
        if let Some(photos) = &permissions.photos {
            self.macos_plist_entries.push(PlistEntry {
                key: "NSPhotoLibraryUsageDescription".to_string(),
                value: photos.description.clone(),
            });
        }
    }

    fn map_bluetooth(&mut self, permissions: &PermissionsConfig) {
        if let Some(bt) = &permissions.bluetooth {
            self.macos_plist_entries.push(PlistEntry {
                key: "NSBluetoothAlwaysUsageDescription".to_string(),
                value: bt.description.clone(),
            });
        }
    }

    fn map_contacts(&mut self, permissions: &PermissionsConfig) {
        if let Some(contacts) = &permissions.contacts {
            self.macos_plist_entries.push(PlistEntry {
                key: "NSContactsUsageDescription".to_string(),
                value: contacts.description.clone(),
            });
        }
    }

    fn map_calendar(&mut self, permissions: &PermissionsConfig) {
        if let Some(cal) = &permissions.calendar {
            self.macos_plist_entries.push(PlistEntry {
                key: "NSCalendarsUsageDescription".to_string(),
                value: cal.description.clone(),
            });
        }
    }

    /// Map deep link config to platform-specific URL schemes, associated domains, and intent filters
    fn map_deep_links(&mut self, deep_links: &DeepLinkConfig, macos: &MacosConfig) {
        // Merge unified schemes with platform-specific overrides
        let mut macos_schemes: Vec<String> = deep_links.schemes.clone();
        macos_schemes.extend(macos.url_schemes.clone());
        macos_schemes.dedup();
        self.macos_url_schemes = macos_schemes;
    }
}
