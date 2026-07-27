// FILE: brep_gprop.rs
//! `BRepGProp` / `GProp_GProps` — mass properties of boundary-representation
//! shapes, mirroring OpenCascade's `BRepGProp` package and `GProp_GProps`.
//!
//! # Scope
//!
//! * [`GProps`] — accumulator for mass properties (mass, centre of mass,
//!   inertia matrix).  Mirrors `GProp_GProps`.
//! * [`BRepGProp`] — static methods that compute volume/surface/linear
//!   properties by integrating over a shape.  Mirrors `BRepGProp`.
//! * [`InertiaMatrix`] — a symmetric 3×3 matrix of second moments.
//! * [`PrincipalProps`] — principal axes and radii of gyration.
//!
//! # Algorithm
//!
//! Volume properties use the **divergence theorem**: for a closed, outward-
//! oriented triangle mesh the volume, centroid, and inertia integrals reduce to
//! sums over faces.  Surface properties simply sum over triangles.  Linear
//! (edge-length) properties walk the edges of a topology graph.
//!
//! All integrals are exact for linear (triangle) elements and converge to the
//! analytic answer as the mesh is refined.

use crate::gp::{Ax1, Pnt};
use crate::mesh::TriMesh;
use crate::topods::{Face, Solid, Wire};

// ─────────────────────────────────────────────────────────────────────────────
// InertiaMatrix
// ─────────────────────────────────────────────────────────────────────────────

/// A symmetric 3×3 matrix of second moments of inertia (upper-triangular
/// storage: `ixx`, `iyy`, `izz`, `ixy`, `ixz`, `iyz`).
///
/// Storage layout (`gp_Mat` convention):
///
/// ```text
///  [ ixx  ixy  ixz ]
///  [ ixy  iyy  iyz ]
///  [ ixz  iyz  izz ]
/// ```
// occt: GProp_GProps // (inertia matrix field), gp_Mat
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InertiaMatrix {
    pub ixx: f64,
    pub iyy: f64,
    pub izz: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyz: f64,
}

impl Default for InertiaMatrix {
    fn default() -> Self {
        Self::zero()
    }
}

impl InertiaMatrix {
    /// Zero matrix.
    pub const fn zero() -> Self {
        Self {
            ixx: 0.0,
            iyy: 0.0,
            izz: 0.0,
            ixy: 0.0,
            ixz: 0.0,
            iyz: 0.0,
        }
    }

    /// Add another inertia matrix component-wise.
    pub fn add(&self, other: &InertiaMatrix) -> InertiaMatrix {
        InertiaMatrix {
            ixx: self.ixx + other.ixx,
            iyy: self.iyy + other.iyy,
            izz: self.izz + other.izz,
            ixy: self.ixy + other.ixy,
            ixz: self.ixz + other.ixz,
            iyz: self.iyz + other.iyz,
        }
    }

    /// Scale all components by a scalar.
    pub fn scale(&self, s: f64) -> InertiaMatrix {
        InertiaMatrix {
            ixx: self.ixx * s,
            iyy: self.iyy * s,
            izz: self.izz * s,
            ixy: self.ixy * s,
            ixz: self.ixz * s,
            iyz: self.iyz * s,
        }
    }

    /// Trace = ixx + iyy + izz.
    pub fn trace(&self) -> f64 {
        self.ixx + self.iyy + self.izz
    }

    /// Apply the parallel-axis (Steiner) theorem: shift inertia matrix from the
    /// centre of mass `com` to a new reference point `ref_pt`, for a body of
    /// mass `m`.
    ///
    /// `I_ref = I_com + m * (|d|² * Id  -  d ⊗ d)`
    ///
    /// where `d = com - ref_pt`.
    pub fn shift_to_point(&self, com: Pnt, ref_pt: Pnt, mass: f64) -> InertiaMatrix {
        let d = com - ref_pt;
        let d2 = d.dot(d);
        InertiaMatrix {
            ixx: self.ixx + mass * (d2 - d.x * d.x),
            iyy: self.iyy + mass * (d2 - d.y * d.y),
            izz: self.izz + mass * (d2 - d.z * d.z),
            ixy: self.ixy - mass * d.x * d.y,
            ixz: self.ixz - mass * d.x * d.z,
            iyz: self.iyz - mass * d.y * d.z,
        }
    }

