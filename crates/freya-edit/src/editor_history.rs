use std::time::{
    Duration,
    Instant,
};

use ropey::Rope;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn undo(&mut self, rope: &mut Rope) -> Option<usize> {
        if !self.can_undo() {
            return None;
        }

        let last_transaction = self.transactions.get(self.current_transaction - 1);
        if let Some(last_transaction) = last_transaction {
            let mut idx_end = None;
            for change in last_transaction.changes.iter().rev() {
                idx_end.replace(match change {
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
                });
            }

            self.current_transaction -= 1;
            self.version += 1;
            idx_end
        } else {
            None
        }
    }

    pub fn redo(&mut self, rope: &mut Rope) -> Option<usize> {
        if !self.can_redo() {
            return None;
        }

        let last_transaction = self.transactions.get(self.current_transaction);
        if let Some(last_transaction) = last_transaction {
            let mut idx_end = None;
            for change in &last_transaction.changes {
                idx_end.replace(match change {
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
                });
            }
            self.current_transaction += 1;
            self.version += 1;
            idx_end
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

    #[test]
    fn test() {
        let mut rope = Rope::new();
        let mut history = EditorHistory::new(Duration::ZERO);

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
