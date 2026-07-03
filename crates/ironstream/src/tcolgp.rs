// FILE: tcolgp.rs
//! `TColgp` — collection types for 3D/2D points and vectors, mirroring
//! OpenCascade's `TColgp` package (TKMath).
//!
//! These collections map closely to `NCollection_Array1`, `NCollection_Array2`,
//! and `NCollection_Sequence` specialised for `gp_Pnt`, `gp_Pnt2d`, and
//! `gp_Vec`. Indexing follows OCCT convention: **1-based** for Array1/Array2,
//! and **1-based** for Sequence. All bounds violations panic with a descriptive
//! message, mirroring `Standard_OutOfRange`.

use crate::gp::{Pnt, Vec3};
use crate::gp2d::Pnt2d;

// ─── TColgp_Array1OfPnt ──────────────────────────────────────────────────────

/// A 1-D, fixed-size, **1-based** array of `gp_Pnt` (3D points).
///
/// Mirrors `TColgp_Array1OfPnt` from OpenCascade's `TColgp` package.
/// The array is indexed from `lower()` to `upper()` inclusive, defaulting
/// to 1-based when constructed with `new(n)`.
// occt: TColgp_Array1OfPnt
#[derive(Clone, Debug)]
pub struct Array1OfPnt {
    data: Vec<Pnt>,
    lower: usize,
}

impl Array1OfPnt {
    /// Construct a 1-based array of `n` elements, all initialised to the
    /// origin. Mirrors `TColgp_Array1OfPnt(1, n)`.
    ///
    /// # Panics
    /// Panics when `n == 0`.
    pub fn new(n: usize) -> Self {
        assert!(n > 0, "TColgp_Array1OfPnt: size must be >= 1");
        Self {
            data: vec![Pnt::origin(); n],
            lower: 1,
        }
    }

