use std::num::NonZeroU128;
use std::sync::{Arc, Mutex};

use accesskit::{
    Action, DefaultActionVerb, Node, NodeBuilder, NodeClassSet, NodeId, Rect, Role, Tree,
    TreeUpdate,
};
use accesskit_winit::Adapter;
use freya_common::{EventMessage, NodeArea};
use freya_core::process_render;
use freya_core::{process_layout, ViewportsCollection};
use freya_layout::{DioxusDOM, Layers, RenderData};
use gl::types::*;
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::{window::WindowBuilder, GlProfile};
use skia_safe::{gpu::DirectContext, textlayout::FontCollection};
use skia_safe::{
    gpu::{gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
    ColorType, Surface,
};
use skia_safe::{Color, FontMgr};

use crate::renderer::render_skia;
use crate::window_config::WindowConfig;
use crate::HoveredNode;

pub type SharedAccessibilityState = Arc<Mutex<AccessibilityState>>;

const WINDOW_ID: NodeId = NodeId(unsafe { NonZeroU128::new_unchecked(1) });

pub struct AccessibilityState {
    pub nodes: Vec<(NodeId, Node)>,
    pub node_classes: NodeClassSet,
    pub focus: Option<NodeId>,
}

#[derive(PartialEq)]
pub enum FocusDirection {
    Forward,
    Backward,
}

impl AccessibilityState {
    pub fn wrap(self) -> SharedAccessibilityState {
        Arc::new(Mutex::new(self))
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn add_element(
        &mut self,
        dioxus_node: &RenderData,
        accessibility_id: NodeId,
        children: Option<Vec<NodeId>>,
    ) {
        let mut builder = NodeBuilder::new(Role::Button);

        if let Some(children) = children {
            builder.set_children(children);
        }

        builder.set_bounds(Rect {
            x0: dioxus_node.node_area.x as f64,
            x1: (dioxus_node.node_area.x + dioxus_node.node_area.width) as f64,
            y0: dioxus_node.node_area.y as f64,
            y1: (dioxus_node.node_area.y + dioxus_node.node_area.height) as f64,
        });
        builder.add_action(Action::Focus);
        builder.set_default_action_verb(DefaultActionVerb::Click);
        builder.set_name("Button");
        let node = builder.build(&mut self.node_classes);
        self.nodes.push((accessibility_id, node));
    }

    pub fn build_root(&mut self) -> Node {
        let mut builder = NodeBuilder::new(Role::Window);
        builder.set_children(
            self.nodes
                .iter()
                .map(|(id, _)| *id)
                .collect::<Vec<NodeId>>(),
        );
        builder.set_name("window");

        builder.build(&mut self.node_classes)
    }

    pub fn process(&mut self) -> TreeUpdate {
        let root = self.build_root();
        let mut nodes = vec![(WINDOW_ID, root)];
        nodes.extend(self.nodes.clone());

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(WINDOW_ID)),
            focus: self.focus,
        }
    }

    pub fn set_focus(&mut self, adapter: &Adapter, id: NodeId) {
        self.focus = Some(id);
        adapter.update(TreeUpdate {
            nodes: Vec::new(),
            tree: None,
            focus: self.focus,
        });
    }

    pub fn set_focus_on_next_node(&mut self, adapter: &Adapter, direction: FocusDirection) {
        if let Some(focused_node_id) = self.focus {
            let current_node = self
                .nodes
                .iter()
                .enumerate()
                .find(|(_, node)| node.0 == focused_node_id);

            if let Some((node_index, _)) = current_node {
                let target_node = if direction == FocusDirection::Forward {
                    self.nodes
                        .iter()
                        .enumerate()
                        .find(|(i, _)| *i == node_index + 1)
                        .map(|(_, node)| node)
                } else {
                    self.nodes
                        .iter()
                        .enumerate()
                        .find(|(i, _)| i + 1 == node_index)
                        .map(|(_, node)| node)
                };

                if let Some((next_node_id, _)) = target_node {
                    self.focus = Some(*next_node_id);
                } else if direction == FocusDirection::Forward {
                    self.focus = self.nodes.first().map(|(id, _)| *id)
                } else if direction == FocusDirection::Backward {
                    self.focus = self.nodes.last().map(|(id, _)| *id)
                }
            } else {
                self.focus = self.nodes.first().map(|(id, _)| *id)
            }

            adapter.update(TreeUpdate {
                nodes: Vec::new(),
                tree: None,
                focus: self.focus,
            });
        }
    }
}

