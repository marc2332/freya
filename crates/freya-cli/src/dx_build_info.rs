pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const GIT_COMMIT_HASH: Option<&str> = option_env!("FREYA_CLI_GIT_SHA");
pub const GIT_COMMIT_HASH_SHORT: Option<&str> = option_env!("FREYA_CLI_GIT_SHA_SHORT");
