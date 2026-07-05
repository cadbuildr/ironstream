// FILE: extrema_h_array2_of_p_on_surf.rs
// occt: Extrema_HArray2OfPOnSurf

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct POnSurf {}

pub type POnSurfHandle = Rc<RefCell<POnSurf>>;

#[derive(Clone, Debug)]
pub struct HArray2OfPOnSurf {
    items: Vec<Vec<POnSurfHandle>>,
    lower1: usize,
    lower2: usize,
}

impl HArray2OfPOnSurf {
    pub fn new(lower1: usize, upper1: usize, lower2: usize, upper2: usize) -> Self {
        let size1 = if upper1 >= lower1 { upper1 - lower1 + 1 } else { 0 };
        let size2 = if upper2 >= lower2 { upper2 - lower2 + 1 } else { 0 };
        let default_handle = Rc::new(RefCell::new(POnSurf {}));
        HArray2OfPOnSurf {
            items: vec![vec![default_handle.clone(); size2]; size1],
            lower1,
            lower2,
        }
    }

    pub fn at(&self, i: usize, j: usize) -> Option<POnSurfHandle> {
        let i_idx = i.checked_sub(self.lower1)?;
        let j_idx = j.checked_sub(self.lower2)?;
        self.items.get(i_idx)?.get(j_idx).map(|h| h.clone())
    }

    pub fn set(&mut self, i: usize, j: usize, val: POnSurfHandle) -> bool {
        if let Some(i_idx) = i.checked_sub(self.lower1) {
            if let Some(j_idx) = j.checked_sub(self.lower2) {
                if let Some(row) = self.items.get_mut(i_idx) {
                    if let Some(elem) = row.get_mut(j_idx) {
                        *elem = val;
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn lower1(&self) -> usize { self.lower1 }
    pub fn lower2(&self) -> usize { self.lower2 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harray2_creation() {
        let arr = HArray2OfPOnSurf::new(1, 3, 1, 2);
        assert_eq!(arr.lower1(), 1);
    }

    #[test]
    fn test_harray2_access() {
        let arr = HArray2OfPOnSurf::new(1, 3, 1, 2);
        assert!(arr.at(1, 1).is_some());
    }
}
