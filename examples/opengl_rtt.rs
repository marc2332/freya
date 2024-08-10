#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    ffi::CString,
    ptr,
    sync::{
        Arc,
        Mutex,
    },
};

use freya::prelude::*;
use freya_testing::prelude::CanvasRunnerContext;
use gl::types::*;
use skia_safe::Image;
fn main() {
    launch(app);
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        let c_str = std::ffi::CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            panic!("Failed to compile shader");
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    let program;
    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        // Check for linking errors
        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            panic!("Failed to link program");
        }
    }
    program
}

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core

layout (location = 0) in vec3 aPos;


void main() {
    gl_Position = vec4(aPos, 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
out vec4 FragColor;

// Define a uniform variable to accept the color from the application
uniform vec4 u_color;

void main() {
    // Set the fragment color to the color provided by the uniform
    FragColor = u_color;
}
"#;

/// Stores saved opengl state to safely mess with native rendering and restore it back.
struct GLStateGuard {
    old_framebuffer: i32,
    old_texture: i32,
    old_vao: i32,
    old_buffer: i32,
    old_unpack_alignment: i32,
    old_unpack_row_length: i32,
    old_unpack_skip_pixels: i32,
    old_unpack_skip_rows: i32,
    old_viewport: [i32; 4],
    old_scissor_box: [i32; 4],
    old_program: i32,
    old_blend: bool,
    old_blend_src_rgb: i32,
    old_blend_dst_rgb: i32,
    old_blend_src_alpha: i32,
    old_blend_dst_alpha: i32,
    old_depth_test: bool,
    old_stencil_test: bool,
    old_stencil_func: i32,
    old_stencil_ref: i32,
    old_stencil_value_mask: i32,
    old_stencil_fail: i32,
    old_stencil_pass_depth_fail: i32,
    old_stencil_pass_depth_pass: i32,
    old_stencil_writemask: i32,
    old_cull_face: bool,
    old_cull_face_mode: i32,
    old_polygon_mode: [i32; 2],
}

impl GLStateGuard {
    /// Capture current opengl context and save it for further restoring.
    /// It definitely captures more than required for this example, so you can adjust it to your
    /// needs.
    fn new() -> GLStateGuard {
        unsafe {
            // Save framebuffer binding
            let mut old_framebuffer = 0;
            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut old_framebuffer);

            // Save texture binding
            let mut old_texture = 0;
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut old_texture);

            // Save vertex array binding
            let mut old_vao = 0;
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut old_vao);

            // Save array buffer binding
            let mut old_buffer = 0;
            gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut old_buffer);

            // Save pixel store parameters
            let mut old_unpack_alignment = 0;
            gl::GetIntegerv(gl::UNPACK_ALIGNMENT, &mut old_unpack_alignment);
            let mut old_unpack_row_length = 0;
            gl::GetIntegerv(gl::UNPACK_ROW_LENGTH, &mut old_unpack_row_length);
            let mut old_unpack_skip_pixels = 0;
            gl::GetIntegerv(gl::UNPACK_SKIP_PIXELS, &mut old_unpack_skip_pixels);
            let mut old_unpack_skip_rows = 0;
            gl::GetIntegerv(gl::UNPACK_SKIP_ROWS, &mut old_unpack_skip_rows);

            // Save viewport and scissor box
            let mut old_viewport = [0; 4];
            gl::GetIntegerv(gl::VIEWPORT, old_viewport.as_mut_ptr());
            let mut old_scissor_box = [0; 4];
            gl::GetIntegerv(gl::SCISSOR_BOX, old_scissor_box.as_mut_ptr());

            // Save current program
            let mut old_program = 0;
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut old_program);

            // Save blend state
            let old_blend = gl::IsEnabled(gl::BLEND) == gl::TRUE;
            let mut old_blend_src_rgb = 0;
            gl::GetIntegerv(gl::BLEND_SRC_RGB, &mut old_blend_src_rgb);
            let mut old_blend_dst_rgb = 0;
            gl::GetIntegerv(gl::BLEND_DST_RGB, &mut old_blend_dst_rgb);
            let mut old_blend_src_alpha = 0;
            gl::GetIntegerv(gl::BLEND_SRC_ALPHA, &mut old_blend_src_alpha);
            let mut old_blend_dst_alpha = 0;
            gl::GetIntegerv(gl::BLEND_DST_ALPHA, &mut old_blend_dst_alpha);

            // Save depth test state
            let old_depth_test = gl::IsEnabled(gl::DEPTH_TEST) == gl::TRUE;

            // Save stencil test state
            let old_stencil_test = gl::IsEnabled(gl::STENCIL_TEST) == gl::TRUE;
            let mut old_stencil_func = 0;
            gl::GetIntegerv(gl::STENCIL_FUNC, &mut old_stencil_func);
            let mut old_stencil_ref = 0;
            gl::GetIntegerv(gl::STENCIL_REF, &mut old_stencil_ref);
            let mut old_stencil_value_mask = 0;
            gl::GetIntegerv(gl::STENCIL_VALUE_MASK, &mut old_stencil_value_mask);
            let mut old_stencil_fail = 0;
            gl::GetIntegerv(gl::STENCIL_FAIL, &mut old_stencil_fail);
            let mut old_stencil_pass_depth_fail = 0;
            gl::GetIntegerv(
                gl::STENCIL_PASS_DEPTH_FAIL,
                &mut old_stencil_pass_depth_fail,
            );
            let mut old_stencil_pass_depth_pass = 0;
            gl::GetIntegerv(
                gl::STENCIL_PASS_DEPTH_PASS,
                &mut old_stencil_pass_depth_pass,
            );
            let mut old_stencil_writemask = 0;
            gl::GetIntegerv(gl::STENCIL_WRITEMASK, &mut old_stencil_writemask);

            // Save cull face state
            let old_cull_face = gl::IsEnabled(gl::CULL_FACE) == gl::TRUE;
            let mut old_cull_face_mode = 0;
            gl::GetIntegerv(gl::CULL_FACE_MODE, &mut old_cull_face_mode);

            // Save polygon mode
            let mut old_polygon_mode = [0; 2];
            gl::GetIntegerv(gl::POLYGON_MODE, old_polygon_mode.as_mut_ptr());

            GLStateGuard {
                old_framebuffer,
                old_texture,
                old_vao,
                old_buffer,
                old_unpack_alignment,
                old_unpack_row_length,
                old_unpack_skip_pixels,
                old_unpack_skip_rows,
                old_viewport,
                old_scissor_box,
                old_program,
                old_blend,
                old_blend_src_rgb,
                old_blend_dst_rgb,
                old_blend_src_alpha,
                old_blend_dst_alpha,
                old_depth_test,
                old_stencil_test,
                old_stencil_func,
                old_stencil_ref,
                old_stencil_value_mask,
                old_stencil_fail,
                old_stencil_pass_depth_fail,
                old_stencil_pass_depth_pass,
                old_stencil_writemask,
                old_cull_face,
                old_cull_face_mode,
                old_polygon_mode,
            }
        }
    }
}
impl Drop for GLStateGuard {
    fn drop(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.old_framebuffer as u32);

