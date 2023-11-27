use dioxus_core::ScopeState;
use dioxus_hooks::UseRef;
use fermi::AtomState;

#[derive(Clone)]
pub enum StateMode {
    Ref(UseRef<i32>),
    Atom(AtomState<i32>)
}

impl StateMode {
    pub fn scroll(&self, pos: i32) {
        match self {
            Self::Ref(state_ref) => state_ref.set(pos),
            Self::Atom(state_atom) => state_atom.set(pos)
        }
    }

    pub fn read(&self) -> i32 {
        match self {
            Self::Ref(state_ref) => *state_ref.read(),
            Self::Atom(state_atom) => *state_atom.current()
        }
    }
}

impl From<&UseRef<i32>> for StateMode {
    fn from(value: &UseRef<i32>) -> Self {
        StateMode::Ref(value.clone())
    }
}

impl From<&AtomState<i32>> for StateMode {
    fn from(value: &AtomState<i32>) -> Self {
        StateMode::Atom(value.clone())
    }
}

#[derive(Clone)]
pub enum ScrollController {
    Vertical(StateMode),
    Horizontal(StateMode),
    Both {
        vertical: StateMode,
        horizontal: StateMode
    }
}

impl ScrollController {
    pub fn new_vertical(mode: StateMode) -> Self {
        Self::Vertical(mode)
    }

    pub fn new_horizontal(mode: StateMode) -> Self {
        Self::Horizontal(mode)
    }

    pub fn new_both(vertical: StateMode, horizontal: StateMode) -> Self {
        Self::Both {
             vertical,
             horizontal
        }
    }

    pub fn scroll_y(&self, pos: i32) {
        match &self {
            ScrollController::Vertical(state) => state.scroll(pos),
            ScrollController::Both { vertical, .. } => vertical.scroll(pos),
            _ => {}
        }
    }

    pub fn scroll_x(&self, pos: i32) {
        match &self {
            ScrollController::Horizontal(state) => state.scroll(pos),
            ScrollController::Both { horizontal, .. } => horizontal.scroll(pos),
            _ => {}
        }
    }

    pub fn read_y(&self) -> i32 {
        match &self {
            ScrollController::Vertical(state) => state.read(),
            ScrollController::Both { vertical, .. } => vertical.read(),
            _ => 0
        }
    }
    
    pub fn read_x(&self) -> i32 {
        match &self {
            ScrollController::Horizontal(state) => state.read(),
            ScrollController::Both { horizontal, .. } => horizontal.read(),
            _ => 0
        }
    }
}

pub fn use_scroll_controller(cx: &ScopeState, cb: impl FnOnce() -> ScrollController) -> &ScrollController {
    cx.use_hook(cb)
}