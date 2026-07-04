// FILE: t_doc_std_multi_transaction_manager.rs
// occt: TDocStd_MultiTransactionManager

use std::collections::VecDeque;

/// Manages multiple concurrent transactions for undo/redo operations.
#[derive(Clone, Debug)]
pub struct TDocStd_MultiTransactionManager {
    transactions: VecDeque<String>,
    current_transaction_index: Option<usize>,
    undo_limit: usize,
}

impl TDocStd_MultiTransactionManager {
    /// Create a new multi-transaction manager.
    pub fn new(undo_limit: usize) -> Self {
        Self {
            transactions: VecDeque::new(),
            current_transaction_index: None,
            undo_limit,
        }
    }

    /// Begin a new transaction.
    pub fn begin_transaction(&mut self, name: String) {
        self.transactions.push_back(name);
        self.current_transaction_index = Some(self.transactions.len() - 1);
    }

    /// Commit the current transaction.
    pub fn commit_transaction(&mut self) {
        // Limit undo history
        while self.transactions.len() > self.undo_limit {
            self.transactions.pop_front();
            if let Some(ref mut idx) = self.current_transaction_index {
                if *idx > 0 {
                    *idx -= 1;
                }
            }
        }
    }

    /// Abort the current transaction.
    pub fn abort_transaction(&mut self) {
        if let Some(idx) = self.current_transaction_index {
            if idx < self.transactions.len() {
                self.transactions.remove(idx);
                self.current_transaction_index = if idx > 0 { Some(idx - 1) } else { None };
            }
        }
    }

    /// Get the current transaction name.
    pub fn current_transaction_name(&self) -> Option<&str> {
        self.current_transaction_index
            .and_then(|idx| self.transactions.get(idx).map(|s| s.as_str()))
    }

    /// Get the number of transactions.
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Can undo.
    pub fn can_undo(&self) -> bool {
        self.current_transaction_index.is_some()
    }

    /// Can redo.
    pub fn can_redo(&self) -> bool {
        if let Some(idx) = self.current_transaction_index {
            idx < self.transactions.len() - 1
        } else {
            !self.transactions.is_empty()
        }
    }

    /// Undo the last transaction.
    pub fn undo(&mut self) -> bool {
        if self.can_undo() {
            if let Some(idx) = self.current_transaction_index {
                if idx > 0 {
                    self.current_transaction_index = Some(idx - 1);
                    return true;
                }
            }
        }
        false
    }

    /// Redo the next transaction.
    pub fn redo(&mut self) -> bool {
        if self.can_redo() {
            if let Some(idx) = self.current_transaction_index {
                if idx < self.transactions.len() - 1 {
                    self.current_transaction_index = Some(idx + 1);
                    return true;
                }
            } else if !self.transactions.is_empty() {
                self.current_transaction_index = Some(0);
                return true;
            }
        }
        false
    }
}

impl Default for TDocStd_MultiTransactionManager {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_manager() {
        let mgr = TDocStd_MultiTransactionManager::new(50);
        assert_eq!(mgr.transaction_count(), 0);
    }

    #[test]
    fn test_begin_transaction() {
        let mut mgr = TDocStd_MultiTransactionManager::new(10);
        mgr.begin_transaction("tx1".to_string());
        assert_eq!(mgr.transaction_count(), 1);
        assert_eq!(mgr.current_transaction_name(), Some("tx1"));
    }

    #[test]
    fn test_undo_redo() {
        let mut mgr = TDocStd_MultiTransactionManager::new(10);
        mgr.begin_transaction("tx1".to_string());
        mgr.commit_transaction();
        mgr.begin_transaction("tx2".to_string());
        mgr.commit_transaction();

        assert!(mgr.can_undo());
        assert!(mgr.undo());
        assert!(mgr.can_redo());
        assert!(mgr.redo());
    }

    #[test]
    fn test_undo_limit() {
        let mut mgr = TDocStd_MultiTransactionManager::new(2);
        mgr.begin_transaction("tx1".to_string());
        mgr.commit_transaction();
        mgr.begin_transaction("tx2".to_string());
        mgr.commit_transaction();
        mgr.begin_transaction("tx3".to_string());
        mgr.commit_transaction();
        // Should keep only the last 2
        assert!(mgr.transaction_count() <= 2);
    }

    #[test]
    fn test_default() {
        let mgr = TDocStd_MultiTransactionManager::default();
        assert_eq!(mgr.transaction_count(), 0);
    }
}
