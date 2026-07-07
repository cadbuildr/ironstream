// FILE: extrema_sequence_of_p_on_surf.rs
// occt: Extrema_SequenceOfPOnSurf

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct POnSurf {}

pub type POnSurfHandle = Rc<RefCell<POnSurf>>;

#[derive(Clone, Debug)]
pub struct SequenceOfPOnSurf {
    items: Vec<POnSurfHandle>,
}

impl SequenceOfPOnSurf {
    pub fn new() -> Self {
        SequenceOfPOnSurf {
            items: Vec::new(),
        }
    }

    pub fn append(&mut self, item: POnSurfHandle) {
        self.items.push(item);
    }

    pub fn at(&self, i: usize) -> Option<POnSurfHandle> {
        if i > 0 && i <= self.items.len() {
            Some(self.items[i - 1].clone())
        } else { None }
    }

    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn clear(&mut self) { self.items.clear() }
}

impl Default for SequenceOfPOnSurf {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq = SequenceOfPOnSurf::new();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_sequence_append() {
        let mut seq = SequenceOfPOnSurf::new();
        let item = Rc::new(RefCell::new(POnSurf {}));
        seq.append(item);
        assert_eq!(seq.len(), 1);
    }
}
