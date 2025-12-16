use std::{
    cell::RefCell,
    rc::Rc,
};

use crate::{
    prelude::{
        State,
        spawn,
        use_hook,
        use_reactive,
    },
    reactive_context::ReactiveContext,
};

pub struct Effect;

impl Effect {
    pub fn create(mut callback: impl FnMut() + 'static) {
        let (rx, rc) = ReactiveContext::new_for_task();
        spawn(async move {
            loop {
                ReactiveContext::run(rc.clone(), &mut callback);
                rx.notified().await;
            }
        });
    }

    // TODO: This should probably not be sync but instead Freya should prioritize effect tasks
    pub fn create_sync_with_gen(mut callback: impl FnMut(usize) + 'static) {
        let (rx, rc) = ReactiveContext::new_for_task();
        ReactiveContext::run(rc.clone(), || callback(0));
        spawn(async move {
            let mut current_gen = 1;
            loop {
                rx.notified().await;
                ReactiveContext::run(rc.clone(), || callback(current_gen));
                current_gen += 1;
            }
        });
    }

    pub fn create_sync(mut callback: impl FnMut() + 'static) {
        let (rx, rc) = ReactiveContext::new_for_task();
        ReactiveContext::run(rc.clone(), &mut callback);
        spawn(async move {
            loop {
                rx.notified().await;
                ReactiveContext::run(rc.clone(), &mut callback);
            }
        });
    }

    pub fn create_after(callback: impl FnMut() + 'static) {
        let (rx, rc) = ReactiveContext::new_for_task();
        let callback = Rc::new(RefCell::new(callback));
        spawn(async move {
            loop {
                let callback = callback.clone();
                let rc = rc.clone();
                spawn(async move {
                    ReactiveContext::run(rc, &mut *callback.borrow_mut());
                });
                rx.notified().await;
            }
        });
    }

    pub fn create_value<T: 'static>(mut callback: impl FnMut() -> T + 'static) -> State<T> {
        let (rx, rc) = ReactiveContext::new_for_task();
        let mut state = State::create(ReactiveContext::run(rc.clone(), &mut callback));
        spawn(async move {
            let mut current_gen = 0;
            loop {
                if current_gen > 0 {
                    state.set(ReactiveContext::run(rc.clone(), &mut callback));
                }
                rx.notified().await;
                current_gen += 1;
            }
        });
        state
    }
}

/// Registers a callback that will run every time a [crate::lifecycle::state::State] which was [crate::lifecycle::state::State::read] inside, changes.
pub fn use_side_effect(callback: impl FnMut() + 'static) {
    use_hook(|| Effect::create(callback));
}

pub fn use_after_side_effect(callback: impl FnMut() + 'static) {
    use_hook(|| Effect::create_after(callback));
}

pub fn use_side_effect_value<T: 'static>(callback: impl FnMut() -> T + 'static) -> State<T> {
    use_hook(|| Effect::create_value(callback))
}

pub fn use_side_effect_with_deps<D: 'static + Clone + PartialEq>(
    deps: &D,
    mut callback: impl FnMut(&D) + 'static,
) {
    let deps = use_reactive(deps);
    use_hook(move || {
        Effect::create(move || {
            let deps = deps.read();
            callback(&deps)
        })
    });
}
