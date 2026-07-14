// FILE: rust/ironstream/crates/ironstream/src/geom_mirror.rs
//!
//! Mirror and homogeneous-transform primitives ported from OCCT's
//! `BRepBuilderAPI_Transform` / `gp_Trsf` mirror/transform API.
//!
//! All types are zero-dependency (std only) and operate on raw `[f64; 3]`
//! coordinate triples so they compose without allocation.

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Normalize a 3-vector. Returns the unit vector; panics if the input is zero.
#[inline]
fn normalize(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    assert!(len > 0.0, "geom_mirror: cannot normalize a zero vector");
    [v[0] / len, v[1] / len, v[2] / len]
}

/// Dot product of two 3-vectors.
#[inline]
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Multiply a 4×4 row-major matrix by a homogeneous column `[x, y, z, 1]`
/// and return the resulting `[x', y', z']` (dividing by the `w` component,
/// which is 1 for all affine matrices produced here).
#[inline]
fn mat4_apply(m: &[[f64; 4]; 4], p: [f64; 3]) -> [f64; 3] {
    let x = m[0][0] * p[0] + m[0][1] * p[1] + m[0][2] * p[2] + m[0][3];
    let y = m[1][0] * p[0] + m[1][1] * p[1] + m[1][2] * p[2] + m[1][3];
    let z = m[2][0] * p[0] + m[2][1] * p[1] + m[2][2] * p[2] + m[2][3];
    [x, y, z]
}

/// Multiply two 4×4 row-major matrices: `a * b`.
#[inline]
fn mat4_mul(a: &[[f64; 4]; 4], b: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut r = [[0.0f64; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            r[i][j] = a[i][0] * b[0][j]
                + a[i][1] * b[1][j]
                + a[i][2] * b[2][j]
                + a[i][3] * b[3][j];
        }
    }
    r
}

