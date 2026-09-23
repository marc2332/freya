use std::fmt;

use freya_winit::drivers::ExternalGpuDevice;

#[derive(Clone)]
pub struct WgpuSetupOptions {
    pub label: Option<&'static str>,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
    pub instance_flags: wgpu::InstanceFlags,
    pub power_preference: wgpu::PowerPreference,
}

impl Default for WgpuSetupOptions {
    fn default() -> Self {
        Self {
            label: Some("freya-wgpu"),
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            instance_flags: wgpu::InstanceFlags::from_build_config(),
            power_preference: wgpu::PowerPreference::HighPerformance,
        }
    }
}

#[derive(Debug)]
pub enum WgpuSetupError {
    NoAdapter,
    RequestDevice(wgpu::RequestDeviceError),
    /// The platform has no Freya driver that can share a device.
    UnsupportedPlatform,
    /// The device runs on a backend other than the one Freya shares with.
    WrongBackend,
}

impl fmt::Display for WgpuSetupError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoAdapter => formatter
                .write_str("No GPU adapter was found on a backend Freya can share textures with"),
            Self::RequestDevice(error) => {
                write!(formatter, "Could not create a wgpu device: {error}")
            }
            Self::UnsupportedPlatform => formatter
                .write_str("Texture sharing needs Vulkan on Linux and Windows or Metal on macOS"),
            Self::WrongBackend => {
                formatter.write_str("The wgpu device does not run on the backend Freya shares with")
            }
        }
    }
}

impl std::error::Error for WgpuSetupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::RequestDevice(error) => Some(error),
            _ => None,
        }
    }
}

/// A wgpu device Freya can render on, created before launching.
pub struct WgpuSetup {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl WgpuSetup {
    /// The only backends with a matching Freya graphics driver.
    pub fn interop_backends() -> wgpu::Backends {
        if cfg!(target_os = "macos") {
            wgpu::Backends::METAL
        } else {
            wgpu::Backends::VULKAN
        }
    }

    pub async fn new(options: WgpuSetupOptions) -> Result<Self, WgpuSetupError> {
        let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
        descriptor.backends = Self::interop_backends();
        descriptor.flags = options.instance_flags;
        let instance = wgpu::Instance::new(descriptor);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: options.power_preference,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .map_err(|_| WgpuSetupError::NoAdapter)?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: options.label,
                required_features: options.features,
                required_limits: options.limits,
                ..Default::default()
            })
            .await
            .map_err(WgpuSetupError::RequestDevice)?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }

    /// The raw handles Freya needs to build its graphics driver on this device.
    #[allow(clippy::needless_return)]
    pub fn external_gpu_device(&self) -> Result<ExternalGpuDevice, WgpuSetupError> {
        #[cfg(any(target_os = "linux", target_os = "windows"))]
        {
            use std::{
                ffi::CString,
                sync::Arc,
            };

            use freya_winit::drivers::ExternalVulkanDevice;

            let hal_device = unsafe { self.device.as_hal::<wgpu::hal::api::Vulkan>() }
                .ok_or(WgpuSetupError::WrongBackend)?;
            let shared_instance = hal_device.shared_instance();

            return Ok(ExternalGpuDevice::Vulkan(ExternalVulkanDevice {
                entry: shared_instance.entry().clone(),
                instance: shared_instance.raw_instance().clone(),
                physical_device: hal_device.raw_physical_device(),
                device: Arc::new(hal_device.raw_device().clone()),
                queue: hal_device.raw_queue(),
                queue_family_index: hal_device.queue_family_index(),
                api_version: shared_instance.instance_api_version(),
                instance_extensions: shared_instance
                    .extensions()
                    .iter()
                    .map(|extension| CString::from(*extension))
                    .collect(),
                device_extensions: hal_device
                    .enabled_device_extensions()
                    .iter()
                    .map(|extension| CString::from(*extension))
                    .collect(),
            }));
        }

        #[cfg(target_os = "macos")]
        {
            use freya_winit::drivers::ExternalMetalDevice;

            let hal_device = unsafe { self.device.as_hal::<wgpu::hal::api::Metal>() }
                .ok_or(WgpuSetupError::WrongBackend)?;
            let hal_queue = unsafe { self.queue.as_hal::<wgpu::hal::api::Metal>() }
                .ok_or(WgpuSetupError::WrongBackend)?;

            return Ok(ExternalGpuDevice::Metal(ExternalMetalDevice {
                device: hal_device.raw_device().clone(),
                command_queue: hal_queue.as_raw().retain(),
            }));
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            return Err(WgpuSetupError::UnsupportedPlatform);
        }
    }

    /// Exposes this device to components as a [`crate::WgpuContext`].
    pub fn plugin(&self) -> crate::WgpuPlugin {
        crate::WgpuPlugin::new(self.device.clone(), self.queue.clone())
    }
}