    /// Construct an array with an explicit lower bound: elements are indexed
    /// `lower ..= lower + n - 1`. Mirrors `TColgp_Array1OfPnt(lower, upper)`.
    ///
    /// # Panics
    /// Panics when `n == 0`.
    pub fn with_bounds(lower: usize, upper: usize) -> Self {
        assert!(
            upper >= lower,
            "TColgp_Array1OfPnt: upper ({upper}) < lower ({lower})"
        );
        let n = upper - lower + 1;
        Self {
            data: vec![Pnt::origin(); n],
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

    /// True when the array contains no elements (length 0 arrays cannot be
    /// constructed but the predicate is part of the OCCT interface).
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Access element at 1-based index `i`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[lower, upper]`.
    #[inline]
    pub fn value(&self, i: usize) -> Pnt {
        self.check_bounds(i);
        self.data[i - self.lower]
    }

    /// Mutable reference to element at 1-based index `i`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[lower, upper]`.
    #[inline]
    pub fn value_mut(&mut self, i: usize) -> &mut Pnt {
        self.check_bounds(i);
        let idx = i - self.lower;
        &mut self.data[idx]
    }

    /// Set element at 1-based index `i` to `p`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[lower, upper]`.
    #[inline]
    pub fn set_value(&mut self, i: usize, p: Pnt) {
        self.check_bounds(i);
        self.data[i - self.lower] = p;
    }

    /// Fill every element with `p`.
    pub fn fill(&mut self, p: Pnt) {
        for v in &mut self.data {
            *v = p;
        }
    }

    /// Iterator over elements in index order.
    pub fn iter(&self) -> impl Iterator<Item = &Pnt> {
        self.data.iter()
    }

    /// Iterator (index, value) pairs, where index is 1-based.
    pub fn iter_indexed(&self) -> impl Iterator<Item = (usize, Pnt)> + '_ {
        let lower = self.lower;
        self.data
            .iter()
            .enumerate()
            .map(move |(i, &p)| (i + lower, p))
    }

    /// Compute the bounding box of all points (min/max per axis).
    /// Returns `(min, max)` as `Pnt`.
    pub fn bounding_box(&self) -> (Pnt, Pnt) {
        assert!(!self.data.is_empty(), "bounding_box on empty array");
        let mut lo = self.data[0];
        let mut hi = self.data[0];
        for &p in &self.data[1..] {
            lo.x = lo.x.min(p.x);
            lo.y = lo.y.min(p.y);
            lo.z = lo.z.min(p.z);
            hi.x = hi.x.max(p.x);
            hi.y = hi.y.max(p.y);
            hi.z = hi.z.max(p.z);
        }
        (lo, hi)
    }

    /// Compute the centroid (arithmetic mean) of all points.
    pub fn centroid(&self) -> Pnt {
        assert!(!self.data.is_empty(), "centroid of empty array");
        let n = self.data.len() as f64;
        let mut sum = Pnt::origin();
        for &p in &self.data {
            sum.x += p.x;
            sum.y += p.y;
            sum.z += p.z;
        }
        Pnt::new(sum.x / n, sum.y / n, sum.z / n)
    }

    #[inline]
    fn check_bounds(&self, i: usize) {
        assert!(
            i >= self.lower && i <= self.upper(),
            "TColgp_Array1OfPnt: index {i} out of range [{}, {}]",
            self.lower,
            self.upper()
        );
    }
}

// ─── TColgp_Array2OfPnt ──────────────────────────────────────────────────────

/// A 2-D, fixed-size, **1-based** array of `gp_Pnt` (3D points).
///
/// Mirrors `TColgp_Array2OfPnt`. Row and column indices run from their
/// respective lower bounds to upper bounds inclusive. Default construction
/// uses 1-based indexing.
// occt: TColgp_Array2OfPnt
#[derive(Clone, Debug)]
pub struct Array2OfPnt {
    data: Vec<Pnt>,
    row_lower: usize,
    row_upper: usize,
    col_lower: usize,
    col_upper: usize,
}

impl Array2OfPnt {
    /// Construct a `rows × cols` 1-based 2-D array.
    ///
    /// # Panics
    /// Panics when `rows == 0` or `cols == 0`.
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows > 0, "TColgp_Array2OfPnt: rows must be >= 1");
        assert!(cols > 0, "TColgp_Array2OfPnt: cols must be >= 1");
        Self {
            data: vec![Pnt::origin(); rows * cols],
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
        assert!(r2 >= r1, "Array2OfPnt: r2 ({r2}) < r1 ({r1})");
        assert!(c2 >= c1, "Array2OfPnt: c2 ({c2}) < c1 ({c1})");
        let rows = r2 - r1 + 1;
        let cols = c2 - c1 + 1;
        Self {
            data: vec![Pnt::origin(); rows * cols],
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

    /// Get element at (row, col) using their respective lower-bounded indices.
    ///
    /// # Panics
    /// Panics on out-of-range indices.
    #[inline]
    pub fn value(&self, row: usize, col: usize) -> Pnt {
        self.check_bounds(row, col);
        self.data[self.flat(row, col)]
    }

    /// Set element at (row, col).
    ///
    /// # Panics
    /// Panics on out-of-range indices.
    #[inline]
    pub fn set_value(&mut self, row: usize, col: usize, p: Pnt) {
        self.check_bounds(row, col);
        let idx = self.flat(row, col);
        self.data[idx] = p;
    }

    /// Mutable reference to element at (row, col).
    ///
    /// # Panics
    /// Panics on out-of-range indices.
    #[inline]
    pub fn value_mut(&mut self, row: usize, col: usize) -> &mut Pnt {
        self.check_bounds(row, col);
        let idx = self.flat(row, col);
        &mut self.data[idx]
    }

    /// Fill every element with `p`.
    pub fn fill(&mut self, p: Pnt) {
        for v in &mut self.data {
            *v = p;
        }
    }

    /// Extract a row as a new `Array1OfPnt` (1-based).
    ///
    /// # Panics
    /// Panics when `row` is out of range.
    pub fn row(&self, row: usize) -> Array1OfPnt {
        assert!(
            row >= self.row_lower && row <= self.row_upper,
            "Array2OfPnt::row: {row} out of [{}, {}]",
            self.row_lower,
            self.row_upper
        );
        let n = self.n_cols();
        let mut arr = Array1OfPnt::new(n);
        for c in self.col_lower..=self.col_upper {
            arr.set_value(c - self.col_lower + 1, self.value(row, c));
        }
        arr
    }

    /// Extract a column as a new `Array1OfPnt` (1-based).
    ///
    /// # Panics
    /// Panics when `col` is out of range.
    pub fn col(&self, col: usize) -> Array1OfPnt {
        assert!(
            col >= self.col_lower && col <= self.col_upper,
            "Array2OfPnt::col: {col} out of [{}, {}]",
            self.col_lower,
            self.col_upper
        );
        let n = self.n_rows();
        let mut arr = Array1OfPnt::new(n);
        for r in self.row_lower..=self.row_upper {
            arr.set_value(r - self.row_lower + 1, self.value(r, col));
        }
        arr
    }

    /// Iterator over all elements in row-major order.
    pub fn iter(&self) -> impl Iterator<Item = &Pnt> {
        self.data.iter()
    }

    #[inline]
    fn flat(&self, row: usize, col: usize) -> usize {
        (row - self.row_lower) * self.n_cols() + (col - self.col_lower)
    }

    #[inline]
    fn check_bounds(&self, row: usize, col: usize) {
        assert!(
            row >= self.row_lower && row <= self.row_upper,
            "Array2OfPnt: row {row} out of [{}, {}]",
            self.row_lower,
            self.row_upper
        );
        assert!(
            col >= self.col_lower && col <= self.col_upper,
            "Array2OfPnt: col {col} out of [{}, {}]",
            self.col_lower,
            self.col_upper
        );
    }
}

// ─── TColgp_SequenceOfPnt ────────────────────────────────────────────────────

/// A dynamic, ordered sequence of `gp_Pnt` (3D points).
///
/// Mirrors `TColgp_SequenceOfPnt` from the OpenCascade `TColgp` package,
/// itself a specialisation of `NCollection_Sequence<gp_Pnt>`. Unlike arrays
/// this collection can grow or shrink. Indexing is **1-based** to match OCCT.
// occt: TColgp_SequenceOfPnt
#[derive(Clone, Debug, Default)]
pub struct SequenceOfPnt {
    data: Vec<Pnt>,
}

impl SequenceOfPnt {
    /// Empty sequence.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct from a slice of points.
    pub fn from_slice(pts: &[Pnt]) -> Self {
        Self {
            data: pts.to_vec(),
        }
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

    /// Append `p` to the end. Mirrors `Append(p)`.
    #[inline]
    pub fn append(&mut self, p: Pnt) {
        self.data.push(p);
    }

    /// Prepend `p` at the front. Mirrors `Prepend(p)`.
    #[inline]
    pub fn prepend(&mut self, p: Pnt) {
        self.data.insert(0, p);
    }

    /// Insert `p` before position `i` (1-based). After insertion the element
    /// is accessible at index `i`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[1, length+1]`.
    pub fn insert_before(&mut self, i: usize, p: Pnt) {
        assert!(
            i >= 1 && i <= self.data.len() + 1,
            "SequenceOfPnt::insert_before: index {i} out of [1, {}]",
            self.data.len() + 1
        );
        self.data.insert(i - 1, p);
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
    pub fn value(&self, i: usize) -> Pnt {
        self.check_bounds(i);
        self.data[i - 1]
    }

    /// Set element at 1-based index `i`. Mirrors `SetValue(i, p)`.
    ///
    /// # Panics
    /// Panics when `i` is out of `[1, length]`.
    #[inline]
    pub fn set_value(&mut self, i: usize, p: Pnt) {
        self.check_bounds(i);
        self.data[i - 1] = p;
    }

    /// Access the first element. Mirrors `First()`.
    ///
    /// # Panics
    /// Panics on an empty sequence.
    #[inline]
    pub fn first(&self) -> Pnt {
        assert!(!self.data.is_empty(), "SequenceOfPnt::first: empty sequence");
        self.data[0]
    }

    /// Access the last element. Mirrors `Last()`.
    ///
    /// # Panics
    /// Panics on an empty sequence.
    #[inline]
    pub fn last(&self) -> Pnt {
        assert!(!self.data.is_empty(), "SequenceOfPnt::last: empty sequence");
        *self.data.last().unwrap()
    }

    /// Iterator over elements in order.
    pub fn iter(&self) -> impl Iterator<Item = &Pnt> {
        self.data.iter()
    }

    /// Reverse the sequence in-place.
    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    /// Compute the centroid (arithmetic mean) of all points.
    ///
    /// # Panics
    /// Panics on an empty sequence.
    pub fn centroid(&self) -> Pnt {
        assert!(!self.data.is_empty(), "centroid of empty sequence");
        let n = self.data.len() as f64;
        let mut sum = Pnt::origin();
        for &p in &self.data {
            sum.x += p.x;
            sum.y += p.y;
            sum.z += p.z;
        }
        Pnt::new(sum.x / n, sum.y / n, sum.z / n)
    }

    /// Approximate arc-length: sum of chord lengths between consecutive points.
    pub fn arc_length(&self) -> f64 {
        self.data
            .windows(2)
            .map(|w| w[0].distance(w[1]))
            .sum()
    }

    #[inline]
    fn check_bounds(&self, i: usize) {
        assert!(
            i >= 1 && i <= self.data.len(),
            "SequenceOfPnt: index {i} out of [1, {}]",
            self.data.len()
        );
    }
}

// ─── TColgp_Array1OfPnt2d ────────────────────────────────────────────────────

/// A 1-D, fixed-size, **1-based** array of `gp_Pnt2d` (2D points).
///
/// Mirrors `TColgp_Array1OfPnt2d`.
// occt: TColgp_Array1OfPnt2d
#[derive(Clone, Debug)]
pub struct Array1OfPnt2d {
    data: Vec<Pnt2d>,
    lower: usize,
}

impl Array1OfPnt2d {
    /// Construct a 1-based array of `n` 2D points, all initialised to the
    /// origin.
    ///
    /// # Panics
    /// Panics when `n == 0`.
    pub fn new(n: usize) -> Self {
        assert!(n > 0, "TColgp_Array1OfPnt2d: size must be >= 1");
        Self {
            data: vec![Pnt2d::origin(); n],
            lower: 1,
        }
    }

    /// Construct with explicit lower/upper bounds.
    ///
    /// # Panics
    /// Panics when `upper < lower`.
    pub fn with_bounds(lower: usize, upper: usize) -> Self {
        assert!(
            upper >= lower,
            "TColgp_Array1OfPnt2d: upper ({upper}) < lower ({lower})"
        );
        let n = upper - lower + 1;
        Self {
            data: vec![Pnt2d::origin(); n],
            lower,
        }
    }

    #[inline] pub fn lower(&self) -> usize { self.lower }
    #[inline] pub fn upper(&self) -> usize { self.lower + self.data.len() - 1 }
    #[inline] pub fn length(&self) -> usize { self.data.len() }
    #[inline] pub fn is_empty(&self) -> bool { self.data.is_empty() }

    /// Get element at 1-based index `i`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn value(&self, i: usize) -> Pnt2d {
        self.check_bounds(i);
        self.data[i - self.lower]
    }

    /// Set element at 1-based index `i`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn set_value(&mut self, i: usize, p: Pnt2d) {
        self.check_bounds(i);
        self.data[i - self.lower] = p;
    }

    /// Mutable reference to element at 1-based index `i`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn value_mut(&mut self, i: usize) -> &mut Pnt2d {
        self.check_bounds(i);
        let idx = i - self.lower;
        &mut self.data[idx]
    }

    /// Fill every element with `p`.
    pub fn fill(&mut self, p: Pnt2d) {
        for v in &mut self.data {
            *v = p;
        }
    }

    /// Iterator over elements in index order.
    pub fn iter(&self) -> impl Iterator<Item = &Pnt2d> {
        self.data.iter()
    }

    /// Compute the 2-D centroid.
    pub fn centroid(&self) -> Pnt2d {
        assert!(!self.data.is_empty(), "centroid of empty Array1OfPnt2d");
        let n = self.data.len() as f64;
        let mut sx = 0.0_f64;
        let mut sy = 0.0_f64;
        for &p in &self.data {
            sx += p.x;
            sy += p.y;
        }
        Pnt2d::new(sx / n, sy / n)
    }

    /// Compute 2-D bounding box as `(min, max)`.
    pub fn bounding_box(&self) -> (Pnt2d, Pnt2d) {
        assert!(!self.data.is_empty(), "bounding_box on empty Array1OfPnt2d");
        let mut lo = self.data[0];
        let mut hi = self.data[0];
        for &p in &self.data[1..] {
            lo.x = lo.x.min(p.x);
            lo.y = lo.y.min(p.y);
            hi.x = hi.x.max(p.x);
            hi.y = hi.y.max(p.y);
        }
        (lo, hi)
    }

    #[inline]
    fn check_bounds(&self, i: usize) {
        assert!(
            i >= self.lower && i <= self.upper(),
            "TColgp_Array1OfPnt2d: index {i} out of range [{}, {}]",
            self.lower,
            self.upper()
        );
    }
}

// ─── TColgp_Array1OfVec ──────────────────────────────────────────────────────

/// A 1-D, fixed-size, **1-based** array of `gp_Vec` (3D vectors).
///
/// Mirrors `TColgp_Array1OfVec`. Internally `Vec3 = Pnt` so the element
/// type is the same underlying struct; the alias documents the semantic intent.
// occt: TColgp_Array1OfVec
#[derive(Clone, Debug)]
pub struct Array1OfVec {
    data: Vec<Vec3>,
    lower: usize,
}

impl Array1OfVec {
    /// Construct a 1-based array of `n` vectors, all zero.
    ///
    /// # Panics
    /// Panics when `n == 0`.
    pub fn new(n: usize) -> Self {
        assert!(n > 0, "TColgp_Array1OfVec: size must be >= 1");
        Self {
            data: vec![Vec3::origin(); n],
            lower: 1,
        }
    }

