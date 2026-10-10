use std::{
    cell::Cell,
    rc::Rc,
    task::Waker,
};

use freya_core::prelude::*;
use freya_radio::prelude::*;

#[derive(Default)]
struct Data {
    count: u32,
    step: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Channel {
    Count,
    Step,
}

impl RadioChannel<Data> for Channel {}

#[test]
fn reads_only_subscribe_to_the_selected_channel() {
    let globals = GlobalContexts::register();
    let tasks = globals.insert_context(GlobalTasks::new(Waker::noop().clone()));
    let mut station = RadioStation::<Data, Channel>::create_global(Data::default());
    let runs = Rc::new(Cell::new(0));
    let effect = Effect::create_global({
        let runs = runs.clone();
        move || {
            station.read_channel(Channel::Step);
            runs.set(runs.get() + 1);
        }
    });
    tasks.poll();
    assert_eq!(runs.get(), 1);

    station.write_channel(Channel::Count).count = 1;
    tasks.poll();
    assert_eq!(runs.get(), 1);

    station.write_channel(Channel::Step).step = 2;
    tasks.poll();
    assert_eq!(runs.get(), 2);

    effect.cancel();
    globals.unregister();
}