/// Identity 4×4 matrix.
const fn mat4_identity() -> [[f64; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

// ─────────────────────────────────────────────────────────────────────────────
// MirrorPlane
// ─────────────────────────────────────────────────────────────────────────────

/// A mirror plane defined by an origin point and an outward normal.
///
/// Mirrors a point `p` across the plane by the Householder reflection formula:
/// `p' = p - 2 * dot(p - origin, n) * n`
// occt-note: mirror plane (gp_Ax2 / BRepBuilderAPI_Transform planar symmetry)
#[derive(Clone, Copy, Debug)]
pub struct MirrorPlane {
    origin: [f64; 3],
    normal: [f64; 3], // always unit length
}

impl MirrorPlane {
    /// Construct a `MirrorPlane` from an `origin` point and a `normal` vector.
    /// The normal is normalized on construction.
    pub fn new(origin: [f64; 3], normal: [f64; 3]) -> Self {
        Self {
            origin,
            normal: normalize(normal),
        }
    }

    /// Return the plane origin.
    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }

    /// Return the (unit) plane normal.
    pub fn normal(&self) -> [f64; 3] {
        self.normal
    }

    /// Reflect `p` across this plane.
    ///
    /// Formula: `p' = p - 2 * dot(p - origin, n) * n`
    ///
    /// Matches OCCT's `gp_Trsf::SetMirror(gp_Ax2)` applied to a point.
    pub fn reflect_point(&self, p: [f64; 3]) -> [f64; 3] {
        let n = self.normal;
        let o = self.origin;
        // Vector from origin to p
        let d = [p[0] - o[0], p[1] - o[1], p[2] - o[2]];
        // Signed distance of p from the plane
        let dist = dot(d, n);
        // Subtract the out-of-plane component twice
        [
            p[0] - 2.0 * dist * n[0],
            p[1] - 2.0 * dist * n[1],
            p[2] - 2.0 * dist * n[2],
        ]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MirrorAxis
// ─────────────────────────────────────────────────────────────────────────────

/// A mirror axis defined by a point on the axis and a direction vector.
///
/// Reflects a point across the infinite line (axis) in 3-D space by keeping
/// the component along the axis direction and negating the two orthogonal
/// components (relative to the closest point on the axis).
// occt-note: mirror axis (gp_Ax1 / BRepBuilderAPI_Transform axial symmetry)
#[derive(Clone, Copy, Debug)]
pub struct MirrorAxis {
    origin:    [f64; 3],
    direction: [f64; 3], // always unit length
}

impl MirrorAxis {
    /// Construct a `MirrorAxis` from a point `origin` on the axis and a
    /// `direction` vector along the axis.  The direction is normalized on
    /// construction.
    pub fn new(origin: [f64; 3], direction: [f64; 3]) -> Self {
        Self {
            origin,
            direction: normalize(direction),
        }
    }

    /// Return the axis origin.
    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }

    /// Return the (unit) axis direction.
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    /// Reflect `p` across this axis (the infinite line through `origin` along
    /// `direction`).
    ///
    /// Decompose `p - origin` into a component along the axis (`d_parallel`)
    /// and a perpendicular component (`d_perp`).  The reflected point is
    /// `origin + d_parallel - d_perp`.
    ///
    /// Matches OCCT's `gp_Trsf::SetMirror(gp_Ax1)` applied to a point.
    pub fn reflect_point(&self, p: [f64; 3]) -> [f64; 3] {
        let ax = self.direction;
        let o  = self.origin;
        // Vector from axis origin to p
        let v = [p[0] - o[0], p[1] - o[1], p[2] - o[2]];
        // Scalar projection onto the axis direction
        let t = dot(v, ax);
        // Parallel component
        let v_par = [t * ax[0], t * ax[1], t * ax[2]];
        // Perpendicular component: v - v_par
        let v_perp = [v[0] - v_par[0], v[1] - v_par[1], v[2] - v_par[2]];
        // Reflected: origin + v_par - v_perp
        [
            o[0] + v_par[0] - v_perp[0],
            o[1] + v_par[1] - v_perp[1],
            o[2] + v_par[2] - v_perp[2],
        ]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TransformMatrix
// ─────────────────────────────────────────────────────────────────────────────

/// A 4×4 homogeneous affine transform matrix stored in row-major order.
///
/// Represents the same mathematical object as OCCT's `gp_Trsf` when used from
/// `BRepBuilderAPI_Transform`: an affine map `p' = M * [p | 1]`.
///
/// The bottom row is always `[0, 0, 0, 1]` for affine transforms.
// occt-ref: gp_Trsf // 4×4 homogeneous transform (BRepBuilderAPI_Transform)
#[derive(Clone, Copy, Debug)]
pub struct TransformMatrix {
    pub mat: [[f64; 4]; 4],
}

impl TransformMatrix {
    /// Return the identity transform.
    pub fn new() -> Self {
        Self { mat: mat4_identity() }
    }

    /// Build a pure translation transform.
    ///
    /// Matches OCCT `gp_Trsf::SetTranslation(gp_Vec)`.
    pub fn from_translation(v: [f64; 3]) -> Self {
        let mut m = mat4_identity();
        m[0][3] = v[0];
        m[1][3] = v[1];
        m[2][3] = v[2];
        Self { mat: m }
    }

    /// Build a uniform scaling transform about an `origin` by factor `s`.
    ///
    /// The mapping is: `p' = origin + s * (p - origin)`.
    ///
    /// Matches OCCT `gp_Trsf::SetScale(gp_Pnt, factor)`.
    pub fn from_scale(s: f64, origin: [f64; 3]) -> Self {
        let ox = origin[0];
        let oy = origin[1];
        let oz = origin[2];
        // p' = s*p + (1-s)*origin
        let t = 1.0 - s;
        let mat = [
            [s,   0.0, 0.0, t * ox],
            [0.0, s,   0.0, t * oy],
            [0.0, 0.0, s,   t * oz],
            [0.0, 0.0, 0.0, 1.0   ],
        ];
        Self { mat }
    }

    /// Build a planar mirror transform from a `MirrorPlane`.
    ///
    /// The 3×3 linear part is the Householder reflection `I - 2*n*nᵀ`; the
    /// translation encodes the plane origin.
    ///
    /// Matches OCCT `gp_Trsf::SetMirror(gp_Ax2)`.
    pub fn from_mirror_plane(plane: &MirrorPlane) -> Self {
        let n = plane.normal();
        let o = plane.origin();
        let nx = n[0]; let ny = n[1]; let nz = n[2];

        // Householder: R = I - 2*n*nᵀ
        let r00 = 1.0 - 2.0 * nx * nx;
        let r11 = 1.0 - 2.0 * ny * ny;
        let r22 = 1.0 - 2.0 * nz * nz;
        let r01 = -2.0 * nx * ny;
        let r02 = -2.0 * nx * nz;
        let r12 = -2.0 * ny * nz;

        // Translation: o - R*o  (fixed-point at plane origin)
        let rox = r00 * o[0] + r01 * o[1] + r02 * o[2];
        let roy = r01 * o[0] + r11 * o[1] + r12 * o[2];
        let roz = r02 * o[0] + r12 * o[1] + r22 * o[2];

        let mat = [
            [r00, r01, r02, o[0] - rox],
            [r01, r11, r12, o[1] - roy],
            [r02, r12, r22, o[2] - roz],
            [0.0, 0.0, 0.0, 1.0       ],
        ];
        Self { mat }
    }

    /// Build an axial mirror transform from a `MirrorAxis`.
    ///
    /// The 3×3 linear part is `2*d*dᵀ - I` (rotation by π about the axis),
    /// and the translation encodes the axis origin.
    ///
    /// Matches OCCT `gp_Trsf::SetMirror(gp_Ax1)`.
    pub fn from_mirror_axis(axis: &MirrorAxis) -> Self {
        let d = axis.direction();
        let o = axis.origin();
        let dx = d[0]; let dy = d[1]; let dz = d[2];

        // R = 2*d*dᵀ - I
        let r00 = 2.0 * dx * dx - 1.0;
        let r11 = 2.0 * dy * dy - 1.0;
        let r22 = 2.0 * dz * dz - 1.0;
        let r01 = 2.0 * dx * dy;
        let r02 = 2.0 * dx * dz;
        let r12 = 2.0 * dy * dz;

        // Translation: o - R*o
        let rox = r00 * o[0] + r01 * o[1] + r02 * o[2];
        let roy = r01 * o[0] + r11 * o[1] + r12 * o[2];
        let roz = r02 * o[0] + r12 * o[1] + r22 * o[2];

        let mat = [
            [r00, r01, r02, o[0] - rox],
            [r01, r11, r12, o[1] - roy],
            [r02, r12, r22, o[2] - roz],
            [0.0, 0.0, 0.0, 1.0       ],
        ];
        Self { mat }
    }

    /// Apply this transform to the point `p`, returning the transformed point.
    pub fn apply(&self, p: [f64; 3]) -> [f64; 3] {
        mat4_apply(&self.mat, p)
    }

    /// Compose `self * other`: apply `other` first, then `self`.
    ///
    /// Matches OCCT `gp_Trsf::Multiply`.
    pub fn compose(&self, other: &TransformMatrix) -> TransformMatrix {
        TransformMatrix {
            mat: mat4_mul(&self.mat, &other.mat),
        }
    }
}

impl Default for TransformMatrix {
    fn default() -> Self {
        Self::new()
    }
}
