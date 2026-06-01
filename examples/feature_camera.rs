#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    camera::{
        nokhwa::{
            Camera as NokhwaCamera,
            pixel_format::RgbAFormat,
            utils::{
                RequestedFormat,
                RequestedFormatType,
            },
        },
        *,
    },
    prelude::*,
};

fn main() {
    tracing_subscriber::fmt::init();

    if !freya::camera::init() {
        eprintln!("camera access denied");
        return;
    }

    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_size(800., 600.)
                .with_title("Camera"),
        ),
    )
}

fn app() -> impl IntoElement {
    let devices = use_hook(usable_cameras);
    let mut selected = use_state(|| 0);
    let selected_device = devices
        .get(*selected.read())
        .map(|info| info.index().clone());

    let sidebar = rect()
        .width(Size::px(220.))
        .height(Size::fill())
        .padding(12.)
        .spacing(6.)
        .background((30, 30, 30))
        .child(label().color(Color::WHITE).text("Cameras"))
        .children(devices.iter().enumerate().map(|(i, info)| {
            let background = if *selected.read() == i {
                (60, 90, 130)
            } else {
                (40, 40, 40)
            };
            rect()
                .padding((6., 8.))
                .corner_radius(6.)
                .background(background)
                .on_press(move |_| selected.set(i))
                .child(label().color(Color::WHITE).text(format!(
                    "{}: {}",
                    info.index(),
                    info.human_name()
                )))
                .into()
        }));

    let main = rect()
        .expanded()
        .center()
        .map(selected_device, |el, device| {
            el.child(CameraPanel { device })
        });

    rect()
        .expanded()
        .background((20, 20, 20))
        .horizontal()
        .child(sidebar)
        .child(main)
}

fn usable_cameras() -> Vec<CameraInfo> {
    query()
        .unwrap_or_default()
        .into_iter()
        .filter(|info| {
            let requested =
                RequestedFormat::new::<RgbAFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
            NokhwaCamera::new(info.index().clone(), requested).is_ok()
        })
        .collect()
}

#[derive(PartialEq)]
struct CameraPanel {
    device: CameraIndex,
}

impl Component for CameraPanel {
    fn render(&self) -> impl IntoElement {
        let device = self.device.clone();
        let camera = use_camera(move || CameraConfig::new().device(device));

        CameraViewer::new(camera)
            .corner_radius(12.)
            .loading_placeholder(label().text("Opening camera...").color(Color::WHITE))
            .error_renderer(|err: CameraError| {
                label()
                    .color((255, 120, 120))
                    .text(format!("Camera error: {err}"))
                    .into()
            })
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::from(&self.device)
    }
}
