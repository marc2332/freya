use std::sync::Arc;

use tokio::sync::watch;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NavigationMode {
    Keyboard,
    NotKeyboard,
}

/// Manages the navigation mode.
#[derive(Clone)]
pub struct NavigatorState {
    _setter: Arc<watch::Sender<NavigationMode>>,
    _getter: watch::Receiver<NavigationMode>,
}

impl NavigatorState {
    pub fn new(mode: NavigationMode) -> Self {
        let (set, get) = watch::channel(mode);
        Self {
            _setter: Arc::new(set),
            _getter: get,
        }
    }

    pub fn getter(&self) -> watch::Receiver<NavigationMode> {
        self._getter.clone()
    }

    pub fn get(&self) -> NavigationMode {
        *self._getter.borrow()
    }

    pub fn set(&self, mode: NavigationMode) {
        if self.get() != mode {
            self._setter.send(mode).unwrap();
        }
    }
}
