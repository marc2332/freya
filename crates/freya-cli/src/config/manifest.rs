//! Unified manifest configuration for cross-platform app packaging.
//!
//! This module provides configuration structs for permissions and platform-specific
//! manifest customization. Permissions declared here are automatically mapped to
//! platform-specific identifiers (AndroidManifest.xml, Info.plist, etc.)
//!
//! ## JSON Schema Generation
//!
//! Generate a JSON schema for IDE autocomplete:
//! ```bash
//! dx config --schema > dioxus-schema.json
//! ```

use std::{
    collections::HashMap,
    path::PathBuf,
};

use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};

// ============================================================================
// Unified Permissions
// ============================================================================

/// Unified permission configuration that maps to platform-specific identifiers.
///
/// Example:
/// ```toml
/// [permissions]
/// location = { precision = "fine", description = "Track your runs" }
/// camera = { description = "Take photos for your profile" }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct PermissionsConfig {
    /// Location permission with precision level.
    /// Maps to ACCESS_FINE_LOCATION/ACCESS_COARSE_LOCATION on Android,
    /// NSLocationWhenInUseUsageDescription on iOS/macOS.
    #[serde(default)]
    pub location: Option<LocationPermission>,

    /// Camera access permission.
    #[serde(default)]
    pub camera: Option<SimplePermission>,

    /// Microphone access permission.
    #[serde(default)]
    pub microphone: Option<SimplePermission>,

    /// Push notifications permission.
    #[serde(default)]
    pub notifications: Option<SimplePermission>,

    /// Photo library access.
    #[serde(default)]
    pub photos: Option<StoragePermission>,

    /// Bluetooth connectivity.
    #[serde(default)]
    pub bluetooth: Option<SimplePermission>,

    /// Background location updates.
    #[serde(default, rename = "background-location")]
    pub background_location: Option<SimplePermission>,

    /// Contacts access.
    #[serde(default)]
    pub contacts: Option<StoragePermission>,

    /// Calendar access.
    #[serde(default)]
    pub calendar: Option<StoragePermission>,

    /// Biometric authentication (Face ID, fingerprint).
    #[serde(default)]
    pub biometrics: Option<SimplePermission>,

    /// NFC access.
    #[serde(default)]
    pub nfc: Option<SimplePermission>,

    /// Motion and fitness data.
    #[serde(default)]
    pub motion: Option<SimplePermission>,

    /// Health data access.
    #[serde(default)]
    pub health: Option<StoragePermission>,

    /// Speech recognition.
    #[serde(default)]
    pub speech: Option<SimplePermission>,

    /// Media library access.
    #[serde(default, rename = "media-library")]
    pub media_library: Option<SimplePermission>,

    /// Siri integration (iOS only).
    #[serde(default)]
    pub siri: Option<SimplePermission>,

    /// HomeKit integration (iOS only).
    #[serde(default)]
    pub homekit: Option<SimplePermission>,

    /// Local network access.
    #[serde(default, rename = "local-network")]
    pub local_network: Option<SimplePermission>,

    /// Nearby Wi-Fi devices (Android).
    #[serde(default, rename = "nearby-wifi")]
    pub nearby_wifi: Option<SimplePermission>,
}

/// Simple permission with just a description.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SimplePermission {
    /// User-facing description shown in permission dialogs.
    pub description: String,
}

// ============================================================================
// Unified Deep Linking
// ============================================================================

/// Unified deep linking configuration.
///
/// This provides a cross-platform interface for URL schemes and universal/app links.
/// Platform-specific overrides can be configured in `[ios]` and `[android]` sections.
///
/// Example:
/// ```toml
/// [deep_links]
/// schemes = ["myapp", "com.example.myapp"]
/// hosts = ["example.com", "*.example.com"]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct DeepLinkConfig {
    /// Custom URL schemes (e.g., "myapp" for myapp://path).
    /// Maps to CFBundleURLSchemes on iOS/macOS and intent-filter on Android.
    #[serde(default)]
    pub schemes: Vec<String>,

    /// Universal link / App link hosts (e.g., "example.com").
    /// Maps to Associated Domains on iOS and App Links on Android.
    /// Supports wildcards like "*.example.com".
    #[serde(default)]
    pub hosts: Vec<String>,

    /// Path patterns for universal/app links (e.g., "/app/*", "/share/*").
    /// If empty, all paths are matched.
    #[serde(default)]
    pub paths: Vec<String>,
}

