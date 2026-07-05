// FILE: t_obj_sequence_of_iterator.rs
// occt: TObj_SequenceOfIterator

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_Sequence<opencascade::handle<TObj_ObjectIterator>>
//!    TObj_SequenceOfIterator;`
//!
//! This sequence feeds `TObj_ModelIterator`, which drains child iterators
//! one after another. Items are handles to object iterators; the local
//! iterator stub honestly implements the More/Next/Value protocol over
//! a fixed name list.

use std::cell::RefCell;
use std::rc::Rc;

/// Local stand-in for `TObj_ObjectIterator`: More/Next/Value protocol.
#[derive(Debug)]
pub struct TObjObjectIteratorStubSoi {
    names: Vec<String>,
    cursor: RefCell<usize>,
}

impl TObjObjectIteratorStubSoi {
    pub fn over(names: &[&str]) -> Rc<Self> {
        Rc::new(TObjObjectIteratorStubSoi {
            names: names.iter().map(|s| s.to_string()).collect(),
            cursor: RefCell::new(0),
        })
    }

    /// TObj_ObjectIterator::More.
    pub fn more(&self) -> bool {
        *self.cursor.borrow() < self.names.len()
    }

    /// TObj_ObjectIterator::Next.
    pub fn next(&self) {
        let mut c = self.cursor.borrow_mut();
        if *c < self.names.len() {
            *c += 1;
        }
    }

    /// TObj_ObjectIterator::Value (name of current object).
    pub fn value(&self) -> Option<String> {
        self.names.get(*self.cursor.borrow()).cloned()
    }
}

pub type HandleObjectIteratorSoi = Rc<TObjObjectIteratorStubSoi>;

/// `TObj_SequenceOfIterator` with NCollection_Sequence semantics (1-based).
#[derive(Default)]
pub struct TObjSequenceOfIterator {
    items: Vec<HandleObjectIteratorSoi>,
}

impl TObjSequenceOfIterator {
    pub fn new() -> Self {
        TObjSequenceOfIterator { items: Vec::new() }
    }

    pub fn length(&self) -> i32 {
        self.items.len() as i32
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn append(&mut self, it: HandleObjectIteratorSoi) {
        self.items.push(it);
    }

    pub fn prepend(&mut self, it: HandleObjectIteratorSoi) {
        self.items.insert(0, it);
    }

    fn offset(&self, index: i32) -> usize {
        assert!(
            index >= 1 && index <= self.items.len() as i32,
            "Sequence: index {} out of range [1, {}]",
            index,
            self.items.len()
        );
        (index - 1) as usize
    }

    /// Value(index) — 1-based.
    pub fn value(&self, index: i32) -> &HandleObjectIteratorSoi {
        &self.items[self.offset(index)]
    }

    /// First() — used by TObj_ModelIterator to get the active iterator.
    pub fn first(&self) -> &HandleObjectIteratorSoi {
        assert!(!self.items.is_empty(), "Sequence: First on empty sequence");
        &self.items[0]
    }

    /// Remove(index) — 1-based; ModelIterator removes drained iterators
    /// with Remove(1).
    pub fn remove(&mut self, index: i32) {
        let off = self.offset(index);
        self.items.remove(off);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_one_based_access() {
        let mut seq = TObjSequenceOfIterator::new();
        seq.append(TObjObjectIteratorStubSoi::over(&["a"]));
        seq.append(TObjObjectIteratorStubSoi::over(&["b", "c"]));
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(1).value(), Some("a".to_string()));
        assert_eq!(seq.value(2).value(), Some("b".to_string()));
    }

    #[test]
    fn model_iterator_drain_pattern() {
        // Reproduce TObj_ModelIterator::Next: drain First(), Remove(1), repeat.
        let mut seq = TObjSequenceOfIterator::new();
        seq.append(TObjObjectIteratorStubSoi::over(&["p1", "p2"]));
        seq.append(TObjObjectIteratorStubSoi::over(&["q1"]));
        let mut visited = Vec::new();
        while !seq.is_empty() {
            let it = seq.first().clone();
            while it.more() {
                visited.push(it.value().unwrap());
                it.next();
            }
            seq.remove(1);
        }
        assert_eq!(visited, vec!["p1", "p2", "q1"]);
        assert_eq!(seq.length(), 0);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn remove_out_of_range_panics() {
        let mut seq = TObjSequenceOfIterator::new();
        seq.remove(1);
    }
}
