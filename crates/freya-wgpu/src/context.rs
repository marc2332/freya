use std::{
    cell::RefCell,
    rc::Rc,
};

use freya_core::prelude::try_consume_root_context;
use freya_engine::prelude::{
    BackendTexture,
    Image as SkImage,
    MutableTextureState,
};
use freya_winit::gpu_interop::GpuInterop;

/// A texture Skia sampled this frame, held until its work is flushed and its layout restored.
pub(crate) struct SampledTexture {
    pub(crate) _texture: wgpu::Texture,
    pub(crate) _image: SkImage,
    pub(crate) backend_texture: BackendTexture,
    pub(crate) restore_state: Option<MutableTextureState>,
}

#[derive(Default)]
pub(crate) struct FrameState {
    pub(crate) sampled: Vec<SampledTexture>,
    /// Images waiting for the frames that read them to finish on the GPU.
    pub(crate) retiring: Vec<Vec<wgpu::Texture>>,
}

/// The wgpu device Freya renders on, available to every component under the window.
#[derive(Clone)]
pub struct WgpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub(crate) interop: GpuInterop,
    pub(crate) frame: Rc<RefCell<FrameState>>,
}

impl WgpuContext {
    pub(crate) fn new(device: wgpu::Device, queue: wgpu::Queue, interop: GpuInterop) -> Self {
        Self {
            frame: Rc::new(RefCell::new(FrameState {
                sampled: Vec::new(),
                retiring: vec![Vec::new(); interop.retire_frames().unwrap_or(1).max(1)],
            })),
            device,
            queue,
            interop,
        }
    }

    #[track_caller]
    pub fn try_get() -> Option<Self> {
        try_consume_root_context()
    }

    #[track_caller]
    pub fn get() -> Self {
        Self::try_get().expect(
            "No wgpu device is shared with this window, launch with `WgpuSetup::plugin` and `LaunchConfig::with_external_gpu_device`",
        )
    }

    /// Whether Freya renders on this same device, so textures can be shared.
    pub fn is_shared(&self) -> bool {
        self.interop.is_shared()
    }

    /// How many times the shared device has been lost, invalidating every texture made before.
    pub fn generation(&self) -> u64 {
        self.interop.generation()
    }

    /// Hold an image until the frames that read it have finished on the GPU.
    pub(crate) fn retire(&self, texture: wgpu::Texture) {
        self.frame.borrow_mut().retiring[0].push(texture);
    }

    /// Put every texture Skia sampled back into the layout wgpu expects, and release the images
    /// the driver has finished reading.
    pub(crate) fn end_frame(&self) {
        let sampled = {
            let mut frame = self.frame.borrow_mut();
            frame.retiring.rotate_right(1);
            frame.retiring[0].clear();
            std::mem::take(&mut frame.sampled)
        };

        if sampled.is_empty() {
            return;
        }

        // The restore needs its own submit, the frame's flush is too late for wgpu.
        self.interop.with_context(|context| {
            let mut restored_any = false;
            for entry in &sampled {
                if let Some(state) = &entry.restore_state {
                    context.set_backend_texture_state(&entry.backend_texture, state);
                    restored_any = true;
                }
            }
            if restored_any {
                context.flush_and_submit();
            }
        });
    }

    /// Forget everything without touching the GPU, for a device that is already gone.
    pub(crate) fn abandon_frame(&self) {
        let mut frame = self.frame.borrow_mut();
        frame.sampled.clear();
        for bucket in &mut frame.retiring {
            bucket.clear();
        }
    }
}
