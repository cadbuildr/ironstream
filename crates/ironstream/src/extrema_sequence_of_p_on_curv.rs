// FILE: extrema_sequence_of_p_on_curv.rs
// occt: Extrema_SequenceOfPOnCurv

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct POnCurv {}

pub type POnCurvHandle = Rc<RefCell<POnCurv>>;

#[derive(Clone, Debug)]
pub struct SequenceOfPOnCurv {
    items: Vec<POnCurvHandle>,
}

impl SequenceOfPOnCurv {
    pub fn new() -> Self {
        SequenceOfPOnCurv {
            items: Vec::new(),
        }
    }

    pub fn append(&mut self, item: POnCurvHandle) {
        self.items.push(item);
    }

    pub fn at(&self, i: usize) -> Option<POnCurvHandle> {
        if i > 0 && i <= self.items.len() {
            Some(self.items[i - 1].clone())
        } else { None }
    }

    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn clear(&mut self) { self.items.clear() }
}

impl Default for SequenceOfPOnCurv {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq = SequenceOfPOnCurv::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_sequence_append() {
        let mut seq = SequenceOfPOnCurv::new();
        let item = Rc::new(RefCell::new(POnCurv {}));
        seq.append(item);
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_sequence_at() {
        let mut seq = SequenceOfPOnCurv::new();
        let item = Rc::new(RefCell::new(POnCurv {}));
        seq.append(item);
        assert!(seq.at(1).is_some());
        assert!(seq.at(2).is_none());
    }
}
