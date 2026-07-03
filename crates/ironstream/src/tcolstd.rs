// FILE: tcolstd.rs
//! `TColStd` — collection types for standard scalar values, mirroring
//! OpenCascade's `TColStd` package (TKMath).
//!
//! These collections map closely to `NCollection_Array1`, `NCollection_Array2`,
//! and `NCollection_Sequence` specialised for `Standard_Integer` (`i32`) and
//! `Standard_Real` (`f64`). Indexing follows OCCT convention: **1-based** for
//! Array1/Array2, and **1-based** for Sequence. All bounds violations panic with
//! a descriptive message, mirroring `Standard_OutOfRange`.

// ─── TColStd_Array1OfInteger ─────────────────────────────────────────────────

/// A 1-D, fixed-size, **1-based** array of `Standard_Integer` (`i32`).
///
/// Mirrors `TColStd_Array1OfInteger` from OpenCascade's `TColStd` package.
/// The array is indexed from `lower()` to `upper()` inclusive, defaulting
/// to 1-based when constructed with `new(n)`.
// occt: TColStd_Array1OfInteger
#[derive(Clone, Debug)]
pub struct Array1OfInteger {
    data: Vec<i32>,
    lower: usize,
}

impl Array1OfInteger {
    /// Construct a 1-based array of `n` elements, all initialised to zero.
    /// Mirrors `TColStd_Array1OfInteger(1, n)`.
    ///
    /// # Panics
    /// Panics when `n == 0`.
    pub fn new(n: usize) -> Self {
        assert!(n > 0, "TColStd_Array1OfInteger: size must be >= 1");
        Self {
            data: vec![0; n],
            lower: 1,
        }
    }

    /// Construct an array with an explicit lower bound: elements are indexed
    /// `lower ..= upper`. Mirrors `TColStd_Array1OfInteger(lower, upper)`.
    ///
    /// # Panics
    /// Panics when `upper < lower`.
    pub fn with_bounds(lower: usize, upper: usize) -> Self {
        assert!(
            upper >= lower,
            "TColStd_Array1OfInteger: upper ({upper}) < lower ({lower})"
        );
        let n = upper - lower + 1;
        Self {
            data: vec![0; n],
            lower,
        }
    }

    /// Lower index (1 by default).
    #[inline]
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Upper index (`lower + len - 1`).
    #[inline]
    pub fn upper(&self) -> usize {
        self.lower + self.data.len() - 1
    }

    /// Number of elements.
    #[inline]
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// True when the array contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Access element at 1-based index `i`. Mirrors `Value(i)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[lower, upper]`.
    #[inline]
    pub fn value(&self, i: usize) -> i32 {
        self.check_bounds(i);
        self.data[i - self.lower]
    }

    /// Mutable reference to element at 1-based index `i`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[lower, upper]`.
    #[inline]
    pub fn value_mut(&mut self, i: usize) -> &mut i32 {
        self.check_bounds(i);
        let idx = i - self.lower;
        &mut self.data[idx]
    }

    /// Set element at 1-based index `i` to `v`. Mirrors `SetValue(i, v)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[lower, upper]`.
    #[inline]
    pub fn set_value(&mut self, i: usize, v: i32) {
        self.check_bounds(i);
        self.data[i - self.lower] = v;
    }

    /// Fill every element with `v`. Mirrors `Init(v)`.
    pub fn init(&mut self, v: i32) {
        for e in &mut self.data {
            *e = v;
        }
    }

    /// Iterator over elements in index order.
    pub fn iter(&self) -> impl Iterator<Item = &i32> {
        self.data.iter()
    }

    /// Iterator of (index, value) pairs, where index is 1-based.
    pub fn iter_indexed(&self) -> impl Iterator<Item = (usize, i32)> + '_ {
        let lower = self.lower;
        self.data.iter().enumerate().map(move |(i, &v)| (i + lower, v))
    }

    /// Compute the minimum value in the array.
    ///
    /// # Panics
    /// Panics on an empty array.
    pub fn min_value(&self) -> i32 {
        assert!(!self.data.is_empty(), "TColStd_Array1OfInteger::min_value: empty array");
        *self.data.iter().min().unwrap()
    }

    /// Compute the maximum value in the array.
    ///
    /// # Panics
    /// Panics on an empty array.
    pub fn max_value(&self) -> i32 {
        assert!(!self.data.is_empty(), "TColStd_Array1OfInteger::max_value: empty array");
        *self.data.iter().max().unwrap()
    }

    /// Compute the sum of all elements.
    pub fn sum(&self) -> i64 {
        self.data.iter().map(|&v| v as i64).sum()
    }

    /// Sort elements in ascending order in place.
    pub fn sort(&mut self) {
        self.data.sort_unstable();
    }

    /// Return true if the array contains `v`.
    pub fn contains(&self, v: i32) -> bool {
        self.data.contains(&v)
    }

    #[inline]
    fn check_bounds(&self, i: usize) {
        assert!(
            i >= self.lower && i <= self.upper(),
            "TColStd_Array1OfInteger: index {i} out of range [{}, {}]",
            self.lower,
            self.upper()
        );
    }
}

