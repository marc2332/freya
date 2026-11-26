use std::{
    cell::RefCell,
    ffi::{
        CStr,
        CString,
        c_char,
    },
    mem::ManuallyDrop,
    ptr,
    rc::{
        Rc,
        Weak,
    },
};

use ash::{
    Device,
    Entry,
    Instance,
    khr::{
        surface::Instance as InstanceSurfaceFns,
        swapchain::Device as DeviceSwapchainFns,
    },
    vk::{
        API_VERSION_1_0,
        ApplicationInfo,
        ColorSpaceKHR,
        CompositeAlphaFlagsKHR,
        DeviceCreateInfo,
        DeviceQueueCreateInfo,
        Extent2D,
        Fence,
        FenceCreateInfo,
        Format,
        Handle,
        Image,
        ImageUsageFlags,
        InstanceCreateInfo,
        KHR_SWAPCHAIN_NAME,
        PhysicalDevice,
        PhysicalDeviceFeatures,
        PhysicalDeviceType,
        PresentInfoKHR,
        PresentModeKHR,
        Queue,
        QueueFlags,
        Semaphore,
        SemaphoreCreateInfo,
        SharingMode,
        SurfaceKHR,
        SwapchainCreateInfoKHR,
        SwapchainKHR,
        api_version_major,
        api_version_minor,
        api_version_patch,
        make_api_version,
    },
};
use ash_window::enumerate_required_extensions;
use freya_engine::prelude::{
    ColorType,
    DirectContext,
    Surface as SkiaSurface,
    SurfaceOrigin,
    backend_render_targets,
    direct_contexts,
    gpu::{
        ContextOptions,
        FlushInfo,
        MutableTextureState,
        SemaphoresSubmitted,
        SyncCpu,
        backend_semaphores,
    },
    vk,
};
use raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle,
    RawDisplayHandle,
};
use rustc_hash::FxHashMap;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{
        Window,
        WindowAttributes,
    },
};

use crate::{
    config::GpuPreference,
    drivers::{
        DriverError,
        surface::wrap_render_target,
    },
};

/// Vulkan device extensions used by Skia.
const DEVICE_EXTENSIONS: &[&CStr] = &[KHR_SWAPCHAIN_NAME];

/// DRM device major and minor numbers.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DrmDevice(pub u32, pub u32);

/// Weak references to Vulkan contexts shared by GPU selection policy.
#[derive(Default)]
pub struct SharedVulkan {
    contexts: FxHashMap<GpuPreference, Weak<VulkanInner>>,
    #[cfg(target_os = "linux")]
    compositor_device: std::cell::OnceCell<Option<DrmDevice>>,
}

impl SharedVulkan {
    /// Get or create the shared context and window surface.
    fn acquire(
        &mut self,
        window: &dyn Window,
        gpu_resource_cache_limit: usize,
        gpu_preference: GpuPreference,
    ) -> Result<(Rc<VulkanInner>, SurfaceKHR), Box<dyn std::error::Error>> {
        if let Some(inner) = self.contexts.get(&gpu_preference).and_then(Weak::upgrade) {
            let surface = inner.create_presentable_surface(window)?;
            tracing::info!(gpu = %inner.gpu_name, "Reusing Vulkan GPU context");
            return Ok((inner, surface));
        }

        #[cfg(target_os = "linux")]
        let compositor_device = if gpu_preference == GpuPreference::Auto
            && matches!(
                window.display_handle()?.as_raw(),
                RawDisplayHandle::Wayland(_)
            ) {
            Some(
                self.compositor_device
                    .get_or_init(|| crate::drivers::wayland_gpu::preferred_device(window))
                    .ok_or("Wayland's preferred GPU is unavailable, trying OpenGL")?,
            )
        } else {
            None
        };
        #[cfg(target_os = "windows")]
        let compositor_device = None;

        tracing::info!(?gpu_preference, ?compositor_device, "Selecting Vulkan GPU");

        let (inner, surface) = VulkanInner::new(
            window,
            gpu_resource_cache_limit,
            gpu_preference,
            compositor_device,
        )?;
        self.contexts.insert(gpu_preference, Rc::downgrade(&inner));

        Ok((inner, surface))
    }
}

