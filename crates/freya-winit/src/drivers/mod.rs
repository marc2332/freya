// RENDERER AND GPU SELECTION
//
//                            +---------------------+
//             Auto [default] | Renderer preference |    explicit
//             ,--------------|                     |--------------.
//             v              +---------------------+              v
//   +-------------------+                               +-------------------+
//   | FREYA_RENDERER    |                               | Keep config       |
//   | case-insensitive  |                               | Ignore env        |
//   +-------------------+                               +-------------------+
//             |                                                   |
//             `------------------------.--------------------------'
//                                      v
//                           +----------------------+
//                           | Resolved renderer    |
//                           +----------------------+
//                                      |
//                             Software | other renderers
//                    ,-----------------+-----------------.
//                    v                                   v
//                   [E]                         +----------------+
//                                               |    Platform    |
//                                               +----------------+
//                                                        |
//                    ,-----------------------.-----------+------.-----------------.
//                    |                       |                  |                 |
//              Linux Wayland           Linux X11 /            macOS            Android
//                    |                   Windows                |                 |
//                    v                       v                  v                 v
//                   [A]                     [B]                [C]               [D]
//
// [A] LINUX WAYLAND
//                                        |
//                                        v
//                             +---------------------+
//                             |  Resolved renderer  |
//                             +---------------------+
//                                        |
// Auto [default] / Vulkan                |                                       OpenGl
//              ,-------------------------+-------------------------------------------.
//              |                                                                     |
//              v                                                                     |
//   +---------------------+                                                          |
//   |    GPU preference   |                                                          |
//   +---------------------+                                                          |
//              |                                                                     |
//              |                                                                     |
//              +-------------------------+-------------------------.                 |
//              |                         |                         |                 |
//       Auto [default]              Integrated                 Discrete              |
//              |                         |                         |                 |
//              v                         v                         v                 |
//   +---------------------+   +---------------------+   +---------------------+      |
//   |        Vulkan       |   |        Vulkan       |   |        Vulkan       |      |
//   |    compositor GPU   |   |    iGPU preferred   |   |    dGPU preferred   |      |
//   +---------------------+   +---------------------+   +---------------------+      |
//              |                         |                         |                 |
//              | failure                 | failure                 | failure         |
//              |                         |                         |                 |
//              |                         |                         |                 |
//              `-------------------------+-------------------------'                 |
//                                        |                                           |
//                                        +-------------------------------------------'
//                                        v
//                             +---------------------+
//                             |        OpenGL       |
//                             +---------------------+
//                                        | failure
//                                        |
//                                        v
//                             +---------------------+
//                             |       Software      |
//                             +---------------------+
//
// [B] LINUX X11 / WINDOWS
//                                        |
//                                        v
//                             +---------------------+
//                             |  Resolved renderer  |
//                             +---------------------+
//                                        |
// Auto [default] / Vulkan                |                                       OpenGl
//              ,-------------------------+-------------------------------------------.
//              |                                                                     |
//              v                                                                     |
//   +---------------------+                                                          |
//   |    GPU preference   |                                                          |
//   +---------------------+                                                          |
//              |                                                                     |
//              |                                                                     |
//              +---------------------------------------------------.                 |
//              |                                                   |                 |
//       Auto [default]                                         Discrete              |
//        / Integrated                                              |                 |
//              v                                                   v                 |
//   +---------------------+                             +---------------------+      |
//   |        Vulkan       |                             |        Vulkan       |      |
//   |    iGPU preferred   |                             |    dGPU preferred   |      |
//   +---------------------+                             +---------------------+      |
//              |                                                   |                 |
//              | failure                                           | failure         |
//              |                                                   |                 |
//              |                                                   |                 |
//              `-------------------------+-------------------------'                 |
//                                        |                                           |
//                                        +-------------------------------------------'
//                                        v
//                             +---------------------+
//                             |        OpenGL       |
//                             +---------------------+
//                                        | failure
//                                        |
//                                        v
//                             +---------------------+
//                             |       Software      |
//                             +---------------------+
//
// [C] MACOS
//             |
//             v
//   +-------------------+
//   |       Metal       |
//   |   platform GPU    |
//   +-------------------+
//             | failure
//             v
//   +-------------------+
//   |       Panic       |
//   +-------------------+
//
// [D] ANDROID
//             |
//             v
//   +-------------------+
//   |      OpenGL       |
//   |   platform GPU    |
//   +-------------------+
//             | failure
//             v
//   +-------------------+
//   |     Software      |
//   +-------------------+
//
// [E] EXPLICIT SOFTWARE
//             |
//             v
//   +-------------------+            +-------------------+
//   | Software          |--failure-->|     Platform      |
//   +-------------------+            +-------------------+
//                                        |           |
//                                  macOS |           | Linux / Windows / Android
//                                        v           v
//                                +-----------+ +-----------+           +----------+
//                                | Metal     | | OpenGL    |--failure->| Software |
//                                +-----------+ +-----------+           +----------+
//
// After a driver failure, keep the window and try OpenGL, then software, without retrying Vulkan.
//
//                          +-------------------+
//   Linux/Windows/Android  | Existing window   |  other platforms / no gpu
//   with gpu  ,------------|                   |-------------.
//             v            +-------------------+             |
//   +-------------------+                                    |
//   | OpenGL            |                                    |
//   +-------------------+                                    |
//             | failure                                      |
//             `----------------------.-----------------------'
//                                    v
//                          +-------------------+
//                          | Software          |
//                          +-------------------+

