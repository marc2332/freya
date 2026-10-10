use std::{
    cell::{
        Cell,
        RefCell,
    },
    rc::Rc,
    time::Duration,
};

use freya_core::{
    current_context::CurrentContext,
    prelude::*,
    runner::Runner,
};
use freya_testing::prelude::launch_test;
use futures_channel::oneshot;
use futures_util::{
    future::pending,
    task::noop_waker,
};

#[test]
fn global_handles_cancel_without_context() {
    let tasks = GlobalTasks::new(noop_waker());
    let captured = Rc::new(());
    let weak_captured = Rc::downgrade(&captured);
    let child = tasks.spawn(pending()).owned();
    let child_handle = child.downgrade();
    let handle = tasks.spawn(async move {
        let _child = child;
        let _captured = captured;
        pending::<()>().await;
    });
    let owned = handle.clone().owned();
    let cloned = owned.clone();

    tasks.poll();
    drop(owned);
    assert!(!handle.is_finished());
    drop(cloned);
    assert!(handle.is_finished());
    assert!(child_handle.is_finished());
    assert!(weak_captured.upgrade().is_none());

    let handle = tasks.spawn(pending());
    let owned = handle.clone().owned();
    drop(tasks);
    assert!(handle.is_finished());
    assert!(owned.is_finished());
    handle.cancel();
}

#[test]
fn global_tasks_survive_window_close() {
    let global_contexts = GlobalContexts::register();
    global_contexts.insert_context(GlobalTasks::new(noop_waker()));
    let phases = global_contexts.insert_context(Rc::new(Cell::new(0)));
    assert!(Rc::ptr_eq(
        &phases,
        &GlobalContexts::register().get_context::<Rc<Cell<i32>>>()
    ));
    let runner = Runner::new(|| rect().into());

    let (sender, receiver) = oneshot::channel::<()>();
    let task = runner.run_in(|| {
        spawn_global(async move {
            assert!(CurrentContext::try_with(|_| ()).is_none());
            let phases = GlobalContexts::get().get_context::<Rc<Cell<i32>>>();
            phases.set(1);
            receiver.await.unwrap();
            assert!(CurrentContext::try_with(|_| ()).is_none());
            assert!(Rc::ptr_eq(
                &phases,
                &GlobalContexts::get().get_context::<Rc<Cell<i32>>>()
            ));
            phases.set(2);
            GlobalContexts::get().get_context_or_insert(|| {
                spawn_global(async {
                    GlobalContexts::get().get_context::<Rc<Cell<i32>>>().set(3);
                })
            });
        })
    });

    GlobalTasks::get().poll();
    assert_eq!(phases.get(), 1);
    assert!(GlobalContexts::try_get().is_some());
    assert!(!task.is_finished());
    drop(runner);
    sender.send(()).unwrap();
    GlobalTasks::get().poll();
    assert_eq!(phases.get(), 2);
    assert!(task.is_finished());
    GlobalTasks::get().poll();
    assert_eq!(phases.get(), 3);
    global_contexts.unregister();
    assert!(GlobalContexts::try_get().is_none());
}

#[test]
fn global_handles_cancel_on_unmount() {
    #[derive(PartialEq)]
    struct Worker {
        key: DiffKey,
        handle: Rc<RefCell<Option<TaskHandle>>>,
    }

    impl KeyExt for Worker {
        fn write_key(&mut self) -> &mut DiffKey {
            &mut self.key
        }
    }

    impl Component for Worker {
        fn render(&self) -> impl IntoElement {
            use_hook(|| {
                let owned = spawn_global(pending()).owned();
                self.handle.replace(Some(owned.downgrade()));
                owned
            });
            rect()
        }

        fn render_key(&self) -> DiffKey {
            self.key.clone().or(self.default_key())
        }
    }

    let global_contexts = GlobalContexts::register();
    global_contexts.insert_context(GlobalTasks::new(noop_waker()));
    let handle = Rc::new(RefCell::new(None));
    let mut show = State::create_global(true);
    let mut runner = launch_test({
        let handle = handle.clone();
        move || {
            rect().maybe(*show.read(), |el| {
                el.child(Worker {
                    key: DiffKey::None,
                    handle: handle.clone(),
                })
            })
        }
    });

    let handle = handle.borrow().clone().unwrap();
    assert!(!handle.is_finished());
    show.set(false);
    runner.sync_and_update();
    assert!(handle.is_finished());
    drop(runner);
    global_contexts.unregister();
}

#[test]
fn global_tasks_work_in_headless_runner() {
    let completed = State::create_global(false);
    let mut runner = launch_test(move || {
        use_hook(|| {
            let mut completed = completed;
            spawn_global(async move {
                timer(Duration::from_millis(5)).await;
                assert!(GlobalContexts::try_get().is_some());
                completed.set(true);
            })
        });
        rect()
    });

    runner.poll_n(Duration::from_millis(1), 30);
    assert!(*completed.peek());
    let global_contexts = GlobalContexts::get();
    drop(runner);
    global_contexts.unregister();
}
