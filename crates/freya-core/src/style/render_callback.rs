use std::{
    cell::RefCell,
    fmt,
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};

use freya_engine::prelude::{
    Canvas,
    FontCollection,
};
use torin::prelude::{
    Point2D,
    Size2D,
};

use crate::data::TextStyleState;

/// Context for drawing a rect background or overlaid text in local logical coordinates.
pub struct RenderContext<'a> {
    pub canvas: &'a Canvas,
    pub font_collection: &'a mut FontCollection,
    /// Position in the window in logical coordinates. The canvas remains local to this position.
    pub origin: Point2D,
    pub size: Size2D,
    pub text_style_state: &'a TextStyleState,
}

type Callback = Rc<RefCell<dyn FnMut(&mut RenderContext)>>;

#[derive(Clone)]
pub struct RenderCallback(Callback);

impl RenderCallback {
    pub fn new(callback: impl FnMut(&mut RenderContext) + 'static) -> Self {
        Self(Rc::new(RefCell::new(callback)))
    }

    pub fn call(&self, context: &mut RenderContext) {
        (self.0.borrow_mut())(context);
    }
}

impl<F: FnMut(&mut RenderContext) + 'static> From<F> for RenderCallback {
    fn from(callback: F) -> Self {
        Self::new(callback)
    }
}

impl PartialEq for RenderCallback {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Hash for RenderCallback {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}

impl fmt::Debug for RenderCallback {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("RenderCallback").finish()
    }
}

impl fmt::Display for RenderCallback {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "callback({:p})", Rc::as_ptr(&self.0))
    }
}
