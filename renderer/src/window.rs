use accesskit::{
    Action, DefaultActionVerb, Node, NodeBuilder, NodeClassSet, NodeId, Rect, Role, Tree,
    TreeUpdate,
};
use accesskit_winit::Adapter;
use freya_common::{EventMessage, NodeArea};
use freya_core::{events::EventsProcessor, process_render, EventEmitter, SharedFreyaEvents};
use freya_core::{process_events, process_layout, ViewportsCollection};
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
use std::collections::HashMap;
use std::num::NonZeroU128;
use std::sync::{Arc, Mutex};

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
}

type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

/// Information related to a specific window
pub struct WindowEnv<T: Clone> {
    pub(crate) surface: Surface,
    pub(crate) gr_context: DirectContext,
    pub(crate) windowed_context: WindowedContext,
    pub(crate) fb_info: FramebufferInfo,
    pub(crate) freya_events: SharedFreyaEvents,
    pub(crate) event_emitter: EventEmitter,
    pub(crate) font_collection: FontCollection,
    pub(crate) events_processor: EventsProcessor,
    pub(crate) window_config: WindowConfig<T>,
    pub(crate) layers: Layers,
    pub(crate) viewports_collection: ViewportsCollection,
    pub(crate) accessibility_state: SharedAccessibilityState,
}

impl<T: Clone> WindowEnv<T> {
    /// Create a Window environment from a set of configuration
    pub fn from_config(
        event_emitter: EventEmitter,
        window_config: WindowConfig<T>,
        event_loop: &EventLoop<EventMessage>,
        accessibility_state: SharedAccessibilityState,
    ) -> Self {
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::default(), "Fira Sans");
        let events_processor = EventsProcessor::default();
        let freya_events = Arc::new(Mutex::new(Vec::new()));
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
            freya_events,
            event_emitter,
            font_collection,
            events_processor,
            window_config,
            layers: Layers::default(),
            viewports_collection: HashMap::default(),
            accessibility_state,
        }
    }

    // Process the events and emit them to the DOM
    pub fn process_events(&mut self, rdom: &DioxusDOM) {
        process_events(
            rdom,
            &self.layers,
            &self.freya_events,
            &self.event_emitter,
            &mut self.events_processor,
            &self.viewports_collection,
        );
    }

    pub fn process_accessibility(&mut self, rdom: &DioxusDOM) {
        // TODO: move logic to core
        for layer in self.layers.layers.values() {
            for node in layer.values() {
                if let Some(accessibility_id) =
                    node.get_node(rdom).state.accessibility.accessibility_id
                {
                    let children = node.get_accessibility_children(rdom);
                    self.accessibility_state.lock().unwrap().add_element(
                        node,
                        accessibility_id,
                        children,
                    );
                }
            }
        }
    }

    // Reprocess the layout
    pub fn process_layout(&mut self, rdom: &DioxusDOM) {
        let window_size = self.windowed_context.window().inner_size();
        let (layers, viewports) = process_layout(
            rdom,
            NodeArea {
                width: window_size.width as f32,
                height: window_size.height as f32,
                x: 0.0,
                y: 0.0,
            },
            &mut self.font_collection,
        );

        self.layers = layers;
        self.viewports_collection = viewports;

        self.process_accessibility(rdom);
    }

    /// Redraw the window
    pub fn render(&mut self, hovered_node: &HoveredNode, rdom: &DioxusDOM) {
        let canvas = self.surface.canvas();

        canvas.clear(if self.window_config.decorations {
            Color::WHITE
        } else {
            Color::TRANSPARENT
        });

        process_render(
            &self.viewports_collection,
            rdom,
            &mut self.font_collection,
            &self.layers,
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