/// Shared Vulkan and Skia state.
struct VulkanInner {
    entry: Entry,
    instance: Instance,
    surface_fns: InstanceSurfaceFns,
    swapchain_fns: DeviceSwapchainFns,
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    device: Device,
    queue: Queue,
    gr_context: RefCell<ManuallyDrop<DirectContext>>,
    gpu_name: String,
}

impl Drop for VulkanInner {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.device_wait_idle();
            // Drop Skia before the Vulkan device.
            ManuallyDrop::drop(self.gr_context.get_mut());
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

impl VulkanInner {
    /// Create shared Vulkan and Skia state.
    fn new(
        window: &dyn Window,
        gpu_resource_cache_limit: usize,
        gpu_preference: GpuPreference,
        compositor_device: Option<DrmDevice>,
    ) -> Result<(Rc<Self>, SurfaceKHR), Box<dyn std::error::Error>> {
        let entry = unsafe { Entry::load()? };

        // Use the Vulkan loader's API version for Skia.
        let api_version =
            unsafe { entry.try_enumerate_instance_version() }?.unwrap_or(API_VERSION_1_0);
        if compositor_device.is_some() && api_version < ash::vk::API_VERSION_1_1 {
            return Err("Matching Wayland's GPU requires Vulkan 1.1".into());
        }

        let display_handle = window.display_handle()?.as_raw();
        let window_handle = window.window_handle()?.as_raw();
        let instance_extensions = enumerate_required_extensions(display_handle)?;
        let instance = create_instance(&entry, instance_extensions, api_version)?;
        let surface_fns = InstanceSurfaceFns::new(&entry, &instance);
        let surface = match unsafe {
            ash_window::create_surface(&entry, &instance, display_handle, window_handle, None)
        } {
            Ok(surface) => surface,
            Err(error) => {
                unsafe { instance.destroy_instance(None) };
                return Err(error.into());
            }
        };

        let (physical_device, queue_family_index, gpu_name) = match pick_physical_device(
            &instance,
            &surface_fns,
            surface,
            gpu_preference,
            compositor_device,
        ) {
            Ok(device) => device,
            Err(error) => {
                unsafe {
                    surface_fns.destroy_surface(surface, None);
                    instance.destroy_instance(None);
                }
                return Err(error);
            }
        };

        let (device, queue) =
            match create_logical_device(&instance, physical_device, queue_family_index) {
                Ok(created) => created,
                Err(error) => {
                    unsafe {
                        surface_fns.destroy_surface(surface, None);
                        instance.destroy_instance(None);
                    }
                    return Err(error);
                }
            };

        let gr_context = match create_gr_context(
            &entry,
            &instance,
            physical_device,
            &device,
            queue,
            queue_family_index,
            gpu_resource_cache_limit,
            instance_extensions,
            api_version,
        ) {
            Ok(context) => context,
            Err(error) => {
                unsafe {
                    device.destroy_device(None);
                    surface_fns.destroy_surface(surface, None);
                    instance.destroy_instance(None);
                }
                return Err(error);
            }
        };

        let swapchain_fns = DeviceSwapchainFns::new(&instance, &device);

        let inner = Rc::new(Self {
            entry,
            instance,
            surface_fns,
            swapchain_fns,
            physical_device,
            queue_family_index,
            device,
            queue,
            gr_context: RefCell::new(ManuallyDrop::new(gr_context)),
            gpu_name,
        });

        Ok((inner, surface))
    }

    /// Create a presentable window surface.
    fn create_presentable_surface(
        &self,
        window: &dyn Window,
    ) -> Result<SurfaceKHR, Box<dyn std::error::Error>> {
        let surface = self.create_surface(window)?;
        let supported = unsafe {
            self.surface_fns.get_physical_device_surface_support(
                self.physical_device,
                self.queue_family_index,
                surface,
            )?
        };

        if !supported {
            unsafe { self.surface_fns.destroy_surface(surface, None) };
            return Err("the shared Vulkan device cannot present to this window".into());
        }

        Ok(surface)
    }

