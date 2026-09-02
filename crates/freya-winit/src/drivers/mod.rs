#[cfg(any(target_os = "linux", target_os = "windows", target_os = "android"))]
mod gl;
#[cfg(target_os = "macos")]
mod metal;
mod software;
#[cfg(any(target_os = "linux", target_os = "windows"))]
mod vulkan;

use freya_engine::prelude::Surface as SkiaSurface;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{
        Window,
        WindowAttributes,
    },
};

use crate::config::RendererPreference;

/// Unrecoverable graphics error requiring a driver rebuild.
#[derive(Debug)]
// Only the Vulkan driver reports these.
#[cfg_attr(not(any(target_os = "linux", target_os = "windows")), allow(dead_code))]
pub enum DriverError {
    DeviceLost,
    OutOfMemory,
}

#[allow(clippy::large_enum_variant)]
pub enum GraphicsDriver {
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "android"))]
    OpenGl(gl::OpenGLDriver),
    #[cfg(target_os = "macos")]
    Metal(metal::MetalDriver),
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    Vulkan(vulkan::VulkanDriver),
    Software(software::SoftwareDriver),
}

impl GraphicsDriver {
    #[allow(clippy::needless_return)]
    pub fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        gpu_resource_cache_limit: usize,
        preference: RendererPreference,
    ) -> (Self, Window) {
        let from_env = std::env::var("FREYA_RENDERER")
            .ok()
            .map(|value| value.to_ascii_lowercase());
        let renderer = preference.as_name().or(from_env.as_deref());

        // Opt-in via FREYA_RENDERER=software, available on every platform.
        if renderer == Some("software") {
            match software::SoftwareDriver::new(event_loop, window_attributes.clone()) {
                Ok((driver, window)) => return (Self::Software(driver), window),
                Err(err) => {
                    tracing::warn!(
                        "Software renderer initialization failed, falling back to default: {err}"
                    );
                }
            }
        }

        // Metal (macOS)
        #[cfg(target_os = "macos")]
        {
            let (driver, window) =
                metal::MetalDriver::new(event_loop, window_attributes, gpu_resource_cache_limit);

            return (Self::Metal(driver), window);
        }

        // OpenGL only on Android.
        #[cfg(target_os = "android")]
        {
            match gl::OpenGLDriver::new(
                event_loop,
                window_attributes.clone(),
                gpu_resource_cache_limit,
            ) {
                Ok((driver, window)) => return (Self::OpenGl(driver), window),
                Err(err) => {
                    tracing::warn!("OpenGL initialization failed, falling back to software: {err}");
                }
            }

            let (driver, window) = software::SoftwareDriver::new(event_loop, window_attributes)
                .expect("Failed to initialize software renderer fallback");
            return (Self::Software(driver), window);
        }

        // Linux: Vulkan by default, set FREYA_RENDERER=opengl to force OpenGL.
        // Windows: OpenGL by default, set FREYA_RENDERER=vulkan to force Vulkan.
        // If both fail, falls back to the software renderer.
        #[cfg(all(not(target_os = "macos"), not(target_os = "android")))]
        {
            let use_vulkan = if cfg!(target_os = "windows") {
                renderer == Some("vulkan")
            } else {
                renderer != Some("opengl")
            };

            if use_vulkan {
                match vulkan::VulkanDriver::new(
                    event_loop,
                    window_attributes.clone(),
                    gpu_resource_cache_limit,
                ) {
                    Ok((driver, window)) => return (Self::Vulkan(driver), window),
                    Err(err) => {
                        tracing::warn!(
                            "Vulkan initialization failed, falling back to OpenGL: {err}"
                        );
                    }
                }
            }

            match gl::OpenGLDriver::new(
                event_loop,
                window_attributes.clone(),
                gpu_resource_cache_limit,
            ) {
                Ok((driver, window)) => return (Self::OpenGl(driver), window),
                Err(err) => {
                    tracing::warn!("OpenGL initialization failed, falling back to software: {err}");
                }
            }

            let (driver, window) = software::SoftwareDriver::new(event_loop, window_attributes)
                .expect("Failed to initialize software renderer fallback");
            return (Self::Software(driver), window);
        }
    }

    /// Rebuild the driver on the existing window, skipping Vulkan.
    #[cfg_attr(target_os = "macos", allow(unused_variables))]
    pub fn recover_reusing_window(
        event_loop: &ActiveEventLoop,
        window: &Window,
        gpu_resource_cache_limit: usize,
        transparent: bool,
    ) -> Self {
        #[cfg(any(target_os = "linux", target_os = "windows", target_os = "android"))]
        match gl::OpenGLDriver::from_window(
            event_loop,
            window,
            gpu_resource_cache_limit,
            transparent,
        ) {
            Ok(driver) => return Self::OpenGl(driver),
            Err(error) => {
                tracing::warn!("OpenGL recovery failed, falling back to software: {error}");
            }
        }

        let driver = software::SoftwareDriver::from_window(window)
            .expect("Failed to initialize software renderer fallback");
        Self::Software(driver)
    }

    pub fn present(
        &mut self,
        size: PhysicalSize<u32>,
        window: &Window,
        render: impl FnOnce(&mut SkiaSurface),
    ) -> Result<(), DriverError> {
        match self {
            #[cfg(any(target_os = "linux", target_os = "windows", target_os = "android"))]
            Self::OpenGl(gl) => {
                gl.present(window, render);
                Ok(())
            }
            #[cfg(target_os = "macos")]
            Self::Metal(mtl) => {
                mtl.present(size, window, render);
                Ok(())
            }
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::Vulkan(vk) => vk.present(size, window, render),
            Self::Software(sw) => {
                sw.present(size, window, render);
                Ok(())
            }
        }
    }

    /// The name of the active graphics driver.
    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(any(target_os = "linux", target_os = "windows", target_os = "android"))]
            Self::OpenGl(_) => "OpenGL",
            #[cfg(target_os = "macos")]
            Self::Metal(_) => "Metal",
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::Vulkan(_) => "Vulkan",
            Self::Software(_) => "Software",
        }
    }

    /// The name of the GPU picked by the driver, only known for OpenGL and Vulkan.
    pub fn gpu_name(&self) -> Option<&str> {
        match self {
            #[cfg(any(target_os = "linux", target_os = "windows", target_os = "android"))]
            Self::OpenGl(gl) => gl.gpu_name.as_deref(),
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::Vulkan(vk) => Some(vk.gpu_name.as_str()),
            _ => None,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> Result<(), DriverError> {
        match self {
            #[cfg(any(target_os = "linux", target_os = "windows", target_os = "android"))]
            Self::OpenGl(gl) => {
                gl.resize(size);
                Ok(())
            }
            #[cfg(target_os = "macos")]
            Self::Metal(mtl) => {
                mtl.resize(size);
                Ok(())
            }
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::Vulkan(vk) => vk.resize(size),
            Self::Software(sw) => {
                sw.resize(size);
                Ok(())
            }
        }
    }
}
