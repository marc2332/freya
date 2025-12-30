use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use freya_core::{
    integration::*,
    prelude::*,
};
use freya_engine::prelude::{
    ClipOp,
    Paint,
    PaintStyle,
    SkRect,
};
pub use freya_plotters_backend::*;
pub use plotters;

type Callback = Rc<RefCell<dyn FnMut(&mut RenderContext)>>;

pub struct RenderCallback(Callback);

impl RenderCallback {
    pub fn new(callback: impl FnMut(&mut RenderContext) + 'static) -> Self {
        Self(Rc::new(RefCell::new(callback)))
    }

    pub fn call(&self, data: &mut RenderContext) {
        (self.0.borrow_mut())(data)
    }
}

impl Clone for RenderCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for RenderCallback {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<H: FnMut(&mut RenderContext) + 'static> From<H> for RenderCallback {
    fn from(value: H) -> Self {
        RenderCallback::new(value)
    }
}

#[derive(PartialEq, Clone)]
pub struct PlotElement {
    pub layout: LayoutData,
    pub event_handlers: FxHashMap<EventName, EventHandlerType>,
    pub effect: Option<EffectData>,
    pub on_render: RenderCallback,
}

impl PlotElement {}

impl ElementExt for PlotElement {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(rect) = (other.as_ref() as &dyn Any).downcast_ref::<Self>() else {
            return false;
        };

        self != rect
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(rect) = (other.as_ref() as &dyn Any).downcast_ref::<Self>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.effect != rect.effect {
            diff.insert(DiffModifies::EFFECT);
        }

        if !self.layout.layout.self_layout_eq(&rect.layout.layout) {
            diff.insert(DiffModifies::STYLE);
            diff.insert(DiffModifies::LAYOUT);
        }

        if !self.layout.layout.inner_layout_eq(&rect.layout.layout) {
            diff.insert(DiffModifies::STYLE);
            diff.insert(DiffModifies::INNER_LAYOUT);
        }

        if self.event_handlers != rect.event_handlers {
            diff.insert(DiffModifies::EVENT_HANDLERS);
        }

        if self.on_render != rect.on_render {
            diff.insert(DiffModifies::STYLE);
        }

        diff
    }

    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout)
    }

    fn effect(&'_ self) -> Option<Cow<'_, EffectData>> {
        self.effect.as_ref().map(Cow::Borrowed)
    }

    fn style(&'_ self) -> Cow<'_, StyleState> {
        Cow::Owned(StyleState::default())
    }

    fn text_style(&'_ self) -> Cow<'_, TextStyleData> {
        Cow::Owned(TextStyleData::default())
    }

    fn accessibility(&'_ self) -> Cow<'_, AccessibilityData> {
        Cow::Owned(AccessibilityData::default())
    }

    fn events_handlers(&'_ self) -> Option<Cow<'_, FxHashMap<EventName, EventHandlerType>>> {
        Some(Cow::Borrowed(&self.event_handlers))
    }

    fn clip(&self, context: ClipContext) {
        let area = context.visible_area;

        context.canvas.clip_rect(
            SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            ClipOp::Intersect,
            true,
        );
    }

    fn render(&self, mut context: RenderContext) {
        let style = self.style();
        let area = context.layout_node.area;

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        style.background.apply_to_paint(&mut paint, area);

        context.canvas.draw_rect(
            SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            &paint,
        );

        context
            .canvas
            .scale((context.scale_factor as f32, context.scale_factor as f32));
        context.canvas.translate((area.min_x(), area.min_y()));
        self.on_render.call(&mut context);
        context.canvas.restore();
    }
}

pub struct Plot {
    element: PlotElement,
    elements: Vec<Element>,
    key: DiffKey,
}

impl ChildrenExt for Plot {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for Plot {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl EventHandlersExt for Plot {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType> {
        &mut self.element.event_handlers
    }
}

impl MaybeExt for Plot {}

impl From<Plot> for Element {
    fn from(value: Plot) -> Self {
        Element::Element {
            key: value.key,
            element: Rc::new(value.element),
            elements: value.elements,
        }
    }
}

/// Create a new `Plot` element.
///
/// See the available methods in [Plot].
pub fn plot(on_render: RenderCallback) -> Plot {
    Plot::new(on_render)
}

impl Plot {
    pub fn new(on_render: RenderCallback) -> Self {
        Self {
            element: PlotElement {
                on_render,
                layout: LayoutData::default(),
                event_handlers: HashMap::default(),
                effect: None,
            },
            elements: Vec::default(),
            key: DiffKey::None,
        }
    }

    pub fn try_downcast(element: &dyn ElementExt) -> Option<PlotElement> {
        (element as &dyn Any).downcast_ref::<PlotElement>().cloned()
    }
}

impl LayoutExt for Plot {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl ContainerExt for Plot {}

impl ContainerWithContentExt for Plot {}
