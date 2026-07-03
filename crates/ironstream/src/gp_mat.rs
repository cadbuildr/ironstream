//! `gp_Mat` / `gp_Mat2d` — 3×3 and 2×2 matrices of `f64`, mirroring
//! OpenCascade's `gp_Mat` and `gp_Mat2d`.
//!
//! This is a faithful, from-scratch reimplementation of OCCT's
//! `src/FoundationClasses/TKMath/gp/gp_Mat.hxx` and
//! `src/FoundationClasses/TKMath/gp/gp_Mat2d.hxx`.
//!
//! Conventions matching OCCT:
//! - Indices `(row, col)` are **1-based** in [`Mat::value`] / [`Mat::set_value`]
//!   / [`Mat::change_value`], exactly like `gp_Mat::Value` / `operator()`.
//! - Internally the coefficients live in a `[[f64; 3]; 3]` laid out as
//!   `m[row][col]` with 0-based storage, just like OCCT's `myMat`.
//! - `gp_XYZ` (rows / columns / cross-and-dot references) maps to the existing
//!   kernel [`Pnt`], which already serves as OCCT's `gp_XYZ`/`gp_Vec`/`gp_Dir`.
//!
//! Singularity / division-by-zero use OCCT's `gp::Resolution()` (`1e-15`).

use crate::gp::Pnt;

/// `gp::Resolution()` — the smallest non-zero modulus OCCT treats as non-null.
/// Used by [`Mat3::is_singular`], [`Mat3::invert`] and rotation helpers.
pub const RESOLUTION: f64 = 1.0e-15;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Error raised by fallible `gp_Mat` / `gp_Mat2d` operations.
///
/// Mirrors OCCT's `Standard_ConstructionError` (singular inversion / divide by
/// zero / null rotation axis). Index-out-of-range mirrors
/// `Standard_OutOfRange`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatError {
    /// The matrix is singular and cannot be inverted, or a divisor is ~0.
    Construction,
    /// A 1-based row/column index was outside `1..=3`.
    OutOfRange,
}

impl std::fmt::Display for MatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatError::Construction => {
                write!(f, "gp_Mat: construction error (singular / divide by 0)")
            }
            MatError::OutOfRange => write!(f, "gp_Mat: index out of range (must be 1..=3)"),
        }
    }
}

impl std::error::Error for MatError {}

// ---------------------------------------------------------------------------
// Mat3 — 3×3 matrix (occt: gp_Mat)
// ---------------------------------------------------------------------------

/// A three-column, three-row matrix of `f64`.
// occt: gp_Mat
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3 {
    /// Row-major storage: `m[row][col]`, 0-based, mirroring OCCT's `myMat`.
    pub m: [[f64; 3]; 3],
}

impl Default for Mat3 {
    /// `gp_Mat()` — a matrix with null coefficients.
    #[inline]
    fn default() -> Self {
        Mat3 { m: [[0.0; 3]; 3] }
    }
}

impl Mat3 {
    /// `gp_Mat()` — creates a 3×3 null (zero) matrix.
    #[inline]
    pub fn new() -> Self {
        Mat3 { m: [[0.0; 3]; 3] }
    }

    /// Creates a matrix from a row-major 3×3 array.
    #[inline]
    pub fn from_array(m: [[f64; 3]; 3]) -> Self {
        Mat3 { m }
    }

    /// Returns the 3×3 identity matrix.
    #[inline]
    pub fn identity() -> Self {
        Mat3 {
            m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    /// Returns the 3×3 zero matrix.
    #[inline]
    pub fn zero() -> Self {
        Mat3 { m: [[0.0; 3]; 3] }
    }

    /// Returns the coefficient at 0-based `(i, j)`.
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.m[i][j]
    }

    /// Sets the coefficient at 0-based `(i, j)` to `v`.
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, v: f64) {
        self.m[i][j] = v;
    }

