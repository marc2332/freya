use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    rc::Rc,
};

use freya_core::{
    data::{
        AccessibilityData,
        LayoutData,
    },
    diff_key::DiffKey,
    element::{
        Element,
        ElementExt,
        EventHandlerType,
        RenderContext,
    },
    events::name::EventName,
    prelude::{
        AccessibilityExt,
        EventHandlersExt,
        LayoutExt,
    },
    tree::DiffModifies,
};
use freya_engine::prelude::ClipOp;
use rustc_hash::FxHashMap;

use crate::state::BlitzState;

/// Paints a Blitz document into the scene and forwards input events into it.
pub(crate) struct HtmlElement {
    pub layout: LayoutData,
    pub accessibility: AccessibilityData,
    pub event_handlers: FxHashMap<EventName, EventHandlerType>,
    pub state: Rc<RefCell<BlitzState>>,
}

impl PartialEq for HtmlElement {
    fn eq(&self, other: &Self) -> bool {
        self.layout == other.layout
            && self.accessibility == other.accessibility
            && Rc::ptr_eq(&self.state, &other.state)
    }
}

impl ElementExt for HtmlElement {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(other) = (other.as_ref() as &dyn Any).downcast_ref::<HtmlElement>() else {
            return true;
        };
        self != other
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(other) = (other.as_ref() as &dyn Any).downcast_ref::<HtmlElement>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();
        if self.layout != other.layout {
            diff.insert(DiffModifies::LAYOUT);
        }
        if self.accessibility != other.accessibility {
            diff.insert(DiffModifies::ACCESSIBILITY);
        }
        diff
    }

    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout)
    }

    fn accessibility(&'_ self) -> Cow<'_, AccessibilityData> {
        Cow::Borrowed(&self.accessibility)
    }

    fn events_handlers(&'_ self) -> Option<Cow<'_, FxHashMap<EventName, EventHandlerType>>> {
        Some(Cow::Borrowed(&self.event_handlers))
    }

    fn should_measure_inner_children(&self) -> bool {
        false
    }

    fn render(&self, context: RenderContext) {
        let area = context.layout_node.visible_area();
        let scale = context.scale_factor as f32;

        context.canvas.save();
        context
            .canvas
            .clip_rrect(self.render_rect(&area, scale), ClipOp::Intersect, true);

        self.state.borrow_mut().paint(
            context.canvas,
            area.min_x(),
            area.min_y(),
            area.width().round() as u32,
            area.height().round() as u32,
            scale,
        );

        context.canvas.restore();
    }
}

pub(crate) struct Html {
    pub(crate) element: HtmlElement,
}

impl Html {
    pub fn new(state: Rc<RefCell<BlitzState>>) -> Self {
        Self {
            element: HtmlElement {
                layout: LayoutData::default(),
                accessibility: AccessibilityData::default(),
                event_handlers: FxHashMap::default(),
                state,
            },
        }
    }
}

impl From<Html> for Element {
    fn from(value: Html) -> Self {
        Element::Element {
            key: DiffKey::None,
            element: Rc::new(value.element),
            elements: vec![],
        }
    }
}

impl LayoutExt for Html {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl AccessibilityExt for Html {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.element.accessibility
    }
}

impl EventHandlersExt for Html {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType> {
        &mut self.element.event_handlers
    }
}