    fn create_surface(
        &self,
        window: &dyn Window,
    ) -> Result<SurfaceKHR, Box<dyn std::error::Error>> {
        Ok(unsafe {
            ash_window::create_surface(
                &self.entry,
                &self.instance,
                window.display_handle()?.as_raw(),
                window.window_handle()?.as_raw(),
                None,
            )?
        })
    }
}

/// Graphics driver using Vulkan.
pub struct VulkanDriver {
    shared: Rc<VulkanInner>,
    surface: SurfaceKHR,
    swapchain: SwapchainKHR,
    swapchain_images: Vec<Image>,
    swapchain_extent: Extent2D,
    transparent: bool,
    wayland: bool,
    /// One per swapchain image: Skia signals it, and the present of that image waits on it.
    present_ready: Vec<Semaphore>,
    present_layout: MutableTextureState,
    /// Signalled by each acquire.
    in_flight_fence: Fence,
    gpu_cache_purged: bool,
}

impl Drop for VulkanDriver {
    fn drop(&mut self) {
        // Flush pending Skia work.
        if let Err(error) = self.shared.gr_context.borrow_mut().flush_and_submit() {
            tracing::error!(
                ?error,
                "Failed to submit Vulkan rendering commands during teardown"
            );
        }
        let device = &self.shared.device;
        unsafe {
            let _ = device.device_wait_idle();
            for semaphore in &self.present_ready {
                device.destroy_semaphore(*semaphore, None);
            }
            device.destroy_fence(self.in_flight_fence, None);
            self.shared
                .swapchain_fns
                .destroy_swapchain(self.swapchain, None);
            self.shared.surface_fns.destroy_surface(self.surface, None);
        }
    }
}

impl VulkanDriver {
    pub fn resource_cache_usage(&self) -> (usize, usize) {
        let gr_context = self.shared.gr_context.borrow();
        let usage = gr_context.resource_cache_usage();
        (usage.resource_bytes, gr_context.resource_cache_limit())
    }

    pub fn new(
        event_loop: &dyn ActiveEventLoop,
        window_attributes: WindowAttributes,
        gpu_resource_cache_limit: usize,
        gpu_preference: GpuPreference,
        shared_context: &mut SharedVulkan,
    ) -> Result<(Self, Box<dyn Window>), Box<dyn std::error::Error>> {
        let transparent = window_attributes.transparent();
        let window = event_loop.create_window(window_attributes)?;

        let (shared, surface) =
            shared_context.acquire(window.as_ref(), gpu_resource_cache_limit, gpu_preference)?;
        let wayland = matches!(
            window.display_handle()?.as_raw(),
            RawDisplayHandle::Wayland(_)
        );

        // Initialize handles for partial cleanup.
        let mut driver = Self {
            shared,
            surface,
            swapchain: SwapchainKHR::null(),
            swapchain_images: Vec::new(),
            swapchain_extent: Extent2D::default(),
            transparent,
            wayland,
            present_ready: Vec::new(),
            present_layout: vk::mutable_texture_states::new_vulkan(
                vk::ImageLayout::PRESENT_SRC_KHR,
                ash::vk::QUEUE_FAMILY_IGNORED,
            ),
            in_flight_fence: Fence::null(),
            gpu_cache_purged: false,
        };

        let (swapchain, swapchain_images, swapchain_extent) = create_swapchain(
            &driver.shared,
            surface,
            window.surface_size(),
            None,
            transparent,
            wayland,
        )?;
        driver.swapchain = swapchain;
        driver.swapchain_images = swapchain_images;
        driver.swapchain_extent = swapchain_extent;

        driver.present_ready =
            create_semaphores(&driver.shared.device, driver.swapchain_images.len())?;
        // Unsignalled: the first acquire signals it.
        driver.in_flight_fence = unsafe {
            driver
                .shared
                .device
                .create_fence(&FenceCreateInfo::default(), None)?
        };

        Ok((driver, window))
    }

