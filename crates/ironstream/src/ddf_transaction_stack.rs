// FILE: ddf_transaction_stack.rs
// occt: DDF_TransactionStack

//! Deprecated typedef for backward compatibility.
//! This was `NCollection_List<opencascade::handle<DDF_Transaction>>` in OCCT.

use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// DDF_Transaction placeholder.
#[derive(Clone, Debug)]
pub struct DdfTransaction {
    id: u32,
    status: i32,
}

impl DdfTransaction {
    pub fn new(id: u32) -> Self {
        DdfTransaction { id, status: 0 }
    }
}

/// A stack (list) of transaction handles.
/// Models `NCollection_List<opencascade::handle<DDF_Transaction>>` from OCCT.
#[derive(Clone, Debug)]
pub struct DdfTransactionStack {
    items: Vec<Arc<DdfTransaction>>,
}

impl DdfTransactionStack {
    /// Create an empty stack.
    pub fn new() -> Self {
        DdfTransactionStack {
            items: Vec::new(),
        }
    }

    /// Push a transaction onto the stack.
    pub fn push(&mut self, tx: Arc<DdfTransaction>) {
        self.items.push(tx);
    }

    /// Pop a transaction from the stack.
    pub fn pop(&mut self) -> Option<Arc<DdfTransaction>> {
        self.items.pop()
    }

    /// Append a transaction.
    pub fn append(&mut self, tx: Arc<DdfTransaction>) {
        self.items.push(tx);
    }

    /// Prepend a transaction.
    pub fn prepend(&mut self, tx: Arc<DdfTransaction>) {
        self.items.insert(0, tx);
    }

    /// Get the number of transactions.
    pub fn length(&self) -> usize {
        self.items.len()
    }

    /// Check if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the top transaction without removing it.
    pub fn top(&self) -> Option<&Arc<DdfTransaction>> {
        self.items.last()
    }

    /// Clear the stack.
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for DdfTransactionStack {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for DdfTransactionStack {
    type Target = Vec<Arc<DdfTransaction>>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for DdfTransactionStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

/// Iterator for DDF_TransactionStack.
pub struct DdfListIteratorOfTransactionStack {
    items: Vec<Arc<DdfTransaction>>,
    index: usize,
}

impl DdfListIteratorOfTransactionStack {
    /// Create an iterator from a stack.
    pub fn new(stack: &DdfTransactionStack) -> Self {
        DdfListIteratorOfTransactionStack {
            items: stack.items.clone(),
            index: 0,
        }
    }

    /// Check if there are more items.
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    /// Get the current item and move to the next.
    pub fn next(&mut self) -> Option<Arc<DdfTransaction>> {
        if self.more() {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_creation() {
        let stack = DdfTransactionStack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.length(), 0);
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = DdfTransactionStack::new();
        let tx1 = Arc::new(DdfTransaction::new(1));
        let tx2 = Arc::new(DdfTransaction::new(2));

        stack.push(tx1.clone());
        stack.push(tx2.clone());

        assert_eq!(stack.length(), 2);
        assert!(Arc::ptr_eq(&stack.pop().unwrap(), &tx2));
        assert!(Arc::ptr_eq(&stack.pop().unwrap(), &tx1));
    }

    #[test]
    fn test_append() {
        let mut stack = DdfTransactionStack::new();
        let tx = Arc::new(DdfTransaction::new(1));

        stack.append(tx.clone());
        assert_eq!(stack.length(), 1);
    }

    #[test]
    fn test_prepend() {
        let mut stack = DdfTransactionStack::new();
        let tx1 = Arc::new(DdfTransaction::new(1));
        let tx2 = Arc::new(DdfTransaction::new(2));

        stack.append(tx2.clone());
        stack.prepend(tx1.clone());

        assert_eq!(stack.length(), 2);
        assert!(Arc::ptr_eq(&stack.items[0], &tx1));
    }

    #[test]
    fn test_top() {
        let mut stack = DdfTransactionStack::new();
        let tx = Arc::new(DdfTransaction::new(1));

        stack.push(tx.clone());
        assert!(Arc::ptr_eq(stack.top().unwrap(), &tx));
    }

    #[test]
    fn test_clear() {
        let mut stack = DdfTransactionStack::new();
        stack.push(Arc::new(DdfTransaction::new(1)));
        stack.push(Arc::new(DdfTransaction::new(2)));

        assert_eq!(stack.length(), 2);
        stack.clear();
        assert_eq!(stack.length(), 0);
    }

    #[test]
    fn test_iterator() {
        let mut stack = DdfTransactionStack::new();
        stack.push(Arc::new(DdfTransaction::new(1)));
        stack.push(Arc::new(DdfTransaction::new(2)));

        let mut iter = DdfListIteratorOfTransactionStack::new(&stack);
        assert!(iter.more());
        let first = iter.next();
        assert!(first.is_some());
        assert!(iter.more());
    }
}