// ─── TColStd_Array1OfReal ────────────────────────────────────────────────────

/// A 1-D, fixed-size, **1-based** array of `Standard_Real` (`f64`).
///
/// Mirrors `TColStd_Array1OfReal` from OpenCascade's `TColStd` package.
/// The array is indexed from `lower()` to `upper()` inclusive, defaulting
/// to 1-based when constructed with `new(n)`.
// occt: TColStd_Array1OfReal
#[derive(Clone, Debug)]
pub struct Array1OfReal {
    data: Vec<f64>,
    lower: usize,
}

impl Array1OfReal {
    /// Construct a 1-based array of `n` elements, all initialised to 0.0.
    /// Mirrors `TColStd_Array1OfReal(1, n)`.
    ///
    /// # Panics
    /// Panics when `n == 0`.
    pub fn new(n: usize) -> Self {
        assert!(n > 0, "TColStd_Array1OfReal: size must be >= 1");
        Self {
            data: vec![0.0; n],
            lower: 1,
        }
    }

    /// Construct an array with an explicit lower bound: elements are indexed
    /// `lower ..= upper`. Mirrors `TColStd_Array1OfReal(lower, upper)`.
    ///
    /// # Panics
    /// Panics when `upper < lower`.
    pub fn with_bounds(lower: usize, upper: usize) -> Self {
        assert!(
            upper >= lower,
            "TColStd_Array1OfReal: upper ({upper}) < lower ({lower})"
        );
        let n = upper - lower + 1;
        Self {
            data: vec![0.0; n],
            lower,
        }
    }

    /// Construct from a slice of values (1-based).
    pub fn from_slice(vals: &[f64]) -> Self {
        assert!(!vals.is_empty(), "TColStd_Array1OfReal::from_slice: empty slice");
        Self {
            data: vals.to_vec(),
            lower: 1,
        }
    }

    /// Lower index (1 by default).
    #[inline]
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Upper index (`lower + len - 1`).
    #[inline]
    pub fn upper(&self) -> usize {
        self.lower + self.data.len() - 1
    }

    /// Number of elements.
    #[inline]
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// True when the array contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Access element at 1-based index `i`. Mirrors `Value(i)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[lower, upper]`.
    #[inline]
    pub fn value(&self, i: usize) -> f64 {
        self.check_bounds(i);
        self.data[i - self.lower]
    }

    /// Mutable reference to element at 1-based index `i`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[lower, upper]`.
    #[inline]
    pub fn value_mut(&mut self, i: usize) -> &mut f64 {
        self.check_bounds(i);
        let idx = i - self.lower;
        &mut self.data[idx]
    }

    /// Set element at 1-based index `i` to `v`. Mirrors `SetValue(i, v)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[lower, upper]`.
    #[inline]
    pub fn set_value(&mut self, i: usize, v: f64) {
        self.check_bounds(i);
        self.data[i - self.lower] = v;
    }

    /// Fill every element with `v`. Mirrors `Init(v)`.
    pub fn init(&mut self, v: f64) {
        for e in &mut self.data {
            *e = v;
        }
    }

    /// Iterator over elements in index order.
    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.data.iter()
    }

    /// Iterator of (index, value) pairs, where index is 1-based.
    pub fn iter_indexed(&self) -> impl Iterator<Item = (usize, f64)> + '_ {
        let lower = self.lower;
        self.data.iter().enumerate().map(move |(i, &v)| (i + lower, v))
    }

    /// Minimum value (NaN-ignoring).
    ///
    /// # Panics
    /// Panics on an empty array.
    pub fn min_value(&self) -> f64 {
        assert!(!self.data.is_empty(), "TColStd_Array1OfReal::min_value: empty array");
        self.data.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    /// Maximum value (NaN-ignoring).
    ///
    /// # Panics
    /// Panics on an empty array.
    pub fn max_value(&self) -> f64 {
        assert!(!self.data.is_empty(), "TColStd_Array1OfReal::max_value: empty array");
        self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }

    /// Compute the sum of all elements.
    pub fn sum(&self) -> f64 {
        self.data.iter().sum()
    }

    /// Compute the arithmetic mean.
    ///
    /// # Panics
    /// Panics on an empty array.
    pub fn mean(&self) -> f64 {
        assert!(!self.data.is_empty(), "TColStd_Array1OfReal::mean: empty array");
        self.sum() / self.data.len() as f64
    }

    /// Sort elements in ascending order in place (NaN-safe: NaNs go last).
    pub fn sort(&mut self) {
        self.data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater));
    }

    /// Normalize elements so the array spans [0, 1] linearly.
    /// If min == max (flat array) every element becomes 0.0.
    pub fn normalize(&mut self) {
        let lo = self.min_value();
        let hi = self.max_value();
        let range = hi - lo;
        if range == 0.0 {
            self.init(0.0);
        } else {
            for v in &mut self.data {
                *v = (*v - lo) / range;
            }
        }
    }

    /// Compute knot multiplicity: given a knot vector (sorted), returns the
    /// number of times `knot` appears (within `tol`). Useful for B-spline
    /// operations that use `TColStd_Array1OfReal` as a knot vector.
    pub fn count_near(&self, target: f64, tol: f64) -> usize {
        self.data.iter().filter(|&&v| (v - target).abs() <= tol).count()
    }

    #[inline]
    fn check_bounds(&self, i: usize) {
        assert!(
            i >= self.lower && i <= self.upper(),
            "TColStd_Array1OfReal: index {i} out of range [{}, {}]",
            self.lower,
            self.upper()
        );
    }
}

