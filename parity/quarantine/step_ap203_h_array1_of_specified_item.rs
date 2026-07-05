// FILE: step_ap203_h_array1_of_specified_item.rs
// occt: StepAP203_HArray1OfSpecifiedItem

use std::rc::Rc;
use std::cell::RefCell;

/// Deprecated newtype: a reference-counted handle to a 1-based Array1 of StepAP203_SpecifiedItem.
/// Mirrors OCCT NCollection_HArray1<StepAP203_SpecifiedItem>.
/// Uses Lower/Upper bounds (1-based indexing by default, like OCCT Array1).
pub struct StepAP203HArray1OfSpecifiedItem {
    inner: Rc<RefCell<HArray1Inner>>,
}

struct HArray1Inner {
    data: Vec<i32>, // Placeholder for StepAP203_SpecifiedItem
    lower: usize,
    upper: usize,
}

impl StepAP203HArray1OfSpecifiedItem {
    /// Create a new handle with capacity from lower to upper (1-based).
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

    /// Get the lower bound (first valid index).
    pub fn lower(&self) -> usize {
        self.inner.borrow().lower
    }

    /// Get the upper bound (last valid index).
    pub fn upper(&self) -> usize {
        self.inner.borrow().upper
    }

    /// Get length of the array.
    pub fn len(&self) -> usize {
        let inner = self.inner.borrow();
        inner.upper.saturating_sub(inner.lower) + 1
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get element at index (1-based, OCCT style).
    pub fn value(&self, idx: usize) -> i32 {
        let inner = self.inner.borrow();
        if idx < inner.lower || idx > inner.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, inner.lower, inner.upper);
        }
        inner.data[idx - inner.lower]
    }

    /// Set element at index (1-based, OCCT style).
    pub fn set_value(&self, idx: usize, val: i32) {
        let mut inner = self.inner.borrow_mut();
        if idx < inner.lower || idx > inner.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, inner.lower, inner.upper);
        }
        inner.data[idx - inner.lower] = val;
    }
}

impl Clone for StepAP203HArray1OfSpecifiedItem {
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
    fn test_harray1_create() {
        let arr = StepAP203HArray1OfSpecifiedItem::new(1, 10);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 10);
        assert_eq!(arr.len(), 10);
    }

    #[test]
    fn test_harray1_bounds() {
        let arr = StepAP203HArray1OfSpecifiedItem::new(5, 15);
        assert_eq!(arr.lower(), 5);
        assert_eq!(arr.upper(), 15);
        assert_eq!(arr.len(), 11);
    }

    #[test]
    fn test_harray1_get_set() {
        let arr = StepAP203HArray1OfSpecifiedItem::new(1, 5);
        arr.set_value(1, 100);
        arr.set_value(3, 200);
        arr.set_value(5, 300);

        assert_eq!(arr.value(1), 100);
        assert_eq!(arr.value(3), 200);
        assert_eq!(arr.value(5), 300);
        assert_eq!(arr.value(2), 0); // default initialized
    }

    #[test]
    fn test_harray1_clone_shares_data() {
        let arr1 = StepAP203HArray1OfSpecifiedItem::new(1, 3);
        arr1.set_value(2, 42);

        let arr2 = arr1.clone();
        assert_eq!(arr2.value(2), 42);

        arr2.set_value(2, 99);
        assert_eq!(arr1.value(2), 99); // same underlying data
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_harray1_panic_lower() {
        let arr = StepAP203HArray1OfSpecifiedItem::new(5, 10);
        arr.value(4); // below lower bound
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_harray1_panic_upper() {
        let arr = StepAP203HArray1OfSpecifiedItem::new(1, 5);
        arr.value(6); // above upper bound
    }
}
