//! `gp_GTrsf2d` -- general 2D transformation (affine, non-orthogonal).
//!
//! This is a from-scratch reimplementation of OpenCascade's `gp_GTrsf2d`
//! (package `TKMath`, header `gp/gp_GTrsf2d.hxx`).
//!
//! A `GTrsf2d` represents the most general (possibly non-orthogonal,
//! non-uniform-scale) affine map in 2D:
//!
//! ```text
//!   | x' |   | a11  a12 | | x |   | tx |
//!   | y' | = | a21  a22 | | y | + | ty |
//! ```
//!
//! The 2×2 matrix `M` is the *vectorial part*; `(tx, ty)` is the
//! *translation part*.
//!
//! The transformation *form* classifies whether `M` has any special
//! structure (identity, scale, rotation, …), exactly as OCCT's
//! `gp_TrsfForm` enum.
//!
//! ## OCCT semantics mirrored
//! * `SetAffinity` — axial symmetry (reflection + scale along an axis).
//! * `SetValue` / `Value` — direct coefficient access (1-based).
//! * `TranslationPart` — returns the translation column.
//! * `VectorialPart` — returns the 2×2 matrix.
//! * `IsNegative` — true when `det(M) < 0`.
//! * `Form` — returns the detected [`TrsfForm`].
//! * `Transforms` — applies the map in-place to `(x, y)`.
//! * `Inverted` — returns the inverse transform.
//! * `Multiplied` — right-composition `self * other`.
//! * `Power` — integer power (via repeated composition or inversion).

use crate::gp2d::{Ax2d, Pnt2d};

// ─────────────────────────────────────────────────────────────────────────────
// TrsfForm  (mirrors gp_TrsfForm)
// ─────────────────────────────────────────────────────────────────────────────

/// Classification of the vectorial part of a [`GTrsf2d`].
///
/// Mirrors `gp_TrsfForm`.  The ordering follows OCCT's enum values so that
/// callers can rely on integer comparisons when needed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// occt: gp_TrsfForm
pub enum TrsfForm {
    /// Identity (M = I, t = 0).
    Identity,
    /// Pure translation (M = I, t ≠ 0).
    Translation,
    /// Rotation (orthogonal, det = +1, no scale).
    Rotation,
    /// Uniform positive scale (M = s·I, s > 0).
    Scale,
    /// Compound rotation + uniform scale (positive).
    CompoundTrsf,
    /// Axial symmetry / reflection or negative-det isometry.
    PntMirror,
    /// General affine (non-orthogonal or non-uniform scale).
    Other,
    /// Negative scaling (det < 0, uniform |scale|).
    NegativeUniformScale,
}

// ─────────────────────────────────────────────────────────────────────────────
// GTrsf2d
// ─────────────────────────────────────────────────────────────────────────────

/// General 2D affine transformation.
///
/// Stores the 2×2 vectorial matrix and a 2-component translation column.
/// Coefficient layout (1-based, as in OCCT):
///
/// ```text
///   row 1: m[1][1]  m[1][2]   translation: t[1]
///   row 2: m[2][1]  m[2][2]   translation: t[2]
/// ```
///
/// Internal storage is 0-based: `mat[row][col]`.
// occt: gp_GTrsf2d
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GTrsf2d {
    /// 2×2 vectorial part, stored as `mat[row][col]` (0-based).
    mat: [[f64; 2]; 2],
    /// Translation part.
    loc: [f64; 2],
    /// Cached form classification.
    form: TrsfForm,
}

// ─────────────── helpers ───────────────

/// Determinant of a 2×2 matrix stored as `m[row][col]` (0-based).
#[inline]
fn det2(m: &[[f64; 2]; 2]) -> f64 {
    m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

/// Multiply two 2×2 matrices: result = a * b.
#[inline]
fn mul2(a: &[[f64; 2]; 2], b: &[[f64; 2]; 2]) -> [[f64; 2]; 2] {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1],
        ],
    ]
}