// ─── TColStd_Array2OfReal ────────────────────────────────────────────────────

/// A 2-D, fixed-size, **1-based** array of `Standard_Real` (`f64`).
///
/// Mirrors `TColStd_Array2OfReal` from OpenCascade's `TColStd` package.
/// Row and column indices run from their respective lower bounds to upper
/// bounds inclusive. Default construction uses 1-based indexing.
// occt: TColStd_Array2OfReal
#[derive(Clone, Debug)]
pub struct Array2OfReal {
    data: Vec<f64>,
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
}

impl Array2OfReal {
    /// Construct a `rows × cols` 1-based 2-D array, all zeros.
    ///
    /// # Panics
    /// Panics when `rows == 0` or `cols == 0`.
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows > 0, "TColStd_Array2OfReal: rows must be >= 1");
        assert!(cols > 0, "TColStd_Array2OfReal: cols must be >= 1");
        Self {
            data: vec![0.0; rows * cols],
            row_lower: 1,
            row_upper: rows,
            col_lower: 1,
            col_upper: cols,
        }
    }

    /// Construct with explicit bounds: rows `r1..=r2`, cols `c1..=c2`.
    ///
    /// # Panics
    /// Panics when `r2 < r1` or `c2 < c1`.
    pub fn with_bounds(r1: usize, r2: usize, c1: usize, c2: usize) -> Self {
        assert!(r2 >= r1, "TColStd_Array2OfReal: r2 ({r2}) < r1 ({r1})");
        assert!(c2 >= c1, "TColStd_Array2OfReal: c2 ({c2}) < c1 ({c1})");
        let rows = r2 - r1 + 1;
        let cols = c2 - c1 + 1;
        Self {
            data: vec![0.0; rows * cols],
            row_lower: r1,
            row_upper: r2,
            col_lower: c1,
            col_upper: c2,
        }
    }

    #[inline] pub fn row_lower(&self) -> usize { self.row_lower }
    #[inline] pub fn row_upper(&self) -> usize { self.row_upper }
    #[inline] pub fn col_lower(&self) -> usize { self.col_lower }
    #[inline] pub fn col_upper(&self) -> usize { self.col_upper }

    /// Number of rows.
    #[inline]
    pub fn n_rows(&self) -> usize {
        self.row_upper - self.row_lower + 1
    }

    /// Number of columns.
    #[inline]
    pub fn n_cols(&self) -> usize {
        self.col_upper - self.col_lower + 1
    }

    /// Total number of elements.
    #[inline]
    pub fn size(&self) -> usize {
        self.n_rows() * self.n_cols()
    }

    /// Get element at `(row, col)` using their respective lower-bounded indices.
    ///
    /// # Panics
    /// Panics on out-of-range indices.
    #[inline]
    pub fn value(&self, row: usize, col: usize) -> f64 {
        self.check_bounds(row, col);
        self.data[self.flat(row, col)]
    }

    /// Mutable reference to element at `(row, col)`.
    ///
    /// # Panics
    /// Panics on out-of-range indices.
    #[inline]
    pub fn value_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        self.check_bounds(row, col);
        let idx = self.flat(row, col);
        &mut self.data[idx]
    }

    /// Set element at `(row, col)`. Mirrors `SetValue(row, col, v)`.
    ///
    /// # Panics
    /// Panics on out-of-range indices.
    #[inline]
    pub fn set_value(&mut self, row: usize, col: usize, v: f64) {
        self.check_bounds(row, col);
        let idx = self.flat(row, col);
        self.data[idx] = v;
    }

    /// Fill every element with `v`. Mirrors `Init(v)`.
    pub fn init(&mut self, v: f64) {
        for e in &mut self.data {
            *e = v;
        }
    }

    /// Extract a row as a new `Array1OfReal` (1-based).
    ///
    /// # Panics
    /// Panics when `row` is out of range.
    pub fn row(&self, row: usize) -> Array1OfReal {
        assert!(
            row >= self.row_lower && row <= self.row_upper,
            "TColStd_Array2OfReal::row: {row} out of [{}, {}]",
            self.row_lower,
            self.row_upper
        );
        let n = self.n_cols();
        let mut arr = Array1OfReal::new(n);
        for c in self.col_lower..=self.col_upper {
            arr.set_value(c - self.col_lower + 1, self.value(row, c));
        }
        arr
    }

    /// Extract a column as a new `Array1OfReal` (1-based).
    ///
    /// # Panics
    /// Panics when `col` is out of range.
    pub fn col(&self, col: usize) -> Array1OfReal {
        assert!(
            col >= self.col_lower && col <= self.col_upper,
            "TColStd_Array2OfReal::col: {col} out of [{}, {}]",
            self.col_lower,
            self.col_upper
        );
        let n = self.n_rows();
        let mut arr = Array1OfReal::new(n);
        for r in self.row_lower..=self.row_upper {
            arr.set_value(r - self.row_lower + 1, self.value(r, col));
        }
        arr
    }

    /// Iterator over all elements in row-major order.
    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.data.iter()
    }

    /// Compute the minimum value in the matrix (NaN-ignoring).
    ///
    /// # Panics
    /// Panics when the matrix is empty.
    pub fn min_value(&self) -> f64 {
        assert!(!self.data.is_empty(), "TColStd_Array2OfReal::min_value: empty");
        self.data.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    /// Compute the maximum value in the matrix (NaN-ignoring).
    ///
    /// # Panics
    /// Panics when the matrix is empty.
    pub fn max_value(&self) -> f64 {
        assert!(!self.data.is_empty(), "TColStd_Array2OfReal::max_value: empty");
        self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }

    /// Transpose: return a new `Array2OfReal` where rows and columns are
    /// swapped. Result is always 1-based.
    pub fn transposed(&self) -> Self {
        let mut result = Array2OfReal::new(self.n_cols(), self.n_rows());
        for r in self.row_lower..=self.row_upper {
            for c in self.col_lower..=self.col_upper {
                let ri = r - self.row_lower + 1;
                let ci = c - self.col_lower + 1;
                result.set_value(ci, ri, self.value(r, c));
            }
        }
        result
    }

    #[inline]
    fn flat(&self, row: usize, col: usize) -> usize {
        (row - self.row_lower) * self.n_cols() + (col - self.col_lower)
    }

    #[inline]
    fn check_bounds(&self, row: usize, col: usize) {
        assert!(
            row >= self.row_lower && row <= self.row_upper,
            "TColStd_Array2OfReal: row {row} out of [{}, {}]",
            self.row_lower,
            self.row_upper
        );
        assert!(
            col >= self.col_lower && col <= self.col_upper,
            "TColStd_Array2OfReal: col {col} out of [{}, {}]",
            self.col_lower,
            self.col_upper
        );
    }
}