    /// Construct with explicit lower/upper bounds.
    ///
    /// # Panics
    /// Panics when `upper < lower`.
    pub fn with_bounds(lower: usize, upper: usize) -> Self {
        assert!(
            upper >= lower,
            "TColgp_Array1OfVec: upper ({upper}) < lower ({lower})"
        );
        let n = upper - lower + 1;
        Self {
            data: vec![Vec3::origin(); n],
            lower,
        }
    }

    #[inline] pub fn lower(&self) -> usize { self.lower }
    #[inline] pub fn upper(&self) -> usize { self.lower + self.data.len() - 1 }
    #[inline] pub fn length(&self) -> usize { self.data.len() }
    #[inline] pub fn is_empty(&self) -> bool { self.data.is_empty() }

    /// Get vector at 1-based index `i`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn value(&self, i: usize) -> Vec3 {
        self.check_bounds(i);
        self.data[i - self.lower]
    }

    /// Set vector at 1-based index `i`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn set_value(&mut self, i: usize, v: Vec3) {
        self.check_bounds(i);
        self.data[i - self.lower] = v;
    }

    /// Mutable reference to vector at 1-based index `i`.
    ///
    /// # Panics
    /// Panics on out-of-range index.
    #[inline]
    pub fn value_mut(&mut self, i: usize) -> &mut Vec3 {
        self.check_bounds(i);
        let idx = i - self.lower;
        &mut self.data[idx]
    }

    /// Fill every element with `v`.
    pub fn fill(&mut self, v: Vec3) {
        for e in &mut self.data {
            *e = v;
        }
    }

    /// Iterator over elements in index order.
    pub fn iter(&self) -> impl Iterator<Item = &Vec3> {
        self.data.iter()
    }

    /// Sum all vectors (vector addition).
    pub fn sum(&self) -> Vec3 {
        let mut acc = Vec3::origin();
        for &v in &self.data {
            acc.x += v.x;
            acc.y += v.y;
            acc.z += v.z;
        }
        acc
    }

    /// Compute the L2-norm (magnitude) of each vector and return them.
    pub fn magnitudes(&self) -> Vec<f64> {
        self.data.iter().map(|v| v.norm()).collect()
    }

    /// Normalise every vector in place; zero vectors become `+Z`.
    pub fn normalize_all(&mut self) {
        for v in &mut self.data {
            *v = v.normalized();
        }
    }

    #[inline]
    fn check_bounds(&self, i: usize) {
        assert!(
            i >= self.lower && i <= self.upper(),
            "TColgp_Array1OfVec: index {i} out of range [{}, {}]",
            self.lower,
            self.upper()
        );
    }
}