/// Apply a 2×2 matrix to a 2-vector.
#[inline]
fn apply_mat(m: &[[f64; 2]; 2], v: [f64; 2]) -> [f64; 2] {
    [
        m[0][0] * v[0] + m[0][1] * v[1],
        m[1][0] * v[0] + m[1][1] * v[1],
    ]
}

/// Tolerance for form detection: same as OCCT's `gp::Resolution()`.
const RESOLUTION: f64 = 1.0e-15;

impl Default for GTrsf2d {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl GTrsf2d {
    // ───────────────────────── constructors ──────────────────────────

    /// `gp_GTrsf2d()` — identity transform.
    #[inline]
    pub const fn identity() -> Self {
        Self {
            mat: [[1.0, 0.0], [0.0, 1.0]],
            loc: [0.0, 0.0],
            form: TrsfForm::Identity,
        }
    }

    // ───────────────────────── SetAffinity ───────────────────────────

    /// `SetAffinity(A, Ratio)` — axial affinity along axis `A` with ratio
    /// `Ratio`.
    ///
    /// An *axial affinity* is the transformation that:
    /// * leaves every point on the axis `A` fixed,
    /// * multiplies the perpendicular component by `Ratio`.
    ///
    /// When `|Ratio| < Resolution` the transform collapses to projection onto
    /// the axis (det = 0), which is still representable as an `Other` form.
    ///
    /// OCCT formula (gp_GTrsf2d.cxx):
    ///   Let `d` be the unit direction of `A`, `p` the location of `A`.
    ///   For a point `Q`:
    ///     Q' = p + (d.(Q-p)).d + Ratio.(d_perp.(Q-p)).d_perp
    ///
    ///   which gives M = I + (Ratio-1)*d_perp.d_perp^T and t = (1-Ratio)*(d_perp.p)*d_perp.
    pub fn set_affinity(&mut self, axis: Ax2d, ratio: f64) {
        // Unit direction and its perpendicular.
        let d = axis.direction; // already normalised in Ax2d::new
        let px = axis.location.x;
        let py = axis.location.y;

        // Perpendicular unit direction (rotated +90°).
        let nx = -d.y;
        let ny = d.x;

        // M = I + (ratio - 1) * (n ⊗ n)
        let k = ratio - 1.0;
        self.mat[0][0] = 1.0 + k * nx * nx;
        self.mat[0][1] = k * nx * ny;
        self.mat[1][0] = k * ny * nx;
        self.mat[1][1] = 1.0 + k * ny * ny;

        // t = p - M * p  (so that every point on the axis maps to itself)
        let mp = apply_mat(&self.mat, [px, py]);
        self.loc[0] = px - mp[0];
        self.loc[1] = py - mp[1];

        self.form = TrsfForm::Other;
        self.update_form();
    }

    // ───────────────────────── SetValue ──────────────────────────────

    /// `SetValue(Row, Col, Value)` — sets coefficient (Row, Col) of the full
    /// 2×3 augmented matrix.
    ///
    /// Columns 1 and 2 address the vectorial part; column 3 addresses the
    /// translation.  Row and Col are **1-based**.
    ///
    /// Panics (matching OCCT's `Standard_OutOfRange`) if indices are out of
    /// `1..=2` (rows) / `1..=3` (cols).
    pub fn set_value(&mut self, row: usize, col: usize, value: f64) {
        assert!(
            (1..=2).contains(&row) && (1..=3).contains(&col),
            "gp_GTrsf2d::SetValue: index out of range (row={row}, col={col})"
        );
        if col <= 2 {
            self.mat[row - 1][col - 1] = value;
        } else {
            self.loc[row - 1] = value;
        }
        self.form = TrsfForm::Other;
        self.update_form();
    }

    // ───────────────────────── Value ─────────────────────────────────

    /// `Value(Row, Col)` — returns coefficient (Row, Col) of the 2×3 matrix.
    ///
    /// Row ∈ 1..=2, Col ∈ 1..=3.  Panics on out-of-range.
    #[inline]
    pub fn value(&self, row: usize, col: usize) -> f64 {
        assert!(
            (1..=2).contains(&row) && (1..=3).contains(&col),
            "gp_GTrsf2d::Value: index out of range (row={row}, col={col})"
        );
        if col <= 2 {
            self.mat[row - 1][col - 1]
        } else {
            self.loc[row - 1]
        }
    }

