use dioxus_native_core::NodeId;
use freya_common::{Area, EventMessage, Size2D};
use freya_core::process_render;
use freya_core::{process_layout, ViewportsCollection};
use freya_dom::FreyaDOM;
use freya_layout::Layers;
use std::ffi::CString;
use std::num::NonZeroU32;

use gl::types::*;
use glutin::context::GlProfile;
use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{
        ContextApi, ContextAttributesBuilder, NotCurrentGlContextSurfaceAccessor,
        PossiblyCurrentContext,
    },
    display::{GetGlDisplay, GlDisplay},
    prelude::GlSurface,
    surface::{Surface as GlutinSurface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;

use winit::dpi::PhysicalSize;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use skia_safe::{
    gpu::{gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
    textlayout::FontCollection,
    Color, ColorType, FontMgr, Matrix, Surface,
};

use crate::renderer::render_skia;
use crate::window_config::WindowConfig;
use crate::HoveredNode;

/// Manager for a Window
pub struct WindowEnv<T: Clone> {
    surface: Surface,
    gl_surface: GlutinSurface<WindowSurface>,
    gr_context: skia_safe::gpu::DirectContext,
    gl_context: PossiblyCurrentContext,
    pub(crate) window: Window,
    fb_info: FramebufferInfo,
    num_samples: usize,
    stencil_size: usize,
    pub(crate) font_collection: FontCollection,
    pub(crate) window_config: WindowConfig<T>,
}

impl<T: Clone> WindowEnv<T> {
    /// Create a Window environment from a set of configuration
    pub fn from_config(
        window_config: WindowConfig<T>,
        event_loop: &EventLoop<EventMessage>,
    ) -> Self {
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::default(), "Fira Sans");

        let window_builder = WindowBuilder::new()
            .with_title(window_config.title)
            .with_decorations(window_config.decorations)
            .with_transparent(window_config.transparent)
            .with_inner_size(PhysicalSize::<u32>::new(
                window_config.width,
                window_config.height,
            ));

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(window_config.transparent);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let mut window = window.expect("Could not create window with OpenGL context");
        let raw_window_handle = window.raw_window_handle();

        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .build(Some(raw_window_handle));

        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_window_handle));

        let not_current_gl_context = unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_config
                        .display()
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        };

        let (width, height): (u32, u32) = window.inner_size().into();

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .expect("Could not create gl window surface")
        };

        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("Could not make GL context current when setting up skia renderer");

        gl::load_with(|s| {
            gl_config
                .display()
                .get_proc_address(CString::new(s).unwrap().as_c_str())
        });
        let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            gl_config
                .display()
                .get_proc_address(CString::new(name).unwrap().as_c_str())
        })
        .expect("Could not create interface");

        let mut gr_context = skia_safe::gpu::DirectContext::new_gl(Some(interface), None)
            .expect("Could not create direct context");

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
            }
        };

        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;

        let mut surface = create_surface(
            &mut window,
            fb_info,
            &mut gr_context,
            num_samples,
            stencil_size,
        );

        let sf = window.scale_factor() as f32;
        surface.canvas().scale((sf, sf));

        WindowEnv {
            surface,
            gl_surface,
            gl_context,
            gr_context,
            fb_info,
            num_samples,
            stencil_size,
            window,
            font_collection,
            window_config,
        }
    }

    /// Measure the layout
    pub fn process_layout(&mut self, rdom: &FreyaDOM) -> (Layers, ViewportsCollection) {
        let window_size = self.window.inner_size();
        let scale_factor = self.window.scale_factor() as f32;
        process_layout(
            rdom,
            Area::from_size(Size2D::from((
                window_size.width as f32,
                window_size.height as f32,
            ))),
            &mut self.font_collection,
            scale_factor,
        )
    }

    /// Render the RealDOM to Window
    pub fn render(
        &mut self,
        layers: &Layers,
        viewports_collection: &ViewportsCollection,
        hovered_node: &HoveredNode,
        rdom: &FreyaDOM,
    ) {
        let canvas = self.surface.canvas();

        canvas.clear(if self.window_config.decorations {
            Color::WHITE
        } else {
            Color::TRANSPARENT
        });

        let mut matrices: Vec<(Matrix, Vec<NodeId>)> = Vec::default();

        process_render(
            viewports_collection,
            rdom,
            &mut self.font_collection,
            layers,
            &mut (canvas, (&mut matrices)),
            |dom, render_node, font_collection, viewports_collection, (canvas, matrices)| {
                let render_wireframe = if let Some(hovered_node) = &hovered_node {
                    hovered_node
                        .lock()
                        .unwrap()
                        .map(|id| id == *render_node.get_id())
                        .unwrap_or_default()
                } else {
                    false
                };
                if let Some(dioxus_node) = render_node.get_node(dom) {
                    render_skia(
                        dom,
                        canvas,
                        render_node,
                        dioxus_node,
                        font_collection,
                        viewports_collection,
                        render_wireframe,
                        matrices,
                    );
                }
            },
        );

        self.gr_context.flush_and_submit();
        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }

    /// Request a redraw
    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }

    /// Resize the Window
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.surface = create_surface(
            &mut self.window,
            self.fb_info,
            &mut self.gr_context,
            self.num_samples,
            self.stencil_size,
        );

        let (width, height): (u32, u32) = size.into();

        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(width.max(1)).unwrap(),
            NonZeroU32::new(height.max(1)).unwrap(),
        );
    }
}

/// Create the surface for Skia to render in
fn create_surface(
    window: &mut Window,
    fb_info: FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> Surface {
    let size = window.inner_size();
    let size = (
        size.width.try_into().expect("Could not convert width"),
        size.height.try_into().expect("Could not convert height"),
    );
    let backend_render_target =
        BackendRenderTarget::new_gl(size, num_samples, stencil_size, fb_info);

    Surface::from_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .expect("Could not create skia surface")
}
