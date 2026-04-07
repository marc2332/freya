use std::{
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddr,
    },
    path::PathBuf,
};

pub const CLI_ENABLED_ENV: &str = "FREYA_CLI_ENABLED";
pub const SERVER_IP_ENV: &str = "FREYA_HOTRELOAD_ADDRESS";
pub const SERVER_PORT_ENV: &str = "FREYA_HOTRELOAD_PORT";
pub const DEVSERVER_IP_ENV: &str = "FREYA_DEVSERVER_IP";
pub const DEVSERVER_PORT_ENV: &str = "FREYA_DEVSERVER_PORT";
pub const ALWAYS_ON_TOP_ENV: &str = "FREYA_ALWAYS_ON_TOP";
pub const ASSET_ROOT_ENV: &str = "FREYA_ASSET_ROOT";
pub const APP_TITLE_ENV: &str = "FREYA_APP_TITLE";
pub const PRODUCT_NAME_ENV: &str = "FREYA_PRODUCT_NAME";

#[deprecated(since = "0.6.0", note = "The CLI currently does not set this.")]
#[doc(hidden)]
pub const OUT_DIR: &str = "FREYA_OUT_DIR";
pub const SESSION_CACHE_DIR: &str = "FREYA_SESSION_CACHE_DIR";
pub const BUILD_ID: &str = "FREYA_BUILD_ID";

macro_rules! read_env_config {
    ($name:expr) => {{
        #[cfg(debug_assertions)]
        {
            std::env::var($name).ok()
        }

        #[cfg(not(debug_assertions))]
        {
            option_env!($name).map(ToString::to_string)
        }
    }};
}

pub fn devserver_raw_addr() -> Option<SocketAddr> {
    let port = std::env::var(DEVSERVER_PORT_ENV).ok();

    if cfg!(target_os = "android") {
        let port = port.unwrap_or("8080".to_string());
        return Some(format!("127.0.0.1:{port}").parse().unwrap());
    }

    let port = port?;
    let ip = std::env::var(DEVSERVER_IP_ENV).ok()?;

    format!("{ip}:{port}").parse().ok()
}

pub fn devserver_ws_endpoint() -> Option<String> {
    let addr = devserver_raw_addr()?;
    Some(format!("ws://{addr}/_dioxus"))
}

pub fn server_ip() -> Option<IpAddr> {
    std::env::var(SERVER_IP_ENV)
        .ok()
        .and_then(|s| s.parse().ok())
}

pub fn server_port() -> Option<u16> {
    std::env::var(SERVER_PORT_ENV)
        .ok()
        .and_then(|s| s.parse().ok())
}

pub fn fullstack_address_or_localhost() -> SocketAddr {
    let ip = server_ip().unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = server_port().unwrap_or(8080);
    SocketAddr::new(ip, port)
}

pub fn app_title() -> Option<String> {
    read_env_config!("FREYA_APP_TITLE")
}

pub fn always_on_top() -> Option<bool> {
    std::env::var(ALWAYS_ON_TOP_ENV)
        .ok()
        .and_then(|s| s.parse().ok())
}

pub fn is_cli_enabled() -> bool {
    std::env::var(CLI_ENABLED_ENV).is_ok()
}

pub fn base_path() -> Option<String> {
    read_env_config!("FREYA_ASSET_ROOT")
}

pub fn format_base_path_meta_element(base_path: &str) -> String {
    format!(r#"<meta name="{ASSET_ROOT_ENV}" content="{base_path}">"#)
}

#[doc(hidden)]
#[deprecated(
    since = "0.6.0",
    note = "The does not set the OUT_DIR environment variable."
)]
pub fn out_dir() -> Option<PathBuf> {
    #[allow(deprecated)]
    {
        std::env::var(OUT_DIR).ok().map(PathBuf::from)
    }
}

pub fn session_cache_dir() -> Option<PathBuf> {
    if cfg!(target_os = "android") {
        return Some(android_session_cache_dir());
    }

    std::env::var(SESSION_CACHE_DIR).ok().map(PathBuf::from)
}

pub fn android_session_cache_dir() -> PathBuf {
    PathBuf::from("/data/local/tmp/dx/")
}

pub fn build_id() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        0
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var(BUILD_ID)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }
}

pub fn product_name() -> Option<String> {
    read_env_config!("FREYA_PRODUCT_NAME")
}
