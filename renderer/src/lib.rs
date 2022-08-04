use dioxus_core::{
    exports::futures_channel::mpsc::UnboundedSender, ElementId, EventPriority, SchedulerMsg,
    UserEvent,
};
use dioxus_html::{
    geometry::{
        euclid::{Length, Point2D},
        Coordinates,
    },
    input_data::{keyboard_types::Modifiers, MouseButton},
    on::MouseData,
};
use dioxus_native_core::real_dom::{Node, NodeType, RealDom};
use enumset::enum_set;
use glutin::event::WindowEvent;
use layout_engine::{calculate_node, NodeData, Viewport};
use skia_safe::{
    font_style::{Slant, Weight, Width},
    utils::text_utils::Align,
    Canvas, Font, FontStyle, Paint, PaintStyle, Path, Typeface,
};
use state::node::{NodeState, SizeMode};
use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

use gl::types::*;
use glutin::dpi::PhysicalSize;
use glutin::event::ElementState;
use glutin::window::WindowId;
use glutin::{
    event::{Event, KeyboardInput, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    GlProfile,
};
use skia_safe::Color;
use skia_safe::{
    gpu::{gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
    ColorType, Surface,
};
use std::ops::Index;

type SkiaDom = Arc<Mutex<RealDom<NodeState>>>;
type EventEmitter = Arc<Mutex<Option<UnboundedSender<SchedulerMsg>>>>;

pub fn run(skia_dom: SkiaDom, rev_render: Receiver<()>, event_emitter: EventEmitter) {
    type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

    let el = EventLoop::new();

    struct Env {
        surface: Surface,
        gr_context: skia_safe::gpu::DirectContext,
        windowed_context: WindowedContext,
        skia_dom: SkiaDom,
        fb_info: FramebufferInfo,
    }

    impl Env {
        pub fn redraw(&mut self) {
            let canvas = self.surface.canvas();
            canvas.clear(Color::WHITE);
            let window_size = self.windowed_context.window().inner_size();
            render(
                &self.skia_dom,
                canvas,
                Viewport {
                    width: window_size.width as i32,
                    height: window_size.height as i32,
                    x: 0,
                    y: 0,
                },
            );
            self.gr_context.flush(None);
            self.windowed_context.swap_buffers().unwrap();
        }
    }

    let wins = Arc::new(Mutex::new(vec![]));

    let wb = WindowBuilder::new().with_title("test");

    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_stencil_buffer(8)
        .with_pixel_format(24, 8)
        .with_gl_profile(GlProfile::Core);

    #[cfg(not(feature = "wayland"))]
    let cb = cb.with_double_buffer(Some(true));

    let windowed_context = cb.build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let window_id = windowed_context.window().id();

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

    windowed_context
        .window()
        .set_inner_size(PhysicalSize::<u32>::new(300, 300));

    let mut surface = create_surface(&windowed_context, &fb_info, &mut gr_context);
    let sf = windowed_context.window().scale_factor() as f32;
    surface.canvas().scale((sf, sf));

    let env = Env {
        surface,
        gr_context,
        windowed_context,
        fb_info,
        skia_dom,
    };

    wins.lock().unwrap().push(Arc::new(Mutex::new(env)));

    fn create_surface(
        windowed_context: &WindowedContext,
        fb_info: &FramebufferInfo,
        gr_context: &mut skia_safe::gpu::DirectContext,
    ) -> skia_safe::Surface {
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

    {
        let proxy = el.create_proxy();
        thread::spawn(move || {
            while let Ok(msg) = rev_render.recv() {
                proxy.send_event(msg).unwrap();
            }
        });
    }

    let cursor_pos = Arc::new(Mutex::new((0.0, 0.0)));

    // This will calculate the whole layout from the root component and
    // find the first element that matches the cursor position
    // WIP
    let send_mouse_event = {
        let cursor_pos = cursor_pos.clone();
        let wins = wins.clone();
        move |event_name: &'static str| {
            let result = {
                let mut win = None;
                for env in &*wins.lock().unwrap() {
                    if env.lock().unwrap().windowed_context.window().id() == window_id {
                        win = Some(env.clone())
                    }
                }

                win
            };
            if let Some(env) = result {
                let mut env = env.lock().unwrap();
                let event_emitter = event_emitter.lock().unwrap();
                let event_emitter = event_emitter.as_ref().unwrap();

                {
                    let root: Node<NodeState> = {
                        let dom = env.skia_dom.lock().unwrap();
                        dom.index(ElementId(0)).clone()
                    };
                    let window_size = env.windowed_context.window().inner_size();
                    let dom = &env.skia_dom.clone();
                    let mut node_candidates: Vec<ElementId> = Vec::new();
                    calculate_node::<
                        (&SkiaDom, Arc<Mutex<(f64, f64)>>, &mut Vec<ElementId>),
                        NodeState,
                    >(
                        &NodeData::<NodeState> {
                            width: SizeMode::Percentage(100),
                            height: SizeMode::Percentage(100),
                            padding: (0, 0, 0, 0),
                            node: Some(root),
                        },
                        Viewport {
                            x: 0,
                            y: 0,
                            width: window_size.width as i32,
                            height: window_size.height as i32,
                        },
                        Viewport {
                            x: 0,
                            y: 0,
                            width: window_size.width as i32,
                            height: window_size.height as i32,
                        },
                        &mut (dom, cursor_pos.clone(), &mut node_candidates),
                        |node_id, (dom, _, _)| {
                            let child = {
                                let dom = dom.lock().unwrap();
                                dom.index(*node_id).clone()
                            };

                            Some(NodeData::<NodeState> {
                                width: child.state.size.width,
                                height: child.state.size.height,
                                padding: child.state.size.padding,
                                node: Some(child),
                            })
                        },
                        |node, viewport, (_, cursor_pos, node_candidates)| {
                            let x = viewport.x as f64;
                            let y = viewport.y as f64;
                            let width = (viewport.x + viewport.width) as f64;
                            let height = (viewport.y + viewport.height) as f64;
                            let cursor_pos = *cursor_pos.lock().unwrap();

                            // Check if the cursor is inside the 4 points
                            if cursor_pos.0 > x
                                && cursor_pos.0 < width
                                && cursor_pos.1 > y
                                && cursor_pos.1 < height
                            {
                                node_candidates.push(node.node.as_ref().unwrap().id);
                            }
                        },
                    );

                    let dom = dom.lock().unwrap();
                    let listeners = dom.get_listening_sorted(event_name);
                    let cursor_pos = *cursor_pos.lock().unwrap();
                    for listener in listeners {
                        if node_candidates.contains(&listener.id) {
                            // Propagate the Mouse event
                            event_emitter
                                .unbounded_send(SchedulerMsg::Event(UserEvent {
                                    scope_id: None,
                                    priority: EventPriority::Medium,
                                    element: Some(listener.id),
                                    name: event_name,
                                    bubbles: true,
                                    data: Arc::new(MouseData::new(
                                        Coordinates::new(
                                            Point2D::default(),
                                            Point2D::from_lengths(
                                                Length::new(cursor_pos.0),
                                                Length::new(cursor_pos.1),
                                            ),
                                            Point2D::default(),
                                            Point2D::default(),
                                        ),
                                        Some(MouseButton::Primary),
                                        enum_set! {MouseButton::Primary},
                                        Modifiers::empty(),
                                    )),
                                }))
                                .unwrap();
                        }
                    }
                }

                env.redraw();
            }
        }
    };

    let get_window_context = move |window_id: WindowId| -> Option<Arc<Mutex<Env>>> {
        let mut win = None;
        for env in &*wins.lock().unwrap() {
            if env.lock().unwrap().windowed_context.window().id() == window_id {
                win = Some(env.clone())
            }
        }

        win
    };

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        #[allow(deprecated)]
        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    {
                        let mut cursor_pos = cursor_pos.lock().unwrap();
                        cursor_pos.0 = position.x;
                        cursor_pos.1 = position.y;
                    }
                    send_mouse_event("mouseover")
                }
                WindowEvent::MouseInput { state, .. } => {
                    if ElementState::Released == state {
                        send_mouse_event("click")
                    }
                }
                WindowEvent::Resized(physical_size) => {
                    let result = get_window_context(window_id);
                    if let Some(env) = result {
                        let mut env = env.lock().unwrap();
                        let mut context = env.gr_context.clone();
                        env.surface =
                            create_surface(&env.windowed_context, &env.fb_info, &mut context);
                        env.windowed_context.resize(physical_size)
                    }
                }
                WindowEvent::CloseRequested => {
                    // should only remove one window
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode,
                            modifiers,
                            ..
                        },
                    ..
                } => {
                    if modifiers.logo() {
                        if let Some(VirtualKeyCode::Q) = virtual_keycode {
                            *control_flow = ControlFlow::Exit;
                        }
                    }

                    let result = get_window_context(window_id);
                    if let Some(env) = result {
                        let env = env.lock().unwrap();
                        env.windowed_context.window().request_redraw();
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(window_id) => {
                let result = get_window_context(window_id);
                if let Some(env) = result {
                    let mut env = env.lock().unwrap();
                    env.redraw();
                }
            }
            Event::UserEvent(_) => {
                let result = get_window_context(window_id);
                if let Some(env) = result {
                    let mut env = env.lock().unwrap();
                    env.redraw();
                }
            }
            _ => (),
        }
    });
}