// ============================================================================
// Unified Background Modes
// ============================================================================

/// Unified background execution configuration.
///
/// This provides a cross-platform interface for background capabilities.
/// Platform-specific overrides can be configured in `[ios]` and `[android]` sections.
///
/// Example:
/// ```toml
/// [background]
/// location = true
/// audio = true
/// fetch = true
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct BackgroundConfig {
    /// Background location updates.
    /// iOS: UIBackgroundModes "location"
    /// Android: ACCESS_BACKGROUND_LOCATION permission
    #[serde(default)]
    pub location: bool,

    /// Background audio playback.
    /// iOS: UIBackgroundModes "audio"
    /// Android: FOREGROUND_SERVICE_MEDIA_PLAYBACK
    #[serde(default)]
    pub audio: bool,

    /// Background data fetch.
    /// iOS: UIBackgroundModes "fetch"
    /// Android: WorkManager or foreground service
    #[serde(default)]
    pub fetch: bool,

    /// Remote push notifications.
    /// iOS: UIBackgroundModes "remote-notification"
    /// Android: Firebase Cloud Messaging
    #[serde(default, rename = "remote-notifications")]
    pub remote_notifications: bool,

    /// VoIP calls.
    /// iOS: UIBackgroundModes "voip"
    /// Android: FOREGROUND_SERVICE_PHONE_CALL
    #[serde(default)]
    pub voip: bool,

    /// Bluetooth LE accessories.
    /// iOS: UIBackgroundModes "bluetooth-central" and "bluetooth-peripheral"
    /// Android: FOREGROUND_SERVICE_CONNECTED_DEVICE
    #[serde(default)]
    pub bluetooth: bool,

    /// External accessory communication.
    /// iOS: UIBackgroundModes "external-accessory"
    #[serde(default, rename = "external-accessory")]
    pub external_accessory: bool,

    /// Background processing tasks.
    /// iOS: UIBackgroundModes "processing"
    /// Android: WorkManager
    #[serde(default)]
    pub processing: bool,
}

/// Location permission with precision control.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LocationPermission {
    /// Precision level: "fine" (GPS) or "coarse" (network-based).
    #[serde(default)]
    pub precision: LocationPrecision,

    /// User-facing description shown in permission dialogs.
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, JsonSchema)]
pub enum LocationPrecision {
    #[default]
    #[serde(rename = "fine")]
    Fine,
    #[serde(rename = "coarse")]
    Coarse,
}

/// Storage permission with access level control.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StoragePermission {
    /// Access level: "read", "write", or "read-write".
    #[serde(default)]
    pub access: StorageAccess,

    /// User-facing description shown in permission dialogs.
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, JsonSchema)]
pub enum StorageAccess {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[default]
    #[serde(rename = "read-write")]
    ReadWrite,
}

/// iOS document type declaration.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IosDocumentType {
    /// Document type name.
    pub name: String,

    /// File extensions (e.g., ["txt", "md"]).
    #[serde(default)]
    pub extensions: Vec<String>,

    /// MIME types.
    #[serde(default)]
    pub mime_types: Vec<String>,

    /// UTI types.
    #[serde(default)]
    pub types: Vec<String>,

    /// Icon file name.
    #[serde(default)]
    pub icon: Option<String>,

    /// Role: "Editor", "Viewer", "Shell", or "None".
    #[serde(default)]
    pub role: Option<String>,
}

/// iOS Uniform Type Identifier declaration.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IosTypeIdentifier {
    /// UTI identifier (e.g., "com.example.myformat").
    pub identifier: String,

    /// Human-readable description.
    #[serde(default)]
    pub description: Option<String>,

    /// Conforms to these UTIs.
    #[serde(default)]
    pub conforms_to: Vec<String>,

    /// File extensions.
    #[serde(default)]
    pub extensions: Vec<String>,

    /// MIME types.
    #[serde(default)]
    pub mime_types: Vec<String>,
}

// ============================================================================
// macOS Configuration
// ============================================================================

