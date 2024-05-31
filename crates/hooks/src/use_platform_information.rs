use dioxus_hooks::use_context;
use dioxus_signals::Memo;
use freya_core::prelude::PlatformInformation;

/// Get access to information from the platform.
pub fn use_platform_information() -> Memo<PlatformInformation> {
    use_context()
}
