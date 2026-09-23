use std::{
    cell::RefCell,
    rc::Rc,
};

use freya_core::prelude::consume_root_context;
use freya_engine::prelude::DirectContext;

/// The graphics API a shared GPU device runs on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GpuBackend {
    Vulkan {
        /// The queue family Freya submits on.
        queue_family_index: u32,
    },
    Metal,
}

pub struct SharedGpu {
    pub backend: GpuBackend,
    pub context: DirectContext,
    /// Frames a resource must outlive before the driver is done reading it.
    pub retire_frames: usize,
}

/// Access to the GPU context Freya renders with, empty unless the device came from elsewhere.
#[derive(Clone, Default)]
pub struct GpuInterop {
    inner: Rc<RefCell<GpuInteropInner>>,
}

#[derive(Default)]
struct GpuInteropInner {
    shared: Option<SharedGpu>,
    generation: u64,
}

impl GpuInterop {
    #[track_caller]
    pub fn get() -> Self {
        consume_root_context()
    }

    pub fn backend(&self) -> Option<GpuBackend> {
        self.inner
            .borrow()
            .shared
            .as_ref()
            .map(|shared| shared.backend)
    }

    /// Frames a resource must outlive before the driver is done reading it.
    pub fn retire_frames(&self) -> Option<usize> {
        self.inner
            .borrow()
            .shared
            .as_ref()
            .map(|shared| shared.retire_frames)
    }

    /// The queue family Freya submits on, only known on Vulkan.
    pub fn queue_family_index(&self) -> Option<u32> {
        match self.backend()? {
            GpuBackend::Vulkan { queue_family_index } => Some(queue_family_index),
            GpuBackend::Metal => None,
        }
    }

    /// Whether Freya renders on a device shared with an external renderer.
    pub fn is_shared(&self) -> bool {
        self.inner.borrow().shared.is_some()
    }

    /// How many times a device loss has rebuilt the driver, invalidating every resource on it.
    pub fn generation(&self) -> u64 {
        self.inner.borrow().generation
    }

    /// Run `use_context` with the Skia context Freya renders with, `None` if nothing is shared.
    pub fn with_context<T>(&self, use_context: impl FnOnce(&mut DirectContext) -> T) -> Option<T> {
        let mut inner = self.inner.borrow_mut();
        let shared = inner.shared.as_mut()?;
        Some(use_context(&mut shared.context))
    }

    pub(crate) fn share(&self, shared: SharedGpu) {
        self.inner.borrow_mut().shared = Some(shared);
    }

    /// Drop the shared context and mark every resource built on it as invalid.
    pub(crate) fn invalidate(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.shared = None;
        inner.generation += 1;
    }
}
