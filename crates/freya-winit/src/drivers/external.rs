#[cfg(any(target_os = "linux", target_os = "windows"))]
use std::{
    ffi::CString,
    sync::Arc,
};

#[cfg(target_os = "macos")]
use objc2::{
    rc::Retained,
    runtime::ProtocolObject,
};
#[cfg(target_os = "macos")]
use objc2_metal::{
    MTLCommandQueue,
    MTLDevice,
};

/// Raw handles of a GPU device created outside of Freya, whose textures Freya can draw directly.
#[derive(Clone)]
pub enum ExternalGpuDevice {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    Vulkan(ExternalVulkanDevice),
    #[cfg(target_os = "macos")]
    Metal(ExternalMetalDevice),
}

impl ExternalGpuDevice {
    pub fn driver_name(&self) -> &'static str {
        match self {
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::Vulkan(_) => "Vulkan",
            #[cfg(target_os = "macos")]
            Self::Metal(_) => "Metal",
        }
    }
}

/// A Vulkan device owned by an external renderer, its queue family must support presentation.
#[cfg(any(target_os = "linux", target_os = "windows"))]
#[derive(Clone)]
pub struct ExternalVulkanDevice {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub physical_device: ash::vk::PhysicalDevice,
    pub device: Arc<ash::Device>,
    pub queue: ash::vk::Queue,
    pub queue_family_index: u32,
    pub api_version: u32,
    /// Instance extensions enabled by the owner, reported to Skia.
    pub instance_extensions: Vec<CString>,
    /// Device extensions enabled by the owner, reported to Skia.
    pub device_extensions: Vec<CString>,
}

/// A Metal device owned by an external renderer, its command queue is shared for ordering.
#[cfg(target_os = "macos")]
#[derive(Clone)]
pub struct ExternalMetalDevice {
    pub device: Retained<ProtocolObject<dyn MTLDevice>>,
    pub command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
}