            // Restore texture binding
            gl::BindTexture(gl::TEXTURE_2D, self.old_texture as u32);

            // Restore vertex array binding
            gl::BindVertexArray(self.old_vao as u32);

            // Restore array buffer binding
            gl::BindBuffer(gl::ARRAY_BUFFER, self.old_buffer as u32);

            // Restore pixel store parameters
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, self.old_unpack_alignment);
            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, self.old_unpack_row_length);
            gl::PixelStorei(gl::UNPACK_SKIP_PIXELS, self.old_unpack_skip_pixels);
            gl::PixelStorei(gl::UNPACK_SKIP_ROWS, self.old_unpack_skip_rows);

            // Restore viewport and scissor box
            gl::Viewport(
                self.old_viewport[0],
                self.old_viewport[1],
                self.old_viewport[2],
                self.old_viewport[3],
            );
            gl::Scissor(
                self.old_scissor_box[0],
                self.old_scissor_box[1],
                self.old_scissor_box[2],
                self.old_scissor_box[3],
            );

            // Restore current program
            gl::UseProgram(self.old_program as u32);

            // Restore blend state
            if self.old_blend {
                gl::Enable(gl::BLEND);
            } else {
                gl::Disable(gl::BLEND);
            }
            gl::BlendFuncSeparate(
                self.old_blend_src_rgb as u32,
                self.old_blend_dst_rgb as u32,
                self.old_blend_src_alpha as u32,
                self.old_blend_dst_alpha as u32,
            );

            // Restore depth test state
            if self.old_depth_test {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }

            // Restore stencil test state
            if self.old_stencil_test {
                gl::Enable(gl::STENCIL_TEST);
            } else {
                gl::Disable(gl::STENCIL_TEST);
            }
            gl::StencilFunc(
                self.old_stencil_func as u32,
                self.old_stencil_ref,
                self.old_stencil_value_mask as u32,
            );
            gl::StencilOp(
                self.old_stencil_fail as u32,
                self.old_stencil_pass_depth_fail as u32,
                self.old_stencil_pass_depth_pass as u32,
            );
            gl::StencilMask(self.old_stencil_writemask as u32);

            // Restore cull face state
            if self.old_cull_face {
                gl::Enable(gl::CULL_FACE);
            } else {
                gl::Disable(gl::CULL_FACE);
            }
            gl::CullFace(self.old_cull_face_mode as u32);

            // Restore polygon mode
            gl::PolygonMode(gl::FRONT_AND_BACK, self.old_polygon_mode[0] as u32);
        }
    }
}

