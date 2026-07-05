// FILE: step_ap214_array1_of_approval_item.rs
// occt: StepAP214_Array1OfApprovalItem

use std::cell::RefCell;

/// Deprecated newtype: a 1-based Array1 of StepAP214_ApprovalItem.
/// Mirrors OCCT NCollection_Array1<StepAP214_ApprovalItem>.
/// Uses Lower/Upper bounds (1-based indexing by default, like OCCT Array1).
pub struct StepAP214Array1OfApprovalItem {
    data: RefCell<Vec<i32>>, // Placeholder for StepAP214_ApprovalItem
    lower: usize,
    upper: usize,
}

impl StepAP214Array1OfApprovalItem {
    /// Create a new array with capacity from lower to upper (1-based).
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = upper.saturating_sub(lower) + 1;
        Self {
            data: RefCell::new(vec![0; size]),
            lower,
            upper,
        }
    }

    /// Get the lower bound (first valid index).
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Get the upper bound (last valid index).
    pub fn upper(&self) -> usize {
        self.upper
    }

    /// Get length of the array.
    pub fn len(&self) -> usize {
        self.upper.saturating_sub(self.lower) + 1
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get element at index (1-based, OCCT style).
    pub fn value(&self, idx: usize) -> i32 {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data.borrow()[idx - self.lower]
    }

    /// Set element at index (1-based, OCCT style).
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
        let arr = StepAP214Array1OfApprovalItem::new(1, 10);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 10);
        assert_eq!(arr.len(), 10);
    }

    #[test]
    fn test_array1_bounds() {
        let arr = StepAP214Array1OfApprovalItem::new(5, 15);
        assert_eq!(arr.lower(), 5);
        assert_eq!(arr.upper(), 15);
        assert_eq!(arr.len(), 11);
    }

    #[test]
    fn test_array1_get_set() {
        let arr = StepAP214Array1OfApprovalItem::new(1, 5);
        arr.set_value(1, 100);
        arr.set_value(3, 200);
        arr.set_value(5, 300);

        assert_eq!(arr.value(1), 100);
        assert_eq!(arr.value(3), 200);
        assert_eq!(arr.value(5), 300);
        assert_eq!(arr.value(2), 0); // default initialized
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_array1_panic_lower() {
        let arr = StepAP214Array1OfApprovalItem::new(5, 10);
        arr.value(4); // below lower bound
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_array1_panic_upper() {
        let arr = StepAP214Array1OfApprovalItem::new(1, 5);
        arr.value(6); // above upper bound
    }
}
