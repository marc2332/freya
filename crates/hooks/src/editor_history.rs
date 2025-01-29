use ropey::Rope;

#[derive(Clone)]
pub enum HistoryChange {
    InsertChar {
        idx: usize,
        len: usize,
        ch: char,
    },
    InsertText {
        idx: usize,
        len: usize,
        text: String,
    },
    Remove {
        idx: usize,
        len: usize,
        text: String,
    },
}

#[derive(Default, Clone)]
pub struct EditorHistory {
    pub changes: Vec<HistoryChange>,
    pub current_change: usize,
    // Incremental counter for every change.
    pub version: usize,
}

impl EditorHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_change(&mut self, change: HistoryChange) {
        if self.can_redo() {
            self.changes.drain(self.current_change..);
        }

        self.changes.push(change);
        self.current_change = self.changes.len();

        self.version += 1;
    }

    pub fn current_change(&self) -> usize {
        self.current_change
    }

    pub fn any_pending_changes(&self) -> usize {
        self.changes.len() - self.current_change
    }

    pub fn can_undo(&self) -> bool {
        self.current_change > 0
    }

    pub fn can_redo(&self) -> bool {
        self.current_change < self.changes.len()
    }

    pub fn undo(&mut self, rope: &mut Rope) -> Option<usize> {
        if !self.can_undo() {
            return None;
        }

        let last_change = self.changes.get(self.current_change - 1);
        if let Some(last_change) = last_change {
            let idx_end = match last_change {
                HistoryChange::Remove { idx, text, len } => {
                    let start = rope.utf16_cu_to_char(*idx);
                    rope.insert(start, text);
                    *idx + len
                }
                HistoryChange::InsertChar { idx, len, .. } => {
                    let start = rope.utf16_cu_to_char(*idx);
                    let end = rope.utf16_cu_to_char(*idx + len);
                    rope.remove(start..end);
                    *idx
                }
                HistoryChange::InsertText { idx, len, .. } => {
                    let start = rope.utf16_cu_to_char(*idx);
                    let end = rope.utf16_cu_to_char(*idx + len);
                    rope.remove(start..end);
                    *idx
                }
            };
            self.current_change -= 1;
            self.version += 1;
            Some(idx_end)
        } else {
            None
        }
    }

    pub fn redo(&mut self, rope: &mut Rope) -> Option<usize> {
        if !self.can_redo() {
            return None;
        }

        let next_change = self.changes.get(self.current_change);
        if let Some(next_change) = next_change {
            let idx_end = match next_change {
                HistoryChange::Remove { idx, len, .. } => {
                    let start = rope.utf16_cu_to_char(*idx);
                    let end = rope.utf16_cu_to_char(*idx + len);
                    rope.remove(start..end);
                    *idx
                }
                HistoryChange::InsertChar { idx, ch, len } => {
                    let start = rope.utf16_cu_to_char(*idx);
                    rope.insert_char(start, *ch);
                    *idx + len
                }
                HistoryChange::InsertText { idx, text, len } => {
                    let start = rope.utf16_cu_to_char(*idx);
                    rope.insert(start, text);
                    *idx + len
                }
            };
            self.current_change += 1;
            self.version += 1;
            Some(idx_end)
        } else {
            None
        }
    }

    pub fn clear_redos(&mut self) {
        if self.can_redo() {
            self.changes.drain(self.current_change..);
        }
    }

    pub fn clear(&mut self) {
        self.changes.clear();
        self.current_change = 0;
        self.version = 0;
    }
}

#[cfg(test)]
mod test {
    use ropey::Rope;

    use super::{
        EditorHistory,
        HistoryChange,
    };

    #[test]
    fn test() {
        let mut rope = Rope::new();
        let mut history = EditorHistory::new();

        // Initial text
        rope.insert(0, "Hello World");

        assert!(!history.can_undo());
        assert!(!history.can_redo());

        // New change
        rope.insert(11, "\n!!!!");
        history.push_change(HistoryChange::InsertText {
            idx: 11,
            text: "\n!!!!".to_owned(),
            len: "\n!!!!".len(),
        });

        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(rope.to_string(), "Hello World\n!!!!");

        // Undo last change
        history.undo(&mut rope);

        assert!(!history.can_undo());
        assert!(history.can_redo());
        assert_eq!(rope.to_string(), "Hello World");

        // More changes
        rope.insert(11, "\n!!!!");
        history.push_change(HistoryChange::InsertText {
            idx: 11,
            text: "\n!!!!".to_owned(),
            len: "\n!!!!".len(),
        });
        rope.insert(16, "\n!!!!");
        history.push_change(HistoryChange::InsertText {
            idx: 16,
            text: "\n!!!!".to_owned(),
            len: "\n!!!!".len(),
        });
        rope.insert(21, "\n!!!!");
        history.push_change(HistoryChange::InsertText {
            idx: 21,
            text: "\n!!!!".to_owned(),
            len: "\n!!!!".len(),
        });

        assert_eq!(history.any_pending_changes(), 0);
        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(rope.to_string(), "Hello World\n!!!!\n!!!!\n!!!!");

        // Undo last changes
        history.undo(&mut rope);
        assert_eq!(history.any_pending_changes(), 1);
        assert_eq!(rope.to_string(), "Hello World\n!!!!\n!!!!");
        history.undo(&mut rope);
        assert_eq!(history.any_pending_changes(), 2);
        assert_eq!(rope.to_string(), "Hello World\n!!!!");
        history.undo(&mut rope);
        assert_eq!(history.any_pending_changes(), 3);
        assert_eq!(rope.to_string(), "Hello World");

        assert!(!history.can_undo());
        assert!(history.can_redo());

        // Redo last changes
        history.redo(&mut rope);
        assert_eq!(rope.to_string(), "Hello World\n!!!!");
        history.redo(&mut rope);
        assert_eq!(rope.to_string(), "Hello World\n!!!!\n!!!!");
        history.redo(&mut rope);
        assert_eq!(rope.to_string(), "Hello World\n!!!!\n!!!!\n!!!!");

        assert_eq!(history.any_pending_changes(), 0);
        assert!(history.can_undo());
        assert!(!history.can_redo());

        // Undo last change
        history.undo(&mut rope);
        assert_eq!(rope.to_string(), "Hello World\n!!!!\n!!!!");
        assert_eq!(history.any_pending_changes(), 1);
        history.undo(&mut rope);
        assert_eq!(rope.to_string(), "Hello World\n!!!!");
        assert_eq!(history.any_pending_changes(), 2);

        // Dischard any changes that could have been redone
        rope.insert_char(0, '.');
        history.push_change(HistoryChange::InsertChar {
            idx: 0,
            ch: '.',
            len: 1,
        });
        assert_eq!(history.any_pending_changes(), 0);
    }
}
