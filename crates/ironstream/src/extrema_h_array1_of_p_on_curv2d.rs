// FILE: extrema_h_array1_of_p_on_curv2d.rs
// occt: Extrema_HArray1OfPOnCurv2d

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct POnCurv2d {}

pub type POnCurv2dHandle = Rc<RefCell<POnCurv2d>>;

#[derive(Clone, Debug)]
pub struct HArray1OfPOnCurv2d {
    items: Vec<POnCurv2dHandle>,
    lower: usize,
}

impl HArray1OfPOnCurv2d {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        HArray1OfPOnCurv2d {
            items: vec![Rc::new(RefCell::new(POnCurv2d {})); size],
            lower,
        }
    }

    pub fn at(&self, i: usize) -> Option<POnCurv2dHandle> {
        if i >= self.lower && i < self.lower + self.items.len() {
            Some(self.items[i - self.lower].clone())
        } else { None }
    }

    pub fn set(&mut self, i: usize, val: POnCurv2dHandle) -> bool {
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
        let arr = HArray1OfPOnCurv2d::new(1, 3);
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_harray_access() {
        let arr = HArray1OfPOnCurv2d::new(1, 3);
        assert!(arr.at(1).is_some());
    }
}
