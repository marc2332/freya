use ropey::Rope;

#[derive(Clone)]
pub enum HistoryChange {
    InsertChar { idx: usize, char: char },
    InsertText { idx: usize, text: String },
    Remove { idx: usize, text: String },
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
                HistoryChange::Remove { idx, text } => {
                    rope.insert(*idx, text);
                    idx + text.len()
                }
                HistoryChange::InsertChar { idx, .. } => {
                    rope.remove(*idx..*idx + 1);
                    *idx
                }
                HistoryChange::InsertText { idx, text } => {
                    rope.remove(*idx..idx + text.len());
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
                HistoryChange::Remove { idx, text } => {
                    rope.remove(*idx..idx + text.len());
                    *idx
                }
                HistoryChange::InsertChar { idx, char: ch } => {
                    rope.insert_char(*idx, *ch);
                    idx + 1
                }
                HistoryChange::InsertText { idx, text, .. } => {
                    rope.insert(*idx, text);
                    idx + text.len()
                }
            };
            self.current_change += 1;
            self.version += 1;
            Some(idx_end)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use ropey::Rope;

    use super::{EditorHistory, HistoryChange};

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
        });
        rope.insert(16, "\n!!!!");
        history.push_change(HistoryChange::InsertText {
            idx: 16,
            text: "\n!!!!".to_owned(),
        });
        rope.insert(21, "\n!!!!");
        history.push_change(HistoryChange::InsertText {
            idx: 21,
            text: "\n!!!!".to_owned(),
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
        history.push_change(HistoryChange::InsertChar { idx: 0, char: '.' });
        assert_eq!(history.any_pending_changes(), 0);
    }
}