// ─── TColStd_SequenceOfInteger ───────────────────────────────────────────────

/// A dynamic, ordered sequence of `Standard_Integer` (`i32`).
///
/// Mirrors `TColStd_SequenceOfInteger` from the OpenCascade `TColStd` package,
/// itself a specialisation of `NCollection_Sequence<Standard_Integer>`.
/// Unlike arrays this collection can grow or shrink. Indexing is **1-based**.
// occt: TColStd_SequenceOfInteger
#[derive(Clone, Debug, Default)]
pub struct SequenceOfInteger {
    data: Vec<i32>,
}

impl SequenceOfInteger {
    /// Empty sequence.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct from a slice of integers.
    pub fn from_slice(vals: &[i32]) -> Self {
        Self { data: vals.to_vec() }
    }

    /// Number of elements.
    #[inline]
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// True when the sequence is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Append `v` to the end. Mirrors `Append(v)`.
    #[inline]
    pub fn append(&mut self, v: i32) {
        self.data.push(v);
    }

    /// Prepend `v` at the front. Mirrors `Prepend(v)`.
    #[inline]
    pub fn prepend(&mut self, v: i32) {
        self.data.insert(0, v);
    }

    /// Insert `v` before position `i` (1-based). After insertion the element
    /// is accessible at index `i`. Mirrors `InsertBefore(i, v)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[1, length+1]`.
    pub fn insert_before(&mut self, i: usize, v: i32) {
        assert!(
            i >= 1 && i <= self.data.len() + 1,
            "SequenceOfInteger::insert_before: index {i} out of [1, {}]",
            self.data.len() + 1
        );
        self.data.insert(i - 1, v);
    }

