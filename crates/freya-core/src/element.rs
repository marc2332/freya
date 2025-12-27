use std::{
    any::Any,
    borrow::Cow,
    fmt::Debug,
    rc::Rc,
};

use freya_engine::prelude::{
    Canvas,
    FontCollection,
    FontMgr,
};
use rustc_hash::FxHashMap;
use torin::prelude::{
    Area,
    LayoutNode,
    Size2D,
};

use crate::{
    data::{
        AccessibilityData,
        EffectData,
        LayoutData,
        StyleState,
        TextStyleData,
        TextStyleState,
    },
    diff_key::DiffKey,
    event_handler::EventHandler,
    events::{
        data::{
            Event,
            KeyboardEventData,
            MouseEventData,
            PointerEventData,
            SizedEventData,
            TouchEventData,
            WheelEventData,
        },
        name::EventName,
    },
    helpers::from_fn_standalone_borrowed_keyed,
    layers::Layer,
    node_id::NodeId,
    prelude::{
        FileEventData,
        ImePreeditEventData,
        MaybeExt,
    },
    text_cache::TextCache,
    tree::{
        DiffModifies,
        Tree,
    },
};

pub trait ElementExt: Any {
    fn into_element(self) -> Element
    where
        Self: Sized + Into<Element>,
    {
        self.into()
    }

    fn changed(&self, _other: &Rc<dyn ElementExt>) -> bool {
        false
    }

    fn diff(&self, _other: &Rc<dyn ElementExt>) -> DiffModifies {
        DiffModifies::empty()
    }

    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Owned(Default::default())
    }

    fn accessibility(&'_ self) -> Cow<'_, AccessibilityData> {
        Cow::Owned(Default::default())
    }

    fn effect(&'_ self) -> Option<Cow<'_, EffectData>> {
        None
    }

    fn style(&'_ self) -> Cow<'_, StyleState> {
        Cow::Owned(Default::default())
    }

    fn text_style(&'_ self) -> Cow<'_, TextStyleData> {
        Cow::Owned(Default::default())
    }

    fn layer(&self) -> Layer {
        Layer::default()
    }

    fn events_handlers(&'_ self) -> Option<Cow<'_, FxHashMap<EventName, EventHandlerType>>> {
        None
    }

    fn measure(&self, _context: LayoutContext) -> Option<(Size2D, Rc<dyn Any>)> {
        None
    }

    fn should_hook_measurement(&self) -> bool {
        false
    }

    fn should_measure_inner_children(&self) -> bool {
        true
    }

    fn is_point_inside(&self, context: EventMeasurementContext) -> bool {
        context
            .layout_node
            .visible_area()
            .contains(context.cursor.to_f32())
    }

    fn clip(&self, _context: ClipContext) {}

    fn render(&self, _context: RenderContext) {}
}

#[allow(dead_code)]
pub struct LayoutContext<'a> {
    pub node_id: NodeId,
    pub torin_node: &'a torin::node::Node,
    pub area_size: &'a Size2D,
    pub font_collection: &'a FontCollection,
    pub font_manager: &'a FontMgr,
    pub text_style_state: &'a TextStyleState,
    pub fallback_fonts: &'a [Cow<'static, str>],
    pub scale_factor: f64,
    pub text_cache: &'a mut TextCache,
}

#[allow(dead_code)]
pub struct RenderContext<'a> {
    pub font_collection: &'a mut FontCollection,
    pub canvas: &'a Canvas,
    pub layout_node: &'a LayoutNode,
    pub text_style_state: &'a TextStyleState,
    pub tree: &'a Tree,
    pub scale_factor: f64,
}

pub struct EventMeasurementContext<'a> {
    pub cursor: ragnarok::CursorPoint,
    pub layout_node: &'a LayoutNode,
    pub scale_factor: f64,
}

pub struct ClipContext<'a> {
    pub canvas: &'a Canvas,
    pub visible_area: &'a Area,
    pub scale_factor: f64,
}

