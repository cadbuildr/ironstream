// FILE: ddf_transaction.rs
// occt: DDF_Transaction

//! DDF_Transaction: transaction management for TDF data.

use std::sync::Mutex;

/// Transaction status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionStatus {
    Idle,
    Open,
    Committing,
    Aborting,
}

/// DDF_Transaction: manages a single transaction.
#[derive(Clone, Debug)]
pub struct DdfTransaction {
    id: u32,
    status: TransactionStatus,
    changes: Vec<String>,
}

impl DdfTransaction {
    /// Create a new transaction.
    pub fn new(id: u32) -> Self {
        DdfTransaction {
            id,
            status: TransactionStatus::Idle,
            changes: Vec::new(),
        }
    }

    /// Open the transaction.
    pub fn open(&mut self) -> bool {
        if self.status == TransactionStatus::Idle {
            self.status = TransactionStatus::Open;
            true
        } else {
            false
        }
    }

    /// Commit the transaction.
    pub fn commit(&mut self) -> bool {
        if self.status == TransactionStatus::Open {
            self.status = TransactionStatus::Committing;
            // Apply changes
            self.status = TransactionStatus::Idle;
            true
        } else {
            false
        }
    }

    /// Abort the transaction.
    pub fn abort(&mut self) -> bool {
        if self.status == TransactionStatus::Open {
            self.status = TransactionStatus::Aborting;
            self.changes.clear();
            self.status = TransactionStatus::Idle;
            true
        } else {
            false
        }
    }

    /// Check if the transaction is open.
    pub fn is_open(&self) -> bool {
        self.status == TransactionStatus::Open
    }

    /// Get transaction status.
    pub fn status(&self) -> TransactionStatus {
        self.status
    }

    /// Add a change record.
    pub fn add_change(&mut self, change: &str) {
        if self.is_open() {
            self.changes.push(change.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = DdfTransaction::new(1);
        assert_eq!(tx.id, 1);
        assert_eq!(tx.status(), TransactionStatus::Idle);
    }

    #[test]
    fn test_open_transaction() {
        let mut tx = DdfTransaction::new(1);
        assert!(tx.open());
        assert_eq!(tx.status(), TransactionStatus::Open);
    }

    #[test]
    fn test_cannot_open_twice() {
        let mut tx = DdfTransaction::new(1);
        assert!(tx.open());
        assert!(!tx.open());
    }

    #[test]
    fn test_commit_transaction() {
        let mut tx = DdfTransaction::new(1);
        tx.open();
        assert!(tx.commit());
        assert_eq!(tx.status(), TransactionStatus::Idle);
    }

    #[test]
    fn test_cannot_commit_idle() {
        let mut tx = DdfTransaction::new(1);
        assert!(!tx.commit());
    }

    #[test]
    fn test_abort_transaction() {
        let mut tx = DdfTransaction::new(1);
        tx.open();
        assert!(tx.abort());
        assert_eq!(tx.status(), TransactionStatus::Idle);
    }

    #[test]
    fn test_add_changes() {
        let mut tx = DdfTransaction::new(1);
        tx.open();
        tx.add_change("change1");
        tx.add_change("change2");
        assert_eq!(tx.changes.len(), 2);
    }

    #[test]
    fn test_changes_cleared_on_abort() {
        let mut tx = DdfTransaction::new(1);
        tx.open();
        tx.add_change("change1");
        tx.abort();
        assert_eq!(tx.changes.len(), 0);
    }
}