    /// Moment of inertia about an arbitrary axis through the origin.
    ///
    /// `I_axis = n^T · I · n`
    pub fn moment_about(&self, axis_dir: Pnt) -> f64 {
        let n = axis_dir.normalized();
        self.ixx * n.x * n.x
            + self.iyy * n.y * n.y
            + self.izz * n.z * n.z
            + 2.0 * self.ixy * n.x * n.y
            + 2.0 * self.ixz * n.x * n.z
            + 2.0 * self.iyz * n.y * n.z
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PrincipalProps
// ─────────────────────────────────────────────────────────────────────────────

/// Principal inertia moments and axes computed by eigendecomposition of the
/// inertia matrix.  Mirrors the data returned by
/// `GProp_GProps::PrincipalProperties`.
// occt: GProp_PrincipalProps
#[derive(Clone, Copy, Debug)]
pub struct PrincipalProps {
    /// First (smallest) principal moment.
    pub i1: f64,
    /// Second principal moment.
    pub i2: f64,
    /// Third (largest) principal moment.
    pub i3: f64,
    /// Axis of first principal moment (unit vector).
    pub v1: Pnt,
    /// Axis of second principal moment (unit vector).
    pub v2: Pnt,
    /// Axis of third principal moment (unit vector).
    pub v3: Pnt,
}

impl PrincipalProps {
    /// Radius of gyration for principal moment `i` about axis `vi`:
    /// `r = sqrt(I / mass)`.
    pub fn radius_of_gyration(&self, mass: f64) -> (f64, f64, f64) {
        let r = |i: f64| if mass > 0.0 { (i / mass).abs().sqrt() } else { 0.0 };
        (r(self.i1), r(self.i2), r(self.i3))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GProps  (mirrors GProp_GProps)
// ─────────────────────────────────────────────────────────────────────────────

/// Accumulator for mass properties.
///
/// Mirrors OCCT's `GProp_GProps`: stores mass (volume, area, or length),
/// centre of mass, and the second-moment inertia matrix relative to the
/// origin.  Multiple `GProps` can be combined with [`GProps::add`].
// occt: GProp_GProps
#[derive(Clone, Debug)]
pub struct GProps {
    /// Accumulated scalar measure (volume, area, or arc-length).
    mass: f64,
    /// First moment sum (`mass * centre_of_mass`) used for weighted centroid.
    moment: Pnt,
    /// Inertia matrix relative to the *origin* (not the CoM).
    inertia_origin: InertiaMatrix,
}

impl Default for GProps {
    fn default() -> Self {
        GProps {
            mass: 0.0,
            moment: Pnt::origin(),
            inertia_origin: InertiaMatrix::zero(),
        }
    }
}

impl GProps {
    /// Create an empty accumulator.
    // occt: GProp_GProps // ::GProp_GProps()
    pub fn new() -> Self {
        Self::default()
    }

    // ── query ─────────────────────────────────────────────────────────────────

    /// Total mass (volume / area / length depending on what was accumulated).
    // occt: GProp_GProps // ::Mass
    pub fn mass(&self) -> f64 {
        self.mass
    }

    /// Centre of mass.
    // occt: GProp_GProps // ::CentreOfMass
    pub fn centre_of_mass(&self) -> Pnt {
        if self.mass.abs() < 1e-15 {
            Pnt::origin()
        } else {
            self.moment * (1.0 / self.mass)
        }
    }

    /// Inertia matrix relative to the centre of mass.
    // occt: GProp_GProps // ::MatrixOfInertia
    pub fn matrix_of_inertia(&self) -> InertiaMatrix {
        if self.mass.abs() < 1e-15 {
            return self.inertia_origin;
        }
        let com = self.centre_of_mass();
        // Shift I_origin to I_com via the parallel-axis theorem (reversed):
        // I_com = I_origin - m * (|com|² * Id  -  com ⊗ com)
        let d2 = com.dot(com);
        InertiaMatrix {
            ixx: self.inertia_origin.ixx - self.mass * (d2 - com.x * com.x),
            iyy: self.inertia_origin.iyy - self.mass * (d2 - com.y * com.y),
            izz: self.inertia_origin.izz - self.mass * (d2 - com.z * com.z),
            ixy: self.inertia_origin.ixy + self.mass * com.x * com.y,
            ixz: self.inertia_origin.ixz + self.mass * com.x * com.z,
            iyz: self.inertia_origin.iyz + self.mass * com.y * com.z,
        }
    }

    /// Static moment vector = `mass * centre_of_mass`.
    // occt: GProp_GProps // ::StaticMoments
    pub fn static_moments(&self) -> (f64, f64, f64) {
        (self.moment.x, self.moment.y, self.moment.z)
    }

    /// Moment of inertia about an arbitrary `Ax1` (line).
    ///
    /// Uses the inertia matrix relative to the axis origin, shifted via the
    /// parallel-axis theorem.
    // occt: GProp_GProps // ::MomentOfInertia(Ax1)
    pub fn moment_of_inertia(&self, axis: &Ax1) -> f64 {
        // Shift inertia matrix from origin to the axis location.
        let com = self.centre_of_mass();
        let i_com = self.matrix_of_inertia();
        // Shift from CoM to axis location.
        let i_at_ax = i_com.shift_to_point(com, axis.location, self.mass);
        i_at_ax.moment_about(axis.direction)
    }

    /// Radius of gyration about an axis.
    // occt: GProp_GProps // ::RadiusOfGyration(Ax1)
    pub fn radius_of_gyration(&self, axis: &Ax1) -> f64 {
        let i = self.moment_of_inertia(axis);
        if self.mass > 0.0 {
            (i.abs() / self.mass).sqrt()
        } else {
            0.0
        }
    }

    /// Principal inertia properties (eigendecomposition of the CoM-relative
    /// inertia matrix).
    // occt: GProp_GProps // ::PrincipalProperties
    pub fn principal_properties(&self) -> PrincipalProps {
        let mat = self.matrix_of_inertia();
        jacobi_eigen(&mat)
    }

    // ── mutation ──────────────────────────────────────────────────────────────

    /// Add another `GProps` into this accumulator.
    // occt: GProp_GProps // ::Add(GProps, coeff)
    pub fn add(&mut self, other: &GProps) {
        self.add_scaled(other, 1.0);
    }

    /// Add another `GProps` scaled by `coeff`.
    pub fn add_scaled(&mut self, other: &GProps, coeff: f64) {
        self.mass += other.mass * coeff;
        self.moment = self.moment + other.moment * coeff;
        self.inertia_origin = self
            .inertia_origin
            .add(&other.inertia_origin.scale(coeff));
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Jacobi eigendecomposition (symmetric 3×3) — for PrincipalProperties
// ─────────────────────────────────────────────────────────────────────────────

/// Cyclic Jacobi iteration for a symmetric 3×3 matrix.
/// Returns eigenvalues sorted ascending with corresponding eigenvectors.
fn jacobi_eigen(m: &InertiaMatrix) -> PrincipalProps {
    // Build 3×3 symmetric matrix as flat array (row-major).
    let mut a = [
        [m.ixx, m.ixy, m.ixz],
        [m.ixy, m.iyy, m.iyz],
        [m.ixz, m.iyz, m.izz],
    ];
    // Eigenvector matrix (starts as identity).
    let mut v = [[1.0_f64, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    for _ in 0..50 {
        // Off-diagonal norm.
        let off = (a[0][1] * a[0][1] + a[0][2] * a[0][2] + a[1][2] * a[1][2]).sqrt();
        if off < 1e-15 {
            break;
        }
        // Sweep over all off-diagonal pairs (p, q) where p < q.
        for p in 0..2_usize {
            for q in (p + 1)..3_usize {
                let apq = a[p][q];
                if apq.abs() < 1e-15 {
                    continue;
                }
                let theta = 0.5 * (a[q][q] - a[p][p]) / apq;
                let t = if theta >= 0.0 {
                    1.0 / (theta + (1.0 + theta * theta).sqrt())
                } else {
                    1.0 / (theta - (1.0 + theta * theta).sqrt())
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;

                // Update matrix A ← J^T A J.
                let app = a[p][p] - t * apq;
                let aqq = a[q][q] + t * apq;
                a[p][p] = app;
                a[q][q] = aqq;
                a[p][q] = 0.0;
                a[q][p] = 0.0;

                for r in 0..3_usize {
                    if r != p && r != q {
                        let arp = a[r][p];
                        let arq = a[r][q];
                        a[r][p] = c * arp - s * arq;
                        a[p][r] = a[r][p];
                        a[r][q] = s * arp + c * arq;
                        a[q][r] = a[r][q];
                    }
                }

                // Update eigenvector columns.
                for r in 0..3_usize {
                    let vrp = v[r][p];
                    let vrq = v[r][q];
                    v[r][p] = c * vrp - s * vrq;
                    v[r][q] = s * vrp + c * vrq;
                }
            }
        }
    }

    // Eigenvalues on diagonal; eigenvectors in columns of v.
    let mut pairs = [
        (a[0][0], [v[0][0], v[1][0], v[2][0]]),
        (a[1][1], [v[0][1], v[1][1], v[2][1]]),
        (a[2][2], [v[0][2], v[1][2], v[2][2]]),
    ];
    // Sort ascending by eigenvalue.
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let pnt = |arr: [f64; 3]| Pnt::new(arr[0], arr[1], arr[2]).normalized();

    PrincipalProps {
        i1: pairs[0].0,
        i2: pairs[1].0,
        i3: pairs[2].0,
        v1: pnt(pairs[0].1),
        v2: pnt(pairs[1].1),
        v3: pnt(pairs[2].1),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal integration helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Accumulate volume mass properties for a single triangle `(a, b, c)`
/// (assumed outward-facing) using the divergence theorem decomposed into
/// signed tetrahedra from the origin.
///
/// For each triangle we form the tetrahedron (O, a, b, c) and accumulate
/// analytic integrals.  Summing over all faces gives the exact integrals for
/// a closed polyhedron (the divergence theorem).
///
/// Inertia integrals for a single tetrahedron (O, v0, v1, v2):
///   ∫ x^2 dV = (v0x^2 + v1x^2 + v2x^2 + v0x*v1x + v0x*v2x + v1x*v2x) * det/60
///   ∫ xy   dV = (2*v0x*v0y + 2*v1x*v1y + 2*v2x*v2y
///                + v0x*v1y + v1x*v0y + v0x*v2y + v2x*v0y
///                + v1x*v2y + v2x*v1y) * det/120
/// where det = v0·(v1×v2)  (signed volume = det/6).
///
/// Reference: "Explicit Exact Formulas for the 3-D Tetrahedron Inertia Tensor
/// in Terms of its Vertex Coordinates", F. Tonon (2004).
fn accum_volume_tri(props: &mut GProps, a: Pnt, b: Pnt, c: Pnt) {
    // det = a · (b × c) = 6 * signed_volume of tetrahedron (O, a, b, c).
    let cross_bc = b.cross(c);
    let det = a.dot(cross_bc); // = 6 * signed_vol.
    let signed_vol = det / 6.0;

    props.mass += signed_vol;

    // Centroid contribution: integral of position over this tetrahedron.
    // ∫ r dV = (a + b + c) * det / 24.
    let sum = Pnt::new(a.x + b.x + c.x, a.y + b.y + c.y, a.z + b.z + c.z);
    props.moment = props.moment + sum * (det / 24.0);

    // Inertia integrals relative to the origin using Tonon 2004.
    let (x0, y0, z0) = (a.x, a.y, a.z);
    let (x1, y1, z1) = (b.x, b.y, b.z);
    let (x2, y2, z2) = (c.x, c.y, c.z);

    // ∫ x^2 dV * 60 / det, and similarly for y, z.
    let xx = x0 * x0 + x1 * x1 + x2 * x2 + x0 * x1 + x0 * x2 + x1 * x2;
    let yy = y0 * y0 + y1 * y1 + y2 * y2 + y0 * y1 + y0 * y2 + y1 * y2;
    let zz = z0 * z0 + z1 * z1 + z2 * z2 + z0 * z1 + z0 * z2 + z1 * z2;

    // ∫ xy dV * 120 / det, etc.
    let xy = 2.0 * x0 * y0 + 2.0 * x1 * y1 + 2.0 * x2 * y2
        + x0 * y1 + x1 * y0
        + x0 * y2 + x2 * y0
        + x1 * y2 + x2 * y1;
    let xz = 2.0 * x0 * z0 + 2.0 * x1 * z1 + 2.0 * x2 * z2
        + x0 * z1 + x1 * z0
        + x0 * z2 + x2 * z0
        + x1 * z2 + x2 * z1;
    let yz = 2.0 * y0 * z0 + 2.0 * y1 * z1 + 2.0 * y2 * z2
        + y0 * z1 + y1 * z0
        + y0 * z2 + y2 * z0
        + y1 * z2 + y2 * z1;

    // I_xx = ∫(y² + z²) dV, I_yy = ∫(x² + z²) dV, I_zz = ∫(x² + y²) dV.
    props.inertia_origin.ixx += det * (yy + zz) / 60.0;
    props.inertia_origin.iyy += det * (xx + zz) / 60.0;
    props.inertia_origin.izz += det * (xx + yy) / 60.0;
    // Products of inertia: I_xy = -∫xy dV, etc. (standard sign convention).
    props.inertia_origin.ixy -= det * xy / 120.0;
    props.inertia_origin.ixz -= det * xz / 120.0;
    props.inertia_origin.iyz -= det * yz / 120.0;
}

/// Accumulate surface-area mass properties for a single triangle.
fn accum_surface_tri(props: &mut GProps, a: Pnt, b: Pnt, c: Pnt) {
    let edge1 = b - a;
    let edge2 = c - a;
    let cross = edge1.cross(edge2);
    let area = cross.norm() * 0.5;

    if area < 1e-20 {
        return;
    }

    props.mass += area;

    // Centroid of this triangle.
    let cen = Pnt::new(
        (a.x + b.x + c.x) / 3.0,
        (a.y + b.y + c.y) / 3.0,
        (a.z + b.z + c.z) / 3.0,
    );
    props.moment = props.moment + cen * area;

    // Inertia contribution: I = area * (|cen|² * Id  -  cen ⊗ cen)
    // plus the "self" contribution of the triangle about its centroid.
    // For thin surfaces the dominant term is the parallel-axis shift.
    let d2 = cen.dot(cen);
    props.inertia_origin.ixx += area * (d2 - cen.x * cen.x);
    props.inertia_origin.iyy += area * (d2 - cen.y * cen.y);
    props.inertia_origin.izz += area * (d2 - cen.z * cen.z);
    props.inertia_origin.ixy -= area * cen.x * cen.y;
    props.inertia_origin.ixz -= area * cen.x * cen.z;
    props.inertia_origin.iyz -= area * cen.y * cen.z;
}

/// Accumulate linear (length) mass properties for a single edge segment.
fn accum_linear_edge(props: &mut GProps, a: Pnt, b: Pnt) {
    let diff = b - a;
    let length = diff.norm();

    if length < 1e-20 {
        return;
    }

    props.mass += length;

    let mid = Pnt::new(
        (a.x + b.x) * 0.5,
        (a.y + b.y) * 0.5,
        (a.z + b.z) * 0.5,
    );
    props.moment = props.moment + mid * length;

    // Inertia contribution for the segment about its midpoint, then shifted to
    // origin via parallel-axis theorem.
    // Exact formula for a uniform slender rod of length L, mass m:
    //   I_self = m * (L/2)^2 / 3  about each axis perpendicular to the rod.
    let l2 = length * length;
    let dx = diff.x / length;
    let dy = diff.y / length;
    let dz = diff.z / length;
    // I_self about axes perpendicular to rod direction:
    //   ΔI_self = (m * l^2 / 12) * (Id - d⊗d)
    let self_factor = length * l2 / 12.0;
    // Parallel-axis shift from mid to origin.
    let d2 = mid.dot(mid);
    props.inertia_origin.ixx +=
        self_factor * (1.0 - dx * dx) + length * (d2 - mid.x * mid.x);
    props.inertia_origin.iyy +=
        self_factor * (1.0 - dy * dy) + length * (d2 - mid.y * mid.y);
    props.inertia_origin.izz +=
        self_factor * (1.0 - dz * dz) + length * (d2 - mid.z * mid.z);
    props.inertia_origin.ixy +=
        -self_factor * dx * dy - length * mid.x * mid.y;
    props.inertia_origin.ixz +=
        -self_factor * dx * dz - length * mid.x * mid.z;
    props.inertia_origin.iyz +=
        -self_factor * dy * dz - length * mid.y * mid.z;
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepGProp
// ─────────────────────────────────────────────────────────────────────────────

/// Static methods for computing mass properties of shapes.
///
/// Mirrors OCCT's `BRepGProp` class (static methods only).
// occt: BRepGProp
pub struct BRepGProp;

impl BRepGProp {
    /// Compute **volumetric** properties of a closed solid.
    ///
    /// Uses the divergence-theorem integration over the triangulated boundary.
    /// Returns a [`GProps`] where `mass()` is the enclosed volume.
    ///
    /// Mirrors `BRepGProp::VolumeProperties(shape, props)`.
    // occt: BRepGProp // ::VolumeProperties
    pub fn volume_properties(solid: &Solid) -> GProps {
        let mut props = GProps::new();
        let mesh = solid.mesh();
        for tri in &mesh.tris {
            let a = mesh.verts[tri[0]];
            let b = mesh.verts[tri[1]];
            let c = mesh.verts[tri[2]];
            accum_volume_tri(&mut props, a, b, c);
        }
        // The divergence theorem gives a signed result; take absolute value.
        if props.mass < 0.0 {
            props.mass = -props.mass;
            props.moment = Pnt::new(-props.moment.x, -props.moment.y, -props.moment.z);
            props.inertia_origin = props.inertia_origin.scale(-1.0);
        }
        props
    }

    /// Compute **surface area** properties of a solid's boundary.
    ///
    /// Returns a [`GProps`] where `mass()` is the total surface area.
    ///
    /// Mirrors `BRepGProp::SurfaceProperties(shape, props)`.
    // occt: BRepGProp // ::SurfaceProperties
    pub fn surface_properties(solid: &Solid) -> GProps {
        let mut props = GProps::new();
        let mesh = solid.mesh();
        for tri in &mesh.tris {
            let a = mesh.verts[tri[0]];
            let b = mesh.verts[tri[1]];
            let c = mesh.verts[tri[2]];
            accum_surface_tri(&mut props, a, b, c);
        }
        props
    }

    /// Compute **surface area** properties directly from a [`TriMesh`].
    ///
    /// Convenience overload for raw mesh data.
    // occt: BRepGProp // ::SurfaceProperties
    pub fn surface_properties_mesh(mesh: &TriMesh) -> GProps {
        let mut props = GProps::new();
        for tri in &mesh.tris {
            let a = mesh.verts[tri[0]];
            let b = mesh.verts[tri[1]];
            let c = mesh.verts[tri[2]];
            accum_surface_tri(&mut props, a, b, c);
        }
        props
    }

    /// Compute **linear** (arc-length) properties of a planar wire.
    ///
    /// The wire is a closed polygon; properties are accumulated over each edge
    /// segment.  Returns a [`GProps`] where `mass()` is the total perimeter.
    ///
    /// Mirrors `BRepGProp::LinearProperties(shape, props)`.
    // occt: BRepGProp // ::LinearProperties
    pub fn linear_properties(wire: &Wire) -> GProps {
        let mut props = GProps::new();
        let pts = &wire.pts;
        let n = pts.len();
        if n < 2 {
            return props;
        }
        for i in 0..n {
            let a = pts[i];
            let b = pts[(i + 1) % n];
            accum_linear_edge(&mut props, a, b);
        }
        props
    }

    /// Compute **surface area** properties of a planar `Face` by triangulating
    /// it.
    ///
    /// Mirrors `BRepGProp::SurfaceProperties` called with a `TopoDS_Face`.
    // occt: BRepGProp // ::SurfaceProperties (TopoDS_Face overload)
    pub fn face_surface_properties(face: &Face) -> GProps {
        let mut props = GProps::new();
        for tri in face.triangulate() {
            accum_surface_tri(&mut props, tri[0], tri[1], tri[2]);
        }
        props
    }

    /// Compute **volumetric** properties directly from a triangle mesh.
    ///
    /// Useful when the caller already has a [`TriMesh`] without wrapping it in
    /// a [`Solid`].
    // occt: BRepGProp // ::VolumeProperties (mesh overload)
    pub fn volume_properties_mesh(mesh: &TriMesh) -> GProps {
        let mut props = GProps::new();
        for tri in &mesh.tris {
            let a = mesh.verts[tri[0]];
            let b = mesh.verts[tri[1]];
            let c = mesh.verts[tri[2]];
            accum_volume_tri(&mut props, a, b, c);
        }
        if props.mass < 0.0 {
            props.mass = -props.mass;
            props.moment = Pnt::new(-props.moment.x, -props.moment.y, -props.moment.z);
            props.inertia_origin = props.inertia_origin.scale(-1.0);
        }
        props
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper: build a simple unit-cube triangle mesh for tests
// ─────────────────────────────────────────────────────────────────────────────

#[allow(dead_code)]
fn unit_cube_mesh() -> TriMesh {
    use crate::mesh::TriMesh;
    let mut m = TriMesh::new();
    let p = |x: f64, y: f64, z: f64| Pnt::new(x, y, z);
    let mut quad = |a: Pnt, b: Pnt, c: Pnt, d: Pnt| {
        m.push_triangle(a, b, c);
        m.push_triangle(a, c, d);
    };
    quad(p(0., 0., 0.), p(0., 1., 0.), p(1., 1., 0.), p(1., 0., 0.)); // -Z
    quad(p(0., 0., 1.), p(1., 0., 1.), p(1., 1., 1.), p(0., 1., 1.)); // +Z
    quad(p(0., 0., 0.), p(1., 0., 0.), p(1., 0., 1.), p(0., 0., 1.)); // -Y
    quad(p(1., 0., 0.), p(1., 1., 0.), p(1., 1., 1.), p(1., 0., 1.)); // +X
    quad(p(1., 1., 0.), p(0., 1., 0.), p(0., 1., 1.), p(1., 1., 1.)); // +Y
    quad(p(0., 1., 0.), p(0., 0., 0.), p(0., 0., 1.), p(0., 1., 1.)); // -X
    m
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::{Ax1, Pnt};
    use crate::mesh::TriMesh;
    use crate::topods::{Solid, Wire};
    use std::f64::consts::PI;

    const TOL: f64 = 1e-6;

    fn near(a: f64, b: f64, tol: f64, label: &str) {
        assert!(
            (a - b).abs() <= tol,
            "{label}: expected {a} ≈ {b} (tol={tol}), diff={}",
            (a - b).abs()
        );
    }

    // Helper: axis-aligned box mesh centered at origin.
    fn box_mesh(dx: f64, dy: f64, dz: f64) -> TriMesh {
        let p = |x: f64, y: f64, z: f64| Pnt::new(x * dx, y * dy, z * dz);
        let mut m = TriMesh::new();
        let mut quad = |a: Pnt, b: Pnt, c: Pnt, d: Pnt| {
            m.push_triangle(a, b, c);
            m.push_triangle(a, c, d);
        };
        quad(p(0., 0., 0.), p(0., 1., 0.), p(1., 1., 0.), p(1., 0., 0.));
        quad(p(0., 0., 1.), p(1., 0., 1.), p(1., 1., 1.), p(0., 1., 1.));
        quad(p(0., 0., 0.), p(1., 0., 0.), p(1., 0., 1.), p(0., 0., 1.));
        quad(p(1., 0., 0.), p(1., 1., 0.), p(1., 1., 1.), p(1., 0., 1.));
        quad(p(1., 1., 0.), p(0., 1., 0.), p(0., 1., 1.), p(1., 1., 1.));
        quad(p(0., 1., 0.), p(0., 0., 0.), p(0., 0., 1.), p(0., 1., 1.));
        m
    }

    #[test]
    fn unit_cube_volume() {
        let mesh = unit_cube_mesh();
        let solid = Solid::from_mesh(mesh);
        let props = BRepGProp::volume_properties(&solid);
        near(props.mass(), 1.0, TOL, "unit_cube_volume");
    }

    #[test]
    fn unit_cube_surface_area() {
        let mesh = unit_cube_mesh();
        let solid = Solid::from_mesh(mesh);
        let props = BRepGProp::surface_properties(&solid);
        near(props.mass(), 6.0, TOL, "unit_cube_surface_area");
    }

    #[test]
    fn unit_cube_centre_of_mass_volume() {
        let mesh = unit_cube_mesh();
        let solid = Solid::from_mesh(mesh);
        let props = BRepGProp::volume_properties(&solid);
        let com = props.centre_of_mass();
        near(com.x, 0.5, TOL, "com_x");
        near(com.y, 0.5, TOL, "com_y");
        near(com.z, 0.5, TOL, "com_z");
    }

    #[test]
    fn unit_cube_centre_of_mass_surface() {
        let mesh = unit_cube_mesh();
        let solid = Solid::from_mesh(mesh);
        let props = BRepGProp::surface_properties(&solid);
        let com = props.centre_of_mass();
        near(com.x, 0.5, 1e-5, "surf_com_x");
        near(com.y, 0.5, 1e-5, "surf_com_y");
        near(com.z, 0.5, 1e-5, "surf_com_z");
    }

    #[test]
    fn box_2x3x4_volume() {
        let mesh = box_mesh(2.0, 3.0, 4.0);
        let props = BRepGProp::volume_properties_mesh(&mesh);
        near(props.mass(), 24.0, TOL, "2x3x4_volume");
    }

    #[test]
    fn box_2x3x4_surface_area() {
        // Area = 2*(2*3 + 3*4 + 2*4) = 2*(6+12+8) = 52.
        let mesh = box_mesh(2.0, 3.0, 4.0);
        let props = BRepGProp::surface_properties_mesh(&mesh);
        near(props.mass(), 52.0, TOL, "2x3x4_area");
    }

    #[test]
    fn wire_perimeter_square() {
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
        ]);
        let props = BRepGProp::linear_properties(&w);
        near(props.mass(), 4.0, TOL, "square_perimeter");
    }

    #[test]
    fn wire_centroid_square() {
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
        ]);
        let props = BRepGProp::linear_properties(&w);
        let com = props.centre_of_mass();
        near(com.x, 0.5, TOL, "wire_com_x");
        near(com.y, 0.5, TOL, "wire_com_y");
    }

    #[test]
    fn inertia_matrix_unit_cube_volume() {
        // For a solid unit cube (side 1, density 1, mass = 1):
        // I_cm_xx = I_cm_yy = I_cm_zz = m*(b^2+c^2)/12 = 1*2/12 = 1/6
        let mesh = unit_cube_mesh();
        let solid = Solid::from_mesh(mesh);
        let props = BRepGProp::volume_properties(&solid);
        let mat = props.matrix_of_inertia();
        near(mat.ixx, 1.0 / 6.0, 1e-4, "ixx");
        near(mat.iyy, 1.0 / 6.0, 1e-4, "iyy");
        near(mat.izz, 1.0 / 6.0, 1e-4, "izz");
        // Off-diagonal should be ~0 by symmetry.
        near(mat.ixy.abs(), 0.0, 1e-4, "|ixy|");
        near(mat.ixz.abs(), 0.0, 1e-4, "|ixz|");
        near(mat.iyz.abs(), 0.0, 1e-4, "|iyz|");
    }

    #[test]
    fn gprops_add_two_cubes() {
        let m1 = unit_cube_mesh();
        let s1 = Solid::from_mesh(m1);
        let p1 = BRepGProp::volume_properties(&s1);

        let m2 = {
            // Cube shifted by (2, 0, 0).
            let mut m = unit_cube_mesh();
            let t = crate::gp::Trsf::translation(Pnt::new(2.0, 0.0, 0.0));
            m.transform(&t);
            m
        };
        let s2 = Solid::from_mesh(m2);
        let p2 = BRepGProp::volume_properties(&s2);

        let mut combined = p1.clone();
        combined.add(&p2);

        near(combined.mass(), 2.0, TOL, "combined_volume");
        let com = combined.centre_of_mass();
        near(com.x, 1.5, TOL, "combined_com_x"); // midpoint of 0.5 and 2.5
    }

    #[test]
    fn principal_props_sphere_like_diagonal() {
        // A diagonal matrix should return the eigenvalues directly.
        let mat = InertiaMatrix {
            ixx: 3.0,
            iyy: 1.0,
            izz: 2.0,
            ixy: 0.0,
            ixz: 0.0,
            iyz: 0.0,
        };
        let pp = jacobi_eigen(&mat);
        near(pp.i1, 1.0, 1e-10, "min_eigenvalue");
        near(pp.i2, 2.0, 1e-10, "mid_eigenvalue");
        near(pp.i3, 3.0, 1e-10, "max_eigenvalue");
    }

    #[test]
    fn radius_of_gyration_unit_cube() {
        let mesh = unit_cube_mesh();
        let solid = Solid::from_mesh(mesh);
        let props = BRepGProp::volume_properties(&solid);
        // r_g = sqrt(I_axis / mass).  For Z-axis through CoM: I = 1/6, mass = 1.
        let z_axis = Ax1::new(Pnt::new(0.5, 0.5, 0.0), Pnt::new(0.0, 0.0, 1.0));
        let rg = props.radius_of_gyration(&z_axis);
        // Expected: sqrt((1/6)/1) = sqrt(1/6) ≈ 0.4082.
        near(rg, (1.0_f64 / 6.0).sqrt(), 1e-3, "rg_z");
    }
}