impl<T: Any + PartialEq> ComponentProps for T {
    fn changed(&self, other: &dyn ComponentProps) -> bool {
        let other = (other as &dyn Any).downcast_ref::<T>().unwrap();
        self != other
    }
}

pub trait ComponentProps: Any {
    fn changed(&self, other: &dyn ComponentProps) -> bool;
}

#[derive(Clone)]
pub enum Element {
    Component {
        key: DiffKey,

        comp: Rc<dyn Fn(Rc<dyn ComponentProps>) -> Element>,

        props: Rc<dyn ComponentProps>,
    },
    Element {
        key: DiffKey,
        element: Rc<dyn ElementExt>,
        elements: Vec<Element>,
    },
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element { key, elements, .. } => {
                f.write_str(&format!("Element {{ key: {:?} }}", key))?;
                elements.fmt(f)
            }
            Self::Component { key, .. } => f.write_str(&format!("Component {{ key: {:?} }}", key)),
        }
    }
}

pub trait IntoElement {
    fn into_element(self) -> Element;
}

impl<T: Into<Element>> IntoElement for T {
    fn into_element(self) -> Element {
        self.into()
    }
}

#[derive(Clone)]
pub struct FpRender {
    render: Rc<dyn Fn() -> Element + 'static>,
}

impl FpRender {
    pub fn from_render(render: impl Render + 'static) -> Self {
        Self {
            render: Rc::new(move || render.render().into_element()),
        }
    }
}

impl PartialEq for FpRender {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<F, E> From<F> for FpRender
where
    F: Fn() -> E + 'static,
    E: IntoElement,
{
    fn from(render: F) -> Self {
        FpRender {
            render: Rc::new(move || render().into_element()),
        }
    }
}

impl Render for FpRender {
    fn render(&self) -> impl IntoElement {
        (self.render)()
    }
}

pub trait Render: RenderKey + 'static {
    fn render(&self) -> impl IntoElement;

    fn render_key(&self) -> DiffKey {
        self.default_key()
    }
}

pub trait RenderOwned: RenderKey + 'static {
    fn render(self) -> impl IntoElement;

    fn render_key(&self) -> DiffKey {
        self.default_key()
    }
}

pub trait RenderKey {
    fn default_key(&self) -> DiffKey;
}

impl<T> Render for T
where
    T: RenderOwned + Clone,
{
    fn render(&self) -> impl IntoElement {
        <Self as RenderOwned>::render(self.clone())
    }
    fn render_key(&self) -> DiffKey {
        <Self as RenderOwned>::render_key(self)
    }
}

impl<T> RenderKey for T
where
    T: Render,
{
    fn default_key(&self) -> DiffKey {
        DiffKey::U64(Self::render as *const () as u64)
    }
}

impl<T> MaybeExt for T where T: Render {}

impl<T: Render + PartialEq> From<T> for Element {
    fn from(value: T) -> Self {
        from_fn_standalone_borrowed_keyed(value.render_key(), value, |v| v.render().into_element())
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Component {
                    key: key1,
                    props: props1,
                    ..
                },
                Self::Component {
                    key: key2,
                    props: props2,
                    ..
                },
            ) => key1 == key2 && !props1.changed(props2.as_ref()),
            (
                Self::Element {
                    key: key1,
                    element: element1,
                    elements: elements1,
                },
                Self::Element {
                    key: key2,
                    element: element2,
                    elements: elements2,
                },
            ) => key1 == key2 && !element1.changed(element2) && elements1 == elements2,
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum EventHandlerType {
    Mouse(EventHandler<Event<MouseEventData>>),
    Keyboard(EventHandler<Event<KeyboardEventData>>),
    Sized(EventHandler<Event<SizedEventData>>),
    Wheel(EventHandler<Event<WheelEventData>>),
    Touch(EventHandler<Event<TouchEventData>>),
    Pointer(EventHandler<Event<PointerEventData>>),
    ImePreedit(EventHandler<Event<ImePreeditEventData>>),
    File(EventHandler<Event<FileEventData>>),
}
