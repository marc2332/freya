//! A Bevy scene rendered into a texture that Freya draws without any copy.
//!
//! wgpu creates the GPU device, Freya builds its swapchain on it and Bevy renders on it, so all
//! three share one device and one queue.
//!
//! Run it with `cargo run --release` from this directory.

use std::{
    cell::RefCell,
    rc::Rc,
    sync::Arc,
};

use bevy::{
    app::App as BevyApp,
    camera::{
        ManualTextureViewHandle,
        RenderTarget,
    },
    prelude::*,
    render::{
        RenderPlugin,
        renderer::{
            RenderAdapter,
            RenderAdapterInfo,
            RenderDevice,
            RenderInstance,
            RenderQueue,
            WgpuWrapper,
        },
        settings::RenderCreation,
        texture::{
            ManualTextureView,
            ManualTextureViews,
        },
    },
    window::ExitCondition,
};
use freya::{
    prelude::*,
    wgpu::{
        WgpuSetup,
        WgpuSetupOptions,
        prelude::*,
    },
};

/// The slot Bevy renders into, which Freya swaps for a new texture on every resize.
const VIEWPORT: ManualTextureViewHandle = ManualTextureViewHandle(0);

const FORMAT: GpuTextureFormat = GpuTextureFormat::Rgba8UnormSrgb;

fn main() {
    let setup = futures_lite::future::block_on(WgpuSetup::new(WgpuSetupOptions::default()))
        .expect("Could not create a wgpu device Freya can share");

    let external_gpu_device = setup
        .external_gpu_device()
        .expect("Could not read the raw handles of the wgpu device");

    let bevy = Rc::new(RefCell::new(build_scene(&setup)));

    launch(
        LaunchConfig::new()
            .with_external_gpu_device(external_gpu_device)
            .with_plugin(setup.plugin())
            .with_window(WindowConfig::new_app(Viewport { bevy }).with_title("Freya + Bevy")),
    )
}

/// Build a headless Bevy app on the device wgpu already created.
fn build_scene(setup: &WgpuSetup) -> BevyApp {
    let mut app = BevyApp::new();

    app.add_plugins(
        DefaultPlugins
            .set(RenderPlugin {
                render_creation: RenderCreation::manual(
                    RenderDevice::from(setup.device.clone()),
                    RenderQueue(Arc::new(WgpuWrapper::new(setup.queue.clone()))),
                    RenderAdapterInfo(WgpuWrapper::new(setup.adapter.get_info())),
                    RenderAdapter(Arc::new(WgpuWrapper::new(setup.adapter.clone()))),
                    RenderInstance(Arc::new(WgpuWrapper::new(setup.instance.clone()))),
                ),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                close_when_requested: false,
                ..default()
            }),
    )
    .add_systems(Startup, spawn_scene)
    .add_systems(Update, spin_cube);

    app.finish();
    app.cleanup();
    app
}

#[derive(Component)]
struct Spinning;

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.4, 1.4, 1.4))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: bevy::prelude::Color::srgb(0.3, 0.7, 0.9),
            perceptual_roughness: 0.4,
            ..default()
        })),
        Spinning,
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        // The default tonemapper needs LUT assets this example does not ship.
        bevy::core_pipeline::tonemapping::Tonemapping::None,
        RenderTarget::TextureView(VIEWPORT),
        Transform::from_xyz(0.0, 1.6, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spin_cube(time: Res<Time>, mut spinning: Query<&mut Transform, With<Spinning>>) {
    for mut transform in &mut spinning {
        transform.rotate_y(time.delta_secs() * 0.9);
        transform.rotate_x(time.delta_secs() * 0.35);
    }
}

struct Viewport {
    bevy: Rc<RefCell<BevyApp>>,
}

impl freya::prelude::App for Viewport {
    fn render(&self) -> impl IntoElement {
        let bevy = self.bevy.clone();
        let platform = Platform::get();

        WgpuViewer::new(move |texture, frame| {
            let mut bevy = bevy.borrow_mut();

            // Point Bevy's camera at whatever texture Freya sized for this frame.
            bevy.world_mut()
                .resource_mut::<ManualTextureViews>()
                .insert(
                    VIEWPORT,
                    ManualTextureView {
                        texture_view: texture.view().clone().into(),
                        size: UVec2::new(frame.width, frame.height),
                        view_format: FORMAT.as_wgpu(),
                    },
                );

            bevy.update();

            // Bevy animates on its own, so ask Freya for the next frame.
            platform.send(UserEvent::RequestRedraw);
        })
        .format(FORMAT)
        .padding(16.0)
        .main_align(Alignment::end())
        .cross_align(Alignment::start())
        .child(
            rect()
                .padding(10.0)
                .corner_radius(8.0)
                .background(freya::prelude::Color::from_rgb(0, 0, 0))
                .child(
                    label()
                        .text("Freya + Bevy")
                        .color(freya::prelude::Color::from_rgb(235, 235, 240)),
                ),
        )
    }
}
