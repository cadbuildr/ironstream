// FILE: t_short_array2_of_short_real.rs
// occt: TShort_Array2OfShortReal

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_Array2<float> TShort_Array2OfShortReal;`
//!
//! OCCT Array2 of ShortReal: rectangular block with independent row and
//! column bounds (LowerRow/UpperRow, LowerCol/UpperCol), row-major.

/// `TShort_Array2OfShortReal`.
pub struct TShortArray2OfShortReal {
    lower_row: i32,
    upper_row: i32,
    lower_col: i32,
    upper_col: i32,
    data: Vec<f32>,
}

impl TShortArray2OfShortReal {
    /// Creates the block [lowerRow..upperRow] x [lowerCol..upperCol],
    /// zero-initialized.
    pub fn new(lower_row: i32, upper_row: i32, lower_col: i32, upper_col: i32) -> Self {
        assert!(upper_row >= lower_row, "Array2: upperRow must be >= lowerRow");
        assert!(upper_col >= lower_col, "Array2: upperCol must be >= lowerCol");
        let n = ((upper_row - lower_row + 1) as usize) * ((upper_col - lower_col + 1) as usize);
        TShortArray2OfShortReal {
            lower_row,
            upper_row,
            lower_col,
            upper_col,
            data: vec![0.0f32; n],
        }
    }

    pub fn lower_row(&self) -> i32 {
        self.lower_row
    }

    pub fn upper_row(&self) -> i32 {
        self.upper_row
    }

    pub fn lower_col(&self) -> i32 {
        self.lower_col
    }

    pub fn upper_col(&self) -> i32 {
        self.upper_col
    }

    /// NbRows / ColLength.
    pub fn nb_rows(&self) -> i32 {
        self.upper_row - self.lower_row + 1
    }

    /// NbColumns / RowLength.
    pub fn nb_columns(&self) -> i32 {
        self.upper_col - self.lower_col + 1
    }

    /// Total element count (Length in NCollection_Array2).
    pub fn length(&self) -> i32 {
        self.nb_rows() * self.nb_columns()
    }

    fn offset(&self, row: i32, col: i32) -> usize {
        assert!(
            row >= self.lower_row && row <= self.upper_row,
            "Array2: row {} out of range [{}, {}]",
            row,
            self.lower_row,
            self.upper_row
        );
        assert!(
            col >= self.lower_col && col <= self.upper_col,
            "Array2: col {} out of range [{}, {}]",
            col,
            self.lower_col,
            self.upper_col
        );
        ((row - self.lower_row) as usize) * (self.nb_columns() as usize)
            + ((col - self.lower_col) as usize)
    }

    /// Value(row, col).
    pub fn value(&self, row: i32, col: i32) -> f32 {
        self.data[self.offset(row, col)]
    }

    /// SetValue(row, col, v).
    pub fn set_value(&mut self, row: i32, col: i32, v: f32) {
        let off = self.offset(row, col);
        self.data[off] = v;
    }

    /// Init(v).
    pub fn init(&mut self, v: f32) {
        self.data.fill(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_and_sizes() {
        let arr = TShortArray2OfShortReal::new(1, 3, 1, 4);
        assert_eq!(arr.nb_rows(), 3);
        assert_eq!(arr.nb_columns(), 4);
        assert_eq!(arr.length(), 12);
        assert_eq!(arr.value(2, 2), 0.0);
    }

    #[test]
    fn set_get_distinct_cells() {
        let mut arr = TShortArray2OfShortReal::new(1, 2, 1, 2);
        arr.set_value(1, 1, 1.5);
        arr.set_value(1, 2, 2.5);
        arr.set_value(2, 1, 3.5);
        arr.set_value(2, 2, 4.5);
        assert_eq!(arr.value(1, 1), 1.5);
        assert_eq!(arr.value(1, 2), 2.5);
        assert_eq!(arr.value(2, 1), 3.5);
        assert_eq!(arr.value(2, 2), 4.5);
    }

    #[test]
    fn asymmetric_custom_bounds() {
        let mut arr = TShortArray2OfShortReal::new(0, 1, 10, 12);
        assert_eq!((arr.lower_row(), arr.upper_row()), (0, 1));
        assert_eq!((arr.lower_col(), arr.upper_col()), (10, 12));
        arr.set_value(0, 12, 9.0);
        arr.set_value(1, 10, -1.0);
        assert_eq!(arr.value(0, 12), 9.0);
        assert_eq!(arr.value(1, 10), -1.0);
        // Neighbor cells untouched.
        assert_eq!(arr.value(0, 10), 0.0);
    }

    #[test]
    #[should_panic(expected = "col")]
    fn out_of_range_column_panics() {
        let arr = TShortArray2OfShortReal::new(1, 2, 1, 2);
        let _ = arr.value(1, 3);
    }
}