/// macOS-specific configuration.
///
/// Example:
/// ```toml
/// [macos]
/// minimum_system_version = "11.0"
/// identifier = "com.example.myapp.macos"  # Override bundle.identifier for macOS
///
/// # macOS signing (previously in [bundle.macos])
/// signing_identity = "Developer ID Application: My Company"
/// provider_short_name = "MYCOMPANY"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct MacosConfig {
    // === Bundle settings (override [bundle] section) ===
    /// The app's identifier (e.g., "com.example.myapp").
    /// Overrides `bundle.identifier` for macOS builds.
    #[serde(default)]
    pub identifier: Option<String>,

    /// The app's publisher.
    /// Overrides `bundle.publisher` for macOS builds.
    #[serde(default)]
    pub publisher: Option<String>,

    /// Icons for the app.
    /// Overrides `bundle.icon` for macOS builds.
    #[serde(default)]
    pub icon: Option<Vec<String>>,

    /// Additional resources to bundle.
    /// Overrides `bundle.resources` for macOS builds.
    #[serde(default)]
    pub resources: Option<Vec<String>>,

    /// Copyright notice.
    /// Overrides `bundle.copyright` for macOS builds.
    #[serde(default)]
    pub copyright: Option<String>,

    /// Short description.
    /// Overrides `bundle.short_description` for macOS builds.
    #[serde(default)]
    pub short_description: Option<String>,

    /// Long description.
    /// Overrides `bundle.long_description` for macOS builds.
    #[serde(default)]
    pub long_description: Option<String>,

    // === macOS bundle settings (previously in bundle.macos) ===
    /// The bundle version string (CFBundleVersion).
    #[serde(default)]
    pub bundle_version: Option<String>,

    /// The bundle short version string (CFBundleShortVersionString).
    #[serde(default)]
    pub bundle_name: Option<String>,

    /// The signing identity to use for code signing.
    /// E.g., "Developer ID Application: My Company (TEAMID)"
    #[serde(default)]
    pub signing_identity: Option<String>,

    /// The provider short name for notarization.
    #[serde(default)]
    pub provider_short_name: Option<String>,

    /// Path to custom entitlements file for code signing.
    /// This overrides the generated entitlements.
    #[serde(default)]
    pub entitlements_file: Option<String>,

    /// Exception domain for App Transport Security.
    #[serde(default)]
    pub exception_domain: Option<String>,

    /// License file to include in DMG.
    #[serde(default)]
    pub license: Option<String>,

    /// Preserve the hardened runtime version flag.
    /// Setting this to false is useful when using an ad-hoc signature.
    #[serde(default)]
    pub hardened_runtime: Option<bool>,

    /// Additional files to include in the app bundle.
    /// Maps the path in the Contents directory to the source file path.
    #[serde(default)]
    pub files: HashMap<PathBuf, PathBuf>,

    // === macOS-specific settings ===
    /// Minimum macOS version (e.g., "11.0").
    #[serde(default)]
    pub minimum_system_version: Option<String>,

    /// Path to custom Info.plist.
    #[serde(default)]
    pub info_plist: Option<PathBuf>,

    /// Frameworks to embed.
    #[serde(default)]
    pub frameworks: Vec<String>,

    /// macOS entitlements.
    #[serde(default)]
    pub entitlements: MacosEntitlements,

    /// Additional Info.plist keys.
    #[serde(default)]
    pub plist: HashMap<String, serde_json::Value>,

    /// Raw injection points.
    #[serde(default)]
    pub raw: MacosRawConfig,

    // === Platform-specific overrides (extend unified config) ===
    /// Additional URL schemes beyond unified `[deep_links]`.schemes.
    /// These are merged with the unified schemes.
    #[serde(default)]
    pub url_schemes: Vec<String>,

    /// Document types the app can open (uses same format as iOS).
    #[serde(default)]
    pub document_types: Vec<IosDocumentType>,

    /// Exported type identifiers (custom UTIs).
    #[serde(default)]
    pub exported_type_identifiers: Vec<IosTypeIdentifier>,

    /// Imported type identifiers.
    #[serde(default)]
    pub imported_type_identifiers: Vec<IosTypeIdentifier>,

    /// App category for the Mac App Store.
    /// E.g., "public.app-category.productivity"
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct MacosEntitlements {
    /// Enable App Sandbox.
    #[serde(default, rename = "app-sandbox")]
    pub app_sandbox: Option<bool>,

    /// User-selected file access (read-write).
    #[serde(default, rename = "files-user-selected")]
    pub files_user_selected: Option<bool>,

    /// User-selected file access (read-only).
    #[serde(default, rename = "files-user-selected-readonly")]
    pub files_user_selected_readonly: Option<bool>,

    /// Outgoing network connections.
    #[serde(default, rename = "network-client")]
    pub network_client: Option<bool>,

    /// Incoming network connections.
    #[serde(default, rename = "network-server")]
    pub network_server: Option<bool>,

    /// Camera access.
    #[serde(default)]
    pub camera: Option<bool>,

    /// Microphone access.
    #[serde(default)]
    pub microphone: Option<bool>,

    /// USB access.
    #[serde(default)]
    pub usb: Option<bool>,

    /// Bluetooth access.
    #[serde(default)]
    pub bluetooth: Option<bool>,

    /// Printing.
    #[serde(default)]
    pub print: Option<bool>,

    /// Location services.
    #[serde(default)]
    pub location: Option<bool>,

    /// Address book access.
    #[serde(default)]
    pub addressbook: Option<bool>,

    /// Calendars access.
    #[serde(default)]
    pub calendars: Option<bool>,

    /// Disable library validation.
    #[serde(default, rename = "disable-library-validation")]
    pub disable_library_validation: Option<bool>,

    /// Allow JIT.
    #[serde(default, rename = "allow-jit")]
    pub allow_jit: Option<bool>,

    /// Allow unsigned executable memory.
    #[serde(default, rename = "allow-unsigned-executable-memory")]
    pub allow_unsigned_executable_memory: Option<bool>,

    /// Additional entitlements.
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct MacosRawConfig {
    /// Raw XML to inject into Info.plist.
    #[serde(default)]
    pub info_plist: Option<String>,

    /// Raw XML to inject into entitlements.plist.
    #[serde(default)]
    pub entitlements: Option<String>,
}

// ============================================================================
// Windows Configuration
// ============================================================================

/// Windows-specific configuration.
///
/// Example:
/// ```toml
/// [windows]
/// identifier = "com.example.myapp.windows"  # Override bundle.identifier for Windows
///
/// # Windows installer settings (previously in [bundle.windows])
/// [windows.nsis]
/// install_mode = "PerMachine"
///
/// [windows.wix]
/// language = [["en-US", null]]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct WindowsConfig {
    // === Bundle settings (override [bundle] section) ===
    /// The app's identifier (e.g., "com.example.myapp").
    /// Overrides `bundle.identifier` for Windows builds.
    #[serde(default)]
    pub identifier: Option<String>,

    /// The app's publisher.
    /// Overrides `bundle.publisher` for Windows builds.
    #[serde(default)]
    pub publisher: Option<String>,

    /// Icons for the app.
    /// Overrides `bundle.icon` for Windows builds.
    #[serde(default)]
    pub icon: Option<Vec<String>>,

    /// Additional resources to bundle.
    /// Overrides `bundle.resources` for Windows builds.
    #[serde(default)]
    pub resources: Option<Vec<String>>,

    /// Copyright notice.
    /// Overrides `bundle.copyright` for Windows builds.
    #[serde(default)]
    pub copyright: Option<String>,

    /// App category.
    /// Overrides `bundle.category` for Windows builds.
    #[serde(default)]
    pub category: Option<String>,

    /// Short description.
    /// Overrides `bundle.short_description` for Windows builds.
    #[serde(default)]
    pub short_description: Option<String>,

    /// Long description.
    /// Overrides `bundle.long_description` for Windows builds.
    #[serde(default)]
    pub long_description: Option<String>,

    // === Windows bundle settings (previously in bundle.windows) ===
    /// Digest algorithm for code signing.
    #[serde(default)]
    pub digest_algorithm: Option<String>,

    /// Certificate thumbprint for code signing.
    #[serde(default)]
    pub certificate_thumbprint: Option<String>,

    /// Timestamp server URL for code signing.
    #[serde(default)]
    pub timestamp_url: Option<String>,

    /// Use TSP (RFC 3161) timestamp.
    #[serde(default)]
    pub tsp: Option<bool>,

    /// WiX installer settings.
    #[serde(default)]
    pub wix: Option<WindowsWixSettings>,

    /// NSIS installer settings.
    #[serde(default)]
    pub nsis: Option<WindowsNsisSettings>,

    /// Path to custom Windows icon.
    #[serde(default)]
    pub icon_path: Option<PathBuf>,

    /// WebView2 installation mode.
    #[serde(default)]
    pub webview_install_mode: Option<WindowsWebviewInstallMode>,

    /// Allow downgrades when installing.
    #[serde(default)]
    pub allow_downgrades: Option<bool>,

    /// Custom sign command.
    #[serde(default)]
    pub sign_command: Option<WindowsSignCommand>,

    // === Windows-specific settings ===
    /// UWP/MSIX capabilities.
    #[serde(default)]
    pub capabilities: Vec<String>,

    /// Restricted capabilities.
    #[serde(default)]
    pub restricted_capabilities: Vec<String>,

    /// Device capabilities.
    #[serde(default)]
    pub device_capabilities: Vec<String>,
}

