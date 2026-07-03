//! `gp_Trsf2d` -- 2-D affine transformation, mirroring OpenCascade's
//! `gp_Trsf2d` from package `TKMath/gp`.
//!
//! A `Trsf2d` describes any combination of uniform scale, rotation,
//! translation and central/axial symmetry (mirror) in the plane.  It is
//! stored as a 2×2 linear part (`mat`) plus a 2-component translation (`t`),
//! together with an explicit `Form` tag and a `scale_factor` that carry the
//! semantic kind and the absolute scale magnitude exactly as OCCT does.
//!
//! ## Conventions (identical to OCCT)
//!
//! * `mat` stores the *vectorial part* — i.e. the part that transforms
//!   direction vectors (`m[row][col]`, row-major, 0-based).
//! * `t` is the translation applied *after* the linear map.
//! * `scale_factor` is always a **positive** real number for `Scale`,
//!   `Rotation`, `Translation`, `Identity` and negative for `PntMirror`,
//!   `Ax1Mirror`.  The sign of `det(mat)` conveys orientation; `scale_factor`
//!   carries the magnitude.
//! * `Value(row, col)` uses **1-based** indices matching OCCT's API.

use crate::gp2d::{Ax2d, Pnt2d};

// ---------------------------------------------------------------------------
// Form
// ---------------------------------------------------------------------------

/// Transformation kind, mirroring OCCT's `gp_TrsfForm` enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// occt: gp_TrsfForm
pub enum Form {
    /// Identity (no change).
    Identity,
    /// Pure translation.
    Translation,
    /// Pure rotation.
    Rotation,
    /// Uniform scaling about a center.
    Scale,
    /// Compound (general affine — not currently produced internally but
    /// appears after `Multiply`).
    CompoundTrsf,
    /// Central symmetry (mirror through a point) — scale factor = –1.
    PntMirror,
    /// Axial symmetry (mirror through a line).
    Ax1Mirror,
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors raised by fallible `Trsf2d` operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trsf2dError {
    /// A 1-based row/column index was outside `1..=2`.
    OutOfRange,
    /// The transform is singular and cannot be inverted.
    Singular,
    /// The scale factor is zero (which is not a valid transform).
    ZeroScale,
}

impl std::fmt::Display for Trsf2dError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Trsf2dError::OutOfRange => write!(f, "gp_Trsf2d: index out of range (must be 1..=2)"),
            Trsf2dError::Singular => write!(f, "gp_Trsf2d: singular transform — cannot invert"),
            Trsf2dError::ZeroScale => write!(f, "gp_Trsf2d: scale factor must be non-zero"),
        }
    }
}

impl std::error::Error for Trsf2dError {}

// ---------------------------------------------------------------------------
// Trsf2d
// ---------------------------------------------------------------------------

/// A 2-D affine transformation.
///
/// Internally: `y' = mat * x + t`.  `mat` is the 2×2 vectorial part stored
/// row-major; `t` is the translation.
// occt: gp_Trsf2d
#[derive(Clone, Copy, Debug)]
pub struct Trsf2d {
    /// Row-major 2×2 linear (vectorial) part.  `mat[row][col]`, 0-based.
    mat: [[f64; 2]; 2],
    /// Translation vector.
    t: [f64; 2],
    /// Transformation kind tag.
    form: Form,
    /// Absolute scale factor (positive magnitude).
    scale_factor: f64,
}