type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

/// Manager for a Window
pub struct WindowEnv<T: Clone> {
    pub(crate) surface: Surface,
    pub(crate) gr_context: DirectContext,
    pub(crate) windowed_context: WindowedContext,
    pub(crate) fb_info: FramebufferInfo,
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

        let wb = WindowBuilder::new()
            .with_title(window_config.title)
            .with_decorations(window_config.decorations)
            .with_transparent(window_config.transparent)
            .with_inner_size(PhysicalSize::<u32>::new(
                window_config.width,
                window_config.height,
            ));

        let cb = glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_stencil_buffer(8)
            .with_pixel_format(24, 8)
            .with_gl_profile(GlProfile::Core);

        #[cfg(not(target_os = "linux"))]
        let cb = cb.with_double_buffer(Some(true));

        let windowed_context = cb.build_windowed(wb, event_loop).unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        gl::load_with(|s| windowed_context.get_proc_address(s));

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
            }
        };

        let mut gr_context = skia_safe::gpu::DirectContext::new_gl(None, None).unwrap();

        let mut surface = create_surface(&windowed_context, &fb_info, &mut gr_context);
        let sf = windowed_context.window().scale_factor() as f32;
        surface.canvas().scale((sf, sf));

        WindowEnv {
            surface,
            gr_context,
            windowed_context,
            fb_info,
            font_collection,
            window_config,
        }
    }

    // Reprocess the layout
    pub fn process_layout(&mut self, rdom: &DioxusDOM) -> (Layers, ViewportsCollection) {
        let window_size = self.windowed_context.window().inner_size();
        process_layout(
            rdom,
            NodeArea {
                width: window_size.width as f32,
                height: window_size.height as f32,
                x: 0.0,
                y: 0.0,
            },
            &mut self.font_collection,
        )
    }

    /// Redraw the window
    pub fn render(
        &mut self,
        layers: &Layers,
        viewports_collection: &ViewportsCollection,
        hovered_node: &HoveredNode,
        rdom: &DioxusDOM,
    ) {
        let canvas = self.surface.canvas();

        canvas.clear(if self.window_config.decorations {
            Color::WHITE
        } else {
            Color::TRANSPARENT
        });

        process_render(
            viewports_collection,
            rdom,
            &mut self.font_collection,
            layers,
            canvas,
            |dom, element, font_collection, viewports_collection, canvas| {
                canvas.save();
                let render_wireframe = if let Some(hovered_node) = &hovered_node {
                    hovered_node
                        .lock()
                        .unwrap()
                        .map(|id| id == element.node_id)
                        .unwrap_or_default()
                } else {
                    false
                };
                render_skia(
                    dom,
                    canvas,
                    element,
                    font_collection,
                    viewports_collection,
                    render_wireframe,
                );
                canvas.restore();
            },
        );

        self.gr_context.flush(None);
        self.windowed_context.swap_buffers().unwrap();
    }

    pub fn request_redraw(&self) {
        self.windowed_context.window().request_redraw()
    }
}

/// Create the surface for Skia to render in
pub fn create_surface(
    windowed_context: &WindowedContext,
    fb_info: &FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
) -> Surface {
    let pixel_format = windowed_context.get_pixel_format();
    let size = windowed_context.window().inner_size();
    let backend_render_target = BackendRenderTarget::new_gl(
        (
            size.width.try_into().unwrap(),
            size.height.try_into().unwrap(),
        ),
        pixel_format.multisampling.map(|s| s.try_into().unwrap()),
        pixel_format.stencil_bits.try_into().unwrap(),
        *fb_info,
    );
    Surface::from_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .unwrap()
}