    /// Returns `self * other` (matrix multiplication).
    pub fn mul_mat(&self, other: &Mat3) -> Mat3 {
        let a = &self.m;
        let b = &other.m;
        let mut r = [[0.0f64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                r[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
            }
        }
        Mat3 { m: r }
    }

    /// Returns `self * v` (matrix-vector product).
    pub fn mul_vec(&self, v: [f64; 3]) -> [f64; 3] {
        let a = &self.m;
        [
            a[0][0] * v[0] + a[0][1] * v[1] + a[0][2] * v[2],
            a[1][0] * v[0] + a[1][1] * v[1] + a[1][2] * v[2],
            a[2][0] * v[0] + a[2][1] * v[1] + a[2][2] * v[2],
        ]
    }

    /// Returns the transpose of this matrix.
    pub fn transpose(&self) -> Mat3 {
        let a = &self.m;
        Mat3 {
            m: [
                [a[0][0], a[1][0], a[2][0]],
                [a[0][1], a[1][1], a[2][1]],
                [a[0][2], a[1][2], a[2][2]],
            ],
        }
    }

    /// OCCT-style alias for `transpose()`.
    #[inline]
    pub fn transposed(&self) -> Mat3 {
        self.transpose()
    }

    /// Returns the determinant of this matrix.
    #[inline]
    pub fn determinant(&self) -> f64 {
        let a = &self.m;
        a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
            - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
            + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0])
    }

    /// Returns the inverse of this matrix as an Option, or `None` if singular.
    pub fn invert_option(&self) -> Option<Mat3> {
        let det = self.determinant();
        if det.abs() <= RESOLUTION {
            return None;
        }
        let inv = 1.0 / det;
        let a = &self.m;
        Some(Mat3 {
            m: [
                [
                    (a[1][1] * a[2][2] - a[1][2] * a[2][1]) * inv,
                    (a[0][2] * a[2][1] - a[0][1] * a[2][2]) * inv,
                    (a[0][1] * a[1][2] - a[0][2] * a[1][1]) * inv,
                ],
                [
                    (a[1][2] * a[2][0] - a[1][0] * a[2][2]) * inv,
                    (a[0][0] * a[2][2] - a[0][2] * a[2][0]) * inv,
                    (a[0][2] * a[1][0] - a[0][0] * a[1][2]) * inv,
                ],
                [
                    (a[1][0] * a[2][1] - a[1][1] * a[2][0]) * inv,
                    (a[0][1] * a[2][0] - a[0][0] * a[2][1]) * inv,
                    (a[0][0] * a[1][1] - a[0][1] * a[1][0]) * inv,
                ],
            ],
        })
    }

    // ------------------------------------------------------------------
    // Legacy helpers (full OCCT-style API preserved below)
    // ------------------------------------------------------------------

    /// `gp_Mat(a11..a33)` — creates a matrix from its nine coefficients,
    /// given in row order.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn from_coeffs(
        a11: f64, a12: f64, a13: f64,
        a21: f64, a22: f64, a23: f64,
        a31: f64, a32: f64, a33: f64,
    ) -> Self {
        Mat3 {
            m: [[a11, a12, a13], [a21, a22, a23], [a31, a32, a33]],
        }
    }

    /// `gp_Mat(theCol1, theCol2, theCol3)` — creates a matrix whose three
    /// columns are the given number triples.
    #[inline]
    pub fn from_cols(col1: Pnt, col2: Pnt, col3: Pnt) -> Self {
        let mut me = Mat3::zero();
        me.set_cols(col1, col2, col3);
        me
    }

    /// `SetCol` — assigns the three coordinates of `value` to column `col`.
    ///
    /// `col` is 1-based; out-of-range yields [`MatError::OutOfRange`].
    pub fn set_col(&mut self, col: i32, value: Pnt) -> Result<(), MatError> {
        match col {
            1 => { self.m[0][0] = value.x; self.m[1][0] = value.y; self.m[2][0] = value.z; }
            2 => { self.m[0][1] = value.x; self.m[1][1] = value.y; self.m[2][1] = value.z; }
            3 => { self.m[0][2] = value.x; self.m[1][2] = value.y; self.m[2][2] = value.z; }
            _ => return Err(MatError::OutOfRange),
        }
        Ok(())
    }

    /// `SetCols` — assigns the three number triples to the three columns.
    pub fn set_cols(&mut self, col1: Pnt, col2: Pnt, col3: Pnt) {
        self.m[0][0] = col1.x; self.m[1][0] = col1.y; self.m[2][0] = col1.z;
        self.m[0][1] = col2.x; self.m[1][1] = col2.y; self.m[2][1] = col2.z;
        self.m[0][2] = col3.x; self.m[1][2] = col3.y; self.m[2][2] = col3.z;
    }

    /// `SetRow` — assigns the three coordinates of `value` to row `row`.
    ///
    /// `row` is 1-based; out-of-range yields [`MatError::OutOfRange`].
    pub fn set_row(&mut self, row: i32, value: Pnt) -> Result<(), MatError> {
        match row {
            1 => { self.m[0][0] = value.x; self.m[0][1] = value.y; self.m[0][2] = value.z; }
            2 => { self.m[1][0] = value.x; self.m[1][1] = value.y; self.m[1][2] = value.z; }
            3 => { self.m[2][0] = value.x; self.m[2][1] = value.y; self.m[2][2] = value.z; }
            _ => return Err(MatError::OutOfRange),
        }
        Ok(())
    }

    /// `SetRows` — assigns the three number triples to the three rows.
    pub fn set_rows(&mut self, row1: Pnt, row2: Pnt, row3: Pnt) {
        self.m[0] = [row1.x, row1.y, row1.z];
        self.m[1] = [row2.x, row2.y, row2.z];
        self.m[2] = [row3.x, row3.y, row3.z];
    }

    /// `SetCross` — modifies this matrix so that `M * {X,Y,Z}t == ref.Cross({X,Y,Z})`.
    pub fn set_cross(&mut self, r: Pnt) {
        self.m[0][0] = 0.0; self.m[1][1] = 0.0; self.m[2][2] = 0.0;
        self.m[0][1] = -r.z; self.m[0][2] = r.y;
        self.m[1][0] =  r.z; self.m[1][2] = -r.x;
        self.m[2][0] = -r.y; self.m[2][1] = r.x;
    }

    /// `SetDiagonal` — modifies the main diagonal; other coefficients untouched.
    #[inline]
    pub fn set_diagonal(&mut self, x1: f64, x2: f64, x3: f64) {
        self.m[0][0] = x1; self.m[1][1] = x2; self.m[2][2] = x3;
    }

    /// `SetDot` — outer product `r ⊗ r`.
    pub fn set_dot(&mut self, r: Pnt) {
        self.m[0][0] = r.x * r.x; self.m[1][1] = r.y * r.y; self.m[2][2] = r.z * r.z;
        self.m[0][1] = r.x * r.y; self.m[0][2] = r.x * r.z; self.m[1][2] = r.y * r.z;
        self.m[1][0] = self.m[0][1]; self.m[2][0] = self.m[0][2]; self.m[2][1] = self.m[1][2];
    }

    /// `SetIdentity` — modifies this matrix to be the identity.
    #[inline]
    pub fn set_identity(&mut self) {
        *self = Mat3::identity();
    }

    /// `SetRotation` — modifies this matrix to represent a rotation of
    /// `ang` radians about `axis`.
    ///
    /// Raises [`MatError::Construction`] if the axis is too short.
    pub fn set_rotation(&mut self, axis: Pnt, ang: f64) -> Result<(), MatError> {
        let modulus = axis.norm();
        if modulus <= RESOLUTION {
            return Err(MatError::Construction);
        }
        let inv = 1.0 / modulus;
        let (x, y, z) = (axis.x * inv, axis.y * inv, axis.z * inv);
        let cosa = ang.cos();
        let sina = ang.sin();
        let omc = 1.0 - cosa;
        self.m[0][0] = x * x * omc + cosa;
        self.m[0][1] = -z * sina + omc * x * y;
        self.m[0][2] =  y * sina + omc * x * z;
        self.m[1][0] =  z * sina + omc * x * y;
        self.m[1][1] = y * y * omc + cosa;
        self.m[1][2] = -x * sina + omc * y * z;
        self.m[2][0] = -y * sina + omc * x * z;
        self.m[2][1] =  x * sina + omc * y * z;
        self.m[2][2] = z * z * omc + cosa;
        Ok(())
    }

    /// `SetScale` — modifies this matrix to represent a uniform scale by `s`.
    #[inline]
    pub fn set_scale(&mut self, s: f64) {
        *self = Mat3 {
            m: [[s, 0.0, 0.0], [0.0, s, 0.0], [0.0, 0.0, s]],
        };
    }

    /// `SetValue(row, col, value)` — assigns `value` to coefficient (row, col).
    ///
    /// 1-based indices; out-of-range yields [`MatError::OutOfRange`].
    pub fn set_value(&mut self, row: i32, col: i32, value: f64) -> Result<(), MatError> {
        if !(1..=3).contains(&row) || !(1..=3).contains(&col) {
            return Err(MatError::OutOfRange);
        }
        self.m[(row - 1) as usize][(col - 1) as usize] = value;
        Ok(())
    }

    /// `Value(row, col)` / `operator()` — coefficient at 1-based (row, col).
    pub fn value(&self, row: i32, col: i32) -> Result<f64, MatError> {
        if !(1..=3).contains(&row) || !(1..=3).contains(&col) {
            return Err(MatError::OutOfRange);
        }
        Ok(self.m[(row - 1) as usize][(col - 1) as usize])
    }

    /// `ChangeValue(row, col)` — mutable reference to coefficient (row, col).
    pub fn change_value(&mut self, row: i32, col: i32) -> Result<&mut f64, MatError> {
        if !(1..=3).contains(&row) || !(1..=3).contains(&col) {
            return Err(MatError::OutOfRange);
        }
        Ok(&mut self.m[(row - 1) as usize][(col - 1) as usize])
    }

    /// `Column(col)` — returns column `col` (1-based) as a `Pnt`.
    pub fn column(&self, col: i32) -> Result<Pnt, MatError> {
        match col {
            1 => Ok(Pnt::new(self.m[0][0], self.m[1][0], self.m[2][0])),
            2 => Ok(Pnt::new(self.m[0][1], self.m[1][1], self.m[2][1])),
            3 => Ok(Pnt::new(self.m[0][2], self.m[1][2], self.m[2][2])),
            _ => Err(MatError::OutOfRange),
        }
    }

    /// `Row(row)` — returns row `row` (1-based) as a `Pnt`.
    pub fn row(&self, row: i32) -> Result<Pnt, MatError> {
        match row {
            1 => Ok(Pnt::new(self.m[0][0], self.m[0][1], self.m[0][2])),
            2 => Ok(Pnt::new(self.m[1][0], self.m[1][1], self.m[1][2])),
            3 => Ok(Pnt::new(self.m[2][0], self.m[2][1], self.m[2][2])),
            _ => Err(MatError::OutOfRange),
        }
    }

    /// `Diagonal` — returns the main diagonal as a `Pnt`.
    #[inline]
    pub fn diagonal(&self) -> Pnt {
        Pnt::new(self.m[0][0], self.m[1][1], self.m[2][2])
    }

    /// `IsSingular` — true if `|det| <= gp::Resolution()`.
    #[inline]
    pub fn is_singular(&self) -> bool {
        self.determinant().abs() <= RESOLUTION
    }

    /// Legacy alias with explicit tolerance — use `is_singular()` for OCCT default.
    #[inline]
    pub fn is_singular_tol(&self, tol: f64) -> bool {
        self.determinant().abs() <= tol
    }

    /// `Add` — in place: `this(i,j) += other(i,j)`.
    pub fn add(&mut self, other: &Mat3) {
        for i in 0..3 { for j in 0..3 { self.m[i][j] += other.m[i][j]; } }
    }

    /// `Added` — `this + other`, as a new matrix.
    #[inline]
    pub fn added(&self, other: &Mat3) -> Mat3 {
        let mut me = *self; me.add(other); me
    }

    /// `Subtract` — in place: `this(i,j) -= other(i,j)`.
    pub fn subtract(&mut self, other: &Mat3) {
        for i in 0..3 { for j in 0..3 { self.m[i][j] -= other.m[i][j]; } }
    }

    /// `Subtracted` — `this - other`, as a new matrix.
    #[inline]
    pub fn subtracted(&self, other: &Mat3) -> Mat3 {
        let mut me = *self; me.subtract(other); me
    }

    /// `Multiply(other)` — in place `this = this * other`.
    pub fn multiply(&mut self, other: &Mat3) {
        *self = self.mul_mat(other);
    }

    /// `Multiplied(other)` — `this * other`, as a new matrix.
    #[inline]
    pub fn multiplied(&self, other: &Mat3) -> Mat3 {
        self.mul_mat(other)
    }

    /// `PreMultiply(other)` — in place `this = other * this`.
    pub fn pre_multiply(&mut self, other: &Mat3) {
        *self = other.mul_mat(self);
    }

    /// `Multiply(scalar)` — in place: scale every coefficient by `s`.
    pub fn multiply_scalar(&mut self, s: f64) {
        for i in 0..3 { for j in 0..3 { self.m[i][j] *= s; } }
    }

    /// `Multiplied(scalar)` — every coefficient scaled by `s`, as a new matrix.
    #[inline]
    pub fn multiplied_scalar(&self, s: f64) -> Mat3 {
        let mut me = *self; me.multiply_scalar(s); me
    }

    /// `Divide(scalar)` — in place: divide every coefficient by `s`.
    pub fn divide(&mut self, s: f64) -> Result<(), MatError> {
        if s.abs() <= RESOLUTION { return Err(MatError::Construction); }
        let inv = 1.0 / s;
        for i in 0..3 { for j in 0..3 { self.m[i][j] *= inv; } }
        Ok(())
    }

    /// `Divided(scalar)` — every coefficient divided by `s`, as a new matrix.
    #[inline]
    pub fn divided(&self, s: f64) -> Result<Mat3, MatError> {
        let mut me = *self; me.divide(s)?; Ok(me)
    }

    /// `Transpose` — in place: `A(j,i) <- A(i,j)`.
    #[inline]
    pub fn transpose_in_place(&mut self) {
        *self = self.transpose();
    }

    /// `Inverted` — the inverse of this matrix, raising an error if singular.
    /// `Inverted` — the inverse of this matrix, raising an error if singular.
    pub fn inverted(&self) -> Result<Mat3, MatError> {
        self.invert_option().ok_or(MatError::Construction)
    }

    /// `Invert` — in-place inverse; returns `Err` if singular.
    pub fn invert(&mut self) -> Result<(), MatError> {
        *self = self.inverted()?;
        Ok(())
    }

    /// `Power(n)` — in place `this = this^n`.
    pub fn power(&mut self, n: i32) -> Result<(), MatError> {
        *self = self.powered(n)?; Ok(())
    }

    /// `Powered(n)` — `this` raised to the integer power `n`, as a new matrix.
    pub fn powered(&self, n: i32) -> Result<Mat3, MatError> {
        if n == 0 { return Ok(Mat3::identity()); }
        if n == 1 { return Ok(*self); }
        if n == -1 { return self.inverted(); }
        let mut npower = n;
        let mut base = *self;
        if n < 0 { base = self.inverted()?; npower = -n; }
        let mut result = Mat3::identity();
        while npower > 0 {
            if npower & 1 == 1 { result.multiply(&base); }
            npower >>= 1;
            if npower > 0 { let sq = base; base.multiply(&sq); }
        }
        Ok(result)
    }
}

