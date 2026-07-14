//! `gp_GTrsf` — general 3-D affine transformation, mirroring OpenCascade's
//! `gp_GTrsf` from package `TKMath/gp`.
//!
//! A `GTrsf` represents the most general (possibly non-orthogonal, non-uniform
//! scale) affine map in 3-D:
//!
//! ```text
//!   | x' |   | a11 a12 a13 | | x |   | tx |
//!   | y' | = | a21 a22 a23 | | y | + | ty |
//!   | z' |   | a31 a32 a33 | | z |   | tz |
//! ```
//!
//! The 3×3 matrix `M` is the *vectorial part*; `(tx, ty, tz)` is the
//! *translation part*.
//!
//! Unlike `gp_Trsf`, `GTrsf` imposes no orthogonality / uniform-scale
//! constraint on `M`.  It can represent shears, non-uniform scales, affine
//! symmetries, etc.
//!
//! ## OCCT semantics mirrored
//! * `SetAffinity(Ax1/Ax2, ratio)` — axial affinity.
//! * `SetValue(row, col, v)` / `Value(row, col)` — coefficient access (1-based;
//!   cols 1–3 are vectorial, col 4 is translation).
//! * `SetTranslationPart` / `TranslationPart`.
//! * `SetVectorialPart` / `VectorialPart`.
//! * `IsNegative` — true when `det(M) < 0`.
//! * `Form` — cached [`GTrsfForm`].
//! * `Transforms(x, y, z)` — applies the map in-place.
//! * `Inverted` — inverse transform.
//! * `Multiplied` / `Multiply` / `PreMultiply` — composition.
//! * `Power` — integer power.
//! * `to_gp_trsf()` — cast to `gp::Trsf` when the vectorial part has the
//!   structure of an orthogonal map (possibly with uniform scale).

// occt-ref: gp_GTrsf

use crate::gp::{Ax1, Ax2, Pnt, Trsf as GpTrsf};

// ─────────────────────────────────────────────────────────────────────────────
// GTrsfForm
// ─────────────────────────────────────────────────────────────────────────────

/// Transformation form classification for [`GTrsf`].
///
/// Mirrors OCCT's `gp_TrsfForm` extended to cover the general case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// occt-ref: gp_TrsfForm // (general)
pub enum GTrsfForm {
    /// Identity.
    Identity,
    /// Pure translation (`M = I`, `t ≠ 0`).
    Translation,
    /// Rotation (orthogonal, det = +1, no scale).
    Rotation,
    /// Uniform positive scale (`M = s·I`, s > 0).
    Scale,
    /// Compound (rotation + translation, or scaled isometry).
    CompoundTrsf,
    /// Mirror through a point (−I).
    PntMirror,
    /// Mirror through a line (orthogonal, det = −1, no translation).
    Ax1Mirror,
    /// Mirror through a plane (orthogonal, det = −1, with translation).
    Ax2Mirror,
    /// General affine — anything not classifiable above.
    Other,
}

// ─────────────────────────────────────────────────────────────────────────────
// Error
// ─────────────────────────────────────────────────────────────────────────────

/// Errors raised by fallible [`GTrsf`] operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GTrsfError {
    /// Index out of range (row ∈ 1..=3, col ∈ 1..=4).
    OutOfRange,
    /// Singular — cannot invert.
    Singular,
    /// Cannot cast to `gp::Trsf` (non-orthogonal or non-uniform scale).
    NotOrthogonal,
}

impl std::fmt::Display for GTrsfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GTrsfError::OutOfRange => write!(f, "gp_GTrsf: index out of range"),
            GTrsfError::Singular => write!(f, "gp_GTrsf: singular — cannot invert"),
            GTrsfError::NotOrthogonal => {
                write!(f, "gp_GTrsf: cannot cast to gp_Trsf (non-orthogonal)")
            }
        }
    }
}

impl std::error::Error for GTrsfError {}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Resolution constant (same as OCCT `gp::Resolution()`).
const RESOLUTION: f64 = 1.0e-15;

