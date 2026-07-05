// FILE: geom_fill_array1_of_section_law.rs
// occt: GeomFill_Array1OfSectionLaw

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct SectionLaw {}

pub type SectionLawHandle = Rc<RefCell<SectionLaw>>;

#[derive(Clone, Debug)]
pub struct Array1OfSectionLaw {
    items: Vec<SectionLawHandle>,
    lower: usize,
}

impl Array1OfSectionLaw {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower { upper - lower + 1 } else { 0 };
        Array1OfSectionLaw {
            items: vec![Rc::new(RefCell::new(SectionLaw {})); size],
            lower,
        }
    }

    pub fn at(&self, i: usize) -> Option<SectionLawHandle> {
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
    fn test_array_creation() {
        let arr = Array1OfSectionLaw::new(1, 3);
        assert_eq!(arr.len(), 3);
    }
}
