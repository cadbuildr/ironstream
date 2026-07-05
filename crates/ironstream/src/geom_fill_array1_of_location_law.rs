// FILE: geom_fill_array1_of_location_law.rs
// occt: GeomFill_Array1OfLocationLaw

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct LocationLaw {}

pub type LocationLawHandle = Rc<RefCell<LocationLaw>>;

#[derive(Clone, Debug)]
pub struct Array1OfLocationLaw {
    items: Vec<LocationLawHandle>,
    lower: usize,
}

impl Array1OfLocationLaw {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        Array1OfLocationLaw {
            items: vec![Rc::new(RefCell::new(LocationLaw {})); size],
            lower,
        }
    }

    pub fn at(&self, i: usize) -> Option<LocationLawHandle> {
        if i >= self.lower && i < self.lower + self.items.len() {
            Some(self.items[i - self.lower].clone())
        } else { None }
    }

    pub fn set(&mut self, i: usize, val: LocationLawHandle) -> bool {
        if i >= self.lower && i < self.lower + self.items.len() {
            self.items[i - self.lower] = val;
            true
        } else { false }
    }

    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.lower + self.items.len() - 1 }
    pub fn len(&self) -> usize { self.items.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_array_creation() {
        let arr = Array1OfLocationLaw::new(1, 3);
        assert_eq!(arr.len(), 3);
    }
}
