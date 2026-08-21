use std::{
    ffi::{
        CStr,
        CString,
        c_char,
    },
    mem::ManuallyDrop,
    ptr,
    sync::Arc,
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
        AccessFlags,
        ApplicationInfo,
        ColorSpaceKHR,
        CommandBuffer,
        CommandBufferAllocateInfo,
        CommandBufferBeginInfo,
        CommandBufferLevel,
        CommandPool,
        CommandPoolCreateFlags,
        CommandPoolCreateInfo,
        CompositeAlphaFlagsKHR,
        DependencyFlags,
        DeviceCreateInfo,
        DeviceQueueCreateInfo,
        Extent2D,
        Fence,
        FenceCreateFlags,
        FenceCreateInfo,
        Format,
        Handle,
        Image,
        ImageAspectFlags,
        ImageLayout,
        ImageMemoryBarrier,
        ImageSubresourceRange,
        ImageUsageFlags,
        InstanceCreateInfo,
        KHR_SWAPCHAIN_NAME,
        PhysicalDevice,
        PhysicalDeviceFeatures,
        PhysicalDeviceType,
        PipelineStageFlags,
        PresentInfoKHR,
        PresentModeKHR,
        Queue,
        QueueFlags,
        Semaphore,
        SemaphoreCreateInfo,
        SharingMode,
        SubmitInfo,
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
    gpu::ContextOptions,
    vk,
    wrap_backend_render_target,
};
use raw_window_handle::{
    HandleError,
    HasDisplayHandle,
    HasWindowHandle,
};
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{
        Window,
        WindowAttributes,
    },
};

use crate::drivers::DriverError;

/// Extensions enabled on the logical device and reported to Skia.
const DEVICE_EXTENSIONS: &[&CStr] = &[KHR_SWAPCHAIN_NAME];

/// Graphics driver using Vulkan.
pub struct VulkanDriver {
    entry: Entry,
    instance: Instance,
    surface_fns: InstanceSurfaceFns,
    surface: SurfaceKHR,
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    device: Arc<Device>,
    queue: Queue,
    swapchain: SwapchainKHR,
    swapchain_fns: DeviceSwapchainFns,
    swapchain_images: Vec<Image>,
    swapchain_extent: Extent2D,
    transparent: bool,
    gr_context: ManuallyDrop<DirectContext>,
    image_available_semaphore: Semaphore,
    render_finished_semaphore: Semaphore,
    in_flight_fence: Fence,
    cmd_buf: CommandBuffer,
    cmd_pool: CommandPool,
    gpu_cache_purged: bool,
    pub(crate) gpu_name: String,
}

