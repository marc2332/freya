use std::{
    cell::RefCell,
    ops::{
        Deref,
        Div,
    },
    rc::Rc,
};

use torin::prelude::{
    Area,
    CursorPoint,
    Size2D,
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MouseEventData {
    pub global_location: CursorPoint,
    pub element_location: CursorPoint,
    pub button: Option<MouseButton>,
}

/// Data of a Keyboard event.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardEventData {
    pub key: keyboard_types::Key,
    pub code: keyboard_types::Code,
    pub modifiers: keyboard_types::Modifiers,
}

impl KeyboardEventData {
    pub fn new(
        key: keyboard_types::Key,
        code: keyboard_types::Code,
        modifiers: keyboard_types::Modifiers,
    ) -> Self {
        Self {
            key,
            code,
            modifiers,
        }
    }
}

impl KeyboardEventData {
    /// Try to get the text of the key
    pub fn try_as_str(&self) -> Option<&str> {
        if let keyboard_types::Key::Character(c) = &self.key {
            Some(c)
        } else {
            None
        }
    }
}

pub struct Event<D> {
    pub(crate) data: D,
    pub(crate) propagate: Rc<RefCell<bool>>,
    pub(crate) default: Rc<RefCell<bool>>,
}

impl<D> Deref for Event<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<D> Event<D> {
    pub fn map<ND>(self, data: impl FnOnce(D) -> ND) -> Event<ND> {
        Event {
            data: data(self.data),
            propagate: self.propagate,
            default: self.default,
        }
    }

    pub fn try_map<ND>(self, data: impl FnOnce(D) -> Option<ND>) -> Option<Event<ND>> {
        Some(Event {
            data: data(self.data)?,
            propagate: self.propagate,
            default: self.default,
        })
    }

    pub fn data(&self) -> &D {
        &self.data
    }

    pub fn stop_propagation(&self) {
        *self.propagate.borrow_mut() = false;
    }

    pub fn prevent_default(&self) {
        *self.default.borrow_mut() = false;
    }
}

/// Data of a Sized event.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SizedEventData {
    pub area: Area,
    pub visible_area: Area,
    pub inner_sizes: Size2D,
}

impl SizedEventData {
    pub fn div(&mut self, rhs: f32) {
        self.area = self.area.div(rhs);
        self.visible_area = self.visible_area.div(rhs);
        self.inner_sizes = self.inner_sizes.div(rhs);
    }
}

impl SizedEventData {
    pub fn new(area: Area, visible_area: Area, inner_sizes: Size2D) -> Self {
        Self {
            area,
            visible_area,
            inner_sizes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum WheelSource {
    Device,
    Custom,
}

/// Data of a Wheel event.
#[derive(Debug, Clone, PartialEq)]
pub struct WheelEventData {
    pub source: WheelSource,
    pub delta_x: f64,
    pub delta_y: f64,
}

impl WheelEventData {
    pub fn new(delta_x: f64, delta_y: f64, source: WheelSource) -> Self {
        Self {
            delta_x,
            delta_y,
            source,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Force {
    Calibrated {
        force: f64,
        max_possible_force: f64,
        altitude_angle: Option<f64>,
    },
    Normalized(f64),
}

/// Data of a Touch event.
#[derive(Debug, Clone, PartialEq)]
pub struct TouchEventData {
    pub global_location: CursorPoint,
    pub element_location: CursorPoint,
    pub finger_id: u64,
    pub phase: TouchPhase,
    pub force: Option<Force>,
}

impl TouchEventData {
    pub fn new(
        global_location: CursorPoint,
        element_location: CursorPoint,
        finger_id: u64,
        phase: TouchPhase,
        force: Option<Force>,
    ) -> Self {
        Self {
            global_location,
            element_location,
            finger_id,
            phase,
            force,
        }
    }
}

/// Data of a pointer event.
#[derive(Debug, Clone, PartialEq)]
pub enum PointerEventData {
    Mouse(MouseEventData),
    Touch(TouchEventData),
}

impl PointerEventData {
    pub fn global_location(&self) -> CursorPoint {
        match self {
            Self::Mouse(m) => m.global_location,
            Self::Touch(t) => t.global_location,
        }
    }

    pub fn element_location(&self) -> CursorPoint {
        match self {
            Self::Mouse(m) => m.element_location,
            Self::Touch(t) => t.element_location,
        }
    }

    pub fn button(&self) -> Option<MouseButton> {
        match self {
            Self::Mouse(m) => m.button,
            Self::Touch(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Mouse(MouseEventData),
    Keyboard(KeyboardEventData),
    Sized(SizedEventData),
    Wheel(WheelEventData),
    Touch(TouchEventData),
    Pointer(PointerEventData),
}
