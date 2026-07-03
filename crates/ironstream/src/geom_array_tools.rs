// FILE: geom_array_tools.rs
//! Geometric array collection stubs — 1-D and 2-D fixed-size arrays of
//! 3-D points and real values, modelled after OpenCascade's
//! `TColgp` and `TColStd` collection types.
//!
//! Indexing follows the OCCT convention: elements run from `lower` to `upper`
//! inclusive for 1-D arrays. Bounds violations panic with a descriptive
//! message, mirroring `Standard_OutOfRange`.
//!
//! No external crates are used; only `std` is required.

// ─── Array1OfPnt ─────────────────────────────────────────────────────────────

/// A 1-D, fixed-size array of 3-D points (`[f64; 3]`) with explicit lower and
/// upper bounds.
///
/// Mirrors `TColgp_Array1OfPnt` from OpenCascade's `TColgp` package.
/// Elements are indexed from `lower` to `upper` inclusive.
// occt: TColgp_Array1OfPnt
#[derive(Clone, Debug)]
pub struct Array1OfPnt {
    /// The raw storage for the points.
    pub data: Vec<[f64; 3]>,
    /// The lower bound (inclusive) used for element indexing.
    pub lower: i32,
    /// The upper bound (inclusive) used for element indexing.
    pub upper: i32,
}

impl Array1OfPnt {
    /// Construct an array with explicit lower and upper bounds.
    ///
    /// All elements are initialised to the origin `[0.0, 0.0, 0.0]`.
    /// Mirrors `TColgp_Array1OfPnt(lower, upper)`.
    ///
    /// # Panics
    ///
    /// Panics when `upper < lower`.
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(
            upper >= lower,
            "Array1OfPnt: upper ({upper}) must be >= lower ({lower})"
        );
        let len = (upper - lower + 1) as usize;
        Self {
            data: vec![[0.0, 0.0, 0.0]; len],
            lower,
            upper,
        }
    }

    /// Set the value at index `i` (1-based by OCCT convention, but respects
    /// `self.lower`).
    ///
    /// # Panics
    ///
    /// Panics when `i` is outside `[self.lower, self.upper]`.
    pub fn set_value(&mut self, i: i32, v: [f64; 3]) {
        let idx = self.checked_index(i);
        self.data[idx] = v;
    }

    /// Return the value at index `i`.
    ///
    /// # Panics
    ///
    /// Panics when `i` is outside `[self.lower, self.upper]`.
    pub fn value(&self, i: i32) -> [f64; 3] {
        let idx = self.checked_index(i);
        self.data[idx]
    }

    /// Return the number of elements in the array.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Return `true` when the array contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // ── internal helpers ──────────────────────────────────────────────────────

    #[inline]
    fn checked_index(&self, i: i32) -> usize {
        assert!(
            i >= self.lower && i <= self.upper,
            "Array1OfPnt: index {i} out of range [{}, {}]",
            self.lower,
            self.upper
        );
        (i - self.lower) as usize
    }
}

// ─── Array1OfReal ────────────────────────────────────────────────────────────

/// A 1-D, fixed-size array of real values (`f64`) with explicit lower and
/// upper bounds.
///
/// Mirrors `TColStd_Array1OfReal` from OpenCascade's `TColStd` package.
/// Elements are indexed from `lower` to `upper` inclusive.
// occt: TColStd_Array1OfReal
#[derive(Clone, Debug)]
pub struct Array1OfReal {
    /// The raw storage for the real values.
    pub data: Vec<f64>,
    /// The lower bound (inclusive) used for element indexing.
    pub lower: i32,
    /// The upper bound (inclusive) used for element indexing.
    pub upper: i32,
}

impl Array1OfReal {
    /// Construct an array with explicit lower and upper bounds.
    ///
    /// All elements are initialised to `0.0`.
    /// Mirrors `TColStd_Array1OfReal(lower, upper)`.
    ///
    /// # Panics
    ///
    /// Panics when `upper < lower`.
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(
            upper >= lower,
            "Array1OfReal: upper ({upper}) must be >= lower ({lower})"
        );
        let len = (upper - lower + 1) as usize;
        Self {
            data: vec![0.0; len],
            lower,
            upper,
        }
    }

    /// Set the value at index `i`.
    ///
    /// # Panics
    ///
    /// Panics when `i` is outside `[self.lower, self.upper]`.
    pub fn set_value(&mut self, i: i32, v: f64) {
        let idx = self.checked_index(i);
        self.data[idx] = v;
    }

    /// Return the value at index `i`.
    ///
    /// # Panics
    ///
    /// Panics when `i` is outside `[self.lower, self.upper]`.
    pub fn value(&self, i: i32) -> f64 {
        let idx = self.checked_index(i);
        self.data[idx]
    }

    /// Return the number of elements in the array.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    // ── internal helpers ──────────────────────────────────────────────────────

    #[inline]
    fn checked_index(&self, i: i32) -> usize {
        assert!(
            i >= self.lower && i <= self.upper,
            "Array1OfReal: index {i} out of range [{}, {}]",
            self.lower,
            self.upper
        );
        (i - self.lower) as usize
    }
}

