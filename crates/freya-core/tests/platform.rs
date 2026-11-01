use std::{
    cell::RefCell,
    panic::{
        AssertUnwindSafe,
        catch_unwind,
    },
    rc::Rc,
};

use freya_core::prelude::*;
use freya_testing::prelude::launch_test;
use futures_util::task::noop_waker;

#[test]
fn platform_reads_selected_windows_and_keeps_calling_window_context() {
    let runner = launch_test(|| rect());
    let platform = Platform::get();
    assert!(runner.run_in(|| try_consume_root_context::<PlatformWindow>().is_none()));
    let mut current = runner.run_in(|| platform.current_window());
    current.is_app_focused.set(false);

    let other_id = 99_u64;
    let mut other = current.clone();
    other.id = other_id;
    other.root_size = State::create_global((900., 600.).into());
    other.is_app_focused = State::create_global(true);
    platform.register_window(other);

    assert_eq!(platform.window(other_id).root_size.peek().width, 900.);
    assert_eq!(runner.run_in(|| platform.current_window().id), current.id);
    assert!(platform.try_window(100_u64).is_none());
    assert!(catch_unwind(AssertUnwindSafe(|| platform.window(100_u64))).is_err());

    platform.unregister_window(other_id);
    assert!(platform.try_window(other_id).is_none());
    drop(runner);
    assert!(platform.try_window(current.id).is_none());
}

#[test]
fn global_platform_works_after_its_originating_window_closes() {
    let runner = launch_test(|| rect());
    let events = Rc::new(RefCell::new(Vec::new()));
    let platform = Platform::new({
        let events = events.clone();
        move |event| events.borrow_mut().push(event)
    });
    let window = runner.run_in(|| Platform::get().current_window());
    let window_id = window.id;
    platform.register_window(window);
    let contexts = GlobalContexts::get();
    contexts.insert_context(platform.clone());
    let tasks = contexts.insert_context(GlobalTasks::new(noop_waker()));
    let task = runner.run_in(|| {
        spawn_global(async {
            assert!(catch_unwind(AssertUnwindSafe(|| Platform::get().current_window())).is_err());
            Platform::get().open_url("https://freyaui.dev");
            Platform::get().exit();
        })
    });
    drop(runner);
    assert!(platform.try_window(window_id).is_none());
    tasks.poll();

    assert!(task.is_finished());
    assert!(
        matches!(&events.borrow()[0], GlobalUserEvent::OpenUrl(url) if url == "https://freyaui.dev")
    );
    assert!(matches!(&events.borrow()[1], GlobalUserEvent::Exit));
    tasks.clear();
    contexts.unregister();
}
