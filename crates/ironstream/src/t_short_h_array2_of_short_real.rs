// FILE: t_short_h_array2_of_short_real.rs
// occt: TShort_HArray2OfShortReal

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_HArray2<float> TShort_HArray2OfShortReal;`
//!
//! Handle (heap, shared) variant of TShort_Array2OfShortReal.

use std::cell::RefCell;
use std::rc::Rc;

struct TShortArray2PayloadH2 {
    lower_row: i32,
    upper_row: i32,
    lower_col: i32,
    upper_col: i32,
    data: Vec<f32>,
}

impl TShortArray2PayloadH2 {
    fn nb_columns(&self) -> i32 {
        self.upper_col - self.lower_col + 1
    }

    fn offset(&self, row: i32, col: i32) -> usize {
        assert!(
            row >= self.lower_row && row <= self.upper_row,
            "HArray2: row {} out of range [{}, {}]",
            row,
            self.lower_row,
            self.upper_row
        );
        assert!(
            col >= self.lower_col && col <= self.upper_col,
            "HArray2: col {} out of range [{}, {}]",
            col,
            self.lower_col,
            self.upper_col
        );
        ((row - self.lower_row) as usize) * (self.nb_columns() as usize)
            + ((col - self.lower_col) as usize)
    }
}

/// `TShort_HArray2OfShortReal` — handle to a heap Array2<f32>.
#[derive(Clone)]
pub struct TShortHArray2OfShortReal {
    payload: Rc<RefCell<TShortArray2PayloadH2>>,
}

impl TShortHArray2OfShortReal {
    pub fn new(lower_row: i32, upper_row: i32, lower_col: i32, upper_col: i32) -> Self {
        assert!(upper_row >= lower_row, "HArray2: upperRow must be >= lowerRow");
        assert!(upper_col >= lower_col, "HArray2: upperCol must be >= lowerCol");
        let n = ((upper_row - lower_row + 1) as usize) * ((upper_col - lower_col + 1) as usize);
        TShortHArray2OfShortReal {
            payload: Rc::new(RefCell::new(TShortArray2PayloadH2 {
                lower_row,
                upper_row,
                lower_col,
                upper_col,
                data: vec![0.0f32; n],
            })),
        }
    }

    pub fn lower_row(&self) -> i32 {
        self.payload.borrow().lower_row
    }

    pub fn upper_row(&self) -> i32 {
        self.payload.borrow().upper_row
    }

    pub fn lower_col(&self) -> i32 {
        self.payload.borrow().lower_col
    }

    pub fn upper_col(&self) -> i32 {
        self.payload.borrow().upper_col
    }

    pub fn nb_rows(&self) -> i32 {
        let p = self.payload.borrow();
        p.upper_row - p.lower_row + 1
    }

    pub fn nb_columns(&self) -> i32 {
        self.payload.borrow().nb_columns()
    }

    pub fn value(&self, row: i32, col: i32) -> f32 {
        let p = self.payload.borrow();
        let off = p.offset(row, col);
        p.data[off]
    }

    pub fn set_value(&self, row: i32, col: i32, v: f32) {
        let mut p = self.payload.borrow_mut();
        let off = p.offset(row, col);
        p.data[off] = v;
    }

    pub fn init(&self, v: f32) {
        self.payload.borrow_mut().data.fill(v);
    }

    pub fn is_same_handle(&self, other: &TShortHArray2OfShortReal) -> bool {
        Rc::ptr_eq(&self.payload, &other.payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_shape_and_zero_init() {
        let h = TShortHArray2OfShortReal::new(1, 2, 1, 5);
        assert_eq!(h.nb_rows(), 2);
        assert_eq!(h.nb_columns(), 5);
        assert_eq!(h.value(2, 5), 0.0);
    }

    #[test]
    fn shared_handle_mutation() {
        let h1 = TShortHArray2OfShortReal::new(1, 2, 1, 2);
        let h2 = h1.clone();
        h2.set_value(2, 1, 3.75);
        assert_eq!(h1.value(2, 1), 3.75);
        assert!(h1.is_same_handle(&h2));
    }

    #[test]
    fn init_then_overwrite_cell() {
        let h = TShortHArray2OfShortReal::new(0, 1, 0, 1);
        h.init(1.0);
        h.set_value(0, 1, 2.0);
        assert_eq!(h.value(0, 0), 1.0);
        assert_eq!(h.value(0, 1), 2.0);
        assert_eq!(h.value(1, 1), 1.0);
    }

    #[test]
    #[should_panic(expected = "row")]
    fn bad_row_panics() {
        let h = TShortHArray2OfShortReal::new(1, 2, 1, 2);
        let _ = h.value(3, 1);
    }
}