    /// Remove element at 1-based index `i`. Mirrors `Remove(i)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[1, length]`.
    pub fn remove(&mut self, i: usize) {
        self.check_bounds(i);
        self.data.remove(i - 1);
    }

    /// Remove all elements. Mirrors `Clear()`.
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Access element at 1-based index `i`. Mirrors `Value(i)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[1, length]`.
    #[inline]
    pub fn value(&self, i: usize) -> i32 {
        self.check_bounds(i);
        self.data[i - 1]
    }

    /// Set element at 1-based index `i`. Mirrors `SetValue(i, v)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[1, length]`.
    #[inline]
    pub fn set_value(&mut self, i: usize, v: i32) {
        self.check_bounds(i);
        self.data[i - 1] = v;
    }

    /// Access the first element. Mirrors `First()`.
    ///
    /// # Panics
    /// Panics on an empty sequence.
    #[inline]
    pub fn first(&self) -> i32 {
        assert!(!self.data.is_empty(), "SequenceOfInteger::first: empty sequence");
        self.data[0]
    }

    /// Access the last element. Mirrors `Last()`.
    ///
    /// # Panics
    /// Panics on an empty sequence.
    #[inline]
    pub fn last(&self) -> i32 {
        assert!(!self.data.is_empty(), "SequenceOfInteger::last: empty sequence");
        *self.data.last().unwrap()
    }

    /// Iterator over elements in order.
    pub fn iter(&self) -> impl Iterator<Item = &i32> {
        self.data.iter()
    }

    /// Reverse the sequence in-place.
    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    /// Return true if the sequence contains `v`.
    pub fn contains(&self, v: i32) -> bool {
        self.data.contains(&v)
    }

    /// Compute the sum of all elements.
    pub fn sum(&self) -> i64 {
        self.data.iter().map(|&v| v as i64).sum()
    }

    /// Minimum value.
    ///
    /// # Panics
    /// Panics on an empty sequence.
    pub fn min_value(&self) -> i32 {
        assert!(!self.data.is_empty(), "SequenceOfInteger::min_value: empty");
        *self.data.iter().min().unwrap()
    }

    /// Maximum value.
    ///
    /// # Panics
    /// Panics on an empty sequence.
    pub fn max_value(&self) -> i32 {
        assert!(!self.data.is_empty(), "SequenceOfInteger::max_value: empty");
        *self.data.iter().max().unwrap()
    }

    /// Sort elements in ascending order in place.
    pub fn sort(&mut self) {
        self.data.sort_unstable();
    }

    /// Remove duplicate values, keeping the first occurrence and preserving
    /// relative order of remaining elements. Mirrors a common usage pattern
    /// for index lists in OCCT.
    pub fn unique(&mut self) {
        let mut seen = std::collections::HashSet::new();
        self.data.retain(|v| seen.insert(*v));
    }

    #[inline]
    fn check_bounds(&self, i: usize) {
        assert!(
            i >= 1 && i <= self.data.len(),
            "SequenceOfInteger: index {i} out of [1, {}]",
            self.data.len()
        );
    }
}

// ─── TColStd_SequenceOfReal ──────────────────────────────────────────────────

/// A dynamic, ordered sequence of `Standard_Real` (`f64`).
///
/// Mirrors `TColStd_SequenceOfReal` from the OpenCascade `TColStd` package,
/// itself a specialisation of `NCollection_Sequence<Standard_Real>`.
/// Indexing is **1-based** to match OCCT.
// occt: TColStd_SequenceOfReal
#[derive(Clone, Debug, Default)]
pub struct SequenceOfReal {
    data: Vec<f64>,
}

impl SequenceOfReal {
    /// Empty sequence.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct from a slice of reals.
    pub fn from_slice(vals: &[f64]) -> Self {
        Self { data: vals.to_vec() }
    }

    /// Number of elements.
    #[inline]
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// True when the sequence is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Append `v` to the end. Mirrors `Append(v)`.
    #[inline]
    pub fn append(&mut self, v: f64) {
        self.data.push(v);
    }

    /// Prepend `v` at the front. Mirrors `Prepend(v)`.
    #[inline]
    pub fn prepend(&mut self, v: f64) {
        self.data.insert(0, v);
    }

    /// Insert `v` before position `i` (1-based). Mirrors `InsertBefore(i, v)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[1, length+1]`.
    pub fn insert_before(&mut self, i: usize, v: f64) {
        assert!(
            i >= 1 && i <= self.data.len() + 1,
            "SequenceOfReal::insert_before: index {i} out of [1, {}]",
            self.data.len() + 1
        );
        self.data.insert(i - 1, v);
    }

