use std::rc::Rc;

use crate::{
    current_context::CurrentContext,
    runner::Message,
    scope_id::ScopeId,
};

static HOOKS_ERROR: &str = "
Hook functions must follow these rules:
1. You cannot call them conditionally

The following is not allowed and will result in this runtime error.

#[derive(PartialEq)]
struct CoolComp(u8);

impl Render for CoolComp {
    fn render(&self) -> impl IntoElement {
        if self.0 == 2 {
            let state = use_state(|| 5);
        }

        rect().into()
    }
}

2. You cannot call them in for-loops

The following is not allowed and will result in this runtime error.

#[derive(PartialEq)]
struct CoolComp(u8);

impl Render for CoolComp {
    fn render(&self) -> impl IntoElement {
        for i in 0..self.0 {
            let state = use_state(|| 5);
        }

        rect().into()
    }
}

3. You cannot call hooks inside other hooks, event handlers, they should be called in the top of `render` methods from components.

The following is not allowed and will result in this runtime error.

#[derive(PartialEq)]
struct CoolComp(u8);

impl Render for CoolComp {
    fn render(&self) -> impl IntoElement {
        use_side_effect(|| {
            let state = use_state(|| 5);
        })

        rect().into()
    }
}
";

pub fn use_hook<T: Clone + 'static>(init: impl FnOnce() -> T) -> T {
    if let Some(value) = CurrentContext::with(|context| {
        let mut scopes_storages = context.scopes_storages.borrow_mut();
        let scopes_storage = scopes_storages
            .get_mut(&context.scope_id)
            .expect(HOOKS_ERROR);
        if let Some(value) = scopes_storage
            .values
            .get(scopes_storage.current_value)
            .cloned()
        {
            scopes_storage.current_value += 1;
            Some(value.downcast_ref::<T>().cloned().expect(HOOKS_ERROR))
        } else if scopes_storage.current_run > 0 {
            panic!("{HOOKS_ERROR}")
        } else {
            None
        }
    }) {
        value
    } else {
        let value = init();
        CurrentContext::with(|context| {
            let mut scopes_storages = context.scopes_storages.borrow_mut();
            let scopes_storage = scopes_storages
                .get_mut(&context.scope_id)
                .expect(HOOKS_ERROR);
            scopes_storage.values.push(Rc::new(value.clone()));
            scopes_storage.current_value += 1;
            value
        })
    }
}

struct DropInner(Option<Box<dyn FnOnce()>>);

impl std::ops::Drop for DropInner {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

pub fn use_drop(drop: impl FnOnce() + 'static) {
    use_hook(|| Rc::new(DropInner(Some(Box::new(drop)))));
}

pub fn current_scope_id() -> ScopeId {
    CurrentContext::with(|context| context.scope_id)
}

pub fn mark_scope_as_dirty(scope_id: ScopeId) {
    CurrentContext::with(|context| {
        context
            .sender
            .unbounded_send(Message::MarkScopeAsDirty(scope_id))
            .unwrap();
    })
}
