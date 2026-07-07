// FILE: step_data_h_array1_of_field.rs
// occt: StepData_HArray1OfField

use std::cell::RefCell;
use std::rc::Rc;

// Local helper mirroring StepData_Field (external plumbing, subset)
#[derive(Clone, Default)]
pub struct StepDataField {
    kind: i32,
    int_val: i32,
    real_val: f64,
    text: Option<String>,
}

impl StepDataField {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_integer(&mut self, val: i32) {
        self.kind = 1;
        self.int_val = val;
    }
    pub fn integer(&self) -> i32 {
        self.int_val
    }
    pub fn set_real(&mut self, val: f64) {
        self.kind = 5;
        self.real_val = val;
    }
    pub fn real(&self) -> f64 {
        self.real_val
    }
    pub fn set_string(&mut self, val: &str) {
        self.kind = 6;
        self.text = Some(val.to_string());
    }
    pub fn string(&self) -> &str {
        self.text.as_deref().unwrap_or("")
    }
    pub fn is_set(&self) -> bool {
        self.kind != 0
    }
}

// Handle-array of StepData_Field with 1-based (lower..=upper) OCCT indexing.
// Mirrors NCollection_HArray1<StepData_Field>: handle semantics (shared
// ownership), fixed bounds, Value/SetValue/ChangeValue access.
pub struct StepDataHArray1OfField {
    inner: Rc<RefCell<HArray1Inner>>,
}

struct HArray1Inner {
    data: Vec<StepDataField>,
    lower: usize,
    upper: usize,
}

impl StepDataHArray1OfField {
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = upper.saturating_sub(lower) + 1;
        Self {
            inner: Rc::new(RefCell::new(HArray1Inner {
                data: vec![StepDataField::new(); size],
                lower,
                upper,
            })),
        }
    }

    pub fn lower(&self) -> usize {
        self.inner.borrow().lower
    }

    pub fn upper(&self) -> usize {
        self.inner.borrow().upper
    }

    pub fn len(&self) -> usize {
        let inner = self.inner.borrow();
        inner.upper.saturating_sub(inner.lower) + 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // Value(idx): returns a copy of the field at 1-based index idx
    pub fn value(&self, idx: usize) -> StepDataField {
        let inner = self.inner.borrow();
        if idx < inner.lower || idx > inner.upper {
            panic!(
                "Index {} out of bounds [{}, {}]",
                idx, inner.lower, inner.upper
            );
        }
        inner.data[idx - inner.lower].clone()
    }

    // SetValue(idx, field)
    pub fn set_value(&self, idx: usize, val: StepDataField) {
        let mut inner = self.inner.borrow_mut();
        let (lower, upper) = (inner.lower, inner.upper);
        if idx < lower || idx > upper {
            panic!("Index {} out of bounds [{}, {}]", idx, lower, upper);
        }
        inner.data[idx - lower] = val;
    }

    // ChangeValue(idx): mutate the field in place via a closure
    pub fn change_value<F: FnOnce(&mut StepDataField)>(&self, idx: usize, f: F) {
        let mut inner = self.inner.borrow_mut();
        let (lower, upper) = (inner.lower, inner.upper);
        if idx < lower || idx > upper {
            panic!("Index {} out of bounds [{}, {}]", idx, lower, upper);
        }
        f(&mut inner.data[idx - lower]);
    }
}

impl Clone for StepDataHArray1OfField {
    // Handle copy: shares the same underlying array (occ::handle semantics)
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
        let arr = StepDataHArray1OfField::new(1, 5);
        assert_eq!(arr.len(), 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert!(!arr.value(2).is_set());

        let mut f = StepDataField::new();
        f.set_integer(42);
        arr.set_value(2, f);
        assert_eq!(arr.value(2).integer(), 42);
        assert!(arr.value(2).is_set());
    }

    #[test]
    fn test_change_value_and_kinds() {
        let arr = StepDataHArray1OfField::new(3, 6);
        assert_eq!(arr.len(), 4);
        arr.change_value(3, |f| f.set_real(2.5));
        arr.change_value(6, |f| f.set_string("hello"));
        assert!((arr.value(3).real() - 2.5).abs() < 1e-12);
        assert_eq!(arr.value(6).string(), "hello");
    }

    #[test]
    fn test_clone_shares_handle() {
        let arr1 = StepDataHArray1OfField::new(1, 3);
        arr1.change_value(2, |f| f.set_integer(99));
        let arr2 = arr1.clone();
        assert_eq!(arr2.value(2).integer(), 99);
        // handle semantics: mutation through one handle is seen by the other
        arr2.change_value(2, |f| f.set_integer(7));
        assert_eq!(arr1.value(2).integer(), 7);
    }
}
