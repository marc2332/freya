use std::{
    fmt::Display,
    str::FromStr,
};

use anyhow::Result;
use clap::{
    arg,
    Arg,
    ArgMatches,
    Args,
    FromArgMatches,
};
use serde::{
    Deserialize,
    Serialize,
};
use target_lexicon::{
    Environment,
    OperatingSystem,
    Triple,
};

#[derive(
    Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug, Default,
)]
#[non_exhaustive]
pub(crate) enum Platform {
    /// Alias for `--target <host> --renderer webview --bundle-format macos`
    #[serde(rename = "macos")]
    MacOS,

    /// Alias for `--target <host> --renderer webview --bundle-format windows`
    #[serde(rename = "windows")]
    Windows,

    /// Alias for `--target <host> --renderer webview --bundle-format linux`
    #[serde(rename = "linux")]
    Linux,

    /// No platform was specified, so the CLI is free to choose the best one.
    #[default]
    Unknown,
}

impl Platform {
    /// The native "desktop" host app format.
    pub(crate) fn host() -> Self {
        if cfg!(target_os = "macos") {
            Self::MacOS
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "linux") {
            Self::Linux
        } else {
            Self::Unknown
        }
    }
    fn from_identifier(identifier: &str) -> std::result::Result<Self, clap::Error> {
        match identifier {
            "macos" => Ok(Self::MacOS),
            "windows" => Ok(Self::Windows),
            "linux" => Ok(Self::Linux),
            "desktop" => {
                if cfg!(target_os = "macos") {
                    Ok(Self::MacOS)
                } else if cfg!(target_os = "windows") {
                    Ok(Self::Windows)
                } else if cfg!(unix) {
                    Ok(Self::Linux)
                } else {
                    Err(clap::Error::raw(
                        clap::error::ErrorKind::InvalidValue,
                        "Desktop alias is not supported on this platform",
                    ))
                }
            }
            _ => Err(clap::Error::raw(
                clap::error::ErrorKind::InvalidValue,
                format!("Unknown platform: {identifier}"),
            )),
        }
    }
}

impl Args for Platform {
    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        Self::augment_args(cmd)
    }

    fn augment_args(cmd: clap::Command) -> clap::Command {
        const HELP_HEADING: &str = "Platform";
        cmd.arg(arg!(--desktop "Target a desktop app").help_heading(HELP_HEADING))
            .arg(arg!(--macos "Target a macos desktop app").help_heading(HELP_HEADING))
            .arg(arg!(--windows "Target a windows desktop app").help_heading(HELP_HEADING))
            .arg(arg!(--linux "Target a linux desktop app").help_heading(HELP_HEADING))
            .arg(arg!(--ios "Target an ios app").help_heading(HELP_HEADING))
            .arg(arg!(--android "Target an android app").help_heading(HELP_HEADING))
            .arg(
                Arg::new("platform")
                    .long("platform")
                    .value_name("PLATFORM")
                    .help("Manually set the platform (macos, windows, linux, ios, android)")
                    .help_heading(HELP_HEADING)
                    .value_parser(["macos", "windows", "linux", "ios", "android", "desktop"])
                    .conflicts_with("target_alias"),
            )
            .group(
                clap::ArgGroup::new("target_alias")
                    .args(["desktop", "macos", "windows", "linux", "ios", "android"])
                    .multiple(false)
                    .required(false),
            )
    }
}

impl FromArgMatches for Platform {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        if let Some(identifier) = matches.get_one::<String>("platform") {
            Self::from_identifier(identifier)
        } else if let Some(platform) = matches.get_one::<clap::Id>("target_alias") {
            Self::from_identifier(platform.as_str())
        } else {
            Ok(Self::Unknown)
        }
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

#[derive(
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Debug,
    clap::ValueEnum,
)]
#[non_exhaustive]
pub(crate) enum Renderer {
    /// Targeting native renderer
    Native,
}

impl Renderer {
    /// Get the feature name for the platform in the dioxus crate
    pub(crate) fn feature_name(&self, target: &Triple) -> &str {
        match self {
            Renderer::Native => match (target.environment, target.operating_system) {
                (Environment::Android, _) | (_, OperatingSystem::IOS(_)) => "mobile",
                _ => "desktop",
            },
        }
    }

    pub(crate) fn autodetect_from_cargo_feature(feature: &str) -> Option<Self> {
        match feature {
            "desktop" | "native" => Some(Self::Native),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct UnknownRendererError;

impl std::error::Error for UnknownRendererError {}

impl std::fmt::Display for UnknownRendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown renderer")
    }
}

impl FromStr for Renderer {
    type Err = UnknownRendererError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "native" => Ok(Self::Native),
            _ => Err(UnknownRendererError),
        }
    }
}

impl Display for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Renderer::Native => "native",
        })
    }
}
#[derive(
    Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug, Default,
)]
#[non_exhaustive]
pub(crate) enum BundleFormat {
    /// Targeting the linux desktop bundle structure
    #[cfg_attr(target_os = "linux", serde(alias = "desktop"))]
    #[serde(rename = "linux")]
    #[default]
    Linux,

    /// Targeting the macos desktop bundle structure
    #[cfg_attr(target_os = "macos", serde(alias = "desktop"))]
    #[serde(rename = "macos")]
    MacOS,

    /// Targeting the windows desktop bundle structure
    #[cfg_attr(target_os = "windows", serde(alias = "desktop"))]
    #[serde(rename = "windows")]
    Windows,
}

impl BundleFormat {
    /// The native "desktop" host app format.
    pub(crate) fn host() -> Self {
        if cfg!(target_os = "macos") {
            Self::MacOS
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "linux") {
            Self::Linux
        } else {
            unreachable!("What are you doing????");
        }
    }

    /// Get the name of the folder we need to generate for this platform
    ///
    /// Note that web and server share the same platform folder since we'll export the web folder as a bundle on its own
    pub(crate) fn build_folder_name(&self) -> &'static str {
        match self {
            Self::Windows => "windows",
            Self::Linux => "linux",
            Self::MacOS => "macos",
        }
    }

    pub(crate) fn profile_name(&self, release: bool) -> String {
        let base_profile = match self {
            Self::MacOS | Self::Windows | Self::Linux => "desktop",
        };

        let opt_level = if release { "release" } else { "dev" };

        format!("{base_profile}-{opt_level}")
    }

    pub(crate) fn expected_name(&self) -> &'static str {
        match self {
            Self::MacOS => "MacOS",
            Self::Windows => "Windows",
            Self::Linux => "Linux",
        }
    }
}

#[derive(Debug)]
pub(crate) struct UnknownBundleFormatError;

impl std::error::Error for UnknownBundleFormatError {}

impl std::fmt::Display for UnknownBundleFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown bundle format")
    }
}

impl FromStr for BundleFormat {
    type Err = UnknownBundleFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "macos" => Ok(Self::MacOS),
            "windows" => Ok(Self::Windows),
            "linux" => Ok(Self::Linux),
            _ => Err(UnknownBundleFormatError),
        }
    }
}

impl Display for BundleFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BundleFormat::MacOS => "macos",
            BundleFormat::Windows => "windows",
            BundleFormat::Linux => "linux",
        })
    }
}