    /// Return the GPU name.
    pub(crate) fn gpu_name(&self) -> &str {
        &self.shared.gpu_name
    }

    fn recreate_swapchain(&mut self, size: PhysicalSize<u32>) -> Result<(), DriverError> {
        unsafe {
            self.shared
                .device
                .device_wait_idle()
                .map_err(Self::vulkan_to_driver_error)?;
        }
        let old_swapchain = self.swapchain;
        let (swapchain, swapchain_images, swapchain_extent) = create_swapchain(
            &self.shared,
            self.surface,
            size,
            Some(old_swapchain),
            self.transparent,
            self.wayland,
        )
        .map_err(|error| {
            tracing::error!("Failed to recreate Vulkan swapchain: {error}");
            DriverError::DeviceLost
        })?;
        self.swapchain = swapchain;
        self.swapchain_images = swapchain_images;
        self.swapchain_extent = swapchain_extent;
        unsafe {
            self.shared
                .swapchain_fns
                .destroy_swapchain(old_swapchain, None);
            for semaphore in self.present_ready.drain(..) {
                self.shared.device.destroy_semaphore(semaphore, None);
            }
        }
        self.present_ready = create_semaphores(&self.shared.device, self.swapchain_images.len())
            .map_err(|error| {
                tracing::error!("Failed to create Vulkan present semaphores: {error}");
                DriverError::DeviceLost
            })?;
        Ok(())
    }

    pub fn present(
        &mut self,
        size: PhysicalSize<u32>,
        window: &dyn Window,
        render: impl FnOnce(&mut SkiaSurface),
    ) -> Result<(), DriverError> {
        if size.width == 0 || size.height == 0 {
            return Ok(());
        }

        // The acquire signals the fence once the image may be written. Skia's own submission then draws into it,
        // so the drawing never starts before the image is ours. It waits at most 50 ms: a window that cannot be
        // shown now (hidden, being rotated, suspended) tries again on the next redraw instead of blocking.
        let (image_index, suboptimal) = match unsafe {
            self.shared.swapchain_fns.acquire_next_image(
                self.swapchain,
                50_000_000,
                Semaphore::null(),
                self.in_flight_fence,
            )
        } {
            Ok(acquired) => acquired,
            Err(ash::vk::Result::TIMEOUT | ash::vk::Result::NOT_READY) => {
                window.request_redraw();
                return Ok(());
            }
            Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                self.recreate_swapchain(size)?;
                window.request_redraw();
                return Ok(());
            }
            Err(ash::vk::Result::ERROR_SURFACE_LOST_KHR) => {
                self.recreate_surface(window, size)?;
                window.request_redraw();
                return Ok(());
            }
            Err(error) => match Self::vulkan_to_driver_error(error) {
                DriverError::OutOfMemory if !self.gpu_cache_purged => {
                    tracing::warn!("Vulkan out of memory acquiring image, purging GPU cache");
                    self.shared.gr_context.borrow_mut().free_gpu_resources();
                    self.gpu_cache_purged = true;
                    window.request_redraw();
                    return Ok(());
                }
                driver_error => return Err(driver_error),
            },
        };

        self.gpu_cache_purged = false;

        unsafe {
            self.shared
                .device
                .wait_for_fences(&[self.in_flight_fence], true, u64::MAX)
                .map_err(Self::vulkan_to_driver_error)?;
            self.shared
                .device
                .reset_fences(&[self.in_flight_fence])
                .map_err(Self::vulkan_to_driver_error)?;
        }

        let image = self.swapchain_images[image_index as usize];
        // Reacquiring this image proved its previous present no longer waits on this semaphore.
        let present_ready = self.present_ready[image_index as usize];

        let alloc = vk::Alloc::default();
        let sk_image_info = unsafe {
            vk::ImageInfo::new(
                image.as_raw() as _,
                alloc,
                vk::ImageTiling::OPTIMAL,
                vk::ImageLayout::UNDEFINED,
                vk::Format::B8G8R8A8_UNORM,
                1,
                None,
                None,
                None,
                vk::SharingMode::EXCLUSIVE,
            )
        };
        let render_target = backend_render_targets::make_vk(
            (
                self.swapchain_extent.width as i32,
                self.swapchain_extent.height as i32,
            ),
            &sk_image_info,
        );

