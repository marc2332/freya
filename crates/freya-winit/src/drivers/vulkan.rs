use std::{
    ffi::{
        CStr,
        CString,
    },
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
        API_VERSION_1_1,
        AccessFlags,
        ApplicationInfo,
        ColorSpaceKHR,
        CommandBuffer,
        CommandBufferAllocateInfo,
        CommandBufferBeginInfo,
        CommandBufferLevel,
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
    DisplayHandle,
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

use crate::config::WindowConfig;

/// Graphics driver using Vulkan.
pub struct VulkanDriver {
    _entry: Entry, // Dont drop until backend is dropped
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
    swapchain_format: Format,
    swapchain_extent: Extent2D,
    swapchain_image_index: u32,
    swapchain_suboptimal: bool,
    swapchain_size: PhysicalSize<u32>,
    gr_context: DirectContext,
    image_available_semaphore: Semaphore,
    render_finished_semaphore: Semaphore,
    in_flight_fence: Fence,
    cmd_buf: CommandBuffer,
}

impl VulkanDriver {
    pub fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        _window_config: &WindowConfig,
    ) -> (Self, Window) {
        let window = event_loop
            .create_window(window_attributes)
            .expect("Could not create window with Vulkan context");

        let entry = unsafe { Entry::load().unwrap() };

        let instance = create_instance(&entry, window.display_handle().unwrap());
        let surface_fns = InstanceSurfaceFns::new(&entry, &instance);
        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            )
            .unwrap()
        };

        let (physical_device, queue_family_index) =
            pick_physical_device(&instance, &surface_fns, surface);

        let (device, queue) = create_logical_device(&instance, physical_device, queue_family_index);
        let device = Arc::new(device);

        let swapchain_size = window.inner_size();

        let (swapchain, swapchain_fns, swapchain_images, swapchain_format, swapchain_extent) =
            create_swapchain(
                &instance,
                &device,
                physical_device,
                &surface_fns,
                surface,
                queue_family_index,
                swapchain_size,
                None,
            );

        let gr_context = create_gr_context(
            &entry,
            &instance,
            physical_device,
            device.clone(),
            queue,
            queue_family_index,
        );

        let (image_available_semaphore, render_finished_semaphore, in_flight_fence) =
            create_sync_objects(&device);

        let cmd_pool = unsafe {
            device
                .create_command_pool(
                    &CommandPoolCreateInfo::default().flags(
                        CommandPoolCreateFlags::TRANSIENT
                            | CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
                    ),
                    None,
                )
                .unwrap()
        };

        let cmd_buf = unsafe {
            device
                .allocate_command_buffers(
                    &CommandBufferAllocateInfo::default()
                        .command_pool(cmd_pool)
                        .level(CommandBufferLevel::PRIMARY)
                        .command_buffer_count(1),
                )
                .unwrap()[0]
        };

        let driver = Self {
            _entry: entry,
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
            swapchain_format,
            swapchain_extent,
            swapchain_image_index: 0,
            swapchain_suboptimal: false,
            swapchain_size,
            gr_context,
            image_available_semaphore,
            render_finished_semaphore,
            in_flight_fence,
            cmd_buf,
        };

        (driver, window)
    }

    fn recreate_swapchain(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
        }
        let old_swapchain = self.swapchain;
        let (swapchain, swapchain_fns, swapchain_images, swapchain_format, swapchain_extent) =
            create_swapchain(
                &self.instance,
                &self.device,
                self.physical_device,
                &self.surface_fns,
                self.surface,
                self.queue_family_index,
                self.swapchain_size,
                Some(old_swapchain),
            );
        self.swapchain = swapchain;
        self.swapchain_fns = swapchain_fns;
        self.swapchain_images = swapchain_images;
        self.swapchain_format = swapchain_format;
        self.swapchain_extent = swapchain_extent;
        self.swapchain_suboptimal = false;
        unsafe {
            self.swapchain_fns.destroy_swapchain(old_swapchain, None);
        }
    }

    pub fn present(
        &mut self,
        size: PhysicalSize<u32>,
        window: &Window,
        render: impl FnOnce(&mut SkiaSurface),
    ) {
        let mut surface = unsafe {
            self.device
                .wait_for_fences(&[self.in_flight_fence], true, u64::MAX)
                .unwrap();

            self.device.reset_fences(&[self.in_flight_fence]).unwrap();

            let (image_index, suboptimal) = self
                .swapchain_fns
                .acquire_next_image(
                    self.swapchain,
                    u64::MAX,
                    self.image_available_semaphore,
                    Fence::null(),
                )
                .unwrap();

            self.swapchain_image_index = image_index;
            self.swapchain_suboptimal = suboptimal;

            let image = self.swapchain_images[image_index as usize];

            let alloc = vk::Alloc::default();
            let sk_image_info = vk::ImageInfo::new(
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
            );
            let render_target = backend_render_targets::make_vk(
                (
                    self.swapchain_extent.width as i32,
                    self.swapchain_extent.height as i32,
                ),
                &sk_image_info,
            );

            wrap_backend_render_target(
                &mut self.gr_context,
                &render_target,
                SurfaceOrigin::TopLeft,
                ColorType::BGRA8888,
                None,
                None,
            )
            .unwrap()
        };

        render(&mut surface);

        window.pre_present_notify();

        self.gr_context.flush_and_submit();

        let image = self.swapchain_images[self.swapchain_image_index as usize];

        unsafe {
            self.device
                .begin_command_buffer(self.cmd_buf, &CommandBufferBeginInfo::default())
                .unwrap();

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

            self.device.end_command_buffer(self.cmd_buf).unwrap();
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
            self.device
                .queue_submit(self.queue, &submit_infos, self.in_flight_fence)
                .unwrap();
        };

        let swapchains = [self.swapchain];
        let image_indices = [self.swapchain_image_index];
        let present_info = PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        let result = unsafe { self.swapchain_fns.queue_present(self.queue, &present_info) };

        drop(surface);

        if self.swapchain_suboptimal
            || matches!(result, Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR))
        {
            self.swapchain_size = size;
            self.recreate_swapchain();
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.swapchain_size = size;
        self.recreate_swapchain();
    }
}