fn render_skia(
    dom: &mut &SkiaDom,
    canvas: &mut &mut Canvas,
    node: &NodeData<NodeState>,
    viewport: &Viewport,
) {
    let node = node.node.as_ref().unwrap();
    match &node.node_type {
        NodeType::Element { tag, children, .. } => {
            match tag.as_str() {
                "div" => {
                    let mut path = Path::new();
                    let mut paint = Paint::default();

                    paint.set_anti_alias(true);
                    paint.set_style(PaintStyle::Fill);
                    paint.set_color(node.state.style.background);

                    let x = viewport.x;
                    let y = viewport.y;

                    let x2 = x + viewport.width;
                    let y2 = y + viewport.height;

                    path.move_to((x, y));
                    path.line_to((x2, y));
                    path.line_to((x2, y2));
                    path.line_to((x, y2));

                    path.close();
                    canvas.draw_path(&path, &paint);
                }
                "p" => {
                    let style = FontStyle::new(Weight::NORMAL, Width::NORMAL, Slant::Upright);
                    let type_face = Typeface::new("inherit", style).unwrap();
                    let font = Font::new(type_face, 15.0);

                    let mut paint = Paint::default();

                    paint.set_anti_alias(true);
                    paint.set_style(PaintStyle::StrokeAndFill);
                    paint.set_color(Color::WHITE);

                    let child_id = children.get(0);

                    let text = if let Some(child_id) = child_id {
                        let child: Node<NodeState> = {
                            let dom = dom.lock().unwrap();
                            dom.index(*child_id).clone()
                        };

                        if let NodeType::Text { text } = child.node_type {
                            text
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    let x = viewport.x;
                    let y = viewport.y + 10 /* Line height, wip */;

                    canvas.draw_str_align(text, (x, y), &font, &paint, Align::Left);
                }
                _ => {}
            }
        }
        NodeType::Text { .. } => {}
        NodeType::Placeholder => {}
    }
}

fn render(dom: &SkiaDom, canvas: &mut Canvas, viewport: Viewport) {
    let root: Node<NodeState> = {
        let dom = dom.lock().unwrap();
        dom.index(ElementId(0)).clone()
    };
    calculate_node::<(&SkiaDom, &mut Canvas), NodeState>(
        &NodeData::<NodeState> {
            width: SizeMode::Percentage(100),
            height: SizeMode::Percentage(100),
            padding: (0, 0, 0, 0),
            node: Some(root),
        },
        viewport.clone(),
        viewport,
        &mut (dom, canvas),
        |node_id, (dom, _)| {
            let child = {
                let dom = dom.lock().unwrap();
                dom.index(*node_id).clone()
            };

            Some(NodeData::<NodeState> {
                width: child.state.size.width,
                height: child.state.size.height,
                padding: child.state.size.padding,
                node: Some(child),
            })
        },
        |node, viewport, (dom, canvas)| {
            render_skia(dom, canvas, node, viewport);
        },
    );
}
