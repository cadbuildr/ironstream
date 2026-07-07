// FILE: extrema_sequence_of_p_on_curv2d.rs
// occt: Extrema_SequenceOfPOnCurv2d

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct POnCurv2d {}

pub type POnCurv2dHandle = Rc<RefCell<POnCurv2d>>;

#[derive(Clone, Debug)]
pub struct SequenceOfPOnCurv2d {
    items: Vec<POnCurv2dHandle>,
}

impl SequenceOfPOnCurv2d {
    pub fn new() -> Self {
        SequenceOfPOnCurv2d {
            items: Vec::new(),
        }
    }

    pub fn append(&mut self, item: POnCurv2dHandle) {
        self.items.push(item);
    }

    pub fn at(&self, i: usize) -> Option<POnCurv2dHandle> {
        if i > 0 && i <= self.items.len() {
            Some(self.items[i - 1].clone())
        } else { None }
    }

    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn clear(&mut self) { self.items.clear() }
}

impl Default for SequenceOfPOnCurv2d {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq = SequenceOfPOnCurv2d::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_sequence_append() {
        let mut seq = SequenceOfPOnCurv2d::new();
        let item = Rc::new(RefCell::new(POnCurv2d {}));
        seq.append(item);
        assert_eq!(seq.len(), 1);
    }
}
