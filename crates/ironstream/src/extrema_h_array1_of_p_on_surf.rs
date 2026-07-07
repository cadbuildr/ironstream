// FILE: extrema_h_array1_of_p_on_surf.rs
// occt: Extrema_HArray1OfPOnSurf

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct POnSurf {}

pub type POnSurfHandle = Rc<RefCell<POnSurf>>;

#[derive(Clone, Debug)]
pub struct HArray1OfPOnSurf {
    items: Vec<POnSurfHandle>,
    lower: usize,
}

impl HArray1OfPOnSurf {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        HArray1OfPOnSurf {
            items: vec![Rc::new(RefCell::new(POnSurf {})); size],
            lower,
        }
    }

    pub fn at(&self, i: usize) -> Option<POnSurfHandle> {
        if i >= self.lower && i < self.lower + self.items.len() {
            Some(self.items[i - self.lower].clone())
        } else { None }
    }

    pub fn set(&mut self, i: usize, val: POnSurfHandle) -> bool {
        if i >= self.lower && i < self.lower + self.items.len() {
            self.items[i - self.lower] = val;
            true
        } else { false }
    }

    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.lower + self.items.len() - 1 }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harray_creation() {
        let arr = HArray1OfPOnSurf::new(1, 3);
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_harray_access() {
        let arr = HArray1OfPOnSurf::new(1, 3);
        assert!(arr.at(1).is_some());
    }
}
