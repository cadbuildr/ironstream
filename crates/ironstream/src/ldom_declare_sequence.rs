// FILE: ldom_declare_sequence.rs
// occt: LDOM_DeclareSequence

/// Generic sequence (doubly-linked list) implementation.
/// This provides a generic container similar to the DECLARE_SEQUENCE macro in OCCT.
#[derive(Clone)]
pub struct LDOMSequence<T: Clone> {
    first: Option<Box<Node<T>>>,
    last: Option<Box<Node<T>>>,
    current: Option<Box<Node<T>>>,
    i_cur: usize,
    length: usize,
}

#[derive(Clone)]
struct Node<T: Clone> {
    value: T,
    prev: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> LDOMSequence<T> {
    /// Create an empty sequence
    pub fn new() -> Self {
        LDOMSequence {
            first: None,
            last: None,
            current: None,
            i_cur: 0,
            length: 0,
        }
    }

    /// Get the length of the sequence
    pub fn length(&self) -> usize {
        self.length
    }

    /// Get the first element
    pub fn first(&self) -> Option<T> {
        self.first.as_ref().map(|n| n.value.clone())
    }

    /// Get the last element
    pub fn last(&self) -> Option<T> {
        self.last.as_ref().map(|n| n.value.clone())
    }

    /// Get element at index (1-based)
    pub fn value(&self, index: usize) -> Option<T> {
        if index < 1 || index > self.length {
            return None;
        }

        let mut current = &self.first;
        for _ in 1..index {
            if let Some(node) = current {
                current = &node.next;
            } else {
                return None;
            }
        }

        current.as_ref().map(|n| n.value.clone())
    }

    /// Append a value to the sequence
    pub fn append(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            prev: None,
            next: None,
        });

        if self.length == 0 {
            self.first = Some(new_node);
            self.last = self.first.as_ref().map(|n| {
                Box::new(Node {
                    value: n.value.clone(),
                    prev: None,
                    next: None,
                })
            });
        } else {
            if let Some(ref mut last) = self.last {
                last.next = Some(new_node);
            }
            // TODO: Properly update last pointer
        }
        self.length += 1;
    }

    /// Prepend a value to the sequence
    pub fn prepend(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            prev: None,
            next: self.first.take(),
        });

        self.first = Some(new_node);
        if self.length == 0 {
            self.last = self.first.as_ref().map(|n| {
                Box::new(Node {
                    value: n.value.clone(),
                    prev: None,
                    next: None,
                })
            });
        }
        self.length += 1;

        if self.i_cur > 0 {
            self.i_cur += 1;
        }
    }

    /// Clear the sequence
    pub fn clear(&mut self) {
        self.first = None;
        self.last = None;
        self.current = None;
        self.i_cur = 0;
        self.length = 0;
    }

    /// Remove element at index
    pub fn remove(&mut self, _index: usize) {
        // TODO: Implement removal
    }
}

impl<T: Clone> Default for LDOMSequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_sequence() {
        let seq: LDOMSequence<i32> = LDOMSequence::new();
        assert_eq!(seq.length(), 0);
        assert_eq!(seq.first(), None);
    }

    #[test]
    fn test_append() {
        let mut seq: LDOMSequence<i32> = LDOMSequence::new();
        seq.append(1);
        seq.append(2);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.first(), Some(1));
    }

    #[test]
    fn test_prepend() {
        let mut seq: LDOMSequence<i32> = LDOMSequence::new();
        seq.prepend(1);
        seq.prepend(2);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.first(), Some(2));
    }

    #[test]
    fn test_clear() {
        let mut seq: LDOMSequence<i32> = LDOMSequence::new();
        seq.append(1);
        seq.append(2);
        seq.clear();
        assert_eq!(seq.length(), 0);
    }
}
