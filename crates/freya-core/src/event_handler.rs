use std::{
    cell::RefCell,
    rc::Rc,
};

use crate::{
    current_context::CurrentContext,
    prelude::current_scope_id,
    scope_id::ScopeId,
};

pub struct Callback<A, R>(Rc<RefCell<dyn FnMut(A) -> R>>);

impl<A, R> Callback<A, R> {
    pub fn new(callback: impl FnMut(A) -> R + 'static) -> Self {
        Self(Rc::new(RefCell::new(callback)))
    }

    pub fn call(&self, data: A) -> R {
        (self.0.borrow_mut())(data)
    }
}

impl<A, R> Clone for Callback<A, R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A, R> PartialEq for Callback<A, R> {
    fn eq(&self, _other: &Self) -> bool {
        // TODO: Decide whether event handlers should be captured or not.
        false
    }
}

impl<A, R, H: FnMut(A) -> R + 'static> From<H> for Callback<A, R> {
    fn from(value: H) -> Self {
        Callback::new(value)
    }
}

pub struct NoArgCallback<R>(Rc<RefCell<dyn FnMut() -> R>>);

impl<R> NoArgCallback<R> {
    pub fn new(callback: impl FnMut() -> R + 'static) -> Self {
        Self(Rc::new(RefCell::new(callback)))
    }

    pub fn call(&self) -> R {
        (self.0.borrow_mut())()
    }
}

impl<R> Clone for NoArgCallback<R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R> PartialEq for NoArgCallback<R> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<R, H: FnMut() -> R + 'static> From<H> for NoArgCallback<R> {
    fn from(value: H) -> Self {
        NoArgCallback::new(value)
    }
}

pub struct EventHandler<T>(Rc<RefCell<dyn FnMut(T)>>);

impl<T> EventHandler<T> {
    pub fn new(handler: impl FnMut(T) + 'static) -> Self {
        Self(Rc::new(RefCell::new(handler)))
    }

    /// Create an event handler that runs under the given scope.
    ///
    /// APIs that rely on the current scope, like [`spawn`](crate::prelude::spawn), will use
    /// `scope_id` instead of the scope of the element that received the event.
    pub fn new_scoped(scope_id: ScopeId, mut handler: impl FnMut(T) + 'static) -> Self {
        Self::new(move |data| CurrentContext::run_in_scope(scope_id, || handler(data)))
    }

    /// Create an event handler that runs under the scope creating it.
    ///
    /// Shortcut for [`EventHandler::new_scoped`] with the current scope, so e.g tasks spawned in
    /// the handler are cancelled once the component creating it unmounts.
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// # async fn save_document() {}
    /// # fn save_button() -> impl IntoElement {
    /// let on_press = EventHandler::new_current(|_| {
    ///     spawn(async {
    ///         save_document().await;
    ///     });
    /// });
    ///
    /// Button::new().child("Save").on_press(on_press)
    /// # }
    /// ```
    pub fn new_current(handler: impl FnMut(T) + 'static) -> Self {
        Self::new_scoped(current_scope_id(), handler)
    }

    pub fn call(&self, data: T) {
        (self.0.borrow_mut())(data);
    }
}

impl<T> Clone for EventHandler<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> PartialEq for EventHandler<T> {
    fn eq(&self, _other: &Self) -> bool {
        // TODO: Decide whether event handlers should be captured or not.
        false
    }
}

impl<H: FnMut(D) + 'static, D> From<H> for EventHandler<D> {
    fn from(value: H) -> Self {
        EventHandler::new(value)
    }
}
