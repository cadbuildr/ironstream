// FILE: expr_intrp_stack_of_general_function.rs
// occt: ExprIntrp_StackOfGeneralFunction, ExprIntrp_ListIteratorOfStackOfGeneralFunction

use std::collections::LinkedList;

/// Deprecated typedef for backward compatibility.
/// A stack of GeneralFunction handles, implemented as a LinkedList.
pub struct ExprIntrpStackOfGeneralFunction {
    inner: LinkedList<String>,
}

impl ExprIntrpStackOfGeneralFunction {
    /// Create a new empty stack
    pub fn new() -> Self {
        Self {
            inner: LinkedList::new(),
        }
    }

    /// Push a function onto the stack
    pub fn push(&mut self, func: String) {
        self.inner.push_back(func);
    }

    /// Pop a function from the stack
    pub fn pop(&mut self) -> Option<String> {
        self.inner.pop_back()
    }

    /// Peek at the top of the stack
    pub fn peek(&self) -> Option<&str> {
        self.inner.back().map(|s| s.as_str())
    }

    /// Return the size of the stack
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Return true if stack is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clear the stack
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl Default for ExprIntrpStackOfGeneralFunction {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for the stack
pub struct ExprIntrpListIteratorOfStackOfGeneralFunction {
    items: Vec<String>,
    index: usize,
}

impl ExprIntrpListIteratorOfStackOfGeneralFunction {
    /// Create a new iterator from the stack
    pub fn new(stack: &ExprIntrpStackOfGeneralFunction) -> Self {
        let items: Vec<String> = stack.inner.iter().cloned().collect();
        Self { items, index: 0 }
    }

    /// Move to the next element
    pub fn next(&mut self) -> bool {
        if self.index < self.items.len() {
            self.index += 1;
            true
        } else {
            false
        }
    }

    /// Get the current value
    pub fn value(&self) -> Option<&str> {
        if self.index > 0 && self.index <= self.items.len() {
            Some(&self.items[self.index - 1])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_stack() {
        let stack = ExprIntrpStackOfGeneralFunction::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = ExprIntrpStackOfGeneralFunction::new();
        stack.push("func1".to_string());
        stack.push("func2".to_string());
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop(), Some("func2".to_string()));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.pop(), Some("func1".to_string()));
        assert!(stack.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut stack = ExprIntrpStackOfGeneralFunction::new();
        stack.push("first".to_string());
        stack.push("second".to_string());
        assert_eq!(stack.peek(), Some("second"));
        assert_eq!(stack.len(), 2); // peek doesn't remove
    }

    #[test]
    fn test_clear_stack() {
        let mut stack = ExprIntrpStackOfGeneralFunction::new();
        stack.push("test".to_string());
        stack.push("data".to_string());
        assert_eq!(stack.len(), 2);
        stack.clear();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut stack = ExprIntrpStackOfGeneralFunction::new();
        stack.push("a".to_string());
        stack.push("b".to_string());
        stack.push("c".to_string());

        let mut iter = ExprIntrpListIteratorOfStackOfGeneralFunction::new(&stack);
        assert_eq!(iter.value(), None);
        assert!(iter.next());
        assert_eq!(iter.value(), Some("a"));
        assert!(iter.next());
        assert_eq!(iter.value(), Some("b"));
        assert!(iter.next());
        assert_eq!(iter.value(), Some("c"));
        assert!(!iter.next());
    }
}
