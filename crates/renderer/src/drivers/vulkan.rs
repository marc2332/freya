use std::{
    cell::{
        Cell,
        RefCell,
    },
    sync::Arc,
};

use freya_engine::prelude::{
    backend_render_targets,
    direct_contexts,
    wrap_backend_render_target,
    Alloc,
    BackendContext,
    Canvas,
    ColorType,
    DirectContext,
    GetProcOf,
    ImageLayout,
    ImageTiling,
    Surface as SkiaSurface,
    SurfaceOrigin,
    VkFormat,
    VkImageInfo,
};
use raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle,
};
use vulkano::{
    device::{
        physical::PhysicalDeviceType,
        Device,
        DeviceCreateInfo,
        DeviceExtensions,
        Queue,
        QueueCreateInfo,
        QueueFlags,
    },
    image::{
        view::ImageView,
        Image,
        ImageUsage,
    },
    instance::{
        Instance,
        InstanceCreateFlags,
        InstanceCreateInfo,
        InstanceExtensions,
    },
    swapchain::{
        Surface,
        Swapchain,
        SwapchainCreateInfo,
        SwapchainPresentInfo,
    },
    sync::{
        self,
        GpuFuture,
    },
    Handle,
    Validated,
    VulkanError,
    VulkanLibrary,
    VulkanObject,
};
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{
        Window,
        WindowAttributes,
    },
};

use crate::{
    size::WinitSize,
    LaunchConfig,
};

/// Graphics driver using Vulkan.
pub struct VulkanDriver {
    pub(crate) gr_context: DirectContext,
    pub(crate) recreate_swapchain: Cell<bool>,
    pub(crate) device: Arc<Device>,
    pub(crate) previous_frame_end: RefCell<Option<Box<dyn GpuFuture>>>,
    pub(crate) queue: Arc<Queue>,
    pub(crate) swapchain: RefCell<Arc<Swapchain>>,
    pub(crate) swapchain_images: RefCell<Vec<Arc<Image>>>,
    pub(crate) swapchain_image_views: RefCell<Vec<Arc<ImageView>>>,
}