// ─── Internal unit tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Pnt;
    use crate::gp2d::Pnt2d;

    const TOL: f64 = 1e-12;

    fn near(a: f64, b: f64) {
        assert!((a - b).abs() <= TOL, "expected {a} ≈ {b}");
    }

    fn pnt_near(a: Pnt, b: Pnt) {
        near(a.x, b.x);
        near(a.y, b.y);
        near(a.z, b.z);
    }

    // ── Array1OfPnt ──────────────────────────────────────────────────────────

    #[test]
    fn array1_pnt_basic() {
        let mut arr = Array1OfPnt::new(3);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 3);
        assert_eq!(arr.length(), 3);

        arr.set_value(1, Pnt::new(1.0, 0.0, 0.0));
        arr.set_value(2, Pnt::new(0.0, 1.0, 0.0));
        arr.set_value(3, Pnt::new(0.0, 0.0, 1.0));

        pnt_near(arr.value(1), Pnt::new(1.0, 0.0, 0.0));
        pnt_near(arr.value(2), Pnt::new(0.0, 1.0, 0.0));
        pnt_near(arr.value(3), Pnt::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn array1_pnt_with_bounds() {
        let mut arr = Array1OfPnt::with_bounds(2, 5);
        assert_eq!(arr.lower(), 2);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 4);
        arr.set_value(3, Pnt::new(3.0, 3.0, 3.0));
        pnt_near(arr.value(3), Pnt::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn array1_pnt_fill() {
        let mut arr = Array1OfPnt::new(4);
        let p = Pnt::new(7.0, 8.0, 9.0);
        arr.fill(p);
        for i in 1..=4 {
            pnt_near(arr.value(i), p);
        }
    }

    #[test]
    fn array1_pnt_centroid() {
        let mut arr = Array1OfPnt::new(4);
        arr.set_value(1, Pnt::new(0.0, 0.0, 0.0));
        arr.set_value(2, Pnt::new(2.0, 0.0, 0.0));
        arr.set_value(3, Pnt::new(2.0, 2.0, 0.0));
        arr.set_value(4, Pnt::new(0.0, 2.0, 0.0));
        let c = arr.centroid();
        pnt_near(c, Pnt::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn array1_pnt_bounding_box() {
        let mut arr = Array1OfPnt::new(3);
        arr.set_value(1, Pnt::new(-1.0, 2.0, 0.0));
        arr.set_value(2, Pnt::new(3.0, -4.0, 5.0));
        arr.set_value(3, Pnt::new(0.0, 1.0, 2.0));
        let (lo, hi) = arr.bounding_box();
        near(lo.x, -1.0);
        near(lo.y, -4.0);
        near(lo.z, 0.0);
        near(hi.x, 3.0);
        near(hi.y, 2.0);
        near(hi.z, 5.0);
    }

    #[test]
    #[should_panic]
    fn array1_pnt_out_of_bounds_panics() {
        let arr = Array1OfPnt::new(3);
        let _ = arr.value(4);
    }

    // ── Array2OfPnt ──────────────────────────────────────────────────────────

    #[test]
    fn array2_pnt_basic() {
        let mut arr = Array2OfPnt::new(2, 3);
        assert_eq!(arr.n_rows(), 2);
        assert_eq!(arr.n_cols(), 3);
        assert_eq!(arr.size(), 6);

        arr.set_value(1, 2, Pnt::new(1.0, 2.0, 3.0));
        pnt_near(arr.value(1, 2), Pnt::new(1.0, 2.0, 3.0));
        // Other cells remain at origin
        pnt_near(arr.value(1, 1), Pnt::origin());
    }

    #[test]
    fn array2_pnt_row_extraction() {
        let mut arr = Array2OfPnt::new(2, 3);
        arr.set_value(1, 1, Pnt::new(1.0, 0.0, 0.0));
        arr.set_value(1, 2, Pnt::new(2.0, 0.0, 0.0));
        arr.set_value(1, 3, Pnt::new(3.0, 0.0, 0.0));
        let row = arr.row(1);
        assert_eq!(row.length(), 3);
        near(row.value(1).x, 1.0);
        near(row.value(2).x, 2.0);
        near(row.value(3).x, 3.0);
    }

    #[test]
    fn array2_pnt_col_extraction() {
        let mut arr = Array2OfPnt::new(3, 2);
        arr.set_value(1, 2, Pnt::new(10.0, 0.0, 0.0));
        arr.set_value(2, 2, Pnt::new(20.0, 0.0, 0.0));
        arr.set_value(3, 2, Pnt::new(30.0, 0.0, 0.0));
        let col = arr.col(2);
        assert_eq!(col.length(), 3);
        near(col.value(1).x, 10.0);
        near(col.value(2).x, 20.0);
        near(col.value(3).x, 30.0);
    }

    // ── SequenceOfPnt ────────────────────────────────────────────────────────

    #[test]
    fn sequence_pnt_append_and_access() {
        let mut seq = SequenceOfPnt::new();
        assert!(seq.is_empty());
        seq.append(Pnt::new(1.0, 0.0, 0.0));
        seq.append(Pnt::new(2.0, 0.0, 0.0));
        seq.append(Pnt::new(3.0, 0.0, 0.0));
        assert_eq!(seq.length(), 3);
        near(seq.value(2).x, 2.0);
        near(seq.first().x, 1.0);
        near(seq.last().x, 3.0);
    }

    #[test]
    fn sequence_pnt_insert_before() {
        let mut seq = SequenceOfPnt::new();
        seq.append(Pnt::new(1.0, 0.0, 0.0));
        seq.append(Pnt::new(3.0, 0.0, 0.0));
        seq.insert_before(2, Pnt::new(2.0, 0.0, 0.0));
        assert_eq!(seq.length(), 3);
        near(seq.value(2).x, 2.0);
        near(seq.value(3).x, 3.0);
    }

    #[test]
    fn sequence_pnt_remove() {
        let mut seq = SequenceOfPnt::from_slice(&[
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(3.0, 0.0, 0.0),
        ]);
        seq.remove(2);
        assert_eq!(seq.length(), 2);
        near(seq.value(1).x, 1.0);
        near(seq.value(2).x, 3.0);
    }

    #[test]
    fn sequence_pnt_arc_length() {
        // Unit-spaced points along X — arc length = 4.0
        let seq = SequenceOfPnt::from_slice(&[
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(3.0, 0.0, 0.0),
            Pnt::new(4.0, 0.0, 0.0),
        ]);
        near(seq.arc_length(), 4.0);
    }

    #[test]
    fn sequence_pnt_centroid() {
        let seq = SequenceOfPnt::from_slice(&[
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(4.0, 0.0, 0.0),
            Pnt::new(0.0, 4.0, 0.0),
        ]);
        let c = seq.centroid();
        near(c.x, 4.0 / 3.0);
        near(c.y, 4.0 / 3.0);
        near(c.z, 0.0);
    }

    // ── Array1OfPnt2d ────────────────────────────────────────────────────────

    #[test]
    fn array1_pnt2d_basic() {
        let mut arr = Array1OfPnt2d::new(3);
        arr.set_value(1, Pnt2d::new(1.0, 2.0));
        arr.set_value(2, Pnt2d::new(3.0, 4.0));
        arr.set_value(3, Pnt2d::new(5.0, 6.0));
        assert!((arr.value(2).x - 3.0).abs() < TOL);
        assert!((arr.value(2).y - 4.0).abs() < TOL);
    }

    #[test]
    fn array1_pnt2d_centroid() {
        let mut arr = Array1OfPnt2d::new(4);
        arr.set_value(1, Pnt2d::new(0.0, 0.0));
        arr.set_value(2, Pnt2d::new(4.0, 0.0));
        arr.set_value(3, Pnt2d::new(4.0, 4.0));
        arr.set_value(4, Pnt2d::new(0.0, 4.0));
        let c = arr.centroid();
        assert!((c.x - 2.0).abs() < TOL);
        assert!((c.y - 2.0).abs() < TOL);
    }

    // ── Array1OfVec ──────────────────────────────────────────────────────────

    #[test]
    fn array1_vec_basic() {
        let mut arr = Array1OfVec::new(3);
        arr.set_value(1, Pnt::new(1.0, 0.0, 0.0));
        arr.set_value(2, Pnt::new(0.0, 1.0, 0.0));
        arr.set_value(3, Pnt::new(0.0, 0.0, 1.0));

        let s = arr.sum();
        near(s.x, 1.0);
        near(s.y, 1.0);
        near(s.z, 1.0);
    }

    #[test]
    fn array1_vec_magnitudes() {
        let mut arr = Array1OfVec::new(2);
        arr.set_value(1, Pnt::new(3.0, 4.0, 0.0));
        arr.set_value(2, Pnt::new(1.0, 1.0, 1.0));
        let mags = arr.magnitudes();
        near(mags[0], 5.0);
        near(mags[1], 3.0_f64.sqrt());
    }

    #[test]
    fn array1_vec_normalize_all() {
        let mut arr = Array1OfVec::new(2);
        arr.set_value(1, Pnt::new(3.0, 4.0, 0.0));
        arr.set_value(2, Pnt::new(0.0, 0.0, 5.0));
        arr.normalize_all();
        let mags = arr.magnitudes();
        near(mags[0], 1.0);
        near(mags[1], 1.0);
    }
}
