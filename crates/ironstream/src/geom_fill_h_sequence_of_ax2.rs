// FILE: geom_fill_h_sequence_of_ax2.rs
// occt: GeomFill_HSequenceOfAx2

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct Ax2 {}

pub type Ax2Handle = Rc<RefCell<Ax2>>;

#[derive(Clone, Debug)]
pub struct HSequenceOfAx2 {
    items: Vec<Ax2Handle>,
}

impl HSequenceOfAx2 {
    pub fn new() -> Self { HSequenceOfAx2 { items: Vec::new() } }
    pub fn append(&mut self, item: Ax2Handle) { self.items.push(item); }
    pub fn at(&self, i: usize) -> Option<Ax2Handle> {
        if i > 0 && i <= self.items.len() {
            Some(self.items[i - 1].clone())
        } else { None }
    }
    pub fn len(&self) -> usize { self.items.len() }
}

impl Default for HSequenceOfAx2 {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hsequence_creation() {
        let seq = HSequenceOfAx2::new();
        assert_eq!(seq.len(), 0);
    }
}
