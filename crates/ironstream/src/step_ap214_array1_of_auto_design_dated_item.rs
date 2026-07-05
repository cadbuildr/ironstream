// FILE: step_ap214_array1_of_auto_design_dated_item.rs
// occt: StepAP214_Array1OfAutoDesignDatedItem

use std::cell::RefCell;

pub struct StepAP214Array1OfAutoDesignDatedItem {
    data: RefCell<Vec<i32>>,
    lower: usize,
    upper: usize,
}

impl StepAP214Array1OfAutoDesignDatedItem {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = upper.saturating_sub(lower) + 1;
        Self {
            data: RefCell::new(vec![0; size]),
            lower,
            upper,
        }
    }

    pub fn lower(&self) -> usize {
        self.lower
    }

    pub fn upper(&self) -> usize {
        self.upper
    }

    pub fn len(&self) -> usize {
        self.upper.saturating_sub(self.lower) + 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

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
    fn test_array1_create() {
        let arr = StepAP214Array1OfAutoDesignDatedItem::new(1, 10);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.len(), 10);
    }

    #[test]
    fn test_array1_get_set() {
        let arr = StepAP214Array1OfAutoDesignDatedItem::new(1, 5);
        arr.set_value(3, 99);
        assert_eq!(arr.value(3), 99);
    }
}