        let mut surface = wrap_render_target(
            &mut self.shared.gr_context.borrow_mut(),
            &render_target,
            SurfaceOrigin::TopLeft,
            ColorType::BGRA8888,
        )
        .ok_or(DriverError::DeviceLost)?;

        render(&mut surface);

        window.pre_present_notify();

        // Skia moves the image to the presentation layout and signals this image's semaphore in its own
        // submission: no command buffer or submit of ours.
        {
            let mut semaphores =
                [unsafe { backend_semaphores::make_vk(present_ready.as_raw() as _) }];
            let mut flush = FlushInfo::default();
            unsafe { flush.set_signal_semaphores(&mut semaphores) };
            let mut context = self.shared.gr_context.borrow_mut();
            let flushed = context
                .flush_surface_with_texture_state(&mut surface, &flush, Some(&self.present_layout))
                .map_err(|error| {
                    tracing::error!(?error, "Failed to submit Vulkan rendering commands");
                    DriverError::DeviceLost
                })?;
            if flushed != SemaphoresSubmitted::Yes || !context.submit(SyncCpu::No) {
                return Err(DriverError::DeviceLost);
            }
        }

        let wait_semaphores = [present_ready];
        let swapchains = [self.swapchain];
        let image_indices = [image_index];
        let present_info = PresentInfoKHR::default()
            .wait_semaphores(&wait_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        let result = unsafe {
            self.shared
                .swapchain_fns
                .queue_present(self.shared.queue, &present_info)
        };

        drop(surface);

        match result {
            Ok(_) => {}
            Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                self.recreate_swapchain(size)?;
                window.request_redraw();
                return Ok(());
            }
            Err(ash::vk::Result::ERROR_SURFACE_LOST_KHR) => {
                self.recreate_surface(window, size)?;
                window.request_redraw();
                return Ok(());
            }
            Err(error) => return Err(Self::vulkan_to_driver_error(error)),
        }

        if suboptimal {
            self.recreate_swapchain(size)?;
        }

        Ok(())
    }