impl Default for Trsf2d {
    fn default() -> Self {
        Self::identity()
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Multiply a 2×2 matrix by a 2-vector.
#[inline]
fn mat2_mul_vec(m: &[[f64; 2]; 2], v: [f64; 2]) -> [f64; 2] {
    [
        m[0][0] * v[0] + m[0][1] * v[1],
        m[1][0] * v[0] + m[1][1] * v[1],
    ]
}

/// Multiply two 2×2 matrices: `a * b`.
#[inline]
fn mat2_mul(a: &[[f64; 2]; 2], b: &[[f64; 2]; 2]) -> [[f64; 2]; 2] {
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

/// Determinant of a 2×2 matrix.
#[inline]
fn mat2_det(m: &[[f64; 2]; 2]) -> f64 {
    m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

// ---------------------------------------------------------------------------
// Trsf2d implementation
// ---------------------------------------------------------------------------

impl Trsf2d {
    // -----------------------------------------------------------------------
    // Constructors
    // -----------------------------------------------------------------------

    /// Identity transform (OCCT default constructor).
    pub const fn identity() -> Self {
        Self {
            mat: [[1.0, 0.0], [0.0, 1.0]],
            t: [0.0, 0.0],
            form: Form::Identity,
            scale_factor: 1.0,
        }
    }

    // -----------------------------------------------------------------------
    // Setters — correspond to OCCT's `SetXxx` methods
    // -----------------------------------------------------------------------

    /// `SetMirror(P)` — central symmetry (point mirror) about `p`.
    ///
    /// Maps `X` to `2*P - X`.  Determinant of the linear part is `+1` but
    /// the scale factor is stored as `-1` exactly as OCCT does.
    pub fn set_mirror_point(&mut self, p: Pnt2d) {
        // Linear part: negate both axes — same as scale -1.
        self.mat = [[-1.0, 0.0], [0.0, -1.0]];
        // Translation: 2 * P (so that the fixed point is P itself).
        self.t = [2.0 * p.x, 2.0 * p.y];
        self.form = Form::PntMirror;
        self.scale_factor = -1.0;
    }

    /// `SetMirror(A)` — axial symmetry (line mirror) across the axis `a`.
    ///
    /// Maps `X` to its reflection across the line defined by `a`.
    pub fn set_mirror_axis(&mut self, a: Ax2d) {
        let d = a.direction.normalized();
        let (dx, dy) = (d.x, d.y);
        // Householder reflection: R = I - 2 * n*n^T where n is the axis *normal*
        // (perpendicular to the mirror line).  If the mirror line direction is
        // (dx, dy) then the normal is (-dy, dx).
        // Equivalently, for direction (dx, dy):
        //   R = [[ dx² - dy²,  2*dx*dy ],
        //        [ 2*dx*dy,    dy² - dx² ]]
        // which is the standard formula for reflection through a line at angle θ:
        //   R = [[cos2θ, sin2θ],[sin2θ, -cos2θ]]  with (dx,dy) = (cosθ, sinθ).
        let m00 = dx * dx - dy * dy;
        let m01 = 2.0 * dx * dy;
        let m10 = 2.0 * dx * dy;
        let m11 = dy * dy - dx * dx;
        self.mat = [[m00, m01], [m10, m11]];

        // The translation is chosen so that points on the axis are fixed:
        //   t = (I - R) * P   where P = a.location
        let p = a.location;
        self.t = [
            (1.0 - m00) * p.x - m01 * p.y,
            -m10 * p.x + (1.0 - m11) * p.y,
        ];
        self.form = Form::Ax1Mirror;
        self.scale_factor = -1.0;
    }

    /// `SetRotation(P, angle)` — rotation of `angle` radians about point `p`.
    pub fn set_rotation(&mut self, p: Pnt2d, angle: f64) {
        let (s, c) = angle.sin_cos();
        let m = [[c, -s], [s, c]];
        // Fixed-point constraint: t = P - R * P
        let rp = mat2_mul_vec(&m, [p.x, p.y]);
        self.mat = m;
        self.t = [p.x - rp[0], p.y - rp[1]];
        self.form = Form::Rotation;
        self.scale_factor = 1.0;
    }

    /// `SetScale(P, s)` — uniform scaling by factor `s` about center `p`.
    ///
    /// OCCT allows negative `s` (which makes this a "scale with flip"),
    /// but the scale factor is stored as the signed `s` value.  Zero is
    /// rejected as invalid.
    pub fn set_scale(&mut self, p: Pnt2d, s: f64) -> Result<(), Trsf2dError> {
        if s == 0.0 {
            return Err(Trsf2dError::ZeroScale);
        }
        self.mat = [[s, 0.0], [0.0, s]];
        self.t = [p.x * (1.0 - s), p.y * (1.0 - s)];
        self.form = Form::Scale;
        self.scale_factor = s;
        Ok(())
    }

    /// `SetTranslation(V)` — pure translation by vector `v`.
    pub fn set_translation(&mut self, v: Pnt2d) {
        self.mat = [[1.0, 0.0], [0.0, 1.0]];
        self.t = [v.x, v.y];
        self.form = Form::Translation;
        self.scale_factor = 1.0;
    }

    /// `SetTranslation(P1, P2)` — translation by the vector P1 → P2.
    pub fn set_translation_2pts(&mut self, p1: Pnt2d, p2: Pnt2d) {
        self.set_translation(Pnt2d::new(p2.x - p1.x, p2.y - p1.y));
    }

    // -----------------------------------------------------------------------
    // Queries
    // -----------------------------------------------------------------------

    /// `IsNegative` — true if the transform reverses orientation
    /// (i.e. `det(mat) < 0`).
    pub fn is_negative(&self) -> bool {
        mat2_det(&self.mat) < 0.0
    }

    /// `Form` — the transformation kind.
    pub fn form(&self) -> Form {
        self.form
    }

    /// `ScaleFactor` — the signed scale factor stored with the transform.
    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    /// `TranslationPart` — returns `(t[0], t[1])` as a `Pnt2d`.
    pub fn translation_part(&self) -> Pnt2d {
        Pnt2d::new(self.t[0], self.t[1])
    }

    /// `VectorialPart` — the 2×2 linear part as a row-major flat array
    /// `[[m00, m01], [m10, m11]]`.
    pub fn vectorial_part(&self) -> [[f64; 2]; 2] {
        self.mat
    }

    /// `Value(row, col)` — coefficient of the *augmented* 3×3 homogeneous
    /// matrix (rows 1–2 are `[mat | t]`; row 3 is `[0, 0, 1]`).
    ///
    /// Indices are 1-based, matching OCCT's `gp_Trsf2d::Value`.
    pub fn value(&self, row: i32, col: i32) -> Result<f64, Trsf2dError> {
        match (row, col) {
            (1, 1) => Ok(self.mat[0][0]),
            (1, 2) => Ok(self.mat[0][1]),
            (1, 3) => Ok(self.t[0]),
            (2, 1) => Ok(self.mat[1][0]),
            (2, 2) => Ok(self.mat[1][1]),
            (2, 3) => Ok(self.t[1]),
            (3, 1) => Ok(0.0),
            (3, 2) => Ok(0.0),
            (3, 3) => Ok(1.0),
            _ => Err(Trsf2dError::OutOfRange),
        }
    }

    // -----------------------------------------------------------------------
    // Composition / inversion
    // -----------------------------------------------------------------------

    /// Apply this transform to a point: `p' = mat * p + t`.
    pub fn apply(&self, p: Pnt2d) -> Pnt2d {
        let r = mat2_mul_vec(&self.mat, [p.x, p.y]);
        Pnt2d::new(r[0] + self.t[0], r[1] + self.t[1])
    }

    /// Apply only the linear (vectorial) part — useful for transforming
    /// direction vectors.
    pub fn apply_vector(&self, v: Pnt2d) -> Pnt2d {
        let r = mat2_mul_vec(&self.mat, [v.x, v.y]);
        Pnt2d::new(r[0], r[1])
    }

    /// `Invert` — in-place inversion.
    pub fn invert(&mut self) -> Result<(), Trsf2dError> {
        *self = self.inverted()?;
        Ok(())
    }

    /// `Inverted` — returns the inverse transform as a new value.
    pub fn inverted(&self) -> Result<Self, Trsf2dError> {
        let det = mat2_det(&self.mat);
        if det.abs() < 1.0e-15 {
            return Err(Trsf2dError::Singular);
        }
        let inv_det = 1.0 / det;
        // 2×2 inverse: [[d, -b], [-c, a]] / det  for [[a,b],[c,d]]
        let inv_mat = [
            [
                self.mat[1][1] * inv_det,
                -self.mat[0][1] * inv_det,
            ],
            [
                -self.mat[1][0] * inv_det,
                self.mat[0][0] * inv_det,
            ],
        ];
        // Inverse translation: t' = -inv_mat * t
        let inv_t = mat2_mul_vec(&inv_mat, self.t);
        let inv_t = [-inv_t[0], -inv_t[1]];

        // Determine the form of the inverse (same kind as the original).
        let inv_form = self.form;

        // Scale factor of the inverse: 1/s for Scale, otherwise same sign.
        let inv_scale = if self.scale_factor.abs() > 1.0e-15 {
            1.0 / self.scale_factor
        } else {
            return Err(Trsf2dError::Singular);
        };

        Ok(Self {
            mat: inv_mat,
            t: inv_t,
            form: inv_form,
            scale_factor: inv_scale,
        })
    }

    /// `Multiplied(other)` — compose: `self ∘ other` (apply `other` first,
    /// then `self`).  Returns a new `Trsf2d`.
    pub fn multiplied(&self, other: &Trsf2d) -> Trsf2d {
        // Combined linear: self.mat * other.mat
        let new_mat = mat2_mul(&self.mat, &other.mat);
        // Combined translation: self.mat * other.t + self.t
        let ro = mat2_mul_vec(&self.mat, other.t);
        let new_t = [ro[0] + self.t[0], ro[1] + self.t[1]];

        // Determine form heuristically.
        let new_form = compose_form(self.form, other.form);

        // Combined scale: product of the two scale factors.
        let new_scale = self.scale_factor * other.scale_factor;

        Trsf2d {
            mat: new_mat,
            t: new_t,
            form: new_form,
            scale_factor: new_scale,
        }
    }

    /// `Multiply(other)` — in-place composition.
    pub fn multiply(&mut self, other: &Trsf2d) {
        *self = self.multiplied(other);
    }
}

/// Determine the resulting form when composing `outer ∘ inner`.
fn compose_form(outer: Form, inner: Form) -> Form {
    match (outer, inner) {
        (Form::Identity, f) | (f, Form::Identity) => f,
        (Form::Translation, Form::Translation) => Form::Translation,
        (Form::Rotation, Form::Rotation) => Form::Rotation,
        (Form::Rotation, Form::Translation) | (Form::Translation, Form::Rotation) => {
            Form::Rotation
        }
        (Form::PntMirror, Form::PntMirror)
        | (Form::Ax1Mirror, Form::Ax1Mirror)
        | (Form::PntMirror, Form::Ax1Mirror)
        | (Form::Ax1Mirror, Form::PntMirror) => Form::Rotation,
        _ => Form::CompoundTrsf,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, PI};

    const TOL: f64 = 1.0e-12;

    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < TOL, "{a} != {b}");
    }

    #[test]
    fn identity_is_default() {
        let t = Trsf2d::identity();
        assert_eq!(t.form(), Form::Identity);
        near(t.scale_factor(), 1.0);
        assert!(!t.is_negative());
        let p = Pnt2d::new(3.0, 4.0);
        let r = t.apply(p);
        near(r.x, 3.0);
        near(r.y, 4.0);
    }

    #[test]
    fn rotation_90() {
        let mut t = Trsf2d::identity();
        t.set_rotation(Pnt2d::origin(), FRAC_PI_2);
        let r = t.apply(Pnt2d::new(1.0, 0.0));
        near(r.x, 0.0);
        near(r.y, 1.0);
    }

    #[test]
    fn mirror_point_negates() {
        let mut t = Trsf2d::identity();
        t.set_mirror_point(Pnt2d::origin());
        let r = t.apply(Pnt2d::new(3.0, 4.0));
        near(r.x, -3.0);
        near(r.y, -4.0);
        // Point mirror = rotation by PI: det([[-1,0],[0,-1]]) = +1, not negative.
        assert!(!t.is_negative());
    }

    #[test]
    fn mirror_axis_along_x() {
        let mut t = Trsf2d::identity();
        // Mirror axis = X axis: maps (x, y) -> (x, -y)
        t.set_mirror_axis(Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)));
        let r = t.apply(Pnt2d::new(2.0, 3.0));
        near(r.x, 2.0);
        near(r.y, -3.0);
        assert!(t.is_negative());
    }

