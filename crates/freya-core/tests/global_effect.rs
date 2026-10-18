use std::{
    cell::RefCell,
    rc::Rc,
};

use freya_core::{
    current_context::CurrentContext,
    prelude::*,
    runner::Runner,
};
use futures_util::task::noop_waker;

#[test]
fn global_effect_tracks_dependencies_and_cleans_up_without_a_window() {
    let globals = GlobalContexts::register();
    let tasks = globals.insert_context(GlobalTasks::new(noop_waker()));
    let mut choose_first = State::create_global(true);
    let mut first = State::create_global(1);
    let mut second = State::create_global(10);
    let values = Rc::new(RefCell::new(Vec::new()));
    let captured = Rc::new(());
    let weak_captured = Rc::downgrade(&captured);

    let effect = Effect::create_global({
        let values = values.clone();
        move || {
            let _captured = &captured;
            assert!(CurrentContext::try_with(|_| ()).is_none());
            values.borrow_mut().push(if *choose_first.read() {
                *first.read()
            } else {
                *second.read()
            });
        }
    });
    tasks.poll();
    assert_eq!(*values.borrow(), [1]);

    let runner = Runner::new(|| rect().into());
    drop(runner);
    first.set(2);
    tasks.poll();
    assert_eq!(*values.borrow(), [1, 2]);

    choose_first.set(false);
    tasks.poll();
    first.set(3);
    tasks.poll();
    assert_eq!(*values.borrow(), [1, 2, 10]);
    second.set(11);
    tasks.poll();
    assert_eq!(*values.borrow(), [1, 2, 10, 11]);

    effect.cancel();
    assert!(weak_captured.upgrade().is_none());
    second.set(12);
    tasks.poll();
    assert_eq!(*values.borrow(), [1, 2, 10, 11]);

    let captured = Rc::new(());
    let weak_captured = Rc::downgrade(&captured);
    Effect::create_global(move || {
        let _captured = &captured;
        first.read();
    });
    tasks.poll();
    tasks.clear();
    assert!(weak_captured.upgrade().is_none());
    first.set(4);
    globals.unregister();
}
