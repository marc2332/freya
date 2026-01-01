#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::time::Instant;

use freya::helpers::from_fn_captured;
use freya_core::{
    integration::*,
    prelude::*,
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [33, 66, 95, 99]))]
fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    fn app() -> Element {
        rect()
            .children([
                from_fn_captured(|| counter(5)),
                from_fn_captured(|| counter(10)),
            ])
            .into()
    }

    fn counter(stuff: u8) -> Element {
        let mut value = use_state(|| stuff);
        let count = use_consume::<State<i32>>();

        rect()
            .layer(value() as i16)
            .on_mouse_up(move |_| {
                *value.write() += 1;
            })
            .children(
                [
                    label().text(format!("Value is {}", value())).into(),
                    rect()
                        .children(
                            (0..count())
                                .map(|i| label().text(format!("Value is {i}")).into())
                                .collect::<Vec<_>>(),
                        )
                        .into(),
                    label().text("Hey!").into(),
                ]
                .into_iter()
                .chain({
                    if value() == stuff + 1 {
                        vec![label().text("Hello World!").into()]
                    } else {
                        vec![]
                    }
                })
                .collect::<Vec<Element>>(),
            )
            .into()
    }

    let mut runner = Runner::new(app);
    let mut state = runner.provide_root_context(|| State::create(10000));
    let mut tree = Tree::default();

    let instant = Instant::now();
    let updates = runner.sync_and_update();
    tree.apply_mutations(updates);
    println!("+ {}ms", instant.elapsed().as_millis());

    let instant = Instant::now();
    runner.handle_event(
        5,
        EventName::MouseUp,
        EventType::Mouse(MouseEventData::default()),
        false,
    );
    let updates = runner.sync_and_update();
    tree.apply_mutations(updates);
    println!("/ {}ms", instant.elapsed().as_millis());

    state.set(1000);
    let instant = Instant::now();
    let updates = runner.sync_and_update();
    tree.apply_mutations(updates);
    println!("- {}ms", instant.elapsed().as_millis());
}
