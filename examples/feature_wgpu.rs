#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

//! A triangle drawn by wgpu into a texture Freya shows without any copy, with Freya over it.
//!
//! Run it with `cargo run --example feature_wgpu --features wgpu`.

use std::{
    cell::RefCell,
    rc::Rc,
};

use freya::{
    animation::*,
    prelude::*,
    wgpu::{
        WgpuSetup,
        WgpuSetupOptions,
        prelude::*,
        wgpu,
    },
};

const SHADER: &str = r#"
struct Uniforms {
    angle: f32,
    hue: f32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vertex_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var corners = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.6),
        vec2<f32>(-0.55, -0.4),
        vec2<f32>(0.55, -0.4),
    );
    let corner = corners[index];
    let rotated = vec2<f32>(
        corner.x * cos(uniforms.angle) - corner.y * sin(uniforms.angle),
        corner.x * sin(uniforms.angle) + corner.y * cos(uniforms.angle),
    );

    var output: VertexOutput;
    output.position = vec4<f32>(rotated, 0.0, 1.0);
    output.color = vec3<f32>(
        0.5 + 0.5 * cos(uniforms.hue + f32(index) * 2.1),
        0.5 + 0.5 * cos(uniforms.hue + f32(index) * 2.1 + 2.0),
        0.5 + 0.5 * cos(uniforms.hue + f32(index) * 2.1 + 4.0),
    );
    return output;
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
"#;

fn main() {
    let setup = futures_lite::future::block_on(WgpuSetup::new(WgpuSetupOptions::default()))
        .expect("Could not create a wgpu device Freya can share");

    let external_gpu_device = setup
        .external_gpu_device()
        .expect("Could not read the raw handles of the wgpu device");

    launch(
        LaunchConfig::new()
            .with_external_gpu_device(external_gpu_device)
            .with_plugin(setup.plugin())
            .with_window(WindowConfig::new(app).with_title("Freya + WGPU")),
    )
}

fn app() -> impl IntoElement {
    let animation = use_animation(|conf| {
        conf.on_creation(OnCreation::Run);
        conf.on_finish(OnFinish::restart());
        AnimNum::new(0.0, std::f32::consts::TAU).time(4000)
    });

    Viewport {
        angle: animation.read().value(),
    }
}

#[derive(PartialEq)]
struct Viewport {
    angle: f32,
}

impl Component for Viewport {
    fn render(&self) -> impl IntoElement {
        let pipeline = use_hook(|| Rc::new(RefCell::new(None::<TrianglePipeline>)));
        let angle = self.angle;

        WgpuViewer::new(move |texture, frame| {
            pipeline
                .borrow_mut()
                .get_or_insert_with(|| TrianglePipeline::new(&frame.context))
                .draw(&frame.context, texture, angle);
        })
        .padding(16.0)
        .main_align(Alignment::end())
        .cross_align(Alignment::start())
        .child(
            rect()
                .padding(10.0)
                .corner_radius(8.0)
                .background(Color::from_rgb(0, 0, 0))
                .child(
                    label()
                        .text("Freya + WGPU")
                        .color(Color::from_rgb(235, 235, 240)),
                ),
        )
    }
}

struct TrianglePipeline {
    pipeline: wgpu::RenderPipeline,
    uniforms: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl TrianglePipeline {
    fn new(context: &WgpuContext) -> Self {
        let device = &context.device;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });

        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("triangle uniforms"),
            size: 8,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("triangle uniforms"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("triangle uniforms"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms.as_entire_binding(),
            }],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("triangle"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: GpuTextureFormat::Rgba8Unorm.as_wgpu(),
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        Self {
            pipeline,
            uniforms,
            bind_group,
        }
    }

    /// Draw one frame into the shared texture and submit it.
    fn draw(&self, context: &WgpuContext, texture: &GpuTexture, angle: f32) {
        let uniforms = [angle, angle * 0.5];
        let uniform_bytes = unsafe {
            std::slice::from_raw_parts(uniforms.as_ptr() as *const u8, size_of::<[f32; 2]>())
        };
        context.queue.write_buffer(&self.uniforms, 0, uniform_bytes);

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("triangle"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("triangle"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: texture.view(),
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.07,
                            g: 0.07,
                            b: 0.09,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.draw(0..3, 0..1);
        }

        context.queue.submit([encoder.finish()]);
    }
}
