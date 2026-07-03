//! `math_Vector` -- dense real vector with 1-based (or arbitrary) inclusive
//! index bounds, mirroring OpenCascade's `math_Vector` from package `TKMath`.
//!
//! From the OCCT documentation:
//!
//! > This class implements the real-number vector. The range of the index
//! > is user defined. An exception is raised if accessing out of range indices.
//!
//! Constructor(`lower`, `upper`, `init`) initialises every element to `init`.
//! All arithmetic operations (`Add`, `Subtract`, `Multiply`, `Dot`, etc.) are
//! provided in both in-place and value-returning variants, exactly as in OCCT.

use std::fmt;

// occt: math_Vector
/// A dense real vector with an arbitrary inclusive index range `[lower, upper]`.
///
/// Indices are 1-based by convention (matching OCCT), but any `i32` lower/upper
/// pair is accepted. Elements are stored contiguously; access is O(1).
#[derive(Clone, Debug, PartialEq)]
pub struct MathVector {
    lower: i32,
    data: Vec<f64>,
}

impl MathVector {
    // -----------------------------------------------------------------------
    // Constructors
    // -----------------------------------------------------------------------

    /// `math_Vector(Lower, Upper, InitialValue)` — creates a vector whose
    /// index range is `[lower, upper]` with every element set to `init`.
    ///
    /// # Panics
    /// Panics if `upper < lower`.
    pub fn new(lower: i32, upper: i32, init: f64) -> Self {
        assert!(
            upper >= lower,
            "math_Vector: upper ({upper}) < lower ({lower})"
        );
        let len = (upper - lower + 1) as usize;
        Self {
            lower,
            data: vec![init; len],
        }
    }

    /// Convenience constructor that initialises every element to `0.0`.
    pub fn zero(lower: i32, upper: i32) -> Self {
        Self::new(lower, upper, 0.0)
    }

    /// Build from an existing slice, using the given lower bound.
    pub fn from_slice(lower: i32, values: &[f64]) -> Self {
        assert!(!values.is_empty(), "math_Vector::from_slice: empty slice");
        Self {
            lower,
            data: values.to_vec(),
        }
    }

    // -----------------------------------------------------------------------
    // Bounds / size
    // -----------------------------------------------------------------------

    /// `math_Vector::Lower()` — the inclusive lower index bound.
    #[inline]
    pub fn lower(&self) -> i32 {
        self.lower
    }

    /// `math_Vector::Upper()` — the inclusive upper index bound.
    #[inline]
    pub fn upper(&self) -> i32 {
        self.lower + self.data.len() as i32 - 1
    }

    /// `math_Vector::Length()` — number of elements.
    #[inline]
    pub fn length(&self) -> i32 {
        self.data.len() as i32
    }

    // -----------------------------------------------------------------------
    // Element access
    // -----------------------------------------------------------------------

    /// `math_Vector::Value(i)` — read element at index `i`.
    ///
    /// # Panics
    /// Panics if `i` is outside `[lower, upper]`.
    #[inline]
    pub fn value(&self, i: i32) -> f64 {
        let idx = self.check_index(i);
        self.data[idx]
    }

    /// `math_Vector::SetValue(i, val)` — write element at index `i`.
    ///
    /// # Panics
    /// Panics if `i` is outside `[lower, upper]`.
    #[inline]
    pub fn set_value(&mut self, i: i32, val: f64) {
        let idx = self.check_index(i);
        self.data[idx] = val;
    }

    /// Alias for [`value`](Self::value) — mirrors the OCCT `Value` method.
    #[inline]
    pub fn get(&self, i: i32) -> f64 {
        self.value(i)
    }

    /// Alias for [`set_value`](Self::set_value) — mirrors the OCCT `SetValue`.
    #[inline]
    pub fn set(&mut self, i: i32, val: f64) {
        self.set_value(i, val);
    }

    #[inline]
    fn check_index(&self, i: i32) -> usize {
        assert!(
            i >= self.lower && i <= self.upper(),
            "math_Vector: index {i} out of range [{}, {}]",
            self.lower,
            self.upper()
        );
        (i - self.lower) as usize
    }

    // -----------------------------------------------------------------------
    // Initialise
    // -----------------------------------------------------------------------

    /// `math_Vector::Init(v)` — set all elements to `v`.
    pub fn initialized(&mut self, v: f64) {
        for x in &mut self.data {
            *x = v;
        }
    }

