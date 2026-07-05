// FILE: t_short_h_array1_of_short_real.rs
// occt: TShort_HArray1OfShortReal

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_HArray1<float> TShort_HArray1OfShortReal;`
//!
//! The handle (heap, shared) variant of TShort_Array1OfShortReal —
//! historically the storage of Poly_Triangulation normals. Handle
//! sharing means aliased mutation through a RefCell interior.

use std::cell::RefCell;
use std::rc::Rc;

/// Payload array with OCCT Array1 semantics.
pub struct TShortArray1PayloadH1 {
    lower: i32,
    upper: i32,
    data: Vec<f32>,
}

impl TShortArray1PayloadH1 {
    fn offset(&self, index: i32) -> usize {
        assert!(
            index >= self.lower && index <= self.upper,
            "HArray1: index {} out of range [{}, {}]",
            index,
            self.lower,
            self.upper
        );
        (index - self.lower) as usize
    }
}

/// `TShort_HArray1OfShortReal` — handle to a heap Array1<f32>.
#[derive(Clone)]
pub struct TShortHArray1OfShortReal {
    payload: Rc<RefCell<TShortArray1PayloadH1>>,
}

impl TShortHArray1OfShortReal {
    /// new TShort_HArray1OfShortReal(lower, upper) — zero-filled.
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(upper >= lower, "HArray1: upper must be >= lower");
        TShortHArray1OfShortReal {
            payload: Rc::new(RefCell::new(TShortArray1PayloadH1 {
                lower,
                upper,
                data: vec![0.0f32; (upper - lower + 1) as usize],
            })),
        }
    }

    /// new TShort_HArray1OfShortReal(lower, upper, initValue).
    pub fn new_filled(lower: i32, upper: i32, init: f32) -> Self {
        let h = TShortHArray1OfShortReal::new(lower, upper);
        h.payload.borrow_mut().data.fill(init);
        h
    }

    pub fn lower(&self) -> i32 {
        self.payload.borrow().lower
    }

    pub fn upper(&self) -> i32 {
        self.payload.borrow().upper
    }

    pub fn length(&self) -> i32 {
        let p = self.payload.borrow();
        p.upper - p.lower + 1
    }

    pub fn value(&self, index: i32) -> f32 {
        let p = self.payload.borrow();
        let off = p.offset(index);
        p.data[off]
    }

    pub fn set_value(&self, index: i32, v: f32) {
        let mut p = self.payload.borrow_mut();
        let off = p.offset(index);
        p.data[off] = v;
    }

    /// Handle identity (h1 == h2 in OCCT compares pointers).
    pub fn is_same_handle(&self, other: &TShortHArray1OfShortReal) -> bool {
        Rc::ptr_eq(&self.payload, &other.payload)
    }

    /// Array1() accessor: snapshot copy of the payload values.
    pub fn to_vec(&self) -> Vec<f32> {
        self.payload.borrow().data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filled_constructor() {
        let h = TShortHArray1OfShortReal::new_filled(1, 4, 2.5);
        assert_eq!(h.length(), 4);
        for i in 1..=4 {
            assert_eq!(h.value(i), 2.5);
        }
    }

    #[test]
    fn handle_sharing_aliases_storage() {
        let h1 = TShortHArray1OfShortReal::new(1, 3);
        let h2 = h1.clone(); // handle copy, same array
        h2.set_value(2, 9.0);
        assert_eq!(h1.value(2), 9.0, "both handles see the same storage");
        assert!(h1.is_same_handle(&h2));
        let independent = TShortHArray1OfShortReal::new(1, 3);
        assert!(!h1.is_same_handle(&independent));
    }

    #[test]
    fn bounds_respected() {
        let h = TShortHArray1OfShortReal::new(5, 8);
        assert_eq!((h.lower(), h.upper()), (5, 8));
        h.set_value(8, -1.0);
        assert_eq!(h.to_vec(), vec![0.0, 0.0, 0.0, -1.0]);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn out_of_range_panics() {
        let h = TShortHArray1OfShortReal::new(1, 2);
        h.set_value(0, 1.0);
    }
}