    /// Remove element at 1-based index `i`. Mirrors `Remove(i)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[1, length]`.
    pub fn remove(&mut self, i: usize) {
        self.check_bounds(i);
        self.data.remove(i - 1);
    }

    /// Remove all elements. Mirrors `Clear()`.
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Access element at 1-based index `i`. Mirrors `Value(i)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[1, length]`.
    #[inline]
    pub fn value(&self, i: usize) -> f64 {
        self.check_bounds(i);
        self.data[i - 1]
    }

    /// Set element at 1-based index `i`. Mirrors `SetValue(i, v)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[1, length]`.
    #[inline]
    pub fn set_value(&mut self, i: usize, v: f64) {
        self.check_bounds(i);
        self.data[i - 1] = v;
    }

    /// Access the first element. Mirrors `First()`.
    ///
    /// # Panics
    /// Panics on an empty sequence.
    #[inline]
    pub fn first(&self) -> f64 {
        assert!(!self.data.is_empty(), "SequenceOfReal::first: empty sequence");
        self.data[0]
    }

    /// Access the last element. Mirrors `Last()`.
    ///
    /// # Panics
    /// Panics on an empty sequence.
    #[inline]
    pub fn last(&self) -> f64 {
        assert!(!self.data.is_empty(), "SequenceOfReal::last: empty sequence");
        *self.data.last().unwrap()
    }

    /// Iterator over elements in order.
    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.data.iter()
    }

    /// Reverse the sequence in-place.
    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    /// Compute the sum of all elements.
    pub fn sum(&self) -> f64 {
        self.data.iter().sum()
    }

    /// Compute the arithmetic mean.
    ///
    /// # Panics
    /// Panics on an empty sequence.
    pub fn mean(&self) -> f64 {
        assert!(!self.data.is_empty(), "SequenceOfReal::mean: empty sequence");
        self.sum() / self.data.len() as f64
    }

    /// Minimum value (NaN-ignoring).
    ///
    /// # Panics
    /// Panics on an empty sequence.
    pub fn min_value(&self) -> f64 {
        assert!(!self.data.is_empty(), "SequenceOfReal::min_value: empty");
        self.data.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    /// Maximum value (NaN-ignoring).
    ///
    /// # Panics
    /// Panics on an empty sequence.
    pub fn max_value(&self) -> f64 {
        assert!(!self.data.is_empty(), "SequenceOfReal::max_value: empty");
        self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }

    /// Sort elements in ascending order in place (NaN-safe: NaNs go last).
    pub fn sort(&mut self) {
        self.data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater));
    }

    /// Compute differences between consecutive elements: `d[i] = v[i+1] - v[i]`.
    /// Returns an empty `SequenceOfReal` if the sequence has fewer than 2 elements.
    /// Useful for computing knot spans from a knot vector.
    pub fn differences(&self) -> Self {
        if self.data.len() < 2 {
            return Self::new();
        }
        let diffs: Vec<f64> = self.data.windows(2).map(|w| w[1] - w[0]).collect();
        Self { data: diffs }
    }

    /// Compute the L2 norm (Euclidean norm) treating the sequence as a vector.
    pub fn norm(&self) -> f64 {
        self.data.iter().map(|&v| v * v).sum::<f64>().sqrt()
    }

    /// Dot product with another sequence of the same length.
    ///
    /// # Panics
    /// Panics when lengths differ.
    pub fn dot(&self, other: &Self) -> f64 {
        assert_eq!(
            self.data.len(),
            other.data.len(),
            "SequenceOfReal::dot: length mismatch {} vs {}",
            self.data.len(),
            other.data.len()
        );
        self.data.iter().zip(other.data.iter()).map(|(&a, &b)| a * b).sum()
    }

    #[inline]
    fn check_bounds(&self, i: usize) {
        assert!(
            i >= 1 && i <= self.data.len(),
            "SequenceOfReal: index {i} out of [1, {}]",
            self.data.len()
        );
    }
}