/// 3×3 matrix-vector multiply.
#[inline]
fn mat3_mv(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

/// 3×3 matrix multiply: `a * b`.
#[inline]
fn mat3_mm(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut r = [[0.0f64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            r[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
        }
    }
    r
}

/// Determinant of a 3×3 matrix.
#[inline]
fn mat3_det(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// Invert a 3×3 matrix via cofactors.  Returns `None` if singular.
fn mat3_inv(m: &[[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    let det = mat3_det(m);
    if det.abs() <= RESOLUTION {
        return None;
    }
    let inv = 1.0 / det;
    Some([
        [
            (m[1][1] * m[2][2] - m[1][2] * m[2][1]) * inv,
            (m[0][2] * m[2][1] - m[0][1] * m[2][2]) * inv,
            (m[0][1] * m[1][2] - m[0][2] * m[1][1]) * inv,
        ],
        [
            (m[1][2] * m[2][0] - m[1][0] * m[2][2]) * inv,
            (m[0][0] * m[2][2] - m[0][2] * m[2][0]) * inv,
            (m[0][2] * m[1][0] - m[0][0] * m[1][2]) * inv,
        ],
        [
            (m[1][0] * m[2][1] - m[1][1] * m[2][0]) * inv,
            (m[0][1] * m[2][0] - m[0][0] * m[2][1]) * inv,
            (m[0][0] * m[1][1] - m[0][1] * m[1][0]) * inv,
        ],
    ])
}

/// Classify the form from `mat` and `loc`.
fn detect_form(mat: &[[f64; 3]; 3], loc: &[f64; 3]) -> GTrsfForm {
    let has_trans = loc[0].abs() > RESOLUTION
        || loc[1].abs() > RESOLUTION
        || loc[2].abs() > RESOLUTION;

    // Identity matrix?
    let is_id = (mat[0][0] - 1.0).abs() <= RESOLUTION
        && mat[0][1].abs() <= RESOLUTION
        && mat[0][2].abs() <= RESOLUTION
        && mat[1][0].abs() <= RESOLUTION
        && (mat[1][1] - 1.0).abs() <= RESOLUTION
        && mat[1][2].abs() <= RESOLUTION
        && mat[2][0].abs() <= RESOLUTION
        && mat[2][1].abs() <= RESOLUTION
        && (mat[2][2] - 1.0).abs() <= RESOLUTION;

    if is_id {
        return if has_trans {
            GTrsfForm::Translation
        } else {
            GTrsfForm::Identity
        };
    }

    // Negative identity (PntMirror = −I)?
    let is_neg_id = (mat[0][0] + 1.0).abs() <= RESOLUTION
        && mat[0][1].abs() <= RESOLUTION
        && mat[0][2].abs() <= RESOLUTION
        && mat[1][0].abs() <= RESOLUTION
        && (mat[1][1] + 1.0).abs() <= RESOLUTION
        && mat[1][2].abs() <= RESOLUTION
        && mat[2][0].abs() <= RESOLUTION
        && mat[2][1].abs() <= RESOLUTION
        && (mat[2][2] + 1.0).abs() <= RESOLUTION;

    if is_neg_id {
        return GTrsfForm::PntMirror;
    }

    // Uniform scale?  M = s * I
    let s = mat[0][0];
    let is_uniform = (mat[1][1] - s).abs() <= RESOLUTION
        && (mat[2][2] - s).abs() <= RESOLUTION
        && mat[0][1].abs() <= RESOLUTION
        && mat[0][2].abs() <= RESOLUTION
        && mat[1][0].abs() <= RESOLUTION
        && mat[1][2].abs() <= RESOLUTION
        && mat[2][0].abs() <= RESOLUTION
        && mat[2][1].abs() <= RESOLUTION;

    if is_uniform && s > 0.0 {
        return if has_trans {
            GTrsfForm::CompoundTrsf
        } else {
            GTrsfForm::Scale
        };
    }

    // Check orthogonality: M^T * M ≈ s² * I
    let mt = [
        [mat[0][0], mat[1][0], mat[2][0]],
        [mat[0][1], mat[1][1], mat[2][1]],
        [mat[0][2], mat[1][2], mat[2][2]],
    ];
    let mtm = mat3_mm(&mt, mat);
    let diag_avg = (mtm[0][0] + mtm[1][1] + mtm[2][2]) / 3.0;

    if diag_avg > RESOLUTION {
        let tol = diag_avg * 1.0e-10 + RESOLUTION;
        let is_ortho = mtm[0][1].abs() <= tol
            && mtm[0][2].abs() <= tol
            && mtm[1][0].abs() <= tol
            && mtm[1][2].abs() <= tol
            && mtm[2][0].abs() <= tol
            && mtm[2][1].abs() <= tol
            && (mtm[0][0] - diag_avg).abs() <= tol
            && (mtm[1][1] - diag_avg).abs() <= tol
            && (mtm[2][2] - diag_avg).abs() <= tol;

        if is_ortho {
            let det = mat3_det(mat);
            if det > 0.0 {
                return if has_trans {
                    GTrsfForm::CompoundTrsf
                } else {
                    GTrsfForm::Rotation
                };
            } else {
                // det < 0 → axis mirror or plane mirror
                return if has_trans {
                    GTrsfForm::Ax2Mirror
                } else {
                    GTrsfForm::Ax1Mirror
                };
            }
        }
    }

    GTrsfForm::Other
}

// ─────────────────────────────────────────────────────────────────────────────
// GTrsf
// ─────────────────────────────────────────────────────────────────────────────

/// General 3-D affine transformation.
///
/// The applied map is: `p' = mat * p + loc`.
/// `mat` is the 3×3 vectorial part (row-major, 0-based).
/// `loc` is the translation vector.
// occt-ref: gp_GTrsf
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GTrsf {
    /// Row-major 3×3 linear (vectorial) part.  `mat[row][col]`, 0-based.
    mat: [[f64; 3]; 3],
    /// Translation part.
    loc: [f64; 3],
    /// Cached form classification.
    form: GTrsfForm,
}

impl Default for GTrsf {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl GTrsf {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// `gp_GTrsf()` — identity transform.
    #[inline]
    pub const fn identity() -> Self {
        Self {
            mat: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            loc: [0.0, 0.0, 0.0],
            form: GTrsfForm::Identity,
        }
    }

    /// `gp_GTrsf(T)` — construct from a `gp::Trsf`.
    ///
    /// Copies the 3×3 linear part and translation directly.
    pub fn from_gp_trsf(t: &GpTrsf) -> Self {
        let mat = t.m;
        let loc = t.t;
        let form = detect_form(&mat, &loc);
        Self { mat, loc, form }
    }

    // ── SetAffinity ───────────────────────────────────────────────────────────

    /// `SetAffinity(Ax1, Ratio)` — axial affinity along line `A` with ratio
    /// `Ratio`.
    ///
    /// An *axial affinity* fixes every point on the line `A` and scales the
    /// perpendicular component (distance to the line) by `Ratio`.
    ///
    /// OCCT formula: let `d` = unit direction, `P` = location.
    ///   `M = Ratio * I + (1 − Ratio) * d⊗d`
    ///   (the axial direction is unchanged, the two perpendicular directions
    ///   are scaled by `Ratio`).
    ///   `t = P − M·P`  (fixes every point on the axis).
    pub fn set_affinity_ax1(&mut self, axis: Ax1, ratio: f64) {
        let d = axis.direction.normalized();
        let (dx, dy, dz) = (d.x, d.y, d.z);
        // M = ratio * (I - d⊗d) + d⊗d = ratio*I + (1-ratio)*d⊗d
        let k = 1.0 - ratio;
        self.mat[0][0] = ratio + k * dx * dx;
        self.mat[0][1] = k * dx * dy;
        self.mat[0][2] = k * dx * dz;
        self.mat[1][0] = k * dy * dx;
        self.mat[1][1] = ratio + k * dy * dy;
        self.mat[1][2] = k * dy * dz;
        self.mat[2][0] = k * dz * dx;
        self.mat[2][1] = k * dz * dy;
        self.mat[2][2] = ratio + k * dz * dz;
        // Fixed-point condition: t = P - M*P
        let p = [axis.location.x, axis.location.y, axis.location.z];
        let mp = mat3_mv(&self.mat, p);
        self.loc[0] = p[0] - mp[0];
        self.loc[1] = p[1] - mp[1];
        self.loc[2] = p[2] - mp[2];
        self.form = detect_form(&self.mat, &self.loc);
    }

    /// `SetAffinity(Ax2, Ratio)` — planar affinity with respect to the plane
    /// defined by `ax2` scaled by `Ratio`.
    ///
    /// The plane is the X-Y plane of the `Ax2` frame (normal = `ax2.z_dir`,
    /// origin = `ax2.location`).  Points in the plane are unchanged; points
    /// off the plane are scaled by `Ratio` along the normal direction.
    ///
    /// OCCT formula: let `n` = unit normal, `P` = origin.
    ///   `M = I + (Ratio − 1) · n⊗n`
    ///   `t = P − M·P`
    pub fn set_affinity_ax2(&mut self, ax2: Ax2, ratio: f64) {
        let n = ax2.z_dir.normalized();
        let (nx, ny, nz) = (n.x, n.y, n.z);
        let k = ratio - 1.0;
        self.mat[0][0] = 1.0 + k * nx * nx;
        self.mat[0][1] = k * nx * ny;
        self.mat[0][2] = k * nx * nz;
        self.mat[1][0] = k * ny * nx;
        self.mat[1][1] = 1.0 + k * ny * ny;
        self.mat[1][2] = k * ny * nz;
        self.mat[2][0] = k * nz * nx;
        self.mat[2][1] = k * nz * ny;
        self.mat[2][2] = 1.0 + k * nz * nz;
        let p = [ax2.location.x, ax2.location.y, ax2.location.z];
        let mp = mat3_mv(&self.mat, p);
        self.loc[0] = p[0] - mp[0];
        self.loc[1] = p[1] - mp[1];
        self.loc[2] = p[2] - mp[2];
        self.form = detect_form(&self.mat, &self.loc);
    }

    // ── SetValue / Value ──────────────────────────────────────────────────────

    /// `SetValue(Row, Col, Value)` — sets coefficient (Row, Col) of the 3×4
    /// augmented matrix.
    ///
    /// Rows 1–3, columns 1–3 are the vectorial part; column 4 is the
    /// translation.  Indices are **1-based**.
    ///
    /// Returns `Err(GTrsfError::OutOfRange)` on bad indices.
    pub fn set_value(&mut self, row: usize, col: usize, value: f64) -> Result<(), GTrsfError> {
        if !(1..=3).contains(&row) || !(1..=4).contains(&col) {
            return Err(GTrsfError::OutOfRange);
        }
        if col <= 3 {
            self.mat[row - 1][col - 1] = value;
        } else {
            self.loc[row - 1] = value;
        }
        self.form = detect_form(&self.mat, &self.loc);
        Ok(())
    }

    /// `Value(Row, Col)` — coefficient (Row, Col) of the 3×4 matrix.
    ///
    /// Row ∈ 1..=3, Col ∈ 1..=4.  Returns `Err(GTrsfError::OutOfRange)`.
    pub fn value(&self, row: usize, col: usize) -> Result<f64, GTrsfError> {
        if !(1..=3).contains(&row) || !(1..=4).contains(&col) {
            return Err(GTrsfError::OutOfRange);
        }
        if col <= 3 {
            Ok(self.mat[row - 1][col - 1])
        } else {
            Ok(self.loc[row - 1])
        }
    }

    // ── SetTranslationPart / TranslationPart ──────────────────────────────────

    /// `SetTranslationPart(V)` — replaces the translation column with `V`.
    pub fn set_translation_part(&mut self, v: Pnt) {
        self.loc = [v.x, v.y, v.z];
        self.form = detect_form(&self.mat, &self.loc);
    }

    /// `TranslationPart()` — returns the translation as a `Pnt`.
    #[inline]
    pub fn translation_part(&self) -> Pnt {
        Pnt::new(self.loc[0], self.loc[1], self.loc[2])
    }

    // ── SetVectorialPart / VectorialPart ──────────────────────────────────────

    /// `SetVectorialPart(M)` — replaces the 3×3 vectorial part.
    ///
    /// `mat` must be provided row-major: `mat[row][col]`, 0-based.
    pub fn set_vectorial_part(&mut self, mat: [[f64; 3]; 3]) {
        self.mat = mat;
        self.form = detect_form(&self.mat, &self.loc);
    }

    /// `VectorialPart()` — returns the 3×3 vectorial part (row-major, 0-based).
    #[inline]
    pub fn vectorial_part(&self) -> [[f64; 3]; 3] {
        self.mat
    }

    // ── Queries ───────────────────────────────────────────────────────────────

    /// `IsNegative()` — true when `det(VectorialPart) < 0`.
    #[inline]
    pub fn is_negative(&self) -> bool {
        mat3_det(&self.mat) < 0.0
    }

    /// `Form()` — cached form classification of the vectorial part.
    #[inline]
    pub fn form(&self) -> GTrsfForm {
        self.form
    }

    // ── Transforms ────────────────────────────────────────────────────────────

    /// `Transforms(x, y, z)` — applies this transformation in-place.
    ///
    /// `(x', y', z') = M · (x, y, z) + t`
    #[inline]
    pub fn transforms(&self, x: &mut f64, y: &mut f64, z: &mut f64) {
        let r = mat3_mv(&self.mat, [*x, *y, *z]);
        *x = r[0] + self.loc[0];
        *y = r[1] + self.loc[1];
        *z = r[2] + self.loc[2];
    }

    /// Applies this transformation to a [`Pnt`].
    #[inline]
    pub fn transform_pnt(&self, p: Pnt) -> Pnt {
        let r = mat3_mv(&self.mat, [p.x, p.y, p.z]);
        Pnt::new(r[0] + self.loc[0], r[1] + self.loc[1], r[2] + self.loc[2])
    }

    // ── Inverted / Invert ─────────────────────────────────────────────────────

    /// `Inverted()` — returns the inverse transformation.
    ///
    /// Returns `Err(GTrsfError::Singular)` if the vectorial part is singular.
    pub fn inverted(&self) -> Result<Self, GTrsfError> {
        let inv_mat = mat3_inv(&self.mat).ok_or(GTrsfError::Singular)?;
        // Inverse translation: t' = −M⁻¹ · t
        let neg_loc = [-self.loc[0], -self.loc[1], -self.loc[2]];
        let inv_loc = mat3_mv(&inv_mat, neg_loc);
        let form = detect_form(&inv_mat, &inv_loc);
        Ok(Self {
            mat: inv_mat,
            loc: inv_loc,
            form,
        })
    }

    /// `Invert()` — in-place inversion.
    pub fn invert(&mut self) -> Result<(), GTrsfError> {
        *self = self.inverted()?;
        Ok(())
    }

    // ── Multiplied / Multiply / PreMultiply ──────────────────────────────────

    /// `Multiplied(other)` — right-composes: returns `self * other`.
    ///
    /// `(self ∘ other)(x) = self.M · (other.M · x + other.t) + self.t`
    pub fn multiplied(&self, other: &Self) -> Self {
        let new_mat = mat3_mm(&self.mat, &other.mat);
        let rt = mat3_mv(&self.mat, other.loc);
        let new_loc = [
            rt[0] + self.loc[0],
            rt[1] + self.loc[1],
            rt[2] + self.loc[2],
        ];
        let form = detect_form(&new_mat, &new_loc);
        Self {
            mat: new_mat,
            loc: new_loc,
            form,
        }
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

    // ── Power ─────────────────────────────────────────────────────────────────

    /// `Power(n)` — `self` raised to the integer power `n`.
    ///
    /// * `n = 0` → identity.
    /// * `n > 0` → repeated right-composition.
    /// * `n < 0` → composition of the inverse `|n|` times.
    ///
    /// Returns `Err(GTrsfError::Singular)` if `n < 0` and the transform is
    /// singular.
    pub fn power(&self, n: i32) -> Result<Self, GTrsfError> {
        if n == 0 {
            return Ok(Self::identity());
        }
        if n == 1 {
            return Ok(*self);
        }
        if n == -1 {
            return self.inverted();
        }
        let (base, count) = if n > 0 {
            (*self, n as u32)
        } else {
            (self.inverted()?, (-n) as u32)
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
        Ok(result)
    }

    /// `Power(n)` — in-place version.
    pub fn pow_inplace(&mut self, n: i32) -> Result<(), GTrsfError> {
        *self = self.power(n)?;
        Ok(())
    }

    // ── to_gp_trsf — cast to gp::Trsf ────────────────────────────────────────

    /// Cast to `gp::Trsf` when the vectorial part is orthogonal (possibly with
    /// a uniform scale factor), i.e. when `M^T·M = s²·I`.
    ///
    /// This mirrors OCCT's `gp_GTrsf::Trsf()` which raises
    /// `Standard_ConstructionError` when the map is not a valid `gp_Trsf`.
    ///
    /// Returns `Err(GTrsfError::NotOrthogonal)` for general affine maps (shear,
    /// non-uniform scale).
    /// Returns `Err(GTrsfError::Singular)` if the scale factor is zero.
    pub fn to_gp_trsf(&self) -> Result<GpTrsf, GTrsfError> {
        // Check M^T * M ≈ s² * I.
        let mt = [
            [self.mat[0][0], self.mat[1][0], self.mat[2][0]],
            [self.mat[0][1], self.mat[1][1], self.mat[2][1]],
            [self.mat[0][2], self.mat[1][2], self.mat[2][2]],
        ];
        let mtm = mat3_mm(&mt, &self.mat);
        let s2 = (mtm[0][0] + mtm[1][1] + mtm[2][2]) / 3.0;
        let tol = s2.abs() * 1.0e-10 + RESOLUTION;
        let off_ok = mtm[0][1].abs() <= tol
            && mtm[0][2].abs() <= tol
            && mtm[1][0].abs() <= tol
            && mtm[1][2].abs() <= tol
            && mtm[2][0].abs() <= tol
            && mtm[2][1].abs() <= tol;
        let diag_ok = (mtm[0][0] - s2).abs() <= tol
            && (mtm[1][1] - s2).abs() <= tol
            && (mtm[2][2] - s2).abs() <= tol;
        if !off_ok || !diag_ok {
            return Err(GTrsfError::NotOrthogonal);
        }
        if s2.abs() <= RESOLUTION {
            return Err(GTrsfError::Singular);
        }
        // Build gp::Trsf (which has public fields m and t).
        Ok(GpTrsf {
            m: self.mat,
            t: self.loc,
        })
    }
}