fn create_instance(entry: &Entry, display_handle: DisplayHandle<'_>) -> Instance {
    let app_name = CString::new("AnyRender").unwrap();
    let engine_name = CString::new("No Engine").unwrap();
    let app_info = ApplicationInfo::default()
        .application_name(&app_name)
        .application_version(make_api_version(0, 1, 0, 0))
        .engine_name(&engine_name)
        .engine_version(make_api_version(0, 1, 0, 0))
        .api_version(API_VERSION_1_1);

    let extension_names = enumerate_required_extensions(display_handle.as_raw())
        .unwrap()
        .to_vec();

    // Enable the Vulkan validation layer extension:
    // const VALIDATION_LAYER_NAME: &CStr = c"VK_LAYER_KHRONOS_validation";
    // let validation_layer_name_ptr = VALIDATION_LAYER_NAME.as_ptr();

    let create_info = InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names);
    // .enabled_layer_names(std::slice::from_ref(&validation_layer_name_ptr));

    unsafe { entry.create_instance(&create_info, None).unwrap() }
}

fn pick_physical_device(
    instance: &Instance,
    surface_fns: &InstanceSurfaceFns,
    surface: SurfaceKHR,
) -> (PhysicalDevice, u32) {
    let devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    devices
        .into_iter()
        .find_map(|physical_device| {
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
                            .unwrap();
                        if supports_graphics && supports_surface {
                            Some(index as u32)
                        } else {
                            None
                        }
                    })
                    .unwrap()
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
                Some((physical_device, queue_family_index))
            } else {
                None
            }
        })
        .unwrap()
}

fn create_logical_device(
    instance: &Instance,
    physical_device: PhysicalDevice,
    queue_family_index: u32,
) -> (Device, Queue) {
    let queue_priorities = [1.0f32];
    let queue_create_info = DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family_index)
        .queue_priorities(&queue_priorities);

    let features = PhysicalDeviceFeatures::default().sample_rate_shading(true);

    let extensions = [KHR_SWAPCHAIN_NAME.as_ptr()];

    let create_info = DeviceCreateInfo::default()
        .queue_create_infos(std::slice::from_ref(&queue_create_info))
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    let device = unsafe {
        instance
            .create_device(physical_device, &create_info, None)
            .unwrap()
    };

    let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

    (device, queue)
}

#[allow(clippy::too_many_arguments)]
fn create_swapchain(
    instance: &Instance,
    device: &Device,
    physical_device: PhysicalDevice,
    surface_fns: &InstanceSurfaceFns,
    surface: SurfaceKHR,
    queue_family_index: u32,
    size: PhysicalSize<u32>,
    old_swapchain: Option<SwapchainKHR>,
) -> (
    SwapchainKHR,
    DeviceSwapchainFns,
    Vec<Image>,
    Format,
    Extent2D,
) {
    let surface_caps = unsafe {
        surface_fns
            .get_physical_device_surface_capabilities(physical_device, surface)
            .unwrap()
    };

    let surface_formats = unsafe {
        surface_fns
            .get_physical_device_surface_formats(physical_device, surface)
            .unwrap()
    };

    let present_modes = unsafe {
        surface_fns
            .get_physical_device_surface_present_modes(physical_device, surface)
            .unwrap()
    };

    let format = surface_formats
        .iter()
        .find(|f| {
            f.format == Format::B8G8R8A8_UNORM && f.color_space == ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap();

    let present_mode = present_modes
        .iter()
        .cloned()
        .find(|&m| m == PresentModeKHR::MAILBOX)
        .unwrap_or(PresentModeKHR::FIFO);

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
        .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(old_swapchain.unwrap_or(SwapchainKHR::null()));

    let swapchain_fns = DeviceSwapchainFns::new(instance, device);
    let swapchain = unsafe { swapchain_fns.create_swapchain(&create_info, None).unwrap() };
    let images = unsafe { swapchain_fns.get_swapchain_images(swapchain).unwrap() };

    (swapchain, swapchain_fns, images, format.format, extent)
}

fn create_gr_context(
    entry: &Entry,
    instance: &Instance,
    physical_device: PhysicalDevice,
    device: Arc<Device>,
    queue: Queue,
    queue_family_index: u32,
) -> DirectContext {
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
            .unwrap()
        }
    };

    let mut backend_context = unsafe {
        vk::BackendContext::new(
            instance.handle().as_raw() as _,
            physical_device.as_raw() as _,
            device.handle().as_raw() as _,
            (queue.as_raw() as _, queue_family_index as usize),
            &get_proc,
        )
    };
    backend_context.set_max_api_version(vk::Version::new(1, 1, 0));

    let context_options = ContextOptions::default();

    direct_contexts::make_vulkan(&backend_context, &context_options).unwrap()
}

fn create_sync_objects(device: &Device) -> (Semaphore, Semaphore, Fence) {
    let semaphore_info = SemaphoreCreateInfo::default();
    let fence_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);

    let image_available_semaphore =
        unsafe { device.create_semaphore(&semaphore_info, None).unwrap() };
    let render_finished_semaphore =
        unsafe { device.create_semaphore(&semaphore_info, None).unwrap() };
    let in_flight_fence = unsafe { device.create_fence(&fence_info, None).unwrap() };

    (
        image_available_semaphore,
        render_finished_semaphore,
        in_flight_fence,
    )
}
