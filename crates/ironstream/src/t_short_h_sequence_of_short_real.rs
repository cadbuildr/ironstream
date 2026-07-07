// FILE: t_short_h_sequence_of_short_real.rs
// occt: TShort_HSequenceOfShortReal

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_HSequence<float> TShort_HSequenceOfShortReal;`
//!
//! Handle-sequence of ShortReal with OCCT Sequence semantics: 1-based
//! indices, Append/Prepend/InsertAfter/Remove/Split, shared through the
//! handle.

use std::cell::RefCell;
use std::rc::Rc;

/// `TShort_HSequenceOfShortReal` — handle to a heap Sequence<f32>.
#[derive(Clone, Default)]
pub struct TShortHSequenceOfShortReal {
    payload: Rc<RefCell<Vec<f32>>>,
}

impl TShortHSequenceOfShortReal {
    pub fn new() -> Self {
        TShortHSequenceOfShortReal {
            payload: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn length(&self) -> i32 {
        self.payload.borrow().len() as i32
    }

    pub fn is_empty(&self) -> bool {
        self.payload.borrow().is_empty()
    }

    pub fn append(&self, v: f32) {
        self.payload.borrow_mut().push(v);
    }

    pub fn prepend(&self, v: f32) {
        self.payload.borrow_mut().insert(0, v);
    }

    fn checked_offset(&self, index: i32) -> usize {
        let len = self.payload.borrow().len() as i32;
        assert!(
            index >= 1 && index <= len,
            "Sequence: index {} out of range [1, {}]",
            index,
            len
        );
        (index - 1) as usize
    }

    /// Value(index) — 1-based.
    pub fn value(&self, index: i32) -> f32 {
        let off = self.checked_offset(index);
        self.payload.borrow()[off]
    }

    /// SetValue(index, v).
    pub fn set_value(&self, index: i32, v: f32) {
        let off = self.checked_offset(index);
        self.payload.borrow_mut()[off] = v;
    }

    /// InsertAfter(index, v).
    pub fn insert_after(&self, index: i32, v: f32) {
        let off = self.checked_offset(index);
        self.payload.borrow_mut().insert(off + 1, v);
    }

    /// Remove(index).
    pub fn remove(&self, index: i32) {
        let off = self.checked_offset(index);
        self.payload.borrow_mut().remove(off);
    }

    /// Remove(from, to) — inclusive 1-based range.
    pub fn remove_range(&self, from: i32, to: i32) {
        assert!(from <= to, "Sequence: invalid removal range");
        let a = self.checked_offset(from);
        let b = self.checked_offset(to);
        self.payload.borrow_mut().drain(a..=b);
    }

    /// Split(index): the tail starting at `index` is moved into a new
    /// sequence; self keeps items [1, index-1].
    pub fn split(&self, index: i32) -> TShortHSequenceOfShortReal {
        let off = self.checked_offset(index);
        let tail: Vec<f32> = self.payload.borrow_mut().split_off(off);
        TShortHSequenceOfShortReal {
            payload: Rc::new(RefCell::new(tail)),
        }
    }

    pub fn clear(&self) {
        self.payload.borrow_mut().clear();
    }

    pub fn is_same_handle(&self, other: &TShortHSequenceOfShortReal) -> bool {
        Rc::ptr_eq(&self.payload, &other.payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_prepend_one_based() {
        let seq = TShortHSequenceOfShortReal::new();
        seq.append(2.0);
        seq.append(3.0);
        seq.prepend(1.0);
        assert_eq!(seq.length(), 3);
        assert_eq!(seq.value(1), 1.0);
        assert_eq!(seq.value(3), 3.0);
    }

    #[test]
    fn insert_after_remove_range() {
        let seq = TShortHSequenceOfShortReal::new();
        for v in [1.0f32, 4.0, 5.0, 6.0] {
            seq.append(v);
        }
        seq.insert_after(1, 2.0);
        seq.insert_after(2, 3.0);
        assert_eq!(
            (1..=6).map(|i| seq.value(i)).collect::<Vec<_>>(),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
        );
        seq.remove_range(2, 4);
        assert_eq!(seq.length(), 3);
        assert_eq!(seq.value(2), 5.0);
    }

    #[test]
    fn split_moves_tail() {
        let seq = TShortHSequenceOfShortReal::new();
        for v in [10.0f32, 20.0, 30.0, 40.0] {
            seq.append(v);
        }
        let tail = seq.split(3);
        assert_eq!(seq.length(), 2);
        assert_eq!(tail.length(), 2);
        assert_eq!(tail.value(1), 30.0);
        assert!(!seq.is_same_handle(&tail));
    }

    #[test]
    fn handle_sharing() {
        let h1 = TShortHSequenceOfShortReal::new();
        let h2 = h1.clone();
        h2.append(42.0);
        assert_eq!(h1.length(), 1);
        assert_eq!(h1.value(1), 42.0);
        assert!(h1.is_same_handle(&h2));
    }
}