    #[test]
    fn invert_rotation() {
        let mut t = Trsf2d::identity();
        t.set_rotation(Pnt2d::new(1.0, 2.0), PI / 6.0);
        let inv = t.inverted().unwrap();
        let p = Pnt2d::new(5.0, 7.0);
        let roundtrip = inv.apply(t.apply(p));
        near(roundtrip.x, p.x);
        near(roundtrip.y, p.y);
    }

    #[test]
    fn scale_and_inverted() {
        let mut t = Trsf2d::identity();
        t.set_scale(Pnt2d::origin(), 3.0).unwrap();
        near(t.scale_factor(), 3.0);
        let inv = t.inverted().unwrap();
        near(inv.scale_factor(), 1.0 / 3.0);
        let p = Pnt2d::new(1.0, 2.0);
        let rr = inv.apply(t.apply(p));
        near(rr.x, p.x);
        near(rr.y, p.y);
    }

    #[test]
    fn translation_part_correct() {
        let mut t = Trsf2d::identity();
        t.set_translation(Pnt2d::new(5.0, -3.0));
        let tp = t.translation_part();
        near(tp.x, 5.0);
        near(tp.y, -3.0);
    }

    #[test]
    fn value_indexing() {
        let mut t = Trsf2d::identity();
        t.set_rotation(Pnt2d::origin(), FRAC_PI_2);
        // mat = [[0,-1],[1,0]], t = [0,0]
        near(t.value(1, 1).unwrap(), 0.0);
        near(t.value(1, 2).unwrap(), -1.0);
        near(t.value(2, 1).unwrap(), 1.0);
        near(t.value(2, 2).unwrap(), 0.0);
        near(t.value(3, 3).unwrap(), 1.0);
        near(t.value(3, 1).unwrap(), 0.0);
        assert!(t.value(0, 1).is_err());
        assert!(t.value(4, 1).is_err());
    }

    #[test]
    fn multiplied_two_rotations() {
        let mut a = Trsf2d::identity();
        a.set_rotation(Pnt2d::origin(), FRAC_PI_2);
        let mut b = Trsf2d::identity();
        b.set_rotation(Pnt2d::origin(), FRAC_PI_2);
        let c = a.multiplied(&b); // total 180°
        let r = c.apply(Pnt2d::new(1.0, 0.0));
        near(r.x, -1.0);
        near(r.y, 0.0);
    }
}
