//! Share GPU textures between a wgpu renderer and Freya.
//!
//! Works on Vulkan for Linux and Windows, and on Metal for macOS.
//!
//! ```no_run
//! # use freya_wgpu::*;
//! # use freya_winit::config::{LaunchConfig, WindowConfig};
//! # fn app() -> freya_core::element::Element { unimplemented!() }
//! # async fn launch_app() -> Result<(), Box<dyn std::error::Error>> {
//! let setup = WgpuSetup::new(WgpuSetupOptions::default()).await?;
//! let config = LaunchConfig::new()
//!     .with_external_gpu_device(setup.external_gpu_device()?)
//!     .with_plugin(setup.plugin());
//! # Ok(())
//! # }
//! ```

mod context;
mod plugin;
mod setup;
mod texture;
mod viewport;

pub use context::WgpuContext;
pub use freya_winit::gpu_interop::{
    GpuBackend,
    GpuInterop,
};
pub use plugin::WgpuPlugin;
pub use setup::{
    WgpuSetup,
    WgpuSetupError,
    WgpuSetupOptions,
};
pub use texture::{
    GpuTexture,
    GpuTextureDescriptor,
    GpuTextureFormat,
    GpuTextureUsage,
};
pub use viewport::{
    WgpuFrame,
    WgpuViewer,
    WgpuViewport,
    wgpu_viewport,
};
pub use wgpu;

pub mod prelude {
    pub use crate::{
        GpuTexture,
        GpuTextureDescriptor,
        GpuTextureFormat,
        GpuTextureUsage,
        WgpuContext,
        WgpuFrame,
        WgpuPlugin,
        WgpuSetup,
        WgpuSetupOptions,
        WgpuViewer,
        wgpu_viewport,
    };
}