    /// Recreate the surface and its swapchain after the surface was lost.
    fn recreate_surface(
        &mut self,
        window: &dyn Window,
        size: PhysicalSize<u32>,
    ) -> Result<(), DriverError> {
        unsafe {
            self.shared
                .device
                .device_wait_idle()
                .map_err(Self::vulkan_to_driver_error)?;
            self.shared
                .swapchain_fns
                .destroy_swapchain(self.swapchain, None);
            self.shared.surface_fns.destroy_surface(self.surface, None);
        }
        // Initialize handles for partial cleanup.
        self.swapchain = SwapchainKHR::null();
        self.surface = SurfaceKHR::null();

        self.surface = self.shared.create_surface(window).map_err(|error| {
            tracing::error!("Failed to recreate the Vulkan surface: {error}");
            DriverError::DeviceLost
        })?;

        self.recreate_swapchain(size)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> Result<(), DriverError> {
        if size.width == 0 || size.height == 0 {
            return Ok(());
        }
        self.recreate_swapchain(size)
    }

    /// Map Vulkan errors to driver errors.
    fn vulkan_to_driver_error(error: ash::vk::Result) -> DriverError {
        match error {
            ash::vk::Result::ERROR_OUT_OF_DEVICE_MEMORY
            | ash::vk::Result::ERROR_OUT_OF_HOST_MEMORY => DriverError::OutOfMemory,
            ash::vk::Result::ERROR_DEVICE_LOST => DriverError::DeviceLost,
            other => {
                tracing::error!("Unexpected Vulkan error, treating as device loss: {other:?}");
                DriverError::DeviceLost
            }
        }
    }
}

fn create_instance(
    entry: &Entry,
    extension_names: &[*const c_char],
    api_version: u32,
) -> Result<Instance, Box<dyn std::error::Error>> {
    let app_name = CString::new("AnyRender")?;
    let engine_name = CString::new("No Engine")?;
    let app_info = ApplicationInfo::default()
        .application_name(&app_name)
        .application_version(make_api_version(0, 1, 0, 0))
        .engine_name(&engine_name)
        .engine_version(make_api_version(0, 1, 0, 0))
        .api_version(api_version);

    let create_info = InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_extension_names(extension_names);

    Ok(unsafe { entry.create_instance(&create_info, None)? })
}

fn pick_physical_device(
    instance: &Instance,
    surface_fns: &InstanceSurfaceFns,
    surface: SurfaceKHR,
    gpu_preference: GpuPreference,
    compositor_device: Option<DrmDevice>,
) -> Result<(PhysicalDevice, u32, String), Box<dyn std::error::Error>> {
    let prefer_discrete = gpu_preference == GpuPreference::Discrete;
    let devices = unsafe { instance.enumerate_physical_devices()? };
    devices
        .into_iter()
        .filter_map(|physical_device| {
            let properties = unsafe { instance.get_physical_device_properties(physical_device) };
            let gpu_name = properties.device_name_as_c_str().unwrap_or(c"Unknown GPU");
            tracing::info!(
                gpu = ?gpu_name,
                device_type = ?properties.device_type,
                "Considering Vulkan GPU"
            );

            let rank = match properties.device_type {
                PhysicalDeviceType::INTEGRATED_GPU => if prefer_discrete { 1 } else { 0 },
                PhysicalDeviceType::DISCRETE_GPU => if prefer_discrete { 0 } else { 1 },
                PhysicalDeviceType::VIRTUAL_GPU => 2,
                PhysicalDeviceType::OTHER => 3,
                _ => return None,
            };
            let extensions =
                unsafe { instance.enumerate_device_extension_properties(physical_device) }.ok()?;
            let supports_extension = |name| {
                extensions.iter().any(|extension| {
                    (unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) }) == name
                })
            };
            if !supports_extension(KHR_SWAPCHAIN_NAME) {
                tracing::info!(
                    gpu = ?gpu_name,
                    "Skipping Vulkan GPU without swapchain support"
                );
                return None;
            }

            if let Some(DrmDevice(major, minor)) = compositor_device {
                if !supports_extension(ash::vk::EXT_PHYSICAL_DEVICE_DRM_NAME) {
                    tracing::info!(
                        gpu = ?gpu_name,
                        "Skipping Vulkan GPU without DRM device properties"
                    );
                    return None;
                }

                let mut drm = ash::vk::PhysicalDeviceDrmPropertiesEXT::default();
                let mut properties = ash::vk::PhysicalDeviceProperties2::default().push_next(&mut drm);
                unsafe { instance.get_physical_device_properties2(physical_device, &mut properties) };

                let device = (i64::from(major), i64::from(minor));
                // Wayland may identify the same GPU by its primary (display/rendering) or render-only DRM node.
                let matches_primary = drm.has_primary != 0
                    && (drm.primary_major, drm.primary_minor) == device;
                let matches_render = drm.has_render != 0
                    && (drm.render_major, drm.render_minor) == device;
                if !matches_primary && !matches_render {
                    tracing::info!(
                        gpu = ?gpu_name,
                        "Skipping Vulkan GPU that does not match Wayland's preferred device"
                    );
                    return None;
                }
            }

            let queue_family_index = unsafe {
                instance
                    .get_physical_device_queue_family_properties(physical_device)
                    .iter()
                    .enumerate()
                    .find_map(|(index, props)| {
                        let supports_graphics = props.queue_flags.contains(QueueFlags::GRAPHICS);
                        let supports_surface = surface_fns
                            .get_physical_device_surface_support(
                                physical_device,
                                index as u32,
                                surface,
                            )
                            .unwrap_or(false);
                        (supports_graphics && supports_surface).then_some(index as u32)
                    })
            };
            let Some(queue_family_index) = queue_family_index else {
                tracing::info!(
                    gpu = ?gpu_name,
                    "Skipping Vulkan GPU without graphics presentation support"
                );
                return None;
            };

            Some((rank, physical_device, queue_family_index, properties))
        })
        .min_by_key(|(rank, ..)| *rank)
        .map(|(_, physical_device, queue_family_index, properties)| {
            let gpu_name = properties
                .device_name_as_c_str()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            tracing::info!(
                gpu = %gpu_name,
                device_type = ?properties.device_type,
                "Selected Vulkan GPU"
            );
            (physical_device, queue_family_index, gpu_name)
        })
        .ok_or_else(|| "No suitable Vulkan physical device found, GPU-less software implementations are skipped".into())
}

