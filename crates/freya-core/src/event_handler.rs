use std::{
    cell::RefCell,
    rc::Rc,
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
        true
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
