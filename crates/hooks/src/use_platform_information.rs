use dioxus_hooks::use_context;
use dioxus_signals::Signal;
use freya_core::prelude::PlatformInformation;

/// Get access to information from the platform.
pub fn use_platform_information() -> Signal<PlatformInformation> {
    use_context()
}
