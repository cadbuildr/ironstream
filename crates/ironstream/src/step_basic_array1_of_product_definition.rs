// FILE: step_basic_array1_of_product_definition.rs
// occt: StepBasic_Array1OfProductDefinition

use std::cell::RefCell;

pub struct StepBasicArray1OfProductDefinition {
    data: RefCell<Vec<i32>>,
    lower: usize,
    upper: usize,
}

impl StepBasicArray1OfProductDefinition {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = upper.saturating_sub(lower) + 1;
        Self {
            data: RefCell::new(vec![0; size]),
            lower,
            upper,
        }
    }

    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.upper }
    pub fn len(&self) -> usize { self.upper.saturating_sub(self.lower) + 1 }
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    pub fn value(&self, idx: usize) -> i32 {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data.borrow()[idx - self.lower]
    }

    pub fn set_value(&self, idx: usize, val: i32) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data.borrow_mut()[idx - self.lower] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        let arr = StepBasicArray1OfProductDefinition::new(1, 5);
        assert_eq!(arr.len(), 5);
        arr.set_value(2, 42);
        assert_eq!(arr.value(2), 42);
    }
}