struct TriangleRenderer {
    fbo: GLuint,
    program: GLuint,
    texture: GLuint,
    texture_image: Option<Image>,
    vao: GLuint,
    vbo: GLuint,
    width: i32,
    height: i32,
    color_location: GLint,
}

impl Drop for TriangleRenderer {
    fn drop(&mut self) {
        self.texture_image = None;
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteFramebuffers(1, &self.fbo);
            gl::DeleteTextures(1, &self.texture);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

impl TriangleRenderer {
    fn new() -> TriangleRenderer {
        TriangleRenderer {
            fbo: 0,
            program: 0,
            texture: 0,
            texture_image: None,
            vao: 0,
            vbo: 0,
            width: 0,
            height: 0,
            color_location: 0,
        }
    }
    fn allocate_texture(&mut self, ctx: &mut CanvasRunnerContext) {
        let current_width = ctx.area.width().round() as i32;
        let current_height = ctx.area.height().round() as i32;
        let mut create_image = false;
        unsafe {
            if self.texture == 0 {
                gl::GenTextures(1, &mut self.texture);
                gl::BindTexture(gl::TEXTURE_2D, self.texture);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGB as GLint,
                    current_width,
                    current_height,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    ptr::null(),
                );
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
                self.width = current_width;
                self.height = current_height;
                create_image = true;
            } else {
                // resize texture before rendering if required
                if self.width != current_width || self.height != current_height {
                    gl::BindTexture(gl::TEXTURE_2D, self.texture);
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGB as GLint,
                        current_width,
                        current_height,
                        0,
                        gl::RGB,
                        gl::UNSIGNED_BYTE,
                        ptr::null(),
                    );
                    self.width = current_width;
                    self.height = current_height;
                    create_image = true;
                }
            }

            if create_image {
                let backend_texture = skia_safe::gpu::backend_textures::make_gl(
                    (self.width, self.height),
                    skia_safe::gpu::Mipmapped::No,
                    skia_safe::gpu::gl::TextureInfo {
                        target: gl::TEXTURE_2D,
                        format: gl::RGBA8,
                        protected: skia_safe::gpu::Protected::No,
                        id: self.texture,
                    },
                    "rtt",
                );
                let mut direct_context = ctx.canvas.direct_context().unwrap();

                self.texture_image = Image::from_texture(
                    &mut direct_context,
                    &backend_texture,
                    skia_safe::gpu::SurfaceOrigin::TopLeft,
                    skia_safe::ColorType::RGBA8888,
                    skia_safe::AlphaType::Premul,
                    None,
                );
            }
        }
    }
    fn render(&mut self, color: (f64, f64, f64), ctx: &mut CanvasRunnerContext) {
        unsafe {
            if self.fbo == 0 {
                // create framebuffer and texture
                let mut framebuffer: GLuint = 0;
                gl::GenFramebuffers(1, &mut framebuffer);
                gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
                self.allocate_texture(ctx);
                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    gl::COLOR_ATTACHMENT0,
                    gl::TEXTURE_2D,
                    self.texture,
                    0,
                );

                if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                    panic!("Framebuffer is not complete!");
                }

                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

                self.fbo = framebuffer;

                // create shader program
                let vertex_shader = compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
                let fragment_shader = compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
                let program = link_program(vertex_shader, fragment_shader);

                gl::DeleteShader(vertex_shader);
                gl::DeleteShader(fragment_shader);
                let color_loc_name = CString::new("u_color").unwrap();
                self.color_location = gl::GetUniformLocation(program, color_loc_name.as_ptr());
                self.program = program;

                // create buffers
                let vertices: [f32; 9] = [
                    -0.5, -0.5, 0.0, // Bottom-left
                    0.5, -0.5, 0.0, // Bottom-right
                    0.0, 0.5, 0.0, // Top
                ];

                let mut vao: GLuint = 0;
                let mut vbo: GLuint = 0;

                gl::GenVertexArrays(1, &mut vao);
                gl::GenBuffers(1, &mut vbo);

                gl::BindVertexArray(vao);

                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                    vertices.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );

                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    3 * std::mem::size_of::<GLfloat>() as GLsizei,
                    ptr::null(),
                );
                gl::EnableVertexAttribArray(0);

                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::BindVertexArray(0);

                self.vao = vao;
                self.vbo = vbo;
            }

            self.allocate_texture(ctx);

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::Viewport(0, 0, self.width, self.height);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.program);
            gl::Uniform4f(
                self.color_location,
                (color.0 / 100.0) as GLfloat,
                (color.1 / 100.0) as GLfloat,
                (color.2 / 100.0) as GLfloat,
                1.0 as GLfloat,
            );
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn app() -> Element {
    let mut r = use_signal(|| 100.0);
    let mut g: Signal<f64> = use_signal(|| 0.0);
    let mut b = use_signal(|| 0.0);

    let triangle_renderer = use_hook(|| Arc::new(Mutex::new(TriangleRenderer::new())));

    let canvas = use_canvas(move || {
        let color = (*r.read(), *g.read(), *b.read());
        let triangle_renderer = triangle_renderer.clone();

        Box::new(move |ctx| {
            ctx.canvas.translate((ctx.area.min_x(), ctx.area.min_y()));
            let mut renderer_guard = triangle_renderer.lock().unwrap();
            {
                let gl_state = GLStateGuard::new();
                renderer_guard.render(color, ctx);
                drop(gl_state); // ensure we restore original opengl context state after rendering
            }
            ctx.canvas.draw_image(
                renderer_guard.texture_image.clone().unwrap(),
                (ctx.area.min_x(), ctx.area.min_y()),
                None,
            );
            ctx.canvas.restore();
        })
    });

    rsx!(
        rect {
            canvas_reference: canvas.attribute(),
            width: "100%",
            height: "100%",
            Slider {
                width: "300",
                value: *r.read(),
                onmoved: move |value: f64| { r.set(value) }
            }
            Slider {
                width: "300",
                value: *g.read(),
                onmoved: move |value: f64| { g.set(value) }
            }
            Slider {
                width: "300",
                value: *b.read(),
                onmoved: move |value: f64| { b.set(value) }
            }
        }
    )
}
