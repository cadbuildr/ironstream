// FILE: geom_fill_h_array1_of_location_law.rs
// occt: GeomFill_HArray1OfLocationLaw

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct LocationLaw {}

pub type LocationLawHandle = Rc<RefCell<LocationLaw>>;

#[derive(Clone, Debug)]
pub struct HArray1OfLocationLaw {
    items: Vec<LocationLawHandle>,
    lower: usize,
}

impl HArray1OfLocationLaw {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        HArray1OfLocationLaw {
            items: vec![Rc::new(RefCell::new(LocationLaw {})); size],
            lower,
        }
    }

    pub fn at(&self, i: usize) -> Option<LocationLawHandle> {
        if i >= self.lower && i < self.lower + self.items.len() {
            Some(self.items[i - self.lower].clone())
        } else { None }
    }

    pub fn len(&self) -> usize { self.items.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_harray_creation() {
        let arr = HArray1OfLocationLaw::new(1, 3);
        assert_eq!(arr.len(), 3);
    }
}
