use freya_winit::plugins::{
    FreyaPlugin,
    PluginEvent,
    PluginHandle,
};
use rustc_hash::FxHashMap;
use winit::window::WindowId;

use crate::context::WgpuContext;

/// Exposes the shared wgpu device to components and releases sampled textures each frame.
pub struct WgpuPlugin {
    device: wgpu::Device,
    queue: wgpu::Queue,
    /// Built when the runner is created, claimed once its window exists.
    pending: Option<WgpuContext>,
    contexts: FxHashMap<WindowId, WgpuContext>,
}

impl WgpuPlugin {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self {
            device,
            queue,
            pending: None,
            contexts: FxHashMap::default(),
        }
    }
}

impl FreyaPlugin for WgpuPlugin {
    fn plugin_id(&self) -> &'static str {
        "freya-wgpu"
    }

    fn on_event(&mut self, event: &mut PluginEvent, _handle: PluginHandle) {
        match event {
            PluginEvent::RunnerCreated {
                runner,
                gpu_interop,
            } => {
                if !gpu_interop.is_shared() {
                    tracing::warn!(
                        "Freya is not rendering on the wgpu device, textures will not be shared"
                    );
                }

                let context = WgpuContext::new(
                    self.device.clone(),
                    self.queue.clone(),
                    (*gpu_interop).clone(),
                );
                runner.provide_root_context(|| context.clone());
                self.pending = Some(context);
            }
            PluginEvent::WindowCreated { window, .. } => {
                if let Some(context) = self.pending.take() {
                    self.contexts.insert(window.id(), context);
                }
            }
            PluginEvent::AfterRedraw { window, .. } => {
                if let Some(context) = self.contexts.get(&window.id()) {
                    context.end_frame();
                }
            }
            PluginEvent::GraphicsDriverChanged { window, .. } => {
                if let Some(context) = self.contexts.get(&window.id()) {
                    context.abandon_frame();
                }
            }
            PluginEvent::WindowClosed { window, .. } => {
                if let Some(context) = self.contexts.remove(&window.id()) {
                    context.abandon_frame();
                }
            }
            _ => {}
        }
    }
}
