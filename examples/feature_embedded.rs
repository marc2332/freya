#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::borrow::Cow;

use freya_core::{
    integration::*,
    prelude::*,
};
use freya_engine::prelude::{
    EncodedImageFormat,
    FontCollection,
    FontMgr,
    TypefaceFontProvider,
    raster_n32_premul,
};
use futures_channel::mpsc::{
    UnboundedReceiver,
    UnboundedSender,
    unbounded,
};
use torin::prelude::{
    Size,
    Size2D,
};

/// Reads the shared `progress` state, updated once per frame, to animate.
fn app() -> impl IntoElement {
    let progress = use_consume::<State<f32>>();
    let value = progress();

    let bar_width = 60.0 + value * 240.0;
    let hue = (40.0 + value * 200.0) as u8;

    rect()
        .expanded()
        .center()
        .spacing(20.0)
        .background((18, 18, 28))
        .child(
            rect()
                .width(Size::px(bar_width))
                .height(Size::px(70.0))
                .corner_radius(16.0)
                .background((hue, 120, 220)),
        )
        .child(
            label()
                .text(format!("progress: {value:.2}"))
                .color(Color::WHITE),
        )
}

/// A self contained Freya renderer built directly from `freya_core`.
struct EmbeddedFreya {
    runner: Runner,
    tree: Tree,
    progress: State<f32>,

    font_collection: FontCollection,
    font_manager: FontMgr,
    default_fonts: Vec<Cow<'static, str>>,

    events_sender: UnboundedSender<EventsChunk>,
    events_receiver: UnboundedReceiver<EventsChunk>,

    size: Size2D,
    scale_factor: f64,
}

impl EmbeddedFreya {
    /// Builds the renderer and provides the shared `progress` state at the root.
    fn new(size: Size2D, scale_factor: f64) -> Self {
        let (events_sender, events_receiver) = unbounded();

        let mut runner = Runner::new(|| app().into_element());
        let progress = runner.provide_root_context(|| State::create(0.0));

        let mut font_collection = FontCollection::new();
        let default_font_manager = FontMgr::default();
        let dynamic_font_manager: FontMgr = TypefaceFontProvider::new().into();
        font_collection.set_default_font_manager(default_font_manager, None);
        font_collection.set_dynamic_font_manager(dynamic_font_manager.clone());

        Self {
            runner,
            tree: Tree::default(),
            progress,
            font_collection,
            font_manager: dynamic_font_manager,
            default_fonts: default_fonts(),
            events_sender,
            events_receiver,
            size,
            scale_factor,
        }
    }

    /// Updates the state the app animates from.
    fn set_progress(&mut self, value: f32) {
        *self.progress.write() = value;
    }

    /// Advances one frame, applying the tree mutations and measuring layout.
    fn advance(&mut self) {
        let mutations = self.runner.sync_and_update();
        self.runner.run_in(|| self.tree.apply_mutations(mutations));
        self.tree.measure_layout(
            self.size,
            &mut self.font_collection,
            &self.font_manager,
            &self.events_sender,
            self.scale_factor,
            &self.default_fonts,
        );

        // Drain the events queued during the frame so the channel stays bounded.
        while self.events_receiver.try_recv().is_ok() {}
    }

    /// Renders the current tree onto a raster surface and writes it as a PNG.
    fn save_frame(&mut self, path: impl AsRef<std::path::Path>) {
        // This renders on the CPU, but real Freya apps use GPU backends like OpenGL, Vulkan or Metal.
        let mut surface = raster_n32_premul((self.size.width as i32, self.size.height as i32))
            .expect("Failed to create the raster surface.");

        RenderPipeline {
            font_collection: &mut self.font_collection,
            font_manager: &self.font_manager,
            tree: &self.tree,
            canvas: surface.canvas(),
            scale_factor: self.scale_factor,
            background: Color::WHITE,
        }
        .render();

        let image = surface.image_snapshot();
        let mut context = surface.direct_context();
        let data = image
            .encode(context.as_mut(), EncodedImageFormat::PNG, None)
            .expect("Failed to encode the frame.");

        std::fs::write(path, data.as_bytes()).expect("Failed to write the frame file.");
    }
}

fn main() {
    let output_dir = "./embedded-frames";
    std::fs::create_dir_all(output_dir).expect("Failed to create the output directory.");

    let mut embedded = EmbeddedFreya::new(Size2D::new(400.0, 300.0), 1.0);

    let total_frames = 6;
    for frame in 0..total_frames {
        let progress = frame as f32 / (total_frames - 1) as f32;
        embedded.set_progress(progress);
        embedded.advance();

        let path = format!("{output_dir}/frame-{frame}.png");
        embedded.save_frame(&path);
        println!("Saved {path}");
    }
}
