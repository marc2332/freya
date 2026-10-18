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
    Paint,
    TextStyle,
};
use torin::prelude::{
    Point2D,
    Size2D,
};

use crate::{
    data::TextStyleState,
    prelude::Color,
};

/// Custom fill target.
pub enum RenderTarget<'a> {
    /// Draw the background.
    Background {
        canvas: &'a Canvas,
        font_collection: &'a mut FontCollection,
    },
    /// Set the text paint.
    TextStyle(&'a mut TextStyle),
}

/// Context for drawing a background or configuring text paint.
pub struct RenderContext<'a> {
    pub target: RenderTarget<'a>,
    /// Background position in logical coordinates or text fill position within the paragraph.
    pub origin: Point2D,
    /// Background size in logical coordinates or text fill size in physical coordinates.
    pub size: Size2D,
    pub text_style_state: &'a TextStyleState,
}

impl RenderContext<'_> {
    /// Draws with this paint or applies it to the text foreground.
    pub fn set_paint(&mut self, paint: &Paint) {
        match &mut self.target {
            RenderTarget::Background { canvas, .. } => {
                canvas.draw_paint(paint);
            }
            RenderTarget::TextStyle(text_style) => {
                text_style.set_foreground_paint(paint);
            }
        }
    }

    /// Fills the background or sets the text color.
    pub fn set_color(&mut self, color: impl Into<Color>) {
        let color = color.into();
        match &mut self.target {
            RenderTarget::Background { canvas, .. } => {
                let mut paint = Paint::default();
                paint.set_color(color);
                canvas.draw_paint(&paint);
            }
            RenderTarget::TextStyle(text_style) => {
                text_style.clear_foreground_color();
                text_style.set_color(color);
            }
        }
    }
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

impl Default for RenderCallback {
    fn default() -> Self {
        Self::new(|_| {})
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
        write!(formatter, "RenderCallback({:p})", Rc::as_ptr(&self.0))
    }
}