fn create_logical_device(
    instance: &Instance,
    physical_device: PhysicalDevice,
    queue_family_index: u32,
) -> Result<(Device, Queue), Box<dyn std::error::Error>> {
    let queue_priorities = [1.0f32];
    let queue_create_info = DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family_index)
        .queue_priorities(&queue_priorities);

    let features = PhysicalDeviceFeatures::default().sample_rate_shading(true);

    let extensions = DEVICE_EXTENSIONS
        .iter()
        .map(|extension| extension.as_ptr())
        .collect::<Vec<_>>();

    let create_info = DeviceCreateInfo::default()
        .queue_create_infos(std::slice::from_ref(&queue_create_info))
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    let device = unsafe { instance.create_device(physical_device, &create_info, None)? };

    let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

    Ok((device, queue))
}

#[allow(clippy::type_complexity)]
fn create_swapchain(
    shared: &VulkanInner,
    surface: SurfaceKHR,
    size: PhysicalSize<u32>,
    old_swapchain: Option<SwapchainKHR>,
    transparent: bool,
    wayland: bool,
) -> Result<(SwapchainKHR, Vec<Image>, Extent2D), Box<dyn std::error::Error>> {
    let surface_caps = unsafe {
        shared
            .surface_fns
            .get_physical_device_surface_capabilities(shared.physical_device, surface)?
    };

    let surface_formats = unsafe {
        shared
            .surface_fns
            .get_physical_device_surface_formats(shared.physical_device, surface)?
    };

    let format = surface_formats
        .iter()
        .find(|f| {
            f.format == Format::B8G8R8A8_UNORM && f.color_space == ColorSpaceKHR::SRGB_NONLINEAR
        })
        .ok_or("No suitable Vulkan surface format found")?;

    // Use MAILBOX on Wayland when available.
    let present_modes = unsafe {
        shared
            .surface_fns
            .get_physical_device_surface_present_modes(shared.physical_device, surface)?
    };
    let present_mode = if wayland && present_modes.contains(&PresentModeKHR::MAILBOX) {
        PresentModeKHR::MAILBOX
    } else {
        PresentModeKHR::FIFO
    };

    let extent = if surface_caps.current_extent.width == u32::MAX {
        Extent2D {
            width: size
                .width
                .max(surface_caps.min_image_extent.width)
                .min(surface_caps.max_image_extent.width),
            height: size
                .height
                .max(surface_caps.min_image_extent.height)
                .min(surface_caps.max_image_extent.height),
        }
    } else {
        surface_caps.current_extent
    };
    // Use three images for MAILBOX.
    let wanted_image_count = if present_mode == PresentModeKHR::MAILBOX {
        3
    } else {
        2
    };
    let mut image_count = surface_caps.min_image_count.max(wanted_image_count);
    if surface_caps.max_image_count > 0 {
        image_count = image_count.min(surface_caps.max_image_count);
    }

    // Select a supported transparency mode.
    let composite_alpha = if transparent {
        if surface_caps
            .supported_composite_alpha
            .contains(CompositeAlphaFlagsKHR::PRE_MULTIPLIED)
        {
            CompositeAlphaFlagsKHR::PRE_MULTIPLIED
        } else if surface_caps
            .supported_composite_alpha
            .contains(CompositeAlphaFlagsKHR::POST_MULTIPLIED)
        {
            CompositeAlphaFlagsKHR::POST_MULTIPLIED
        } else {
            return Err("Vulkan surface does not support transparent composite alpha".into());
        }
    } else {
        CompositeAlphaFlagsKHR::OPAQUE
    };

    let create_info = SwapchainCreateInfoKHR::default()
        .surface(surface)
        .min_image_count(image_count)
        .image_format(format.format)
        .image_color_space(format.color_space)
        .image_extent(extent)
        .image_array_layers(1)
        .image_usage(
            ImageUsageFlags::COLOR_ATTACHMENT
                | ImageUsageFlags::SAMPLED
                | ImageUsageFlags::TRANSFER_SRC
                | ImageUsageFlags::TRANSFER_DST,
        )
        .image_sharing_mode(SharingMode::EXCLUSIVE)
        .queue_family_indices(std::slice::from_ref(&shared.queue_family_index))
        .pre_transform(surface_caps.current_transform)
        .composite_alpha(composite_alpha)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(old_swapchain.unwrap_or(SwapchainKHR::null()));

    let swapchain = unsafe { shared.swapchain_fns.create_swapchain(&create_info, None)? };
    let images = unsafe { shared.swapchain_fns.get_swapchain_images(swapchain)? };

    Ok((swapchain, images, extent))
}

