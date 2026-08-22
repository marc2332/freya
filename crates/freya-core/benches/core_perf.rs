use std::hint::black_box;

use freya_core::{
    accessibility::tree::AccessibilityTree,
    integration::*,
    prelude::*,
};
use freya_engine::prelude::{
    FontCollection,
    FontMgr,
    TypefaceFontProvider,
};
use torin::{
    prelude::Size2D,
    size::Size,
};

const ROWS: usize = 1000;
const CONTAINERS: usize = 100;
const CASCADE_ROWS: usize = 100;
const ITERATIONS: usize = 30;

#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [33, 66, 95, 99]))]
fn main() {
    elements();
    runner();
    cascades();
    accessibility();
}

#[cfg_attr(not(feature = "hotpath"), allow(unused_variables))]
fn bench<T>(name: &'static str, mut scenario: impl FnMut() -> T) {
    for _ in 0..ITERATIONS {
        hotpath::measure_block!(name, {
            black_box(scenario());
        });
    }
}

fn mounted_runner(app: impl Fn() -> Element + 'static) -> (Runner, Tree, NodeId) {
    let mut runner = Runner::new(app);
    let mut tree = Tree::default();
    tree.apply_mutations(runner.sync_and_update(), 1.0);
    let target = tree
        .listeners
        .get(&EventName::MouseUp)
        .and_then(|listeners| listeners.first().copied())
        .expect("no mouse up listener was registered");
    (runner, tree, target)
}

fn update(runner: &mut Runner, tree: &mut Tree, target: NodeId, event: EventName) {
    runner.handle_event(
        target,
        event,
        EventType::Mouse(MouseEventData::default()),
        false,
    );
    tree.apply_mutations(runner.sync_and_update(), 1.0);
}

fn build_row(index: usize) -> Element {
    rect()
        .horizontal()
        .width(Size::px(300.0))
        .height(Size::px(40.0))
        .padding(8.0)
        .spacing(4.0)
        .background((250, 250, 250))
        .corner_radius(6.0)
        .child(
            label()
                .text(format!("Row {index}"))
                .font_size(14.0)
                .max_lines(1),
        )
        .child(
            rect()
                .width(Size::px(24.0))
                .height(Size::px(24.0))
                .corner_radius(12.0)
                .background((0, 119, 255)),
        )
        .into_element()
}

fn elements() {
    bench("elements: build 1000 rows", || {
        rect()
            .vertical()
            .expanded()
            .spacing(2.0)
            .children((0..ROWS).map(build_row))
            .into_element()
    });
}

fn runner_app(rotate: bool) -> Element {
    let mut version = use_state(|| 0u64);
    let rotation = if rotate { *version.read() as usize } else { 0 };

    rect()
        .on_mouse_up(move |_| *version.write() += 1)
        .children((0..ROWS).map(|slot| {
            let index = (slot + rotation) % ROWS;
            rect()
                .key(index)
                .child(label().text(format!("Row {index}")))
                .into_element()
        }))
        .into()
}

fn runner() {
    bench("runner: mount 1000 rows", || {
        mounted_runner(|| runner_app(false))
    });

    let (mut runner, mut tree, target) = mounted_runner(|| runner_app(false));
    bench("runner: no-change rebuild", || {
        update(&mut runner, &mut tree, target, EventName::MouseUp)
    });

    let (mut runner, mut tree, target) = mounted_runner(|| runner_app(true));
    bench("runner: rotate all keys", || {
        update(&mut runner, &mut tree, target, EventName::MouseUp)
    });
}

fn cascade_app() -> Element {
    let mut color_version = use_state(|| 0u64);
    let mut effect_version = use_state(|| 0u64);
    let mut layer_version = use_state(|| 0u64);

    rect()
        .color((255, (*color_version.read() % 255) as u8, 0))
        .opacity(0.5 + (*effect_version.read() % 4) as f32 * 0.1)
        .layer((*layer_version.read() % 4) as i16)
        .on_mouse_up(move |_| *color_version.write() += 1)
        .on_mouse_down(move |_| *effect_version.write() += 1)
        .on_mouse_move(move |_| *layer_version.write() += 1)
        .children((0..CONTAINERS).map(|container| {
            rect()
                .key(container)
                .font_size(14.0 + (container % 4) as f32)
                .children(
                    (0..CASCADE_ROWS)
                        .map(|row| rect().key(row).color((row as u8, 100, container as u8))),
                )
                .into_element()
        }))
        .into()
}

fn cascades() {
    let (mut runner, mut tree, target) = mounted_runner(cascade_app);

    bench("cascades: text style", || {
        update(&mut runner, &mut tree, target, EventName::MouseUp)
    });
    bench("cascades: effect", || {
        update(&mut runner, &mut tree, target, EventName::MouseDown)
    });
    bench("cascades: layer", || {
        update(&mut runner, &mut tree, target, EventName::MouseMove)
    });
}

fn accessibility() {
    let (_runner, mut tree, _) = mounted_runner(cascade_app);

    let (events_sender, _events_receiver) = futures_channel::mpsc::unbounded();
    let mut font_collection = FontCollection::new();
    let font_manager: FontMgr = TypefaceFontProvider::new().into();
    font_collection.set_default_font_manager(FontMgr::default(), None);
    font_collection.set_dynamic_font_manager(font_manager.clone());
    tree.measure_layout(
        Size2D::new(1000.0, 1000.0),
        &mut font_collection,
        &font_manager,
        &events_sender,
        1.0,
        &[],
    );

    let mut accessibility_tree = AccessibilityTree::default();
    bench("accessibility: full build", || {
        accessibility_tree.init(&mut tree, "core_perf")
    });
}
