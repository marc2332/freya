use dioxus_core::ElementId;
use dioxus_native_core::real_dom::{Node, NodeType, RealDom};
use glutin::event::WindowEvent;
use skia_safe::{
    font_style::{Slant, Weight, Width},
    utils::text_utils::Align,
    Canvas, Font, FontStyle, Paint, PaintStyle, Typeface,
};
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

use crate::{
    elements::div::container,
    node::{NodeState, SizeMode},
};

type SkiaDom = Arc<Mutex<RealDom<NodeState>>>;

pub fn run(skia_dom: SkiaDom, rev_render: Receiver<()>) {
    type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

    let el = EventLoop::new();

    // Guarantee the drop order inside the FnMut closure. `WindowedContext` _must_ be dropped after
    // `DirectContext`.
    //
    // https://github.com/rust-skia/rust-skia/issues/476
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
                &RenderContext {
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

    let surface = create_surface(&windowed_context, &fb_info, &mut gr_context);
    // let sf = windowed_context.window().scale_factor() as f32;
    // surface.canvas().scale((sf, sf));

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

    let get_window_context = move |window_id: WindowId| -> Option<Arc<Mutex<Env>>> {
        let mut win = None;
        for env in &*wins.lock().unwrap() {
            if env.lock().unwrap().windowed_context.window().id() == window_id {
                win = Some(env.clone())
            }
        }

        win
    };

    {
        let proxy = el.create_proxy();
        thread::spawn(move || {
            while let Ok(msg) = rev_render.recv() {
                proxy.send_event(msg).unwrap();
            }
        });
    }

    // let mut cursor_pos = (0.0, 0.0);

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        #[allow(deprecated)]
        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CursorMoved { .. } => {
                    // _cursor_pos = (position.x, position.y);
                }
                WindowEvent::MouseInput { state, .. } => {
                    if ElementState::Pressed == state {
                        let result = get_window_context(window_id);
                        if let Some(env) = result {
                            let mut env = env.lock().unwrap();

                            env.redraw();
                        }
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

use std::ops::Index;

pub struct RenderContext {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}

fn render_element(
    node: Node<NodeState>,
    dom: &SkiaDom,
    canvas: &mut Canvas,
    context: &RenderContext,
) -> Option<RenderContext> {
    match &node.node_type {
        NodeType::Element { tag, children, .. } => match tag.to_string().as_str() {
            "Root" => {
                for child_id in children {
                    let child = {
                        let dom = dom.lock().unwrap();
                        dom.index(*child_id).clone()
                    };
                    render_element(child, dom, canvas, context);
                }
                None
            }
            "div" => {
                let width = match node.state.size.width {
                    SizeMode::Auto => 0,
                    SizeMode::Stretch => context.width,
                    SizeMode::Manual(w) => w,
                };
                let height = match node.state.size.height {
                    SizeMode::Auto => 0,
                    SizeMode::Stretch => context.height,
                    SizeMode::Manual(h) => h,
                };

                let ((mut path, paint), (x, y)) = container(&node, context, (width, height));

                let padding = node.state.size.padding;
                let horizontal_padding = padding.1 + padding.3;
                let vertical_padding = padding.0 + padding.2;

                let padding = node.state.size.padding;

                path.close();
                canvas.draw_path(&path, &paint);

                let mut inner_context = RenderContext {
                    x: x + (padding.3 as i32),
                    y: y + (padding.0 as i32),
                    width: (width - horizontal_padding) as i32,
                    height: (height - vertical_padding) as i32,
                };

                for child_id in children {
                    let child = {
                        let dom = dom.lock().unwrap();
                        dom.index(*child_id).clone()
                    };
                    let child_context = render_element(child, dom, canvas, &inner_context);
                    if let Some(child_context) = child_context {
                        inner_context.y = child_context.y + child_context.height;
                        inner_context.height -= child_context.height;
                    }
                }
                Some(RenderContext {
                    x,
                    y,
                    width: width as i32,
                    height: height as i32,
                })
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

                let x = context.x;
                let y = context.y + 10 /* Line height, wip */;

                canvas.draw_str_align(text, (x, y), &font, &paint, Align::Left);

                Some(RenderContext {
                    x,
                    y,
                    width: context.width,
                    height: 10,
                })
            }
            "button" => {
                /*let width = match node.state.size.width {
                    SizeMode::AUTO => 100.0,
                    SizeMode::STRETCH => context.width as f32,
                    SizeMode::Manual(w) => w,
                };
                let height = match node.state.size.height {
                    SizeMode::AUTO => 40.0,
                    SizeMode::STRETCH => context.height as f32,
                    SizeMode::Manual(h) => h,
                };

                let ((mut path, paint), (x, y)) =
                    container(&node, &context, (width, height));
                path.close();
                canvas.draw_path(&path, &paint);

                let mut inner_context = RenderContext {
                    x: x + (horizontal_padding as i32),
                    y: y + (vertical_padding as i32),
                    width: (width - horizontal_padding) as i32,
                    height: (height - vertical_padding) as i32,
                };

                for child_id in children {
                    let child = {
                        let dom = dom.lock().unwrap();
                        dom.index(child_id.clone()).clone()
                    };
                    let child_context = render_element(child, dom, canvas, &inner_context);
                    if let Some(child_context) = child_context {
                        inner_context.y = child_context.y + child_context.height;
                        inner_context.height -= child_context.height;
                    }
                }

                Some(RenderContext {
                    x,
                    y,
                    width: width as i32,
                    height: height as i32,
                }) */
                None
            }
            _ => None,
        },
        NodeType::Text { .. } => None,
        NodeType::Placeholder => None,
    }
}

fn render(dom: &SkiaDom, canvas: &mut Canvas, context: &RenderContext) {
    let root: Node<NodeState> = {
        let dom = dom.lock().unwrap();
        dom.index(ElementId(0)).clone()
    };
    render_element(root, dom, canvas, context);
}
