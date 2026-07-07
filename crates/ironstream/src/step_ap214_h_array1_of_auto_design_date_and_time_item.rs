// FILE: step_ap214_h_array1_of_auto_design_date_and_time_item.rs
// occt: StepAP214_HArray1OfAutoDesignDateAndTimeItem

use std::rc::Rc;
use std::cell::RefCell;

pub struct StepAP214HArray1OfAutoDesignDateAndTimeItem {
    inner: Rc<RefCell<HArray1Inner>>,
}

struct HArray1Inner {
    data: Vec<i32>,
    lower: usize,
    upper: usize,
}

impl StepAP214HArray1OfAutoDesignDateAndTimeItem {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = upper.saturating_sub(lower) + 1;
        Self {
            inner: Rc::new(RefCell::new(HArray1Inner {
                data: vec![0; size],
                lower,
                upper,
            })),
        }
    }

    pub fn lower(&self) -> usize { self.inner.borrow().lower }
    pub fn upper(&self) -> usize { self.inner.borrow().upper }
    pub fn len(&self) -> usize {
        let inner = self.inner.borrow();
        inner.upper.saturating_sub(inner.lower) + 1
    }
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    pub fn value(&self, idx: usize) -> i32 {
        let inner = self.inner.borrow();
        if idx < inner.lower || idx > inner.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, inner.lower, inner.upper);
        }
        inner.data[idx - inner.lower]
    }

    pub fn set_value(&self, idx: usize, val: i32) {
        let mut inner = self.inner.borrow_mut();
        if idx < inner.lower || idx > inner.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, inner.lower, inner.upper);
        }
        let off = idx - inner.lower;
        inner.data[off] = val;
    }
}

impl Clone for StepAP214HArray1OfAutoDesignDateAndTimeItem {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        let arr = StepAP214HArray1OfAutoDesignDateAndTimeItem::new(1, 5);
        assert_eq!(arr.len(), 5);
        arr.set_value(2, 42);
        assert_eq!(arr.value(2), 42);
    }

    #[test]
    fn test_clone() {
        let arr1 = StepAP214HArray1OfAutoDesignDateAndTimeItem::new(1, 3);
        arr1.set_value(2, 99);
        let arr2 = arr1.clone();
        assert_eq!(arr2.value(2), 99);
    }
}