impl VulkanDriver {
    pub fn new<State: Clone + 'static>(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        config: &LaunchConfig<State>,
    ) -> (Self, Window, SkiaSurface) {
        let window = event_loop
            .create_window(window_attributes)
            .expect("Could not create window with Vulkan context");

        let window_size = window.inner_size();

        let library = VulkanLibrary::new().expect("Could not create Vulkan library");
        let required_extensions = InstanceExtensions {
            khr_surface: true,
            mvk_macos_surface: true,
            ext_metal_surface: true,
            khr_wayland_surface: true,
            khr_xlib_surface: true,
            khr_xcb_surface: true,
            khr_win32_surface: true,
            khr_get_surface_capabilities2: true,
            khr_get_physical_device_properties2: true,
            ..InstanceExtensions::empty()
        }
        .intersection(library.supported_extensions());

        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
        .expect("Could not create Vulkan instance");

        let window_handle = window.window_handle().unwrap();
        let display_handle = window.display_handle().unwrap();

        let surface = create_surface(&instance, window_handle, display_handle)
            .expect("Could not create Vulkan surface");

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .unwrap();

        let (device, mut queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                enabled_extensions: DeviceExtensions {
                    khr_swapchain: true,
                    ..DeviceExtensions::empty()
                },
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .unwrap();

        let queue = queues.next().unwrap();

        let (swapchain, swapchain_images) = {
            let surface_capabilities = device
                .physical_device()
                .surface_capabilities(&surface, Default::default())
                .unwrap();
            let image_format = vulkano::format::Format::B8G8R8A8_UNORM;

            Swapchain::new(
                device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count,
                    image_format,
                    image_extent: [window_size.width, window_size.height],
                    image_usage: ImageUsage::COLOR_ATTACHMENT,
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next()
                        .unwrap(),
                    ..Default::default()
                },
            )
            .unwrap()
        };

        let mut swapchain_image_views = Vec::with_capacity(swapchain_images.len());

        for image in &swapchain_images {
            swapchain_image_views.push(
                ImageView::new_default(image.clone())
                    .expect("Error creating image view for swap chain image"),
            );
        }

        let device_instance = physical_device.instance();
        let device_library = device_instance.library();

        let get_proc = |of| unsafe {
            let result = match of {
                GetProcOf::Instance(device_instance, name) => device_library
                    .get_instance_proc_addr(
                        ash::vk::Instance::from_raw(device_instance as _),
                        name,
                    ),
                GetProcOf::Device(device, name) => {
                    (device_instance.fns().v1_0.get_device_proc_addr)(
                        ash::vk::Device::from_raw(device as _),
                        name,
                    )
                }
            };

            match result {
                Some(f) => f as _,
                None => {
                    //println!("resolve of {} failed", of.name().to_str().unwrap());
                    core::ptr::null()
                }
            }
        };

        let backend_context = unsafe {
            BackendContext::new(
                device_instance.handle().as_raw() as _,
                physical_device.handle().as_raw() as _,
                device.handle().as_raw() as _,
                (queue.handle().as_raw() as _, queue.id_within_family() as _),
                &get_proc,
            )
        };

        let mut gr_context = direct_contexts::make_vulkan(&backend_context, None)
            .expect("Could not create Skia Vulkan context");

        let previous_frame_end = RefCell::new(Some(sync::now(device.clone()).boxed()));

        let (image_index, ..) =
            match vulkano::swapchain::acquire_next_image(swapchain.clone(), None)
                .map_err(Validated::unwrap)
            {
                Ok(r) => r,
                Err(_) => panic!("Vulkan: failed to acquire next image"),
            };

        let image_view = swapchain_image_views[image_index as usize].clone();
        let image_object = image_view.image();

        let width = swapchain.image_extent()[0];
        let width: i32 = width.try_into().unwrap();
        let height = swapchain.image_extent()[1];
        let height: i32 = height.try_into().unwrap();

        let image_info = &unsafe {
            VkImageInfo::new(
                image_object.handle().as_raw() as _,
                Alloc::default(),
                ImageTiling::OPTIMAL,
                ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                VkFormat::B8G8R8A8_UNORM,
                1,
                None,
                None,
                None,
                None,
            )
        };

        let render_target = backend_render_targets::make_vk((width, height), image_info);

        let mut skia_surface = wrap_backend_render_target(
            &mut gr_context,
            &render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::BGRA8888,
            None,
            None,
        )
        .expect("Could not create Skia surface");

        skia_surface.canvas().clear(config.window_config.background);

        gr_context.flush_and_submit();

        let driver = VulkanDriver {
            gr_context,
            recreate_swapchain: Cell::new(false),
            device,
            previous_frame_end,
            queue,
            swapchain: RefCell::new(swapchain),
            swapchain_images: RefCell::new(swapchain_images),
            swapchain_image_views: RefCell::new(swapchain_image_views),
        };

        (driver, window, skia_surface)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> (SkiaSurface, SkiaSurface) {
        let device = self.device.clone();

        self.previous_frame_end
            .borrow_mut()
            .as_mut()
            .unwrap()
            .cleanup_finished();

        let swapchain = self.swapchain.borrow().clone();

        let (image_index, suboptimal, acquire_future) =
            vulkano::swapchain::acquire_next_image(swapchain.clone(), None).unwrap();

        if suboptimal {
            self.recreate_swapchain.set(true);
        }

        let width = swapchain.image_extent()[0];
        let width: i32 = width.try_into().unwrap();
        let height = swapchain.image_extent()[1];
        let height: i32 = height.try_into().unwrap();

        let image_view = self.swapchain_image_views.borrow()[image_index as usize].clone();
        let image_object = image_view.image();

        let (vk_format, color_type) = (VkFormat::B8G8R8A8_UNORM, ColorType::BGRA8888);

        let alloc = Alloc::default();
        let image_info = &unsafe {
            VkImageInfo::new(
                image_object.handle().as_raw() as _,
                alloc,
                ImageTiling::OPTIMAL,
                ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                vk_format,
                1,
                None,
                None,
                None,
                None,
            )
        };

        let render_target = &backend_render_targets::make_vk((width, height), image_info);

        let mut surface = wrap_backend_render_target(
            &mut self.gr_context,
            render_target,
            SurfaceOrigin::TopLeft,
            color_type,
            None,
            None,
        )
        .expect("Could not create skia surface");

        let dirty_surface = surface.new_surface_with_dimensions(size.to_skia()).unwrap();

        self.gr_context.flush_and_submit();

        let future = self
            .previous_frame_end
            .borrow_mut()
            .take()
            .unwrap()
            .join(acquire_future)
            .then_swapchain_present(
                self.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_index),
            )
            .then_signal_fence_and_flush();

        match future.map_err(Validated::unwrap) {
            Ok(future) => {
                *self.previous_frame_end.borrow_mut() = Some(future.boxed());
            }
            Err(VulkanError::OutOfDate) => {
                self.recreate_swapchain.set(true);
                *self.previous_frame_end.borrow_mut() = Some(sync::now(device.clone()).boxed());
            }
            Err(_) => {
                *self.previous_frame_end.borrow_mut() = Some(sync::now(device.clone()).boxed());
            }
        }

        (surface, dirty_surface)
    }
}