    // ───────────────────────── TranslationPart ───────────────────────

    /// `TranslationPart()` — returns the translation vector (tx, ty).
    #[inline]
    pub fn translation_part(&self) -> Pnt2d {
        Pnt2d::new(self.loc[0], self.loc[1])
    }

    // ───────────────────────── VectorialPart ─────────────────────────

    /// `VectorialPart()` — returns the 2×2 vectorial matrix as a row-major
    /// `[[f64; 2]; 2]` with 0-based indexing.
    ///
    /// The caller can index it as `m[row][col]` (0-based).
    #[inline]
    pub fn vectorial_part(&self) -> [[f64; 2]; 2] {
        self.mat
    }

    // ───────────────────────── IsNegative ────────────────────────────

    /// `IsNegative()` — true when `det(VectorialPart) < 0`.
    #[inline]
    pub fn is_negative(&self) -> bool {
        det2(&self.mat) < 0.0
    }

    // ───────────────────────── Form ──────────────────────────────────

    /// `Form()` — cached classification of the vectorial part.
    #[inline]
    pub fn form(&self) -> TrsfForm {
        self.form
    }

    // ───────────────────────── Transforms ────────────────────────────

    /// `Transforms(x, y)` — applies this transformation in-place to `(x, y)`.
    ///
    /// `(x', y') = M * (x, y) + t`
    #[inline]
    pub fn transforms(&self, x: &mut f64, y: &mut f64) {
        let xv = *x;
        let yv = *y;
        *x = self.mat[0][0] * xv + self.mat[0][1] * yv + self.loc[0];
        *y = self.mat[1][0] * xv + self.mat[1][1] * yv + self.loc[1];
    }

    /// Convenience: applies this transformation to a [`Pnt2d`].
    #[inline]
    pub fn transform_pnt(&self, p: Pnt2d) -> Pnt2d {
        let mut x = p.x;
        let mut y = p.y;
        self.transforms(&mut x, &mut y);
        Pnt2d::new(x, y)
    }

    // ───────────────────────── Inverted ──────────────────────────────

    /// `Inverted()` — returns the inverse transformation.
    ///
    /// Panics (matching OCCT's `Standard_ConstructionError`) when the
    /// vectorial part is singular (`|det| <= Resolution`).
    pub fn inverted(&self) -> Self {
        let d = det2(&self.mat);
        assert!(
            d.abs() > RESOLUTION,
            "gp_GTrsf2d::Inverted: singular matrix (det = {d})"
        );
        let inv_d = 1.0 / d;
        // Inverse of [[a,b],[c,d]] = (1/det) * [[d,-b],[-c,a]]
        let inv_mat = [
            [self.mat[1][1] * inv_d, -self.mat[0][1] * inv_d],
            [-self.mat[1][0] * inv_d, self.mat[0][0] * inv_d],
        ];
        // Inverse translation: -M^{-1} * t
        let inv_loc = apply_mat(&inv_mat, [self.loc[0], self.loc[1]]);
        let mut result = Self {
            mat: inv_mat,
            loc: [-inv_loc[0], -inv_loc[1]],
            form: TrsfForm::Other,
        };
        result.update_form();
        result
    }

    // ───────────────────────── Multiplied ────────────────────────────

    /// `Multiplied(other)` — right-composes: returns `self * other`.
    ///
    /// `(self ∘ other)(x) = self.M * (other.M * x + other.t) + self.t`
    ///                     `= (self.M * other.M) * x + (self.M * other.t + self.t)`
    pub fn multiplied(&self, other: &Self) -> Self {
        let new_mat = mul2(&self.mat, &other.mat);
        let transformed_t = apply_mat(&self.mat, other.loc);
        let new_loc = [
            transformed_t[0] + self.loc[0],
            transformed_t[1] + self.loc[1],
        ];
        let mut result = Self {
            mat: new_mat,
            loc: new_loc,
            form: TrsfForm::Other,
        };
        result.update_form();
        result
    }