// ─── Array2OfPnt ─────────────────────────────────────────────────────────────

/// A 2-D, fixed-size array of 3-D points (`[f64; 3]`), stored row-major.
///
/// Mirrors `TColgp_Array2OfPnt` from OpenCascade's `TColgp` package.
/// Elements are indexed by zero-based `(i, j)` where `i < rows` and
/// `j < cols`.
// occt: TColgp_Array2OfPnt
#[derive(Clone, Debug)]
pub struct Array2OfPnt {
    /// The raw storage, organised as `data[row][col]`.
    pub data: Vec<Vec<[f64; 3]>>,
    /// Number of rows.
    pub rows: usize,
    /// Number of columns.
    pub cols: usize,
}

impl Array2OfPnt {
    /// Construct a `rows × cols` array.
    ///
    /// All elements are initialised to the origin `[0.0, 0.0, 0.0]`.
    /// Mirrors `TColgp_Array2OfPnt(1, rows, 1, cols)`.
    ///
    /// # Panics
    ///
    /// Panics when either `rows` or `cols` is zero.
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows > 0, "Array2OfPnt: rows must be >= 1");
        assert!(cols > 0, "Array2OfPnt: cols must be >= 1");
        Self {
            data: vec![vec![[0.0, 0.0, 0.0]; cols]; rows],
            rows,
            cols,
        }
    }

    /// Set the value at row `i`, column `j` (both zero-based).
    ///
    /// # Panics
    ///
    /// Panics when `i >= self.rows` or `j >= self.cols`.
    pub fn set_value(&mut self, i: usize, j: usize, v: [f64; 3]) {
        self.check_bounds(i, j);
        self.data[i][j] = v;
    }

    /// Return the value at row `i`, column `j` (both zero-based).
    ///
    /// # Panics
    ///
    /// Panics when `i >= self.rows` or `j >= self.cols`.
    pub fn value(&self, i: usize, j: usize) -> [f64; 3] {
        self.check_bounds(i, j);
        self.data[i][j]
    }

    // ── internal helpers ──────────────────────────────────────────────────────

    #[inline]
    fn check_bounds(&self, i: usize, j: usize) {
        assert!(
            i < self.rows,
            "Array2OfPnt: row index {i} out of range [0, {})",
            self.rows
        );
        assert!(
            j < self.cols,
            "Array2OfPnt: col index {j} out of range [0, {})",
            self.cols
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array1_of_pnt_roundtrip() {
        let mut a = Array1OfPnt::new(1, 3);
        assert_eq!(a.length(), 3);
        assert!(!a.is_empty());
        a.set_value(2, [1.0, 2.0, 3.0]);
        assert_eq!(a.value(2), [1.0, 2.0, 3.0]);
        assert_eq!(a.value(1), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn array1_of_pnt_negative_lower() {
        let mut a = Array1OfPnt::new(-2, 2);
        assert_eq!(a.length(), 5);
        a.set_value(-2, [9.0, 8.0, 7.0]);
        assert_eq!(a.value(-2), [9.0, 8.0, 7.0]);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn array1_of_pnt_oob() {
        let a = Array1OfPnt::new(1, 3);
        let _ = a.value(4);
    }

    #[test]
    fn array1_of_real_roundtrip() {
        let mut a = Array1OfReal::new(0, 4);
        assert_eq!(a.length(), 5);
        a.set_value(3, std::f64::consts::PI);
        assert!((a.value(3) - std::f64::consts::PI).abs() < f64::EPSILON);
    }

    #[test]
    fn array2_of_pnt_roundtrip() {
        let mut a = Array2OfPnt::new(3, 4);
        assert_eq!(a.rows, 3);
        assert_eq!(a.cols, 4);
        a.set_value(1, 2, [5.0, 6.0, 7.0]);
        assert_eq!(a.value(1, 2), [5.0, 6.0, 7.0]);
        assert_eq!(a.value(0, 0), [0.0, 0.0, 0.0]);
    }

    #[test]
    #[should_panic(expected = "col index")]
    fn array2_of_pnt_col_oob() {
        let a = Array2OfPnt::new(2, 2);
        let _ = a.value(0, 5);
    }
}