// ─── Internal unit tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-12;

    fn near(a: f64, b: f64) {
        assert!((a - b).abs() <= TOL, "expected {a} ≈ {b} (diff {})", (a - b).abs());
    }

    // ── Array1OfInteger ──────────────────────────────────────────────────────

    #[test]
    fn array1_int_basic() {
        let mut arr = Array1OfInteger::new(5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 5);

        for i in 1..=5 {
            arr.set_value(i, i as i32 * 10);
        }
        assert_eq!(arr.value(1), 10);
        assert_eq!(arr.value(3), 30);
        assert_eq!(arr.value(5), 50);
    }

    #[test]
    fn array1_int_with_bounds() {
        let mut arr = Array1OfInteger::with_bounds(3, 7);
        assert_eq!(arr.lower(), 3);
        assert_eq!(arr.upper(), 7);
        assert_eq!(arr.length(), 5);
        arr.set_value(5, 99);
        assert_eq!(arr.value(5), 99);
    }

    #[test]
    fn array1_int_init() {
        let mut arr = Array1OfInteger::new(4);
        arr.init(42);
        for i in 1..=4 {
            assert_eq!(arr.value(i), 42);
        }
    }

    #[test]
    fn array1_int_min_max_sum() {
        let mut arr = Array1OfInteger::new(5);
        arr.set_value(1, -3);
        arr.set_value(2, 7);
        arr.set_value(3, 1);
        arr.set_value(4, -10);
        arr.set_value(5, 4);
        assert_eq!(arr.min_value(), -10);
        assert_eq!(arr.max_value(), 7);
        assert_eq!(arr.sum(), -1);
    }

    #[test]
    fn array1_int_sort() {
        let mut arr = Array1OfInteger::new(4);
        arr.set_value(1, 4);
        arr.set_value(2, 1);
        arr.set_value(3, 3);
        arr.set_value(4, 2);
        arr.sort();
        assert_eq!(arr.value(1), 1);
        assert_eq!(arr.value(2), 2);
        assert_eq!(arr.value(3), 3);
        assert_eq!(arr.value(4), 4);
    }

    #[test]
    fn array1_int_contains() {
        let mut arr = Array1OfInteger::new(3);
        arr.set_value(1, 10);
        arr.set_value(2, 20);
        arr.set_value(3, 30);
        assert!(arr.contains(20));
        assert!(!arr.contains(15));
    }

    #[test]
    #[should_panic]
    fn array1_int_out_of_bounds() {
        let arr = Array1OfInteger::new(3);
        let _ = arr.value(4);
    }

    // ── Array1OfReal ─────────────────────────────────────────────────────────

    #[test]
    fn array1_real_basic() {
        let mut arr = Array1OfReal::new(3);
        arr.set_value(1, 1.5);
        arr.set_value(2, 2.5);
        arr.set_value(3, 3.5);
        near(arr.value(2), 2.5);
    }

    #[test]
    fn array1_real_from_slice() {
        let arr = Array1OfReal::from_slice(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(arr.length(), 4);
        near(arr.value(3), 3.0);
    }

    #[test]
    fn array1_real_min_max_mean() {
        let arr = Array1OfReal::from_slice(&[3.0, 1.0, 4.0, 1.0, 5.0]);
        near(arr.min_value(), 1.0);
        near(arr.max_value(), 5.0);
        near(arr.mean(), 14.0 / 5.0);
    }

    #[test]
    fn array1_real_normalize() {
        let mut arr = Array1OfReal::from_slice(&[0.0, 5.0, 10.0]);
        arr.normalize();
        near(arr.value(1), 0.0);
        near(arr.value(2), 0.5);
        near(arr.value(3), 1.0);
    }

    #[test]
    fn array1_real_count_near_knot_vector() {
        // B-spline knot vector: {0, 0, 0, 1, 2, 2, 3, 3, 3}
        let arr = Array1OfReal::from_slice(&[0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 3.0, 3.0, 3.0]);
        assert_eq!(arr.count_near(0.0, 1e-9), 3);
        assert_eq!(arr.count_near(2.0, 1e-9), 2);
        assert_eq!(arr.count_near(3.0, 1e-9), 3);
        assert_eq!(arr.count_near(1.0, 1e-9), 1);
    }

    #[test]
    fn array1_real_sort() {
        let mut arr = Array1OfReal::from_slice(&[3.14, 1.41, 2.72, 0.58]);
        arr.sort();
        assert!(arr.value(1) <= arr.value(2));
        assert!(arr.value(2) <= arr.value(3));
        assert!(arr.value(3) <= arr.value(4));
    }

    // ── Array2OfReal ─────────────────────────────────────────────────────────

    #[test]
    fn array2_real_basic() {
        let mut arr = Array2OfReal::new(3, 4);
        assert_eq!(arr.n_rows(), 3);
        assert_eq!(arr.n_cols(), 4);
        assert_eq!(arr.size(), 12);

        arr.set_value(2, 3, 7.5);
        near(arr.value(2, 3), 7.5);
        near(arr.value(1, 1), 0.0);
    }

    #[test]
    fn array2_real_row_col_extraction() {
        let mut arr = Array2OfReal::new(3, 3);
        // Identity-like: diagonal = 1
        arr.set_value(1, 1, 1.0);
        arr.set_value(2, 2, 1.0);
        arr.set_value(3, 3, 1.0);

        let row2 = arr.row(2);
        near(row2.value(1), 0.0);
        near(row2.value(2), 1.0);
        near(row2.value(3), 0.0);

        let col3 = arr.col(3);
        near(col3.value(1), 0.0);
        near(col3.value(2), 0.0);
        near(col3.value(3), 1.0);
    }

    #[test]
    fn array2_real_transpose() {
        let mut arr = Array2OfReal::new(2, 3);
        arr.set_value(1, 1, 1.0); arr.set_value(1, 2, 2.0); arr.set_value(1, 3, 3.0);
        arr.set_value(2, 1, 4.0); arr.set_value(2, 2, 5.0); arr.set_value(2, 3, 6.0);

        let t = arr.transposed();
        assert_eq!(t.n_rows(), 3);
        assert_eq!(t.n_cols(), 2);
        near(t.value(1, 2), 4.0); // was arr[2,1]
        near(t.value(3, 1), 3.0); // was arr[1,3]
    }

    #[test]
    fn array2_real_min_max() {
        let mut arr = Array2OfReal::new(2, 2);
        arr.set_value(1, 1, -5.0);
        arr.set_value(1, 2, 3.0);
        arr.set_value(2, 1, 0.0);
        arr.set_value(2, 2, 8.0);
        near(arr.min_value(), -5.0);
        near(arr.max_value(), 8.0);
    }

    // ── SequenceOfInteger ────────────────────────────────────────────────────

    #[test]
    fn seq_int_append_prepend() {
        let mut seq = SequenceOfInteger::new();
        seq.append(2);
        seq.append(3);
        seq.prepend(1);
        assert_eq!(seq.length(), 3);
        assert_eq!(seq.first(), 1);
        assert_eq!(seq.last(), 3);
        assert_eq!(seq.value(2), 2);
    }

    #[test]
    fn seq_int_insert_remove() {
        let mut seq = SequenceOfInteger::from_slice(&[1, 3, 5]);
        seq.insert_before(2, 2);
        assert_eq!(seq.length(), 4);
        assert_eq!(seq.value(2), 2);
        seq.remove(3);
        assert_eq!(seq.length(), 3);
        assert_eq!(seq.value(3), 5);
    }

    #[test]
    fn seq_int_min_max_sum() {
        let seq = SequenceOfInteger::from_slice(&[4, 1, 7, 2, 9, 3]);
        assert_eq!(seq.min_value(), 1);
        assert_eq!(seq.max_value(), 9);
        assert_eq!(seq.sum(), 26);
    }

    #[test]
    fn seq_int_sort_and_unique() {
        let mut seq = SequenceOfInteger::from_slice(&[3, 1, 2, 1, 3, 4]);
        seq.unique();
        assert_eq!(seq.length(), 4); // 3, 1, 2, 4 (first occurrences, stable)
        assert!(seq.contains(3) && seq.contains(1) && seq.contains(2) && seq.contains(4));
        seq.sort();
        assert_eq!(seq.value(1), 1);
        assert_eq!(seq.value(4), 4);
    }

    // ── SequenceOfReal ───────────────────────────────────────────────────────

    #[test]
    fn seq_real_append_and_access() {
        let mut seq = SequenceOfReal::new();
        seq.append(1.0);
        seq.append(2.0);
        seq.append(3.0);
        near(seq.value(2), 2.0);
        near(seq.first(), 1.0);
        near(seq.last(), 3.0);
    }

    #[test]
    fn seq_real_differences() {
        let seq = SequenceOfReal::from_slice(&[0.0, 1.0, 3.0, 6.0]);
        let diff = seq.differences();
        assert_eq!(diff.length(), 3);
        near(diff.value(1), 1.0);
        near(diff.value(2), 2.0);
        near(diff.value(3), 3.0);
    }

    #[test]
    fn seq_real_norm_and_dot() {
        let a = SequenceOfReal::from_slice(&[3.0, 4.0]);
        near(a.norm(), 5.0);

        let b = SequenceOfReal::from_slice(&[1.0, 2.0]);
        near(a.dot(&b), 11.0); // 3*1 + 4*2
    }

    #[test]
    fn seq_real_mean() {
        let seq = SequenceOfReal::from_slice(&[2.0, 4.0, 6.0, 8.0]);
        near(seq.mean(), 5.0);
    }

    #[test]
    fn seq_real_reverse() {
        let mut seq = SequenceOfReal::from_slice(&[1.0, 2.0, 3.0]);
        seq.reverse();
        near(seq.value(1), 3.0);
        near(seq.value(3), 1.0);
    }

    #[test]
    fn seq_real_sort() {
        let mut seq = SequenceOfReal::from_slice(&[3.14, 1.41, 2.72]);
        seq.sort();
        assert!(seq.value(1) <= seq.value(2));
        assert!(seq.value(2) <= seq.value(3));
    }
}