/// WiX installer settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct WindowsWixSettings {
    /// Languages and their locale paths.
    #[serde(default)]
    pub language: Vec<(String, Option<PathBuf>)>,

    /// Path to custom WiX template.
    #[serde(default)]
    pub template: Option<PathBuf>,

    /// WiX fragment files to include.
    #[serde(default)]
    pub fragment_paths: Vec<PathBuf>,

    /// Component group references.
    #[serde(default)]
    pub component_group_refs: Vec<String>,

    /// Component references.
    #[serde(default)]
    pub component_refs: Vec<String>,

    /// Feature group references.
    #[serde(default)]
    pub feature_group_refs: Vec<String>,

    /// Feature references.
    #[serde(default)]
    pub feature_refs: Vec<String>,

    /// Merge module references.
    #[serde(default)]
    pub merge_refs: Vec<String>,

    /// Skip WebView2 installation.
    #[serde(default)]
    pub skip_webview_install: Option<bool>,

    /// License file path.
    #[serde(default)]
    pub license: Option<PathBuf>,

    /// Enable elevated update task.
    #[serde(default)]
    pub enable_elevated_update_task: Option<bool>,

    /// Banner image path.
    #[serde(default)]
    pub banner_path: Option<PathBuf>,

    /// Dialog image path.
    #[serde(default)]
    pub dialog_image_path: Option<PathBuf>,

    /// FIPS compliant mode.
    #[serde(default)]
    pub fips_compliant: Option<bool>,

    /// MSI version string.
    #[serde(default)]
    pub version: Option<String>,

    /// MSI upgrade code (GUID).
    #[serde(default)]
    #[schemars(with = "Option<String>")]
    pub upgrade_code: Option<uuid::Uuid>,
}

