use std::cell::RefCell;

struct MemoryHistoryState {
    current: String,
    history: Vec<String>,
    future: Vec<String>,
}

/// An in-memory navigation history.
///
/// `RouterContext` uses this internally. It is public for applications that need
/// to manage a compatible history stack themselves.
pub struct MemoryHistory {
    state: RefCell<MemoryHistoryState>,
}

impl Default for MemoryHistory {
    fn default() -> Self {
        Self::with_initial_path("/")
    }
}

impl MemoryHistory {
    /// Creates a history whose current route is `path`.
    pub fn with_initial_path(path: impl ToString) -> Self {
        Self {
            state: MemoryHistoryState{
                current: path.to_string().parse().unwrap_or_else(|err| {
                    panic!("index route does not exist:\n{err}\n use MemoryHistory::with_initial_path to set a custom path")
                }),
                history: Vec::new(),
                future: Vec::new(),
            }.into(),
        }
    }
}

impl MemoryHistory {
    /// Returns the current route string.
    pub fn current_route(&self) -> String {
        self.state.borrow().current.clone()
    }

    /// Returns whether a previous route is available.
    pub fn can_go_back(&self) -> bool {
        !self.state.borrow().history.is_empty()
    }

    /// Moves to the previous route, if one is available.
    pub fn go_back(&self) {
        let mut write = self.state.borrow_mut();
        if let Some(last) = write.history.pop() {
            let old = std::mem::replace(&mut write.current, last);
            write.future.push(old);
        }
    }

    /// Returns whether a forward route is available.
    pub fn can_go_forward(&self) -> bool {
        !self.state.borrow().future.is_empty()
    }

    /// Moves to the next route, if one is available.
    pub fn go_forward(&self) {
        let mut write = self.state.borrow_mut();
        if let Some(next) = write.future.pop() {
            let old = std::mem::replace(&mut write.current, next);
            write.history.push(old);
        }
    }

    /// Makes `new` the current route and saves the previous route in history.
    ///
    /// This clears the forward history. Pushing the current route has no effect.
    pub fn push(&self, new: String) {
        let mut write = self.state.borrow_mut();
        // don't push the same route twice
        if write.current == new {
            return;
        }
        let old = std::mem::replace(&mut write.current, new);
        write.history.push(old);
        write.future.clear();
    }

    /// Changes the current route without changing back or forward history.
    pub fn replace(&self, path: String) {
        let mut write = self.state.borrow_mut();
        write.current = path;
    }
}