// Keep the old type alias so existing code continues to compile.
/// Legacy alias — prefer [`Mat3`].
// occt: gp_Mat
pub type Mat = Mat3;

// ---------------------------------------------------------------------------
// Mat2 — 2×2 matrix (occt: gp_Mat2d)
// ---------------------------------------------------------------------------

/// A two-column, two-row matrix of `f64`.
// occt: gp_Mat2d
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat2 {
    /// Row-major storage: `m[row][col]`, 0-based.
    pub m: [[f64; 2]; 2],
}

impl Default for Mat2 {
    #[inline]
    fn default() -> Self {
        Mat2 { m: [[0.0; 2]; 2] }
    }
}

impl Mat2 {
    /// Creates a 2×2 matrix from four coefficients in row order:
    /// `[[a, b], [c, d]]`.
    #[inline]
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Mat2 { m: [[a, b], [c, d]] }
    }

    /// Returns the 2×2 identity matrix.
    #[inline]
    pub fn identity() -> Self {
        Mat2 { m: [[1.0, 0.0], [0.0, 1.0]] }
    }

    /// Returns the determinant `a*d - b*c`.
    #[inline]
    pub fn determinant(&self) -> f64 {
        self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0]
    }

    /// Returns the inverse of this matrix, or `None` if the matrix is singular.
    pub fn invert(&self) -> Option<Mat2> {
        let det = self.determinant();
        if det.abs() <= RESOLUTION {
            return None;
        }
        let inv = 1.0 / det;
        Some(Mat2 {
            m: [
                [ self.m[1][1] * inv, -self.m[0][1] * inv],
                [-self.m[1][0] * inv,  self.m[0][0] * inv],
            ],
        })
    }

    /// Returns `self * v` (matrix-vector product).
    #[inline]
    pub fn mul_vec(&self, v: [f64; 2]) -> [f64; 2] {
        [
            self.m[0][0] * v[0] + self.m[0][1] * v[1],
            self.m[1][0] * v[0] + self.m[1][1] * v[1],
        ]
    }

    // ------------------------------------------------------------------
    // Convenience helpers
    // ------------------------------------------------------------------

    /// Returns the zero matrix.
    #[inline]
    pub fn zero() -> Self {
        Mat2 { m: [[0.0; 2]; 2] }
    }

    /// Returns the coefficient at 0-based `(i, j)`.
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.m[i][j]
    }

    /// Sets the coefficient at 0-based `(i, j)` to `v`.
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, v: f64) {
        self.m[i][j] = v;
    }

    /// Returns the transpose of this matrix.
    #[inline]
    pub fn transpose(&self) -> Mat2 {
        Mat2 {
            m: [[self.m[0][0], self.m[1][0]], [self.m[0][1], self.m[1][1]]],
        }
    }

    /// Returns `self * other` (matrix multiplication).
    pub fn mul_mat(&self, other: &Mat2) -> Mat2 {
        let a = &self.m;
        let b = &other.m;
        Mat2 {
            m: [
                [a[0][0]*b[0][0] + a[0][1]*b[1][0],  a[0][0]*b[0][1] + a[0][1]*b[1][1]],
                [a[1][0]*b[0][0] + a[1][1]*b[1][0],  a[1][0]*b[0][1] + a[1][1]*b[1][1]],
            ],
        }
    }

    /// Returns `true` if `|det| <= tol`.
    #[inline]
    pub fn is_singular(&self, tol: f64) -> bool {
        self.determinant().abs() <= tol
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Mat3 ---------------------------------------------------------------

    #[test]
    fn mat3_identity_roundtrip() {
        let id = Mat3::identity();
        assert_eq!(id.m[0][0], 1.0);
        assert_eq!(id.m[1][1], 1.0);
        assert_eq!(id.m[2][2], 1.0);
        assert_eq!(id.m[0][1], 0.0);
    }

    #[test]
    fn mat3_zero() {
        let z = Mat3::zero();
        for i in 0..3 { for j in 0..3 { assert_eq!(z.m[i][j], 0.0); } }
    }

    #[test]
    fn mat3_new_from_array() {
        let m = Mat3::from_array([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert_eq!(m.get(1, 2), 6.0);
    }

    #[test]
    fn mat3_set_get() {
        let mut m = Mat3::zero();
        m.set(0, 2, 42.0);
        assert_eq!(m.get(0, 2), 42.0);
    }

    #[test]
    fn mat3_mul_mat_identity() {
        let a = Mat3::from_coeffs(1.0,2.0,3.0, 4.0,5.0,6.0, 7.0,8.0,9.0);
        let result = a.mul_mat(&Mat3::identity());
        assert_eq!(result.m, a.m);
    }

    #[test]
    fn mat3_mul_vec() {
        let id = Mat3::identity();
        let v = [1.0, 2.0, 3.0];
        assert_eq!(id.mul_vec(v), v);
    }

    #[test]
    fn mat3_transpose() {
        let a = Mat3::from_coeffs(1.0,2.0,3.0, 4.0,5.0,6.0, 7.0,8.0,9.0);
        let t = a.transpose();
        for i in 0..3 { for j in 0..3 { assert_eq!(t.m[i][j], a.m[j][i]); } }
    }

    #[test]
    fn mat3_determinant_identity() {
        assert!((Mat3::identity().determinant() - 1.0).abs() < 1e-14);
    }

    #[test]
    fn mat3_invert_identity() {
        let inv = Mat3::identity().invert_option().unwrap();
        assert_eq!(inv.m, Mat3::identity().m);
    }

    #[test]
    fn mat3_invert_singular_returns_none() {
        let singular = Mat3::zero();
        assert!(singular.invert_option().is_none());
    }

    #[test]
    fn mat3_invert_roundtrip() {
        let a = Mat3::from_coeffs(2.0,1.0,0.0, 0.0,3.0,0.0, 0.0,0.0,4.0);
        let inv = a.invert_option().unwrap();
        let prod = a.mul_mat(&inv);
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((prod.m[i][j] - expected).abs() < 1e-12,
                    "prod[{i}][{j}] = {} expected {expected}", prod.m[i][j]);
            }
        }
    }

    #[test]
    fn mat3_is_singular_zero() {
        assert!(Mat3::zero().is_singular());
    }

    #[test]
    fn mat3_is_singular_identity_false() {
        assert!(!Mat3::identity().is_singular());
    }

    // --- Mat2 ---------------------------------------------------------------

    #[test]
    fn mat2_identity() {
        let id = Mat2::identity();
        assert_eq!(id.m[0][0], 1.0);
        assert_eq!(id.m[1][1], 1.0);
        assert_eq!(id.m[0][1], 0.0);
    }

    #[test]
    fn mat2_determinant() {
        let m = Mat2::new(3.0, 1.0, 2.0, 4.0);
        assert!((m.determinant() - 10.0).abs() < 1e-14);
    }

    #[test]
    fn mat2_invert_roundtrip() {
        let m = Mat2::new(3.0, 1.0, 2.0, 4.0);
        let inv = m.invert().unwrap();
        let prod = m.mul_mat(&inv);
        for i in 0..2 {
            for j in 0..2 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((prod.m[i][j] - expected).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn mat2_invert_singular_returns_none() {
        let m = Mat2::new(1.0, 2.0, 2.0, 4.0);
        assert!(m.invert().is_none());
    }

    #[test]
    fn mat2_mul_vec_identity() {
        let id = Mat2::identity();
        let v = [5.0, 7.0];
        assert_eq!(id.mul_vec(v), v);
    }

    #[test]
    fn mat2_mul_vec() {
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let v = [1.0, 1.0];
        let r = m.mul_vec(v);
        assert!((r[0] - 3.0).abs() < 1e-14);
        assert!((r[1] - 7.0).abs() < 1e-14);
    }
}