impl Drop for VulkanDriver {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.device_wait_idle();
            // Skia must release its GPU resources while the device is still alive.
            ManuallyDrop::drop(&mut self.gr_context);
            self.device
                .destroy_semaphore(self.image_available_semaphore, None);
            self.device
                .destroy_semaphore(self.render_finished_semaphore, None);
            self.device.destroy_fence(self.in_flight_fence, None);
            self.device.destroy_command_pool(self.cmd_pool, None);
            self.swapchain_fns.destroy_swapchain(self.swapchain, None);
            self.surface_fns.destroy_surface(self.surface, None);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

impl VulkanDriver {
    pub fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        gpu_resource_cache_limit: usize,
    ) -> Result<(Self, Window), Box<dyn std::error::Error>> {
        let transparent = window_attributes.transparent;
        let window = event_loop.create_window(window_attributes)?;

        let entry = unsafe { Entry::load()? };

        // The requested version must match the one the loader reports, otherwise Skia fails to
        // create its Vulkan context.
        let api_version =
            unsafe { entry.try_enumerate_instance_version() }?.unwrap_or(API_VERSION_1_0);

        let instance_extensions = enumerate_required_extensions(window.display_handle()?.as_raw())?;
        let instance = create_instance(&entry, instance_extensions, api_version)?;
        let surface_fns = InstanceSurfaceFns::new(&entry, &instance);
        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle()?.as_raw(),
                window.window_handle()?.as_raw(),
                None,
            )?
        };

        let (physical_device, queue_family_index, gpu_name) =
            pick_physical_device(&instance, &surface_fns, surface)?;

        let (device, queue) =
            create_logical_device(&instance, physical_device, queue_family_index)?;
        let device = Arc::new(device);

        let swapchain_size = window.inner_size();

        let (swapchain, swapchain_fns, swapchain_images, swapchain_extent) = create_swapchain(
            &instance,
            &device,
            physical_device,
            &surface_fns,
            surface,
            queue_family_index,
            swapchain_size,
            None,
            transparent,
        )?;

        let gr_context = create_gr_context(
            &entry,
            &instance,
            physical_device,
            device.clone(),
            queue,
            queue_family_index,
            gpu_resource_cache_limit,
            instance_extensions,
            api_version,
        )?;

        let (image_available_semaphore, render_finished_semaphore, in_flight_fence) =
            create_sync_objects(&device)?;

        let cmd_pool = unsafe {
            device.create_command_pool(
                &CommandPoolCreateInfo::default().flags(
                    CommandPoolCreateFlags::TRANSIENT
                        | CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
                ),
                None,
            )?
        };

        let cmd_buf = unsafe {
            device.allocate_command_buffers(
                &CommandBufferAllocateInfo::default()
                    .command_pool(cmd_pool)
                    .level(CommandBufferLevel::PRIMARY)
                    .command_buffer_count(1),
            )?[0]
        };

        let driver = Self {
            entry,
            instance,
            surface_fns,
            surface,
            physical_device,
            queue_family_index,
            device,
            queue,
            swapchain,
            swapchain_fns,
            swapchain_images,
            swapchain_extent,
            transparent,
            gr_context: ManuallyDrop::new(gr_context),
            image_available_semaphore,
            render_finished_semaphore,
            in_flight_fence,
            cmd_buf,
            cmd_pool,
            gpu_cache_purged: false,
            gpu_name,
        };

        Ok((driver, window))
    }

    fn recreate_swapchain(&mut self, size: PhysicalSize<u32>) -> Result<(), DriverError> {
        unsafe {
            self.device
                .device_wait_idle()
                .map_err(Self::vulkan_to_driver_error)?;
        }
        let old_swapchain = self.swapchain;
        let (swapchain, swapchain_fns, swapchain_images, swapchain_extent) = create_swapchain(
            &self.instance,
            &self.device,
            self.physical_device,
            &self.surface_fns,
            self.surface,
            self.queue_family_index,
            size,
            Some(old_swapchain),
            self.transparent,
        )
        .map_err(|error| {
            tracing::error!("Failed to recreate Vulkan swapchain: {error}");
            DriverError::DeviceLost
        })?;
        self.swapchain = swapchain;
        self.swapchain_fns = swapchain_fns;
        self.swapchain_images = swapchain_images;
        self.swapchain_extent = swapchain_extent;
        unsafe {
            self.swapchain_fns.destroy_swapchain(old_swapchain, None);
        }
        Ok(())
    }

    pub fn present(
        &mut self,
        size: PhysicalSize<u32>,
        window: &Window,
        render: impl FnOnce(&mut SkiaSurface),
    ) -> Result<(), DriverError> {
        if size.width == 0 || size.height == 0 {
            return Ok(());
        }

        unsafe {
            self.device
                .wait_for_fences(&[self.in_flight_fence], true, u64::MAX)
                .map_err(Self::vulkan_to_driver_error)?;
        }

        let (image_index, suboptimal) = match unsafe {
            self.swapchain_fns.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available_semaphore,
                Fence::null(),
            )
        } {
            Ok(acquired) => acquired,
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
                    self.gr_context.free_gpu_resources();
                    self.gpu_cache_purged = true;
                    window.request_redraw();
                    return Ok(());
                }
                driver_error => return Err(driver_error),
            },
        };

        self.gpu_cache_purged = false;

        let image = self.swapchain_images[image_index as usize];

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

        let mut surface = wrap_backend_render_target(
            &mut self.gr_context,
            &render_target,
            SurfaceOrigin::TopLeft,
            ColorType::BGRA8888,
            None,
            None,
        )
        .ok_or(DriverError::DeviceLost)?;

        render(&mut surface);

        window.pre_present_notify();

        self.gr_context.flush_and_submit();

        unsafe {
            self.device
                .begin_command_buffer(self.cmd_buf, &CommandBufferBeginInfo::default())
                .map_err(Self::vulkan_to_driver_error)?;

            let image_barrier = ImageMemoryBarrier::default()
                .src_access_mask(AccessFlags::COLOR_ATTACHMENT_WRITE)
                .dst_access_mask(AccessFlags::MEMORY_READ)
                .old_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(ImageLayout::PRESENT_SRC_KHR)
                .image(image)
                .subresource_range(ImageSubresourceRange {
                    aspect_mask: ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });

            self.device.cmd_pipeline_barrier(
                self.cmd_buf,
                PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                PipelineStageFlags::BOTTOM_OF_PIPE,
                DependencyFlags::empty(),
                &[],
                &[],
                &[image_barrier],
            );

            self.device
                .end_command_buffer(self.cmd_buf)
                .map_err(Self::vulkan_to_driver_error)?;
        };

        let wait_semaphores = [self.image_available_semaphore];
        let wait_stages = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let signal_semaphores = [self.render_finished_semaphore];

        let submit_infos = [SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(std::slice::from_ref(&self.cmd_buf))
            .signal_semaphores(&signal_semaphores)];

        unsafe {
            // Reset only right before submit so earlier returns leave the fence signaled.
            self.device
                .reset_fences(&[self.in_flight_fence])
                .map_err(Self::vulkan_to_driver_error)?;

            self.device
                .queue_submit(self.queue, &submit_infos, self.in_flight_fence)
                .map_err(Self::vulkan_to_driver_error)?;
        };

        let swapchains = [self.swapchain];
        let image_indices = [image_index];
        let present_info = PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        let result = unsafe { self.swapchain_fns.queue_present(self.queue, &present_info) };

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
        window: &Window,
        size: PhysicalSize<u32>,
    ) -> Result<(), DriverError> {
        unsafe {
            self.device
                .device_wait_idle()
                .map_err(Self::vulkan_to_driver_error)?;
            self.swapchain_fns.destroy_swapchain(self.swapchain, None);
            self.surface_fns.destroy_surface(self.surface, None);
        }
        // Null handles keep Drop safe if any of the steps below fail.
        self.swapchain = SwapchainKHR::null();
        self.surface = SurfaceKHR::null();

        let handle_error = |error: HandleError| {
            tracing::error!("Failed to get a window handle: {error}");
            DriverError::DeviceLost
        };
        self.surface = unsafe {
            ash_window::create_surface(
                &self.entry,
                &self.instance,
                window.display_handle().map_err(handle_error)?.as_raw(),
                window.window_handle().map_err(handle_error)?.as_raw(),
                None,
            )
            .map_err(Self::vulkan_to_driver_error)?
        };

        self.recreate_swapchain(size)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> Result<(), DriverError> {
        if size.width == 0 || size.height == 0 {
            return Ok(());
        }
        self.recreate_swapchain(size)
    }

    /// Map a Vulkan error to a recovery action.
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

/// Rank real GPUs by preference, software implementations like llvmpipe are rejected.
fn device_type_rank(device_type: PhysicalDeviceType) -> Option<u32> {
    match device_type {
        PhysicalDeviceType::DISCRETE_GPU => Some(0),
        PhysicalDeviceType::INTEGRATED_GPU => Some(1),
        PhysicalDeviceType::VIRTUAL_GPU => Some(2),
        PhysicalDeviceType::OTHER => Some(3),
        _ => None,
    }
}

fn pick_physical_device(
    instance: &Instance,
    surface_fns: &InstanceSurfaceFns,
    surface: SurfaceKHR,
) -> Result<(PhysicalDevice, u32, String), Box<dyn std::error::Error>> {
    let devices = unsafe { instance.enumerate_physical_devices()? };
    devices
        .into_iter()
        .filter_map(|physical_device| {
            let properties = unsafe { instance.get_physical_device_properties(physical_device) };
            let rank = device_type_rank(properties.device_type)?;
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
                        if supports_graphics && supports_surface {
                            Some(index as u32)
                        } else {
                            None
                        }
                    })?
            };
            let extensions_supported = unsafe {
                instance
                    .enumerate_device_extension_properties(physical_device)
                    .map(|exts| {
                        exts.iter().any(|ext| {
                            CStr::from_ptr(ext.extension_name.as_ptr()) == KHR_SWAPCHAIN_NAME
                        })
                    })
                    .unwrap_or(false)
            };

            if extensions_supported {
                Some((rank, physical_device, queue_family_index, properties))
            } else {
                None
            }
        })
        .min_by_key(|(rank, ..)| *rank)
        .map(|(_, physical_device, queue_family_index, properties)| {
            let gpu_name = properties
                .device_name_as_c_str()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
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

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn create_swapchain(
    instance: &Instance,
    device: &Device,
    physical_device: PhysicalDevice,
    surface_fns: &InstanceSurfaceFns,
    surface: SurfaceKHR,
    queue_family_index: u32,
    size: PhysicalSize<u32>,
    old_swapchain: Option<SwapchainKHR>,
    transparent: bool,
) -> Result<(SwapchainKHR, DeviceSwapchainFns, Vec<Image>, Extent2D), Box<dyn std::error::Error>> {
    let surface_caps =
        unsafe { surface_fns.get_physical_device_surface_capabilities(physical_device, surface)? };

    let surface_formats =
        unsafe { surface_fns.get_physical_device_surface_formats(physical_device, surface)? };

    let format = surface_formats
        .iter()
        .find(|f| {
            f.format == Format::B8G8R8A8_UNORM && f.color_space == ColorSpaceKHR::SRGB_NONLINEAR
        })
        .ok_or("No suitable Vulkan surface format found")?;

    let present_mode = PresentModeKHR::FIFO;

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
    let image_count = surface_caps.min_image_count.max(2);

    // If transparency is requested but the surface doesn't support a suitable
    // composite alpha mode, bail out so the caller can fall back to OpenGL.
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
        .queue_family_indices(std::slice::from_ref(&queue_family_index))
        .pre_transform(surface_caps.current_transform)
        .composite_alpha(composite_alpha)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(old_swapchain.unwrap_or(SwapchainKHR::null()));

    let swapchain_fns = DeviceSwapchainFns::new(instance, device);
    let swapchain = unsafe { swapchain_fns.create_swapchain(&create_info, None)? };
    let images = unsafe { swapchain_fns.get_swapchain_images(swapchain)? };

    Ok((swapchain, swapchain_fns, images, extent))
}

#[allow(clippy::too_many_arguments)]
fn create_gr_context(
    entry: &Entry,
    instance: &Instance,
    physical_device: PhysicalDevice,
    device: Arc<Device>,
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

fn create_sync_objects(
    device: &Device,
) -> Result<(Semaphore, Semaphore, Fence), Box<dyn std::error::Error>> {
    let semaphore_info = SemaphoreCreateInfo::default();
    let fence_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);

    let image_available_semaphore = unsafe { device.create_semaphore(&semaphore_info, None)? };
    let render_finished_semaphore = unsafe { device.create_semaphore(&semaphore_info, None)? };
    let in_flight_fence = unsafe { device.create_fence(&fence_info, None)? };

    Ok((
        image_available_semaphore,
        render_finished_semaphore,
        in_flight_fence,
    ))
}