#[cfg(all(
    any(target_os = "linux", target_os = "windows", target_os = "android"),
    feature = "gpu"
))]
mod gl;
#[cfg(all(target_os = "macos", feature = "gpu"))]
mod metal;
mod software;
mod surface;
#[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
mod vulkan;
#[cfg(all(target_os = "linux", feature = "gpu"))]
mod wayland_gpu;

use freya_engine::prelude::Surface as SkiaSurface;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{
        Window,
        WindowAttributes,
    },
};

use crate::config::{
    GpuPreference,
    RendererPreference,
};

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn wait_for_display(window: &Window, last_present: &mut Option<std::time::Instant>) {
    use std::time::{
        Duration,
        Instant,
    };

    #[cfg(target_os = "linux")]
    {
        use raw_window_handle::{
            HasDisplayHandle,
            RawDisplayHandle,
        };

        if !matches!(
            window.display_handle().map(|handle| handle.as_raw()),
            Ok(RawDisplayHandle::Xlib(_) | RawDisplayHandle::Xcb(_))
        ) {
            return;
        }
    }

    let refresh_rate = window
        .current_monitor()
        .and_then(|monitor| monitor.refresh_rate_millihertz())
        .filter(|refresh_rate| *refresh_rate > 1_000)
        .unwrap_or(60_000);
    let frame_interval = Duration::from_secs_f64(1000.0 / refresh_rate as f64);

    if let Some(last_present) = last_present
        && let Some(remaining) = frame_interval.checked_sub(last_present.elapsed())
    {
        std::thread::sleep(remaining);
    }

    *last_present = Some(Instant::now());
}

/// Unrecoverable graphics error requiring a driver rebuild.
#[derive(Debug)]
// Used by the Vulkan driver.
#[cfg_attr(
    not(all(any(target_os = "linux", target_os = "windows"), feature = "gpu")),
    allow(dead_code)
)]
pub enum DriverError {
    DeviceLost,
    OutOfMemory,
}

/// Shared Vulkan context.
#[derive(Default)]
pub struct GraphicsContext {
    #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
    vulkan: vulkan::SharedVulkan,
}

#[allow(clippy::large_enum_variant)]
pub enum GraphicsDriver {
    #[cfg(all(
        any(target_os = "linux", target_os = "windows", target_os = "android"),
        feature = "gpu"
    ))]
    OpenGl(gl::OpenGLDriver),
    #[cfg(all(target_os = "macos", feature = "gpu"))]
    Metal(metal::MetalDriver),
    #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
    Vulkan(vulkan::VulkanDriver),
    Software(software::SoftwareDriver),
}

