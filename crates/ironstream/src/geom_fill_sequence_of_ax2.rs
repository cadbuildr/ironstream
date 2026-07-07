// FILE: geom_fill_sequence_of_ax2.rs
// occt: GeomFill_SequenceOfAx2

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct Ax2 {}

pub type Ax2Handle = Rc<RefCell<Ax2>>;

#[derive(Clone, Debug)]
pub struct SequenceOfAx2 {
    items: Vec<Ax2Handle>,
}

impl SequenceOfAx2 {
    pub fn new() -> Self { SequenceOfAx2 { items: Vec::new() } }
    pub fn append(&mut self, item: Ax2Handle) { self.items.push(item); }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

impl Default for SequenceOfAx2 {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sequence_creation() {
        let seq = SequenceOfAx2::new();
        assert!(seq.is_empty());
    }
}