    /// `Multiply(other)` — in-place right-composition: `self = self * other`.
    #[inline]
    pub fn multiply(&mut self, other: &Self) {
        *self = self.multiplied(other);
    }

    /// `PreMultiply(other)` — in-place left-composition: `self = other * self`.
    #[inline]
    pub fn pre_multiply(&mut self, other: &Self) {
        *self = other.multiplied(self);
    }

    // ───────────────────────── Power ─────────────────────────────────

    /// `Power(n)` — returns `self` raised to the integer power `n`.
    ///
    /// * `n = 0` → identity.
    /// * `n > 0` → repeated composition.
    /// * `n < 0` → composition of the inverse `|n|` times.
    ///
    /// Panics if `n < 0` and the transform is singular.
    pub fn power(&self, n: i32) -> Self {
        if n == 0 {
            return Self::identity();
        }
        let (base, count) = if n > 0 {
            (*self, n as u32)
        } else {
            (self.inverted(), (-n) as u32)
        };
        // Exponentiation by squaring.
        let mut result = Self::identity();
        let mut cur = base;
        let mut remaining = count;
        while remaining > 0 {
            if remaining & 1 == 1 {
                result = result.multiplied(&cur);
            }
            remaining >>= 1;
            if remaining > 0 {
                cur = cur.multiplied(&cur);
            }
        }
        result
    }

    // ───────────────────────── internal helpers ───────────────────────

    /// Re-classify the form from the current `mat` and `loc` values.
    fn update_form(&mut self) {
        let a = self.mat[0][0];
        let b = self.mat[0][1];
        let c = self.mat[1][0];
        let d = self.mat[1][1];
        let tx = self.loc[0];
        let ty = self.loc[1];

        let has_trans = tx.abs() > RESOLUTION || ty.abs() > RESOLUTION;

        // Is M the identity?
        let is_id_mat = (a - 1.0).abs() <= RESOLUTION
            && b.abs() <= RESOLUTION
            && c.abs() <= RESOLUTION
            && (d - 1.0).abs() <= RESOLUTION;

        if is_id_mat {
            self.form = if has_trans {
                TrsfForm::Translation
            } else {
                TrsfForm::Identity
            };
            return;
        }

        // Is M = s * I  (uniform scale)?
        let is_uniform = (a - d).abs() <= RESOLUTION && b.abs() <= RESOLUTION && c.abs() <= RESOLUTION;
        if is_uniform {
            // s is the diagonal value a == d
            let s = a;
            if s > 0.0 {
                if !has_trans {
                    self.form = TrsfForm::Scale;
                } else {
                    self.form = TrsfForm::CompoundTrsf;
                }
            } else {
                // negative uniform scale
                self.form = TrsfForm::NegativeUniformScale;
            }
            return;
        }

        // Is M orthogonal with det = +1 (rotation)?
        // M^T * M == I  and det == +1
        let det = a * d - b * c;
        let mtm00 = a * a + c * c;
        let mtm01 = a * b + c * d;
        let mtm11 = b * b + d * d;
        let is_ortho_pos = (mtm00 - 1.0).abs() <= RESOLUTION
            && mtm01.abs() <= RESOLUTION
            && (mtm11 - 1.0).abs() <= RESOLUTION
            && (det - 1.0).abs() <= RESOLUTION;

        if is_ortho_pos {
            self.form = if has_trans {
                TrsfForm::CompoundTrsf
            } else {
                TrsfForm::Rotation
            };
            return;
        }

        // Is M orthogonal with det = -1 (reflection / point mirror)?
        let is_ortho_neg = (mtm00 - 1.0).abs() <= RESOLUTION
            && mtm01.abs() <= RESOLUTION
            && (mtm11 - 1.0).abs() <= RESOLUTION
            && (det + 1.0).abs() <= RESOLUTION;

        if is_ortho_neg {
            self.form = TrsfForm::PntMirror;
            return;
        }

        self.form = TrsfForm::Other;
    }
}