/// NSIS installer settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct WindowsNsisSettings {
    /// Path to custom NSIS template.
    #[serde(default)]
    pub template: Option<PathBuf>,

    /// License file path.
    #[serde(default)]
    pub license: Option<PathBuf>,

    /// Header image path.
    #[serde(default)]
    pub header_image: Option<PathBuf>,

    /// Sidebar image path.
    #[serde(default)]
    pub sidebar_image: Option<PathBuf>,

    /// Installer icon path.
    #[serde(default)]
    pub installer_icon: Option<PathBuf>,

    /// Installation mode: "CurrentUser", "PerMachine", or "Both".
    #[serde(default)]
    pub install_mode: Option<String>,

    /// Languages to include.
    #[serde(default)]
    pub languages: Option<Vec<String>>,

    /// Custom language files.
    #[serde(default)]
    pub custom_language_files: Option<HashMap<String, PathBuf>>,

    /// Display language selector.
    #[serde(default)]
    pub display_language_selector: Option<bool>,

    /// Start menu folder name.
    #[serde(default)]
    pub start_menu_folder: Option<String>,

    /// Installer hooks script path.
    #[serde(default)]
    pub installer_hooks: Option<PathBuf>,

    /// Minimum WebView2 version required.
    #[serde(default)]
    pub minimum_webview2_version: Option<String>,
}

/// WebView2 installation mode.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum WindowsWebviewInstallMode {
    /// Skip WebView2 installation.
    Skip,
    /// Download bootstrapper.
    DownloadBootstrapper {
        #[serde(default)]
        silent: bool,
    },
    /// Embed bootstrapper.
    EmbedBootstrapper {
        #[serde(default)]
        silent: bool,
    },
    /// Use offline installer.
    OfflineInstaller {
        #[serde(default)]
        silent: bool,
    },
    /// Use fixed runtime from path.
    FixedRuntime { path: PathBuf },
}

