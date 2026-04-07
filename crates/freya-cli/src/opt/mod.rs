// Minimal opt module for Freya CLI - asset processing removed
// This module is kept for structural compatibility but does not process assets

use std::path::Path;

use serde::{
    Deserialize,
    Serialize,
};

/// Empty asset manifest - assets are not processed in Freya desktop builds
#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub(crate) struct AssetManifest;

impl AssetManifest {
    pub fn new() -> Self {
        Self
    }

    /// Returns an empty iterator - no assets to iterate
    pub fn unique_assets(&self) -> impl Iterator<Item = &BundledAsset> {
        std::iter::empty()
    }

    /// Returns None - no assets to get
    pub fn get_assets_for_source(
        &self,
        _path: &Path,
    ) -> Option<&std::collections::HashSet<BundledAsset>> {
        None
    }

    /// Returns None - no assets to get
    pub fn get_first_asset_for_source(&self, _path: &Path) -> Option<&BundledAsset> {
        None
    }

    /// Always returns false - no assets
    pub fn contains(&self, _asset: &BundledAsset) -> bool {
        false
    }

    /// No-op - no assets to insert
    pub fn insert_asset(&mut self, _asset: BundledAsset) {
        // No-op for Freya desktop builds
    }
}

/// Minimal BundledAsset stub for Freya - not used in desktop builds
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Hash, Eq)]
pub struct BundledAsset;

impl BundledAsset {
    pub fn absolute_source_path(&self) -> &str {
        ""
    }

    pub fn bundled_path(&self) -> &str {
        ""
    }

    pub fn options(&self) -> &AssetOptions {
        &AssetOptions
    }
}

/// Minimal AssetOptions stub
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct AssetOptions;

/// No-op asset processing function for Freya desktop
pub(crate) fn process_file_to(
    _options: &AssetOptions,
    _from: &Path,
    _to: &Path,
    _cache: Option<&Path>,
) -> anyhow::Result<()> {
    // No-op for Freya desktop builds - assets are loaded directly from filesystem
    Ok(())
}