fn create_surface(
    instance: &Arc<Instance>,
    window_handle: raw_window_handle::WindowHandle<'_>,
    display_handle: raw_window_handle::DisplayHandle<'_>,
) -> Result<Arc<Surface>, vulkano::Validated<vulkano::VulkanError>> {
    match (window_handle.as_raw(), display_handle.as_raw()) {
        #[cfg(target_os = "macos")]
        (
            raw_window_handle::RawWindowHandle::AppKit(raw_window_handle::AppKitWindowHandle {
                ns_view,
                ..
            }),
            _,
        ) => unsafe {
            use cocoa::{
                appkit::NSView,
                base::id as cocoa_id,
            };
            use objc::runtime::YES;

            let layer = metal::MetalLayer::new();
            layer.set_opaque(false);
            layer.set_presents_with_transaction(false);
            let view = ns_view.as_ptr() as cocoa_id;
            view.setWantsLayer(YES);
            view.setLayer(layer.as_ref() as *const _ as _);
            Surface::from_metal(instance.clone(), layer.as_ref(), None)
        },
        (
            raw_window_handle::RawWindowHandle::Xlib(raw_window_handle::XlibWindowHandle {
                window,
                ..
            }),
            raw_window_handle::RawDisplayHandle::Xlib(display),
        ) => unsafe {
            Surface::from_xlib(
                instance.clone(),
                display.display.unwrap().as_ptr(),
                window,
                None,
            )
        },
        (
            raw_window_handle::RawWindowHandle::Xcb(raw_window_handle::XcbWindowHandle {
                window,
                ..
            }),
            raw_window_handle::RawDisplayHandle::Xcb(raw_window_handle::XcbDisplayHandle {
                connection,
                ..
            }),
        ) => unsafe {
            Surface::from_xcb(
                instance.clone(),
                connection.unwrap().as_ptr(),
                window.get(),
                None,
            )
        },
        (
            raw_window_handle::RawWindowHandle::Wayland(raw_window_handle::WaylandWindowHandle {
                surface,
                ..
            }),
            raw_window_handle::RawDisplayHandle::Wayland(raw_window_handle::WaylandDisplayHandle {
                display,
                ..
            }),
        ) => unsafe {
            Surface::from_wayland(instance.clone(), display.as_ptr(), surface.as_ptr(), None)
        },
        (
            raw_window_handle::RawWindowHandle::Win32(raw_window_handle::Win32WindowHandle {
                hwnd,
                hinstance,
                ..
            }),
            _,
        ) => unsafe {
            Surface::from_win32(
                instance.clone(),
                hinstance.unwrap().get() as *const std::ffi::c_void,
                hwnd.get() as *const std::ffi::c_void,
                None,
            )
        },
        _ => unimplemented!(),
    }
}