#[allow(clippy::too_many_arguments)]
fn create_gr_context(
    entry: &Entry,
    instance: &Instance,
    physical_device: PhysicalDevice,
    device: &Device,
    queue: Queue,
    queue_family_index: u32,
    gpu_resource_cache_limit: usize,
    instance_extensions: &[*const c_char],
    api_version: u32,
) -> Result<DirectContext, Box<dyn std::error::Error>> {
    let get_proc = unsafe {
        |gpo: vk::GetProcOf| {
            let get_device_proc_addr = instance.fp_v1_0().get_device_proc_addr;

            match gpo {
                vk::GetProcOf::Instance(instance, name) => {
                    let vk_instance = ash::vk::Instance::from_raw(instance as _);
                    entry.get_instance_proc_addr(vk_instance, name)
                }
                vk::GetProcOf::Device(device, name) => {
                    let vk_device = ash::vk::Device::from_raw(device as _);
                    get_device_proc_addr(vk_device, name)
                }
            }
            .map(|f| f as _)
            .unwrap_or(ptr::null())
        }
    };

    let instance_extensions = instance_extensions
        .iter()
        .filter_map(|name| unsafe { CStr::from_ptr(*name) }.to_str().ok())
        .collect::<Vec<_>>();
    let device_extensions = DEVICE_EXTENSIONS
        .iter()
        .filter_map(|extension| extension.to_str().ok())
        .collect::<Vec<_>>();

    let max_api_version = vk::Version::new(
        api_version_major(api_version) as usize,
        api_version_minor(api_version) as usize,
        api_version_patch(api_version) as usize,
    );

    let backend_context = unsafe {
        vk::BackendContext::new_builder(
            instance.handle().as_raw() as _,
            physical_device.as_raw() as _,
            device.handle().as_raw() as _,
            (queue.as_raw() as _, queue_family_index as usize),
            &get_proc,
            Some(max_api_version),
        )
        .with_extensions(&instance_extensions, &device_extensions)
        .build()
    };

    let context_options = ContextOptions::default();

    let mut gr_context = direct_contexts::make_vulkan(&backend_context, &context_options)
        .ok_or("Failed to create Vulkan Skia context")?;

    gr_context.set_resource_cache_limit(gpu_resource_cache_limit);

    Ok(gr_context)
}

/// One present semaphore per swapchain image. On failure none of them is left behind.
fn create_semaphores(
    device: &Device,
    count: usize,
) -> Result<Vec<Semaphore>, Box<dyn std::error::Error>> {
    let mut semaphores = Vec::with_capacity(count);
    for _ in 0..count {
        match unsafe { device.create_semaphore(&SemaphoreCreateInfo::default(), None) } {
            Ok(semaphore) => semaphores.push(semaphore),
            Err(error) => {
                for semaphore in semaphores {
                    unsafe { device.destroy_semaphore(semaphore, None) };
                }
                return Err(error.into());
            }
        }
    }
    Ok(semaphores)
}