impl GraphicsDriver {
    #[allow(clippy::needless_return, unreachable_code, unused_variables)]
    pub fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        gpu_resource_cache_limit: usize,
        preference: RendererPreference,
        graphics_context: &mut GraphicsContext,
    ) -> (Self, Window) {
        let renderer = if preference == RendererPreference::Auto {
            match std::env::var("FREYA_RENDERER")
                .unwrap_or_default()
                .to_ascii_lowercase()
                .as_str()
            {
                "software" => RendererPreference::Software,
                "opengl" => RendererPreference::OpenGl,
                "vulkan" => RendererPreference::Vulkan {
                    gpu_preference: GpuPreference::Auto,
                },
                _ => RendererPreference::Auto,
            }
        } else {
            preference
        };
        tracing::info!(?preference, ?renderer, "Selecting graphics renderer");

        if renderer == RendererPreference::Software {
            match software::SoftwareDriver::new(event_loop, window_attributes.clone()) {
                Ok((driver, window)) => {
                    tracing::info!("Selected software renderer");
                    return (Self::Software(driver), window);
                }
                Err(err) => {
                    tracing::warn!(
                        "Software renderer initialization failed, falling back to default: {err}"
                    );
                }
            }
        }

        #[cfg(all(target_os = "macos", feature = "gpu"))]
        {
            let (driver, window) =
                metal::MetalDriver::new(event_loop, window_attributes, gpu_resource_cache_limit);

            return (Self::Metal(driver), window);
        }

        #[cfg(all(target_os = "android", feature = "gpu"))]
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
        }

        #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
        {
            if matches!(
                renderer,
                RendererPreference::Auto | RendererPreference::Vulkan { .. }
            ) {
                match vulkan::VulkanDriver::new(
                    event_loop,
                    window_attributes.clone(),
                    gpu_resource_cache_limit,
                    renderer.gpu_preference(),
                    &mut graphics_context.vulkan,
                ) {
                    Ok((driver, window)) => {
                        tracing::info!(gpu = %driver.gpu_name(), "Selected Vulkan renderer");
                        return (Self::Vulkan(driver), window);
                    }
                    Err(err) => {
                        tracing::warn!("Vulkan initialization failed, falling back: {err}");
                    }
                }
            }

            match gl::OpenGLDriver::new(
                event_loop,
                window_attributes.clone(),
                gpu_resource_cache_limit,
            ) {
                Ok((driver, window)) => {
                    let gpu_name = driver.gpu_name.as_deref().unwrap_or_default();

                    if renderer == RendererPreference::OpenGl
                        || !gpu_name.contains("Microsoft Basic Render Driver")
                    {
                        tracing::info!(gpu = %gpu_name, "Selected OpenGL renderer");
                        return (Self::OpenGl(driver), window);
                    }

                    tracing::warn!(
                        "OpenGL selected Microsoft's software renderer ({gpu_name}), using Freya's software renderer instead"
                    );
                }
                Err(err) => {
                    tracing::warn!("OpenGL initialization failed, falling back to software: {err}");
                }
            }
        }

        let (driver, window) = software::SoftwareDriver::new(event_loop, window_attributes)
            .expect("Failed to initialize software renderer");
        tracing::info!("Selected software renderer fallback");
        (Self::Software(driver), window)
    }

    /// Rebuild the driver on the existing window, skipping Vulkan.
    #[cfg_attr(
        any(target_os = "macos", not(feature = "gpu")),
        allow(unused_variables)
    )]
    pub fn recover_reusing_window(
        event_loop: &ActiveEventLoop,
        window: &Window,
        gpu_resource_cache_limit: usize,
        transparent: bool,
    ) -> Self {
        #[cfg(all(
            any(target_os = "linux", target_os = "windows", target_os = "android"),
            feature = "gpu"
        ))]
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
            #[cfg(all(
                any(target_os = "linux", target_os = "windows", target_os = "android"),
                feature = "gpu"
            ))]
            Self::OpenGl(gl) => {
                gl.present(window, render);
                Ok(())
            }
            #[cfg(all(target_os = "macos", feature = "gpu"))]
            Self::Metal(mtl) => {
                mtl.present(size, window, render);
                Ok(())
            }
            #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
            Self::Vulkan(vk) => vk.present(size, window, render),
            Self::Software(sw) => {
                sw.present(size, window, render);
                Ok(())
            }
        }
    }

    pub fn resource_cache_usage(&self) -> Option<(usize, usize)> {
        match self {
            #[cfg(all(
                any(target_os = "linux", target_os = "windows", target_os = "android"),
                feature = "gpu"
            ))]
            Self::OpenGl(driver) => Some(driver.resource_cache_usage()),
            #[cfg(all(target_os = "macos", feature = "gpu"))]
            Self::Metal(driver) => Some(driver.resource_cache_usage()),
            #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
            Self::Vulkan(driver) => Some(driver.resource_cache_usage()),
            Self::Software(_) => None,
        }
    }

    /// The name of the active graphics driver.
    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(all(
                any(target_os = "linux", target_os = "windows", target_os = "android"),
                feature = "gpu"
            ))]
            Self::OpenGl(_) => "OpenGL",
            #[cfg(all(target_os = "macos", feature = "gpu"))]
            Self::Metal(_) => "Metal",
            #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
            Self::Vulkan(_) => "Vulkan",
            Self::Software(_) => "Software",
        }
    }

    /// The name of the GPU picked by the driver, only known for OpenGL and Vulkan.
    pub fn gpu_name(&self) -> Option<&str> {
        match self {
            #[cfg(all(
                any(target_os = "linux", target_os = "windows", target_os = "android"),
                feature = "gpu"
            ))]
            Self::OpenGl(gl) => gl.gpu_name.as_deref(),
            #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
            Self::Vulkan(vk) => Some(vk.gpu_name()),
            _ => None,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> Result<(), DriverError> {
        match self {
            #[cfg(all(
                any(target_os = "linux", target_os = "windows", target_os = "android"),
                feature = "gpu"
            ))]
            Self::OpenGl(gl) => {
                gl.resize(size);
                Ok(())
            }
            #[cfg(all(target_os = "macos", feature = "gpu"))]
            Self::Metal(mtl) => {
                mtl.resize(size);
                Ok(())
            }
            #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
            Self::Vulkan(vk) => vk.resize(size),
            Self::Software(sw) => {
                sw.resize(size);
                Ok(())
            }
        }
    }
}