/// Custom sign command for Windows code signing.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WindowsSignCommand {
    /// The command to run.
    pub cmd: String,
    /// Command arguments. Use "%1" as placeholder for binary path.
    pub args: Vec<String>,
}

// ============================================================================
// Linux Configuration
// ============================================================================

/// Linux-specific configuration.
///
/// Example:
/// ```toml
/// [linux]
/// identifier = "com.example.myapp.linux"  # Override bundle.identifier for Linux
/// categories = ["Utility"]
///
/// # Debian package settings (previously in [bundle.deb])
/// [linux.deb]
/// depends = ["libwebkit2gtk-4.0-37"]
/// section = "utils"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct LinuxConfig {
    // === Bundle settings (override [bundle] section) ===
    /// The app's identifier (e.g., "com.example.myapp").
    /// Overrides `bundle.identifier` for Linux builds.
    #[serde(default)]
    pub identifier: Option<String>,

    /// The app's publisher.
    /// Overrides `bundle.publisher` for Linux builds.
    #[serde(default)]
    pub publisher: Option<String>,

    /// Icons for the app.
    /// Overrides `bundle.icon` for Linux builds.
    #[serde(default)]
    pub icon: Option<Vec<String>>,

    /// Additional resources to bundle.
    /// Overrides `bundle.resources` for Linux builds.
    #[serde(default)]
    pub resources: Option<Vec<String>>,

    /// Copyright notice.
    /// Overrides `bundle.copyright` for Linux builds.
    #[serde(default)]
    pub copyright: Option<String>,

    /// App category.
    /// Overrides `bundle.category` for Linux builds.
    #[serde(default)]
    pub category: Option<String>,

    /// Short description.
    /// Overrides `bundle.short_description` for Linux builds.
    #[serde(default)]
    pub short_description: Option<String>,

    /// Long description.
    /// Overrides `bundle.long_description` for Linux builds.
    #[serde(default)]
    pub long_description: Option<String>,

    // === Debian package settings (previously in bundle.deb) ===
    /// Debian-specific package settings.
    #[serde(default)]
    pub deb: Option<LinuxDebSettings>,

    // === Linux-specific settings ===
    /// Flatpak sandbox permissions.
    #[serde(default)]
    pub flatpak_permissions: Vec<String>,

    /// D-Bus interfaces to access.
    #[serde(default)]
    pub dbus_access: Vec<String>,

    /// Desktop entry categories.
    #[serde(default)]
    pub categories: Vec<String>,

    /// Desktop entry keywords.
    #[serde(default)]
    pub keywords: Vec<String>,

    /// MIME types the app can handle.
    #[serde(default)]
    pub mime_types: Vec<String>,
}

/// Debian package settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct LinuxDebSettings {
    /// Package dependencies.
    #[serde(default)]
    pub depends: Option<Vec<String>>,

    /// Recommended packages.
    #[serde(default)]
    pub recommends: Option<Vec<String>>,

    /// Packages this provides.
    #[serde(default)]
    pub provides: Option<Vec<String>>,

    /// Package conflicts.
    #[serde(default)]
    pub conflicts: Option<Vec<String>>,

    /// Packages this replaces.
    #[serde(default)]
    pub replaces: Option<Vec<String>>,

    /// Additional files to include. Maps package path to source path.
    #[serde(default)]
    pub files: HashMap<PathBuf, PathBuf>,

    /// Path to custom desktop template.
    #[serde(default)]
    pub desktop_template: Option<PathBuf>,

    /// Debian section (e.g., "utils", "web").
    #[serde(default)]
    pub section: Option<String>,

    /// Package priority ("required", "important", "standard", "optional", "extra").
    #[serde(default)]
    pub priority: Option<String>,

    /// Path to changelog file.
    #[serde(default)]
    pub changelog: Option<PathBuf>,

    /// Pre-install script path.
    #[serde(default)]
    pub pre_install_script: Option<PathBuf>,

    /// Post-install script path.
    #[serde(default)]
    pub post_install_script: Option<PathBuf>,

    /// Pre-remove script path.
    #[serde(default)]
    pub pre_remove_script: Option<PathBuf>,

    /// Post-remove script path.
    #[serde(default)]
    pub post_remove_script: Option<PathBuf>,
}