    /// Functional variant: returns a new vector equal to `self` with all
    /// elements set to `v`.
    pub fn initialized_copy(&self, v: f64) -> Self {
        Self {
            lower: self.lower,
            data: vec![v; self.data.len()],
        }
    }

    // -----------------------------------------------------------------------
    // Norms
    // -----------------------------------------------------------------------

    /// `math_Vector::Norm()` — Euclidean (L2) norm: sqrt(sum(xᵢ²)).
    pub fn norm(&self) -> f64 {
        self.norm2().sqrt()
    }

    /// `math_Vector::Norm2()` — squared Euclidean norm: sum(xᵢ²).
    pub fn norm2(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum()
    }

    /// `math_Vector::Max()` — maximum element value.
    ///
    /// # Panics
    /// Panics if the vector is empty (impossible via the constructor, but
    /// defensive).
    pub fn max(&self) -> f64 {
        self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }

    /// `math_Vector::Min()` — minimum element value.
    pub fn min(&self) -> f64 {
        self.data.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    // -----------------------------------------------------------------------
    // Dot product
    // -----------------------------------------------------------------------

    /// `math_Vector::Dot(right)` — inner product `self · right`.
    ///
    /// # Panics
    /// Panics if the two vectors have different lengths.
    pub fn dot(&self, right: &Self) -> f64 {
        self.check_same_length(right, "Dot");
        self.data
            .iter()
            .zip(right.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    // -----------------------------------------------------------------------
    // In-place arithmetic
    // -----------------------------------------------------------------------

    /// `math_Vector::Add(right)` — in-place element-wise addition: `self += right`.
    ///
    /// # Panics
    /// Panics if lengths differ.
    pub fn add(&mut self, right: &Self) {
        self.check_same_length(right, "Add");
        for (a, b) in self.data.iter_mut().zip(right.data.iter()) {
            *a += b;
        }
    }

    /// `math_Vector::Subtract(right)` — in-place element-wise subtraction:
    /// `self -= right`.
    ///
    /// # Panics
    /// Panics if lengths differ.
    pub fn subtract(&mut self, right: &Self) {
        self.check_same_length(right, "Subtract");
        for (a, b) in self.data.iter_mut().zip(right.data.iter()) {
            *a -= b;
        }
    }

    /// `math_Vector::Multiply(scalar)` — in-place scalar multiplication:
    /// `self *= scalar`.
    pub fn multiply(&mut self, scalar: f64) {
        for x in &mut self.data {
            *x *= scalar;
        }
    }

    /// `math_Vector::Divide(scalar)` — in-place scalar division:
    /// `self /= scalar`.
    ///
    /// # Panics
    /// Panics if `scalar == 0.0`.
    pub fn divide(&mut self, scalar: f64) {
        assert!(scalar != 0.0, "math_Vector::Divide: division by zero");
        for x in &mut self.data {
            *x /= scalar;
        }
    }

    // -----------------------------------------------------------------------
    // Value-returning arithmetic
    // -----------------------------------------------------------------------

    /// `math_Vector::Added(right)` — returns `self + right` as a new vector.
    ///
    /// # Panics
    /// Panics if lengths differ.
    pub fn added(&self, right: &Self) -> Self {
        self.check_same_length(right, "Added");
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(right.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Self {
            lower: self.lower,
            data,
        }
    }

    /// `math_Vector::Subtracted(right)` — returns `self - right` as a new vector.
    ///
    /// # Panics
    /// Panics if lengths differ.
    pub fn subtracted(&self, right: &Self) -> Self {
        self.check_same_length(right, "Subtracted");
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(right.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        Self {
            lower: self.lower,
            data,
        }
    }

    /// `math_Vector::Multiplied(scalar)` — returns `self * scalar` as a new vector.
    pub fn multiplied(&self, scalar: f64) -> Self {
        let data: Vec<f64> = self.data.iter().map(|x| x * scalar).collect();
        Self {
            lower: self.lower,
            data,
        }
    }

    /// `math_Vector::Divided(scalar)` — returns `self / scalar` as a new vector.
    ///
    /// # Panics
    /// Panics if `scalar == 0.0`.
    pub fn divided(&self, scalar: f64) -> Self {
        assert!(scalar != 0.0, "math_Vector::Divided: division by zero");
        let data: Vec<f64> = self.data.iter().map(|x| x / scalar).collect();
        Self {
            lower: self.lower,
            data,
        }
    }

    /// `math_Vector::Opposite()` — returns `-self` as a new vector.
    pub fn opposite(&self) -> Self {
        let data: Vec<f64> = self.data.iter().map(|x| -x).collect();
        Self {
            lower: self.lower,
            data,
        }
    }

    // -----------------------------------------------------------------------
    // Slice / sub-vector
    // -----------------------------------------------------------------------

    /// `math_Vector::Set(i1, i2, right)` — copy elements of `right` into
    /// `self[i1..=i2]`. `right` must have length `i2 - i1 + 1`.
    ///
    /// # Panics
    /// Panics if ranges are incompatible.
    pub fn set_range(&mut self, i1: i32, i2: i32, right: &Self) {
        assert!(
            i1 >= self.lower && i2 <= self.upper() && i1 <= i2,
            "math_Vector::Set: range [{i1},{i2}] invalid for [{},{}]",
            self.lower,
            self.upper()
        );
        assert!(
            right.length() == i2 - i1 + 1,
            "math_Vector::Set: length mismatch"
        );
        for (offset, j) in (i1..=i2).enumerate() {
            self.data[(j - self.lower) as usize] = right.data[offset];
        }
    }

    /// `math_Vector::Slice(i1, i2)` — returns a sub-vector `self[i1..=i2]`
    /// with lower bound set to `i1`.
    ///
    /// # Panics
    /// Panics if `[i1, i2]` is not inside `[lower, upper]`.
    pub fn slice(&self, i1: i32, i2: i32) -> Self {
        assert!(
            i1 >= self.lower && i2 <= self.upper() && i1 <= i2,
            "math_Vector::Slice: range [{i1},{i2}] invalid for [{},{}]",
            self.lower,
            self.upper()
        );
        let data: Vec<f64> = self.data
            [(i1 - self.lower) as usize..=(i2 - self.lower) as usize]
            .to_vec();
        Self { lower: i1, data }
    }

    // -----------------------------------------------------------------------
    // Dump
    // -----------------------------------------------------------------------

    /// `math_Vector::Dump(Standard_OStream& o)` — writes a human-readable
    /// representation of the vector to a `String`.
    pub fn dump(&self) -> String {
        let mut s = format!(
            "math_Vector of Lower = {} and Upper = {}\n",
            self.lower,
            self.upper()
        );
        for i in self.lower..=self.upper() {
            s.push_str(&format!(
                "  ({i}) = {}\n",
                self.data[(i - self.lower) as usize]
            ));
        }
        s
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    #[inline]
    fn check_same_length(&self, other: &Self, op: &str) {
        assert!(
            self.data.len() == other.data.len(),
            "math_Vector::{op}: length mismatch ({} vs {})",
            self.data.len(),
            other.data.len()
        );
    }
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

impl fmt::Display for MathVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dump())
    }
}

// ---------------------------------------------------------------------------
// Operator overloads (for ergonomics matching OCCT's C++ operators)
// ---------------------------------------------------------------------------

impl std::ops::Add for &MathVector {
    type Output = MathVector;
    fn add(self, rhs: Self) -> MathVector {
        self.added(rhs)
    }
}

impl std::ops::Sub for &MathVector {
    type Output = MathVector;
    fn sub(self, rhs: Self) -> MathVector {
        self.subtracted(rhs)
    }
}

impl std::ops::Mul<f64> for &MathVector {
    type Output = MathVector;
    fn mul(self, rhs: f64) -> MathVector {
        self.multiplied(rhs)
    }
}

impl std::ops::Neg for &MathVector {
    type Output = MathVector;
    fn neg(self) -> MathVector {
        self.opposite()
    }
}

impl std::ops::AddAssign<&MathVector> for MathVector {
    fn add_assign(&mut self, rhs: &MathVector) {
        self.add(rhs);
    }
}

impl std::ops::SubAssign<&MathVector> for MathVector {
    fn sub_assign(&mut self, rhs: &MathVector) {
        self.subtract(rhs);
    }
}

impl std::ops::MulAssign<f64> for MathVector {
    fn mul_assign(&mut self, rhs: f64) {
        self.multiply(rhs);
    }
}
