//! `gp_Trsf` — 3-D affine transformation stub, mirroring OpenCascade's
//! `gp_Trsf` from package `TKMath/gp`.
//!
//! A `Trsf` is stored as an augmented 4×4 row-major matrix `m` where the
//! bottom row is always `[0, 0, 0, 1]`.  The top-left 3×3 block is the
//! vectorial (linear) part and the rightmost column (indices `[r][3]`) is
//! the translation.
//!
//! All arithmetic uses only `std` (`f64::sin`, `f64::cos`, `f64::sqrt`).
//! No external crates are used.

// occt: gp_Trsf

use crate::gp::{Ax1, Ax3, Pnt};

// ─────────────────────────────────────────────────────────────────────────────
// Form
// ─────────────────────────────────────────────────────────────────────────────

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
    /// General compound transformation (produced by `Multiply`).
    CompoundTrsf,
    /// Central symmetry (mirror through a point) — scale factor = −1.
    PntMirror,
    /// Axial symmetry (mirror through a line, `gp_Ax1`).
    Ax1Mirror,
    /// Planar symmetry (mirror through a plane, `gp_Ax2`).
    Ax2Mirror,
}

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

/// Errors raised by fallible `Trsf` operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrsfError {
    /// A 1-based row/column index was outside valid range.
    OutOfRange,
    /// The transform is singular and cannot be inverted.
    Singular,
    /// A zero scale factor was supplied (not a valid transform).
    ZeroScale,
}

impl std::fmt::Display for TrsfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrsfError::OutOfRange => write!(f, "gp_Trsf: index out of range"),
            TrsfError::Singular => write!(f, "gp_Trsf: singular transform — cannot invert"),
            TrsfError::ZeroScale => write!(f, "gp_Trsf: scale factor must be non-zero"),
        }
    }
}

impl std::error::Error for TrsfError {}

// ─────────────────────────────────────────────────────────────────────────────
// Public free function
// ─────────────────────────────────────────────────────────────────────────────

