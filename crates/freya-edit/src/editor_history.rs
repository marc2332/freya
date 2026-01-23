use std::time::{
    Duration,
    Instant,
};

use ropey::Rope;

use crate::text_editor::TextSelection;

#[derive(Clone, Debug, PartialEq)]
pub enum HistoryChange {
    InsertChar {
        idx: usize,
        len: usize,
        ch: char,
        selection: TextSelection,
    },
    InsertText {
        idx: usize,
        len: usize,
        text: String,
        selection: TextSelection,
    },
    Remove {
        idx: usize,
        len: usize,
        text: String,
        selection: TextSelection,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryTransaction {
    pub timestamp: Instant,
    pub changes: Vec<HistoryChange>,
}

#[derive(Clone, Debug)]
pub struct EditorHistory {
    pub transactions: Vec<HistoryTransaction>,
    pub current_transaction: usize,
    // Incremental counter for every transaction.
    pub version: usize,
    /// After how many seconds since the last transaction a change should be grouped with the last transaction.
    transaction_threshold_groping: Duration,
}

impl EditorHistory {
    pub fn new(transaction_threshold_groping: Duration) -> Self {
        Self {
            transactions: Vec::default(),
            current_transaction: 0,
            version: 0,
            transaction_threshold_groping,
        }
    }

    pub fn push_change(&mut self, change: HistoryChange) {
        if self.can_redo() {
            self.transactions.drain(self.current_transaction..);
        }

        let last_transaction = self
            .transactions
            .get_mut(self.current_transaction.saturating_sub(1));
        if let Some(last_transaction) = last_transaction
            && last_transaction.timestamp.elapsed() <= self.transaction_threshold_groping
        {
            last_transaction.changes.push(change);
            last_transaction.timestamp = Instant::now();
            return;
        }

        self.transactions.push(HistoryTransaction {
            timestamp: Instant::now(),
            changes: vec![change],
        });

        self.current_transaction = self.transactions.len();
        self.version += 1;
    }

    pub fn current_change(&self) -> usize {
        self.current_transaction
    }

    pub fn any_pending_changes(&self) -> usize {
        self.transactions.len() - self.current_transaction
    }

    pub fn can_undo(&self) -> bool {
        self.current_transaction > 0
    }

    pub fn can_redo(&self) -> bool {
        self.current_transaction < self.transactions.len()
    }

    pub fn undo(&mut self, rope: &mut Rope) -> Option<TextSelection> {
        if !self.can_undo() {
            return None;
        }

        let last_transaction = self.transactions.get(self.current_transaction - 1);
        if let Some(last_transaction) = last_transaction {
            let mut selection = None;
            for change in last_transaction.changes.iter().rev() {
                match change {
                    HistoryChange::Remove {
                        idx,
                        text,
                        selection: previous_selection,
                        ..
                    } => {
                        let start = rope.utf16_cu_to_char(*idx);
                        rope.insert(start, text);
                        selection = Some(previous_selection.clone());
                    }
                    HistoryChange::InsertChar {
                        idx,
                        len,
                        selection: previous_selection,
                        ..
                    } => {
                        let start = rope.utf16_cu_to_char(*idx);
                        let end = rope.utf16_cu_to_char(*idx + len);
                        rope.remove(start..end);
                        selection = Some(previous_selection.clone());
                    }
                    HistoryChange::InsertText {
                        idx,
                        len,
                        selection: previous_selection,
                        ..
                    } => {
                        let start = rope.utf16_cu_to_char(*idx);
                        let end = rope.utf16_cu_to_char(*idx + len);
                        rope.remove(start..end);
                        selection = Some(previous_selection.clone());
                    }
                }
            }

            self.current_transaction -= 1;
            self.version += 1;
            selection
        } else {
            None
        }
    }

    pub fn redo(&mut self, rope: &mut Rope) -> Option<TextSelection> {
        if !self.can_redo() {
            return None;
        }

        let last_transaction = self.transactions.get(self.current_transaction);
        if let Some(last_transaction) = last_transaction {
            let mut cursor_pos = None;
            for change in &last_transaction.changes {
                cursor_pos = Some(match change {
                    HistoryChange::Remove { idx, len, .. } => {
                        let start = rope.utf16_cu_to_char(*idx);
                        let end = rope.utf16_cu_to_char(*idx + len);
                        rope.remove(start..end);
                        *idx
                    }
                    HistoryChange::InsertChar { idx, len, ch, .. } => {
                        let start = rope.utf16_cu_to_char(*idx);
                        rope.insert_char(start, *ch);
                        *idx + len
                    }
                    HistoryChange::InsertText { idx, text, len, .. } => {
                        let start = rope.utf16_cu_to_char(*idx);
                        rope.insert(start, text);
                        *idx + len
                    }
                });
            }
            self.current_transaction += 1;
            self.version += 1;
            cursor_pos.map(TextSelection::new_cursor)
        } else {
            None
        }
    }

    pub fn clear_redos(&mut self) {
        if self.can_redo() {
            self.transactions.drain(self.current_transaction..);
        }
    }

    pub fn clear(&mut self) {
        self.transactions.clear();
        self.current_transaction = 0;
        self.version = 0;
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use ropey::Rope;

    use super::{
        EditorHistory,
        HistoryChange,
    };
    use crate::text_editor::TextSelection;

    #[test]
    fn test_undo_redo() {
        let mut rope = Rope::new();
        let mut history = EditorHistory::new(Duration::ZERO);

        rope.insert(0, "Hello World");

        assert!(!history.can_undo());
        assert!(!history.can_redo());

        rope.insert(11, "\n!!!!");
        history.push_change(HistoryChange::InsertText {
            idx: 11,
            text: "\n!!!!".to_owned(),
            len: "\n!!!!".len(),
            selection: TextSelection::new_cursor(11),
        });

        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(rope.to_string(), "Hello World\n!!!!");

        history.undo(&mut rope);

        assert!(!history.can_undo());
        assert!(history.can_redo());
        assert_eq!(rope.to_string(), "Hello World");

        rope.insert(11, "\n!!!!");
        history.push_change(HistoryChange::InsertText {
            idx: 11,
            text: "\n!!!!".to_owned(),
            len: "\n!!!!".len(),
            selection: TextSelection::new_cursor(11),
        });
        rope.insert(16, "\n!!!!");
        history.push_change(HistoryChange::InsertText {
            idx: 16,
            text: "\n!!!!".to_owned(),
            len: "\n!!!!".len(),
            selection: TextSelection::new_cursor(16),
        });
        rope.insert(21, "\n!!!!");
        history.push_change(HistoryChange::InsertText {
            idx: 21,
            text: "\n!!!!".to_owned(),
            len: "\n!!!!".len(),
            selection: TextSelection::new_cursor(21),
        });

        assert_eq!(history.any_pending_changes(), 0);
        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(rope.to_string(), "Hello World\n!!!!\n!!!!\n!!!!");

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

        history.redo(&mut rope);
        assert_eq!(rope.to_string(), "Hello World\n!!!!");
        history.redo(&mut rope);
        assert_eq!(rope.to_string(), "Hello World\n!!!!\n!!!!");
        history.redo(&mut rope);
        assert_eq!(rope.to_string(), "Hello World\n!!!!\n!!!!\n!!!!");

        assert_eq!(history.any_pending_changes(), 0);
        assert!(history.can_undo());
        assert!(!history.can_redo());

        history.undo(&mut rope);
        assert_eq!(rope.to_string(), "Hello World\n!!!!\n!!!!");
        assert_eq!(history.any_pending_changes(), 1);
        history.undo(&mut rope);
        assert_eq!(rope.to_string(), "Hello World\n!!!!");
        assert_eq!(history.any_pending_changes(), 2);

        rope.insert_char(0, '.');
        history.push_change(HistoryChange::InsertChar {
            idx: 0,
            ch: '.',
            len: 1,
            selection: TextSelection::new_cursor(0),
        });
        assert_eq!(history.any_pending_changes(), 0);
    }

    #[test]
    fn test_undo_restores_cursor_selection() {
        let mut rope = Rope::new();
        let mut history = EditorHistory::new(Duration::ZERO);

        rope.insert(0, "Hello World");

        rope.insert(11, "!");
        history.push_change(HistoryChange::InsertChar {
            idx: 11,
            ch: '!',
            len: 1,
            selection: TextSelection::new_cursor(11),
        });

        let selection = history.undo(&mut rope).unwrap();
        assert_eq!(selection, TextSelection::new_cursor(11));
        assert_eq!(rope.to_string(), "Hello World");
    }

    #[test]
    fn test_undo_restores_range_selection() {
        let mut rope = Rope::new();
        let mut history = EditorHistory::new(Duration::ZERO);

        rope.insert(0, "Hello World");

        let start = rope.utf16_cu_to_char(0);
        let end = rope.utf16_cu_to_char(5);
        rope.remove(start..end);
        history.push_change(HistoryChange::Remove {
            idx: 0,
            text: "Hello".to_owned(),
            len: 5,
            selection: TextSelection::new_range((0, 5)),
        });
        assert_eq!(rope.to_string(), " World");

        let selection = history.undo(&mut rope).unwrap();
        assert_eq!(selection, TextSelection::new_range((0, 5)));
        assert_eq!(rope.to_string(), "Hello World");
    }

    #[test]
    fn test_redo_returns_cursor_at_end() {
        let mut rope = Rope::new();
        let mut history = EditorHistory::new(Duration::ZERO);

        rope.insert(0, "Hello");

        rope.insert(5, " World");
        history.push_change(HistoryChange::InsertText {
            idx: 5,
            text: " World".to_owned(),
            len: 6,
            selection: TextSelection::new_cursor(5),
        });

        history.undo(&mut rope);
        assert_eq!(rope.to_string(), "Hello");

        let selection = history.redo(&mut rope).unwrap();
        assert_eq!(selection, TextSelection::new_cursor(11));
        assert_eq!(rope.to_string(), "Hello World");
    }

    #[test]
    fn test_undo_redo_with_remove() {
        let mut rope = Rope::new();
        let mut history = EditorHistory::new(Duration::ZERO);

        rope.insert(0, "Hello World");

        let start = rope.utf16_cu_to_char(5);
        let end = rope.utf16_cu_to_char(11);
        rope.remove(start..end);
        history.push_change(HistoryChange::Remove {
            idx: 5,
            text: " World".to_owned(),
            len: 6,
            selection: TextSelection::new_cursor(11),
        });
        assert_eq!(rope.to_string(), "Hello");

        let selection = history.undo(&mut rope).unwrap();
        assert_eq!(selection, TextSelection::new_cursor(11));
        assert_eq!(rope.to_string(), "Hello World");

        let selection = history.redo(&mut rope).unwrap();
        assert_eq!(selection, TextSelection::new_cursor(5));
        assert_eq!(rope.to_string(), "Hello");
    }
}