/// Rodrigues rotation matrix: rotate `angle` radians about `axis`.
///
/// `axis` need not be unit-length; it is normalised internally.
/// Returns a 3×3 row-major matrix.
pub fn rotation_matrix(axis: [f64; 3], angle: f64) -> [[f64; 3]; 3] {
    let len = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let (ux, uy, uz) = if len > 0.0 {
        (axis[0] / len, axis[1] / len, axis[2] / len)
    } else {
        (1.0, 0.0, 0.0)
    };
    let (s, c) = (angle.sin(), angle.cos());
    let t = 1.0 - c;
    [
        [t * ux * ux + c,       t * ux * uy - s * uz, t * ux * uz + s * uy],
        [t * ux * uy + s * uz,  t * uy * uy + c,      t * uy * uz - s * ux],
        [t * ux * uz - s * uy,  t * uy * uz + s * ux, t * uz * uz + c     ],
    ]
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Multiply two 4×4 row-major matrices.
fn mat4_mul(a: &[[f64; 4]; 4], b: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut r = [[0.0f64; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                r[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    r
}

/// Apply a 4×4 homogeneous matrix to a 3-D point (w = 1).
#[inline]
fn mat4_apply_pt(m: &[[f64; 4]; 4], p: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * p[0] + m[0][1] * p[1] + m[0][2] * p[2] + m[0][3],
        m[1][0] * p[0] + m[1][1] * p[1] + m[1][2] * p[2] + m[1][3],
        m[2][0] * p[0] + m[2][1] * p[1] + m[2][2] * p[2] + m[2][3],
    ]
}

/// Apply only the 3×3 linear part of a 4×4 matrix to a free vector (w = 0).
#[inline]
fn mat4_apply_vec(m: &[[f64; 4]; 4], v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

/// Determinant of the top-left 3×3 block of a 4×4 matrix.
#[inline]
fn mat4_det3(m: &[[f64; 4]; 4]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// Invert the 3×3 linear part of `m` via cofactors, then build the full
/// 4×4 inverse.  Returns `None` if the matrix is singular.
fn mat4_inv(m: &[[f64; 4]; 4]) -> Option<[[f64; 4]; 4]> {
    let det = mat4_det3(m);
    if det.abs() <= 1.0e-15 {
        return None;
    }
    let inv = 1.0 / det;

    let r00 = (m[1][1] * m[2][2] - m[1][2] * m[2][1]) * inv;
    let r01 = (m[0][2] * m[2][1] - m[0][1] * m[2][2]) * inv;
    let r02 = (m[0][1] * m[1][2] - m[0][2] * m[1][1]) * inv;

    let r10 = (m[1][2] * m[2][0] - m[1][0] * m[2][2]) * inv;
    let r11 = (m[0][0] * m[2][2] - m[0][2] * m[2][0]) * inv;
    let r12 = (m[0][2] * m[1][0] - m[0][0] * m[1][2]) * inv;

    let r20 = (m[1][0] * m[2][1] - m[1][1] * m[2][0]) * inv;
    let r21 = (m[0][1] * m[2][0] - m[0][0] * m[2][1]) * inv;
    let r22 = (m[0][0] * m[1][1] - m[0][1] * m[1][0]) * inv;

    // Inverse translation: -R_inv * t
    let tx = -(r00 * m[0][3] + r01 * m[1][3] + r02 * m[2][3]);
    let ty = -(r10 * m[0][3] + r11 * m[1][3] + r12 * m[2][3]);
    let tz = -(r20 * m[0][3] + r21 * m[1][3] + r22 * m[2][3]);

    Some([
        [r00, r01, r02, tx],
        [r10, r11, r12, ty],
        [r20, r21, r22, tz],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// Determine the combined form when composing `outer ∘ inner`.
fn compose_form(outer: Form, inner: Form) -> Form {
    match (outer, inner) {
        (Form::Identity, f) | (f, Form::Identity) => f,
        (Form::Translation, Form::Translation) => Form::Translation,
        (Form::Rotation, Form::Rotation) => Form::Rotation,
        (Form::Rotation, Form::Translation) | (Form::Translation, Form::Rotation) => Form::Rotation,
        (Form::PntMirror, Form::PntMirror) => Form::Rotation,
        (Form::Ax1Mirror, Form::Ax1Mirror) => Form::Rotation,
        (Form::Ax2Mirror, Form::Ax2Mirror) => Form::Rotation,
        (Form::PntMirror, Form::Ax1Mirror)
        | (Form::Ax1Mirror, Form::PntMirror)
        | (Form::PntMirror, Form::Ax2Mirror)
        | (Form::Ax2Mirror, Form::PntMirror)
        | (Form::Ax1Mirror, Form::Ax2Mirror)
        | (Form::Ax2Mirror, Form::Ax1Mirror) => Form::Rotation,
        _ => Form::CompoundTrsf,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Trsf
// ─────────────────────────────────────────────────────────────────────────────

/// A 3-D affine transformation stored as an augmented 4×4 row-major matrix.
///
/// `m[r][c]` with `r,c` in `0..4`.  The bottom row is always `[0,0,0,1]`.
/// The top-left 3×3 block is the linear (vectorial) part; column 3 (rows 0–2)
/// is the translation.
///
/// The applied map for a point `p` is:
/// ```text
/// p' = m[0..3][0..3] * p + m[0..3][3]
/// ```
// occt: gp_Trsf
#[derive(Clone, Copy, Debug)]
pub struct Trsf {
    /// Augmented 4×4 row-major transformation matrix.
    pub m: [[f64; 4]; 4],
    /// Transformation kind tag.
    form: Form,
    /// Signed scale factor (positive = orientation-preserving, negative = mirror).
    scale: f64,
}

impl Default for Trsf {
    fn default() -> Self {
        Self::identity()
    }
}

impl Trsf {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// Identity transformation (OCCT default constructor).
    pub const fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            form: Form::Identity,
            scale: 1.0,
        }
    }

    // ── Primary setters ────────────────────────────────────────────────────

    /// `SetTranslation(Vec)` — pure translation by `v`.
    pub fn set_translation(&mut self, v: Pnt) {
        self.m = [
            [1.0, 0.0, 0.0, v.x],
            [0.0, 1.0, 0.0, v.y],
            [0.0, 0.0, 1.0, v.z],
            [0.0, 0.0, 0.0, 1.0],
        ];
        self.form = Form::Translation;
        self.scale = 1.0;
    }

    /// `SetTranslation(P1, P2)` — translation from point P1 to point P2.
    pub fn set_translation_2pts(&mut self, p1: Pnt, p2: Pnt) {
        self.set_translation(p2 - p1);
    }

    /// `SetScale(center, factor)` — uniform scaling by `s` about `center`.
    ///
    /// Returns `Err(TrsfError::ZeroScale)` if `factor` is zero.
    pub fn set_scale(&mut self, center: Pnt, s: f64) -> Result<(), TrsfError> {
        if s == 0.0 {
            return Err(TrsfError::ZeroScale);
        }
        let tx = center.x * (1.0 - s);
        let ty = center.y * (1.0 - s);
        let tz = center.z * (1.0 - s);
        self.m = [
            [s,   0.0, 0.0, tx],
            [0.0, s,   0.0, ty],
            [0.0, 0.0, s,   tz],
            [0.0, 0.0, 0.0, 1.0],
        ];
        self.form = Form::Scale;
        self.scale = s;
        Ok(())
    }

    /// `SetRotation(Ax1, angle)` — rotation about an axis through a point.
    pub fn set_rotation(&mut self, axis: Ax1, angle: f64) {
        let d = axis.direction.normalized();
        let r = rotation_matrix([d.x, d.y, d.z], angle);
        let p = [axis.location.x, axis.location.y, axis.location.z];
        let rp = [
            r[0][0] * p[0] + r[0][1] * p[1] + r[0][2] * p[2],
            r[1][0] * p[0] + r[1][1] * p[1] + r[1][2] * p[2],
            r[2][0] * p[0] + r[2][1] * p[1] + r[2][2] * p[2],
        ];
        self.m = [
            [r[0][0], r[0][1], r[0][2], p[0] - rp[0]],
            [r[1][0], r[1][1], r[1][2], p[1] - rp[1]],
            [r[2][0], r[2][1], r[2][2], p[2] - rp[2]],
            [0.0,     0.0,     0.0,     1.0           ],
        ];
        self.form = Form::Rotation;
        self.scale = 1.0;
    }

    /// `SetRotationRaw` — rotation about an axis through the **origin**, given as `[f64;3]`.
    fn set_rotation_raw(&mut self, axis: [f64; 3], angle: f64) {
        let r = rotation_matrix(axis, angle);
        self.m = [
            [r[0][0], r[0][1], r[0][2], 0.0],
            [r[1][0], r[1][1], r[1][2], 0.0],
            [r[2][0], r[2][1], r[2][2], 0.0],
            [0.0,     0.0,     0.0,     1.0],
        ];
        self.form = Form::Rotation;
        self.scale = 1.0;
    }

    /// `SetMirror(Point)` — central symmetry (point mirror) about `p`.
    pub fn set_mirror_point(&mut self, p: Pnt) {
        self.m = [
            [-1.0, 0.0, 0.0, 2.0 * p.x],
            [0.0, -1.0, 0.0, 2.0 * p.y],
            [0.0,  0.0, -1.0, 2.0 * p.z],
            [0.0,  0.0,  0.0, 1.0      ],
        ];
        self.form = Form::PntMirror;
        self.scale = -1.0;
    }

    /// `SetMirror(Ax1)` — axial symmetry (mirror across a line).
    pub fn set_mirror_axis(&mut self, axis: Ax1) {
        let d = axis.direction.normalized();
        let (dx, dy, dz) = (d.x, d.y, d.z);
        let r = [
            [2.0 * dx * dx - 1.0, 2.0 * dx * dy,       2.0 * dx * dz      ],
            [2.0 * dx * dy,       2.0 * dy * dy - 1.0, 2.0 * dy * dz      ],
            [2.0 * dx * dz,       2.0 * dy * dz,       2.0 * dz * dz - 1.0],
        ];
        let p = [axis.location.x, axis.location.y, axis.location.z];
        let rp = [
            r[0][0] * p[0] + r[0][1] * p[1] + r[0][2] * p[2],
            r[1][0] * p[0] + r[1][1] * p[1] + r[1][2] * p[2],
            r[2][0] * p[0] + r[2][1] * p[1] + r[2][2] * p[2],
        ];
        self.m = [
            [r[0][0], r[0][1], r[0][2], p[0] - rp[0]],
            [r[1][0], r[1][1], r[1][2], p[1] - rp[1]],
            [r[2][0], r[2][1], r[2][2], p[2] - rp[2]],
            [0.0,     0.0,     0.0,     1.0           ],
        ];
        self.form = Form::Ax1Mirror;
        self.scale = -1.0;
    }

    /// `SetMirror(Ax2)` — planar symmetry (mirror across a plane).
    pub fn set_mirror_plane(&mut self, ax2: Ax3) {
        let n = ax2.z_dir.normalized();
        let (nx, ny, nz) = (n.x, n.y, n.z);
        let r = [
            [1.0 - 2.0 * nx * nx, -2.0 * nx * ny,       -2.0 * nx * nz      ],
            [-2.0 * nx * ny,       1.0 - 2.0 * ny * ny, -2.0 * ny * nz      ],
            [-2.0 * nx * nz,       -2.0 * ny * nz,       1.0 - 2.0 * nz * nz],
        ];
        let p = [ax2.location.x, ax2.location.y, ax2.location.z];
        let rp = [
            r[0][0] * p[0] + r[0][1] * p[1] + r[0][2] * p[2],
            r[1][0] * p[0] + r[1][1] * p[1] + r[1][2] * p[2],
            r[2][0] * p[0] + r[2][1] * p[1] + r[2][2] * p[2],
        ];
        self.m = [
            [r[0][0], r[0][1], r[0][2], p[0] - rp[0]],
            [r[1][0], r[1][1], r[1][2], p[1] - rp[1]],
            [r[2][0], r[2][1], r[2][2], p[2] - rp[2]],
            [0.0,     0.0,     0.0,     1.0           ],
        ];
        self.form = Form::Ax2Mirror;
        self.scale = -1.0;
    }

    // ── Application ───────────────────────────────────────────────────────────

    /// Apply this transform to a `Pnt` (w = 1): `p' = M * [p; 1]`.
    #[inline]
    pub fn apply(&self, pt: Pnt) -> Pnt {
        let r = mat4_apply_pt(&self.m, [pt.x, pt.y, pt.z]);
        Pnt::new(r[0], r[1], r[2])
    }

    /// Apply this transform to a raw `[f64;3]` array.
    #[inline]
    pub fn apply_raw(&self, pt: [f64; 3]) -> [f64; 3] {
        mat4_apply_pt(&self.m, pt)
    }

    /// Apply only the linear (vectorial) part to a free vector (w = 0).
    ///
    /// Translation is intentionally not applied; use this for directions and
    /// vectors that should not be shifted.
    #[inline]
    pub fn apply_vec(&self, v: [f64; 3]) -> [f64; 3] {
        mat4_apply_vec(&self.m, v)
    }

    /// Apply only the linear part to a `Pnt` used as a vector (no translation).
    #[inline]
    pub fn apply_vector(&self, v: Pnt) -> Pnt {
        let r = self.apply_vec([v.x, v.y, v.z]);
        Pnt::new(r[0], r[1], r[2])
    }

    /// `Transforms(x, y, z)` — transform a coordinate triple in place.
    #[inline]
    pub fn transforms(&self, x: &mut f64, y: &mut f64, z: &mut f64) {
        let r = self.apply_raw([*x, *y, *z]);
        *x = r[0];
        *y = r[1];
        *z = r[2];
    }

    // ── Composition / Inversion ───────────────────────────────────────────────

    /// `Multiplied(other)` — compose `self ∘ other` (apply `other` first, then
    /// `self`).  Returns a new `Trsf`.
    pub fn multiplied(&self, other: &Trsf) -> Trsf {
        Trsf {
            m: mat4_mul(&self.m, &other.m),
            form: compose_form(self.form, other.form),
            scale: self.scale * other.scale,
        }
    }

    /// `Multiply(other)` — in-place `self = self ∘ other`.
    pub fn multiply(&mut self, other: &Trsf) {
        *self = self.multiplied(other);
    }

    /// `Multiplied` alias matching the requested `compose` method name.
    #[inline]
    pub fn compose(&self, other: &Trsf) -> Trsf {
        self.multiplied(other)
    }

    /// `Inverted` — returns the inverse of this transform.
    ///
    /// Returns `Err(TrsfError::Singular)` if the linear part is singular.
    pub fn inverted(&self) -> Result<Self, TrsfError> {
        let inv_m = mat4_inv(&self.m).ok_or(TrsfError::Singular)?;
        let inv_scale = if self.scale.abs() > 1.0e-15 {
            1.0 / self.scale
        } else {
            return Err(TrsfError::Singular);
        };
        Ok(Self {
            m: inv_m,
            form: self.form,
            scale: inv_scale,
        })
    }

    /// `Invert` — returns the inverse, panicking on singular transforms.
    ///
    /// This is the infallible form matching the requested `invert(&self) -> Trsf`
    /// signature.
    pub fn invert(&self) -> Trsf {
        self.inverted()
            .expect("gp_Trsf: cannot invert singular transform")
    }

    // ── Queries ───────────────────────────────────────────────────────────────

    /// `ScaleFactor` — returns the (signed) uniform scale factor.
    ///
    /// For a pure rotation it is `1.0`; for uniform scale `s` it is `s`;
    /// for a mirror it is `-1.0`.  Computed as the signed cube root of
    /// `det(linear part)`.
    pub fn scale_factor(&self) -> f64 {
        // Use the cached value when reliable (set by explicit set_* calls).
        // Fall back to computing from the matrix for CompoundTrsf.
        match self.form {
            Form::CompoundTrsf => {
                let det = mat4_det3(&self.m);
                if det >= 0.0 { det.cbrt() } else { -((-det).cbrt()) }
            }
            _ => self.scale,
        }
    }

    /// `IsNegative` — `true` when the transform reverses orientation
    /// (`det(linear part) < 0`).
    pub fn is_negative(&self) -> bool {
        mat4_det3(&self.m) < 0.0
    }

    /// `Form` — the transformation kind.
    pub fn form(&self) -> Form {
        self.form
    }

    /// `Value(row, col)` — coefficient of the augmented 4×4 homogeneous matrix.
    ///
    /// Rows 1–3 hold `[linear | t]`; row 4 is `[0, 0, 0, 1]`.
    /// Indices are **1-based**, matching OCCT's `gp_Trsf::Value`.
    pub fn value(&self, row: i32, col: i32) -> Result<f64, TrsfError> {
        match (row, col) {
            (1, 1) => Ok(self.m[0][0]),
            (1, 2) => Ok(self.m[0][1]),
            (1, 3) => Ok(self.m[0][2]),
            (1, 4) => Ok(self.m[0][3]),
            (2, 1) => Ok(self.m[1][0]),
            (2, 2) => Ok(self.m[1][1]),
            (2, 3) => Ok(self.m[1][2]),
            (2, 4) => Ok(self.m[1][3]),
            (3, 1) => Ok(self.m[2][0]),
            (3, 2) => Ok(self.m[2][1]),
            (3, 3) => Ok(self.m[2][2]),
            (3, 4) => Ok(self.m[2][3]),
            (4, 1) => Ok(0.0),
            (4, 2) => Ok(0.0),
            (4, 3) => Ok(0.0),
            (4, 4) => Ok(1.0),
            _ => Err(TrsfError::OutOfRange),
        }
    }

    /// `TranslationPart` — translation component as a `Pnt`.
    pub fn translation_part(&self) -> Pnt {
        Pnt::new(self.m[0][3], self.m[1][3], self.m[2][3])
    }

    /// `VectorialPart` — the 3×3 linear part as a row-major array.
    pub fn vectorial_part(&self) -> [[f64; 3]; 3] {
        [
            [self.m[0][0], self.m[0][1], self.m[0][2]],
            [self.m[1][0], self.m[1][1], self.m[1][2]],
            [self.m[2][0], self.m[2][1], self.m[2][2]],
        ]
    }

    /// Alias for `apply` — apply this transform to a `Pnt`.
    #[inline]
    pub fn apply_pnt(&self, p: Pnt) -> Pnt {
        self.apply(p)
    }

    /// Alias for `apply_vector` — apply only the linear part.
    #[inline]
    pub fn apply_vector_pnt(&self, v: Pnt) -> Pnt {
        self.apply_vector(v)
    }

    /// `apply_point` — alias for `apply`.
    #[inline]
    pub fn apply_point(&self, p: Pnt) -> Pnt {
        self.apply(p)
    }

    /// `PreMultiply(other)` — in-place `self = other ∘ self`.
    pub fn pre_multiply(&mut self, other: &Trsf) {
        *self = other.multiplied(self);
    }

    /// `Powered(n)` — returns `self` raised to integer power `n`.
    ///
    /// `n=0` → identity, `n<0` → inverse power.
    pub fn powered(&self, n: i32) -> Result<Trsf, TrsfError> {
        if n == 0 {
            return Ok(Trsf::identity());
        }
        if n == 1 {
            return Ok(*self);
        }
        if n == -1 {
            return self.inverted();
        }
        let mut npower = n;
        let mut base = if n < 0 {
            npower = -n;
            self.inverted()?
        } else {
            *self
        };
        let mut result = Trsf::identity();
        while npower > 0 {
            if npower & 1 == 1 {
                result.multiply(&base);
            }
            npower >>= 1;
            if npower > 0 {
                let sq = base;
                base.multiply(&sq);
            }
        }
        Ok(result)
    }

    /// `GetRotation` — extract rotation as a quaternion `[w, x, y, z]`.
    ///
    /// Returns `Err(TrsfError::Singular)` if the linear part is not a rotation.
    pub fn get_rotation(&self) -> Result<[f64; 4], TrsfError> {
        let m = &self.m;
        let trace = m[0][0] + m[1][1] + m[2][2];
        let (w, x, y, z);
        if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0; // s = 4*w
            w = 0.25 * s;
            x = (m[2][1] - m[1][2]) / s;
            y = (m[0][2] - m[2][0]) / s;
            z = (m[1][0] - m[0][1]) / s;
        } else if m[0][0] > m[1][1] && m[0][0] > m[2][2] {
            let s = (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt() * 2.0; // s = 4*x
            w = (m[2][1] - m[1][2]) / s;
            x = 0.25 * s;
            y = (m[0][1] + m[1][0]) / s;
            z = (m[0][2] + m[2][0]) / s;
        } else if m[1][1] > m[2][2] {
            let s = (1.0 + m[1][1] - m[0][0] - m[2][2]).sqrt() * 2.0; // s = 4*y
            w = (m[0][2] - m[2][0]) / s;
            x = (m[0][1] + m[1][0]) / s;
            y = 0.25 * s;
            z = (m[1][2] + m[2][1]) / s;
        } else {
            let s = (1.0 + m[2][2] - m[0][0] - m[1][1]).sqrt() * 2.0; // s = 4*z
            w = (m[1][0] - m[0][1]) / s;
            x = (m[0][2] + m[2][0]) / s;
            y = (m[1][2] + m[2][1]) / s;
            z = 0.25 * s;
        }
        Ok([w, x, y, z])
    }

    /// `SetDisplacementOfCS` — set this transform to map `from_cs` to `to_cs`.
    ///
    /// The resulting transform takes a point expressed in `from_cs` and maps
    /// it to the same point expressed in world coordinates aligned with `to_cs`.
    pub fn set_displacement_of_cs(&mut self, from_cs: Ax3, to_cs: Ax3) -> Result<(), TrsfError> {
        // Build transform: T_to_cs_inv * T_from_cs
        // But for "displacement from from_cs to to_cs" the convention is:
        // given a point P in world space, map it as if the CS moved from from_cs to to_cs.
        // This is: to_cs.location + R_to * R_from^T * (P - from_cs.location)
        let rf = [
            [from_cs.x_dir.x, from_cs.y_dir.x, from_cs.z_dir.x],
            [from_cs.x_dir.y, from_cs.y_dir.y, from_cs.z_dir.y],
            [from_cs.x_dir.z, from_cs.y_dir.z, from_cs.z_dir.z],
        ];
        let rt = [
            [to_cs.x_dir.x, to_cs.x_dir.y, to_cs.x_dir.z],
            [to_cs.y_dir.x, to_cs.y_dir.y, to_cs.y_dir.z],
            [to_cs.z_dir.x, to_cs.z_dir.y, to_cs.z_dir.z],
        ];
        // r = R_to * R_from^T (combined rotation)
        let mut r = [[0.0f64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    r[i][j] += rt[i][k] * rf[k][j];
                }
            }
        }
        // Translation: to_cs.location - r * from_cs.location
        let fl = [from_cs.location.x, from_cs.location.y, from_cs.location.z];
        let rfl = [
            r[0][0] * fl[0] + r[0][1] * fl[1] + r[0][2] * fl[2],
            r[1][0] * fl[0] + r[1][1] * fl[1] + r[1][2] * fl[2],
            r[2][0] * fl[0] + r[2][1] * fl[1] + r[2][2] * fl[2],
        ];
        let tx = to_cs.location.x - rfl[0];
        let ty = to_cs.location.y - rfl[1];
        let tz = to_cs.location.z - rfl[2];
        self.m = [
            [r[0][0], r[0][1], r[0][2], tx],
            [r[1][0], r[1][1], r[1][2], ty],
            [r[2][0], r[2][1], r[2][2], tz],
            [0.0,     0.0,     0.0,     1.0],
        ];
        self.form = Form::CompoundTrsf;
        self.scale = 1.0;
        Ok(())
    }
}


// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1.0e-12;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn pnt_eq(a: Pnt, b: Pnt) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
    }

    #[test]
    fn identity_apply() {
        let t = Trsf::identity();
        assert!(pnt_eq(t.apply(Pnt::new(1.0, 2.0, 3.0)), Pnt::new(1.0, 2.0, 3.0)));
    }

    #[test]
    fn translation_apply() {
        let mut t = Trsf::identity();
        t.set_translation(Pnt::new(1.0, 2.0, 3.0));
        assert!(pnt_eq(t.apply(Pnt::origin()), Pnt::new(1.0, 2.0, 3.0)));
        assert!(pnt_eq(t.apply(Pnt::new(1.0, 0.0, 0.0)), Pnt::new(2.0, 2.0, 3.0)));
    }

    #[test]
    fn translation_apply_vec_ignores_shift() {
        let mut t = Trsf::identity();
        t.set_translation(Pnt::new(5.0, 6.0, 7.0));
        let r = t.apply_vec([1.0, 0.0, 0.0]);
        assert!((r[0] - 1.0).abs() < EPS && r[1].abs() < EPS && r[2].abs() < EPS);
    }

    #[test]
    fn scale_apply() {
        let mut t = Trsf::identity();
        t.set_scale(Pnt::origin(), 2.0).unwrap();
        assert!(pnt_eq(t.apply(Pnt::new(1.0, 2.0, 3.0)), Pnt::new(2.0, 4.0, 6.0)));
    }

    #[test]
    fn scale_about_center() {
        let mut t = Trsf::identity();
        t.set_scale(Pnt::new(1.0, 0.0, 0.0), 2.0).unwrap();
        assert!(pnt_eq(t.apply(Pnt::new(1.0, 0.0, 0.0)), Pnt::new(1.0, 0.0, 0.0)));
        assert!(pnt_eq(t.apply(Pnt::new(2.0, 0.0, 0.0)), Pnt::new(3.0, 0.0, 0.0)));
    }

    #[test]
    fn rotation_z_90() {
        let mut t = Trsf::identity();
        let oz = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
        t.set_rotation(oz, core::f64::consts::FRAC_PI_2);
        let p = t.apply(Pnt::new(1.0, 0.0, 0.0));
        assert!(p.x.abs() < EPS);
        assert!((p.y - 1.0).abs() < EPS);
        assert!(p.z.abs() < EPS);
    }

    #[test]
    fn compose_translation_twice() {
        let mut t1 = Trsf::identity();
        t1.set_translation(Pnt::new(1.0, 0.0, 0.0));
        let t2 = t1.compose(&t1);
        assert!(pnt_eq(t2.apply(Pnt::origin()), Pnt::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn invert_translation() {
        let mut t = Trsf::identity();
        t.set_translation(Pnt::new(3.0, -1.0, 2.0));
        let inv = t.inverted().unwrap();
        let composed = t.compose(&inv);
        assert!(pnt_eq(composed.apply(Pnt::new(5.0, 5.0, 5.0)), Pnt::new(5.0, 5.0, 5.0)));
    }

    #[test]
    fn scale_factor_positive() {
        let mut t = Trsf::identity();
        t.set_scale(Pnt::origin(), 3.0).unwrap();
        assert!((t.scale_factor() - 3.0).abs() < EPS);
        assert!(!t.is_negative());
    }

    #[test]
    fn scale_factor_negative() {
        let mut t = Trsf::identity();
        t.set_scale(Pnt::origin(), -1.0).unwrap();
        assert!((t.scale_factor() - (-1.0)).abs() < EPS);
        assert!(t.is_negative());
    }

    #[test]
    fn rotation_matrix_fn() {
        let r = rotation_matrix([0.0, 1.0, 0.0], core::f64::consts::FRAC_PI_2);
        let v = [
            r[0][0] * 1.0 + r[0][1] * 0.0 + r[0][2] * 0.0,
            r[1][0] * 1.0 + r[1][1] * 0.0 + r[1][2] * 0.0,
            r[2][0] * 1.0 + r[2][1] * 0.0 + r[2][2] * 0.0,
        ];
        assert!((v[0] - 0.0).abs() < EPS);
        assert!((v[1] - 0.0).abs() < EPS);
        assert!((v[2] - (-1.0)).abs() < EPS);
    }

    #[test]
    fn form_tag_identity() {
        let t = Trsf::identity();
        assert_eq!(t.form(), Form::Identity);
    }

    #[test]
    fn form_tag_translation() {
        let mut t = Trsf::identity();
        t.set_translation(Pnt::new(1.0, 0.0, 0.0));
        assert_eq!(t.form(), Form::Translation);
    }

    #[test]
    fn form_tag_rotation() {
        let mut t = Trsf::identity();
        t.set_rotation(Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0)), 1.0);
        assert_eq!(t.form(), Form::Rotation);
    }

    #[test]
    fn value_1_based_identity() {
        let t = Trsf::identity();
        assert!(approx_eq(t.value(1, 1).unwrap(), 1.0));
        assert!(approx_eq(t.value(1, 2).unwrap(), 0.0));
        assert!(approx_eq(t.value(1, 4).unwrap(), 0.0));
        assert!(approx_eq(t.value(4, 4).unwrap(), 1.0));
        assert!(t.value(5, 1).is_err());
    }
}
