// FILE: gprop.rs
//! `GProp` — global geometric properties, mirroring OpenCascade's `GProp` package.
//!
//! # Scope
//!
//! * [`GProp_GProps`] — accumulator for scalar mass properties: mass (volume,
//!   area, or length), centre of mass, and second-moment inertia matrix.
//!   Mirrors OCCT's `GProp_GProps`.
//!
//! * [`GProp_PGProps`] — point-mass system (a finite collection of weighted
//!   points).  Mirrors OCCT's `GProp_PGProps`.  Useful for building properties
//!   by discrete sampling.
//!
//! * [`GProp_PrincipalProps`] — principal moments and axes obtained by Jacobi
//!   eigendecomposition of the CoM-relative inertia matrix.
//!   Mirrors OCCT's `GProp_PrincipalProps`.
//!
//! # Relationship to `brep_gprop`
//!
//! `brep_gprop` builds on this module: `BRepGProp::VolumeProperties` integrates
//! over a mesh and deposits results into a `GProp_GProps`.  Here we expose the
//! pure mathematical infrastructure independently of topology.

use crate::gp::{Ax1, Pnt};

// ─────────────────────────────────────────────────────────────────────────────
// InertiaMatrix — symmetric 3×3 second-moment matrix
// ─────────────────────────────────────────────────────────────────────────────

/// A symmetric 3×3 matrix of second moments of inertia stored as six
/// independent scalars (upper-triangular storage).
///
/// Layout:
/// ```text
///  [ ixx  ixy  ixz ]
///  [ ixy  iyy  iyz ]
///  [ ixz  iyz  izz ]
/// ```
// occt-ref: gp_Mat // (symmetric inertia matrix)
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
    /// The zero matrix.
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

    /// Component-wise addition.
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

    /// Multiply all components by `s`.
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

    /// Trace (sum of diagonal elements).
    pub fn trace(&self) -> f64 {
        self.ixx + self.iyy + self.izz
    }

    /// Moment of inertia about an arbitrary axis through the origin.
    ///
    /// `I_axis = n^T · I · n` where `n` is the unit direction of the axis.
    pub fn moment_about(&self, axis_dir: Pnt) -> f64 {
        let n = axis_dir.normalized();
        self.ixx * n.x * n.x
            + self.iyy * n.y * n.y
            + self.izz * n.z * n.z
            + 2.0 * self.ixy * n.x * n.y
            + 2.0 * self.ixz * n.x * n.z
            + 2.0 * self.iyz * n.y * n.z
    }

    /// Parallel-axis (Steiner) theorem: shift the inertia matrix from the
    /// centre of mass `com` to a reference point `ref_pt` for a body of mass
    /// `m`.
    ///
    /// `I_ref = I_com + m * (|d|² * Id  -  d ⊗ d)`  where `d = com - ref_pt`.
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
}

// ─────────────────────────────────────────────────────────────────────────────
// GProp_PrincipalProps
// ─────────────────────────────────────────────────────────────────────────────

/// Principal inertia moments and axes obtained by eigendecomposition of the
/// CoM-relative inertia matrix.
///
/// Eigenvalues are sorted **ascending** (`i1 ≤ i2 ≤ i3`); the corresponding
/// unit eigenvectors are `v1`, `v2`, `v3`.
// occt-ref: GProp_PrincipalProps
#[derive(Clone, Copy, Debug)]
pub struct GProp_PrincipalProps {
    /// Smallest principal moment.
    pub i1: f64,
    /// Middle principal moment.
    pub i2: f64,
    /// Largest principal moment.
    pub i3: f64,
    /// Unit axis for smallest moment.
    pub v1: Pnt,
    /// Unit axis for middle moment.
    pub v2: Pnt,
    /// Unit axis for largest moment.
    pub v3: Pnt,
}

impl GProp_PrincipalProps {
    /// Radii of gyration `(r1, r2, r3)` = `sqrt(Ii / mass)`.
    // occt-ref: GProp_PrincipalProps // ::RadiusOfGyration
    pub fn radius_of_gyration(&self, mass: f64) -> (f64, f64, f64) {
        let r = |i: f64| {
            if mass > 0.0 {
                (i.abs() / mass).sqrt()
            } else {
                0.0
            }
        };
        (r(self.i1), r(self.i2), r(self.i3))
    }

    /// `true` if all three principal moments are distinct (non-degenerate
    /// principal axes).
    // occt-ref: GProp_PrincipalProps // ::HasSymmetryPoint
    pub fn is_non_degenerate(&self, tol: f64) -> bool {
        (self.i1 - self.i2).abs() > tol
            && (self.i2 - self.i3).abs() > tol
            && (self.i1 - self.i3).abs() > tol
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GProp_GProps
// ─────────────────────────────────────────────────────────────────────────────

/// Accumulator for global mass properties: scalar measure (volume / area /
/// length), centre of mass, and second-moment inertia matrix.
///
/// Properties from multiple contributions can be combined with [`add`] /
/// [`add_scaled`].  The inertia matrix is always stored relative to the
/// **origin**; `matrix_of_inertia()` shifts it to the CoM on request.
// occt-ref: GProp_GProps
#[derive(Clone, Debug)]
pub struct GProp_GProps {
    /// Total scalar measure (volume, area, or arc-length).
    mass: f64,
    /// First-moment accumulator: `sum(mass_i * pos_i)`.
    moment: Pnt,
    /// Inertia matrix relative to the *origin* (not the CoM).
    inertia_origin: InertiaMatrix,
}

impl Default for GProp_GProps {
    fn default() -> Self {
        GProp_GProps {
            mass: 0.0,
            moment: Pnt::origin(),
            inertia_origin: InertiaMatrix::zero(),
        }
    }
}

impl GProp_GProps {
    // ── constructors ─────────────────────────────────────────────────────────

    /// Create an empty accumulator.
    // occt-ref: GProp_GProps // ::GProp_GProps()
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an accumulator pre-loaded with a single point mass.
    ///
    /// Mirrors `GProp_GProps(SystemLocation)` — a single lumped mass at a
    /// given point contributes zero inertia about its own location.
    // occt-ref: GProp_GProps // ::GProp_GProps(SystemLocation)
    pub fn from_point_mass(location: Pnt, mass: f64) -> Self {
        let mut g = Self::new();
        g.add_point_mass(location, mass);
        g
    }

    // ── query ─────────────────────────────────────────────────────────────────

    /// Total scalar measure (volume / area / length).
    // occt-ref: GProp_GProps // ::Mass
    pub fn mass(&self) -> f64 {
        self.mass
    }

    /// Centre of mass (weighted centroid).
    // occt-ref: GProp_GProps // ::CentreOfMass
    pub fn centre_of_mass(&self) -> Pnt {
        if self.mass.abs() < 1.0e-15 {
            Pnt::origin()
        } else {
            self.moment * (1.0 / self.mass)
        }
    }

    /// Inertia matrix relative to the **centre of mass**.
    ///
    /// The stored matrix is relative to the origin; this method applies the
    /// reverse parallel-axis theorem to obtain the CoM-relative matrix.
    // occt-ref: GProp_GProps // ::MatrixOfInertia
    pub fn matrix_of_inertia(&self) -> InertiaMatrix {
        if self.mass.abs() < 1.0e-15 {
            return self.inertia_origin;
        }
        let com = self.centre_of_mass();
        let d2 = com.dot(com);
        // I_com = I_origin - m * (|com|² * Id  -  com ⊗ com)
        InertiaMatrix {
            ixx: self.inertia_origin.ixx - self.mass * (d2 - com.x * com.x),
            iyy: self.inertia_origin.iyy - self.mass * (d2 - com.y * com.y),
            izz: self.inertia_origin.izz - self.mass * (d2 - com.z * com.z),
            ixy: self.inertia_origin.ixy + self.mass * com.x * com.y,
            ixz: self.inertia_origin.ixz + self.mass * com.x * com.z,
            iyz: self.inertia_origin.iyz + self.mass * com.y * com.z,
        }
    }

    /// Static moments `(Sx, Sy, Sz)` = `mass * centre_of_mass` components.
    // occt-ref: GProp_GProps // ::StaticMoments
    pub fn static_moments(&self) -> (f64, f64, f64) {
        (self.moment.x, self.moment.y, self.moment.z)
    }

    /// Moment of inertia about an arbitrary axis (line).
    ///
    /// The axis is defined as an [`Ax1`] (location + direction).  Uses the
    /// parallel-axis theorem to shift from the CoM.
    // occt-ref: GProp_GProps // ::MomentOfInertia
    pub fn moment_of_inertia(&self, axis: &Ax1) -> f64 {
        let com = self.centre_of_mass();
        let i_com = self.matrix_of_inertia();
        let i_at_ax = i_com.shift_to_point(com, axis.location, self.mass);
        i_at_ax.moment_about(axis.direction)
    }

    /// Radius of gyration about an axis: `sqrt(I / mass)`.
    // occt-ref: GProp_GProps // ::RadiusOfGyration
    pub fn radius_of_gyration(&self, axis: &Ax1) -> f64 {
        let i = self.moment_of_inertia(axis);
        if self.mass > 0.0 {
            (i.abs() / self.mass).sqrt()
        } else {
            0.0
        }
    }

    /// Compute principal inertia properties by eigendecomposition.
    // occt-ref: GProp_GProps // ::PrincipalProperties
    pub fn principal_properties(&self) -> GProp_PrincipalProps {
        jacobi_eigen(&self.matrix_of_inertia())
    }

    // ── mutation ──────────────────────────────────────────────────────────────

    /// Combine another `GProp_GProps` into this one (unscaled).
    // occt-ref: GProp_GProps // ::Add
    pub fn add(&mut self, other: &GProp_GProps) {
        self.add_scaled(other, 1.0);
    }

    /// Combine another `GProp_GProps` into this one, scaled by `coeff`.
    // occt-ref: GProp_GProps // ::Add (with coefficient)
    pub fn add_scaled(&mut self, other: &GProp_GProps, coeff: f64) {
        self.mass += other.mass * coeff;
        self.moment = self.moment + other.moment * coeff;
        self.inertia_origin = self
            .inertia_origin
            .add(&other.inertia_origin.scale(coeff));
    }

    /// Deposit a single **point mass** `m` at `pos`.
    ///
    /// A point mass has no self-inertia, only a parallel-axis contribution.
    // occt: GProp_PGProps // ::AddPoint (underlying)
    pub fn add_point_mass(&mut self, pos: Pnt, m: f64) {
        self.mass += m;
        self.moment = self.moment + pos * m;
        // Inertia of a point mass at `pos` relative to origin.
        let d2 = pos.dot(pos);
        self.inertia_origin.ixx += m * (d2 - pos.x * pos.x);
        self.inertia_origin.iyy += m * (d2 - pos.y * pos.y);
        self.inertia_origin.izz += m * (d2 - pos.z * pos.z);
        self.inertia_origin.ixy -= m * pos.x * pos.y;
        self.inertia_origin.ixz -= m * pos.x * pos.z;
        self.inertia_origin.iyz -= m * pos.y * pos.z;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GProp_PGProps
// ─────────────────────────────────────────────────────────────────────────────

/// Global properties of a **system of point masses**.
///
/// Mirrors OCCT's `GProp_PGProps`: accumulates a finite collection of weighted
/// points and exposes the same query API as `GProp_GProps` via
/// `into_gprops()`.
// occt: GProp_PGProps
#[derive(Clone, Debug, Default)]
pub struct GProp_PGProps {
    /// The underlying accumulator.
    inner: GProp_GProps,
}

impl GProp_PGProps {
    // ── constructors ─────────────────────────────────────────────────────────

    /// Create an empty point-mass system.
    // occt: GProp_PGProps // ::GProp_PGProps()
    pub fn new() -> Self {
        Self::default()
    }

    /// Create from a slice of `(position, mass)` pairs.
    // occt: GProp_PGProps // ::GProp_PGProps(Pts, Density)
    pub fn from_points(points: &[(Pnt, f64)]) -> Self {
        let mut pg = Self::new();
        for &(pos, m) in points {
            pg.add_point(pos, m);
        }
        pg
    }

    /// Create with all points having equal unit mass.
    // occt: GProp_PGProps // ::GProp_PGProps(Pts)
    pub fn from_uniform_points(points: &[Pnt]) -> Self {
        let mut pg = Self::new();
        for &pos in points {
            pg.add_point(pos, 1.0);
        }
        pg
    }

    // ── mutation ──────────────────────────────────────────────────────────────

    /// Add a point with the given mass.
    // occt: GProp_PGProps // ::AddPoint
    pub fn add_point(&mut self, pos: Pnt, mass: f64) {
        self.inner.add_point_mass(pos, mass);
    }

    // ── query (delegates to inner GProp_GProps) ───────────────────────────────

    /// Total mass (sum of point masses).
    // occt-ref: GProp_GProps // ::Mass
    pub fn mass(&self) -> f64 {
        self.inner.mass()
    }

    /// Centre of mass (mass-weighted centroid).
    // occt-ref: GProp_GProps // ::CentreOfMass
    pub fn centre_of_mass(&self) -> Pnt {
        self.inner.centre_of_mass()
    }

    /// Inertia matrix relative to the centre of mass.
    // occt-ref: GProp_GProps // ::MatrixOfInertia
    pub fn matrix_of_inertia(&self) -> InertiaMatrix {
        self.inner.matrix_of_inertia()
    }

    /// Static moments `(Sx, Sy, Sz)`.
    // occt-ref: GProp_GProps // ::StaticMoments
    pub fn static_moments(&self) -> (f64, f64, f64) {
        self.inner.static_moments()
    }

    /// Moment of inertia about an arbitrary axis.
    // occt-ref: GProp_GProps // ::MomentOfInertia
    pub fn moment_of_inertia(&self, axis: &Ax1) -> f64 {
        self.inner.moment_of_inertia(axis)
    }

    /// Radius of gyration about an axis.
    // occt-ref: GProp_GProps // ::RadiusOfGyration
    pub fn radius_of_gyration(&self, axis: &Ax1) -> f64 {
        self.inner.radius_of_gyration(axis)
    }

    /// Principal inertia properties.
    // occt-ref: GProp_GProps // ::PrincipalProperties
    pub fn principal_properties(&self) -> GProp_PrincipalProps {
        self.inner.principal_properties()
    }

    /// Consume into the underlying [`GProp_GProps`] accumulator.
    pub fn into_gprops(self) -> GProp_GProps {
        self.inner
    }

    /// Borrow the underlying [`GProp_GProps`].
    pub fn as_gprops(&self) -> &GProp_GProps {
        &self.inner
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Jacobi eigendecomposition (symmetric 3×3)
// ─────────────────────────────────────────────────────────────────────────────

/// Cyclic Jacobi iteration for a symmetric 3×3 matrix.
///
/// Returns `GProp_PrincipalProps` with eigenvalues sorted ascending and
/// corresponding unit eigenvectors.
fn jacobi_eigen(m: &InertiaMatrix) -> GProp_PrincipalProps {
    let mut a = [
        [m.ixx, m.ixy, m.ixz],
        [m.ixy, m.iyy, m.iyz],
        [m.ixz, m.iyz, m.izz],
    ];
    let mut v = [[1.0_f64, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    for _ in 0..50 {
        let off = (a[0][1] * a[0][1] + a[0][2] * a[0][2] + a[1][2] * a[1][2]).sqrt();
        if off < 1.0e-15 {
            break;
        }
        for p in 0..2_usize {
            for q in (p + 1)..3_usize {
                let apq = a[p][q];
                if apq.abs() < 1.0e-15 {
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

                for r in 0..3_usize {
                    let vrp = v[r][p];
                    let vrq = v[r][q];
                    v[r][p] = c * vrp - s * vrq;
                    v[r][q] = s * vrp + c * vrq;
                }
            }
        }
    }

    let mut pairs = [
        (a[0][0], [v[0][0], v[1][0], v[2][0]]),
        (a[1][1], [v[0][1], v[1][1], v[2][1]]),
        (a[2][2], [v[0][2], v[1][2], v[2][2]]),
    ];
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let pnt = |arr: [f64; 3]| Pnt::new(arr[0], arr[1], arr[2]).normalized();

    GProp_PrincipalProps {
        i1: pairs[0].0,
        i2: pairs[1].0,
        i3: pairs[2].0,
        v1: pnt(pairs[0].1),
        v2: pnt(pairs[1].1),
        v3: pnt(pairs[2].1),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::{Ax1, Pnt};
    use std::f64::consts::PI;

    fn near(a: f64, b: f64, tol: f64, label: &str) {
        assert!(
            (a - b).abs() <= tol,
            "{label}: got {a}, expected {b} (tol={tol}), diff={}",
            (a - b).abs()
        );
    }

    // ── GProp_GProps ──────────────────────────────────────────────────────────

    #[test]
    fn gprops_empty_mass_is_zero() {
        let g = GProp_GProps::new();
        assert_eq!(g.mass(), 0.0);
    }

    #[test]
    fn gprops_empty_centre_of_mass_is_origin() {
        let g = GProp_GProps::new();
        let com = g.centre_of_mass();
        near(com.x, 0.0, 1e-15, "com_x");
        near(com.y, 0.0, 1e-15, "com_y");
        near(com.z, 0.0, 1e-15, "com_z");
    }

    #[test]
    fn gprops_single_point_mass_centroid() {
        let mut g = GProp_GProps::new();
        g.add_point_mass(Pnt::new(3.0, 4.0, 5.0), 2.0);
        let com = g.centre_of_mass();
        near(com.x, 3.0, 1e-12, "com_x");
        near(com.y, 4.0, 1e-12, "com_y");
        near(com.z, 5.0, 1e-12, "com_z");
        near(g.mass(), 2.0, 1e-12, "mass");
    }

    #[test]
    fn gprops_two_equal_point_masses_centroid() {
        // Two unit masses at opposite ends → CoM at midpoint.
        let mut g = GProp_GProps::new();
        g.add_point_mass(Pnt::new(-1.0, 0.0, 0.0), 1.0);
        g.add_point_mass(Pnt::new(1.0, 0.0, 0.0), 1.0);
        let com = g.centre_of_mass();
        near(com.x, 0.0, 1e-12, "com_x");
        near(g.mass(), 2.0, 1e-12, "mass");
    }

    #[test]
    fn gprops_add_combines_masses() {
        let mut g1 = GProp_GProps::new();
        g1.add_point_mass(Pnt::new(0.0, 0.0, 0.0), 1.0);

        let mut g2 = GProp_GProps::new();
        g2.add_point_mass(Pnt::new(4.0, 0.0, 0.0), 1.0);

        g1.add(&g2);
        near(g1.mass(), 2.0, 1e-12, "combined_mass");
        let com = g1.centre_of_mass();
        near(com.x, 2.0, 1e-12, "combined_com_x");
    }

    #[test]
    fn gprops_add_scaled_half_weight() {
        let mut base = GProp_GProps::new();
        base.add_point_mass(Pnt::new(0.0, 0.0, 0.0), 4.0);

        let mut other = GProp_GProps::new();
        other.add_point_mass(Pnt::new(0.0, 0.0, 0.0), 4.0);

        base.add_scaled(&other, 0.5);
        // base mass was 4, other mass * 0.5 = 2 → total 6.
        near(base.mass(), 6.0, 1e-12, "scaled_mass");
    }

    #[test]
    fn gprops_static_moments_single_mass() {
        let mut g = GProp_GProps::new();
        g.add_point_mass(Pnt::new(2.0, 3.0, 5.0), 7.0);
        let (sx, sy, sz) = g.static_moments();
        near(sx, 14.0, 1e-11, "Sx");
        near(sy, 21.0, 1e-11, "Sy");
        near(sz, 35.0, 1e-11, "Sz");
    }

    #[test]
    fn gprops_inertia_matrix_on_axis() {
        // Unit mass at (1,0,0): inertia about Z axis through origin = m*r^2 = 1.
        let mut g = GProp_GProps::new();
        g.add_point_mass(Pnt::new(1.0, 0.0, 0.0), 1.0);
        let z_axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
        near(g.moment_of_inertia(&z_axis), 1.0, 1e-12, "Iz");
        // About X axis: point is ON the axis, so I = 0.
        let x_axis = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        near(g.moment_of_inertia(&x_axis), 0.0, 1e-12, "Ix");
    }

    #[test]
    fn gprops_radius_of_gyration() {
        // Two unit masses at ±1 on Y axis: I about Z through origin = 2.
        let mut g = GProp_GProps::new();
        g.add_point_mass(Pnt::new(0.0, 1.0, 0.0), 1.0);
        g.add_point_mass(Pnt::new(0.0, -1.0, 0.0), 1.0);
        let z_axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
        let rg = g.radius_of_gyration(&z_axis);
        // I = 2, mass = 2 → rg = sqrt(2/2) = 1.
        near(rg, 1.0, 1e-12, "radius_of_gyration");
    }

    // ── GProp_PrincipalProps (via eigendecomposition) ─────────────────────────

    #[test]
    fn principal_props_diagonal_matrix() {
        // Diagonal inertia matrix — eigenvalues are the diagonal entries.
        let mat = InertiaMatrix {
            ixx: 5.0,
            iyy: 3.0,
            izz: 1.0,
            ixy: 0.0,
            ixz: 0.0,
            iyz: 0.0,
        };
        let pp = jacobi_eigen(&mat);
        near(pp.i1, 1.0, 1e-10, "i1");
        near(pp.i2, 3.0, 1e-10, "i2");
        near(pp.i3, 5.0, 1e-10, "i3");
    }

    #[test]
    fn principal_props_radius_of_gyration_scaled() {
        let mat = InertiaMatrix {
            ixx: 4.0,
            iyy: 4.0,
            izz: 4.0,
            ixy: 0.0,
            ixz: 0.0,
            iyz: 0.0,
        };
        let pp = jacobi_eigen(&mat);
        let (r1, r2, r3) = pp.radius_of_gyration(4.0);
        near(r1, 1.0, 1e-10, "r1");
        near(r2, 1.0, 1e-10, "r2");
        near(r3, 1.0, 1e-10, "r3");
    }

    // ── GProp_PGProps ─────────────────────────────────────────────────────────

    #[test]
    fn pgprops_uniform_three_points_centroid() {
        // Three unit masses at vertices of equilateral triangle at z=0.
        // Centroid should be at (0,0,0) if positioned symmetrically.
        let pts = [
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(-0.5, (3.0_f64).sqrt() / 2.0, 0.0),
            Pnt::new(-0.5, -(3.0_f64).sqrt() / 2.0, 0.0),
        ];
        let pg = GProp_PGProps::from_uniform_points(&pts);
        let com = pg.centre_of_mass();
        near(com.x, 0.0, 1e-12, "com_x");
        near(com.y, 0.0, 1e-12, "com_y");
        near(com.z, 0.0, 1e-12, "com_z");
        near(pg.mass(), 3.0, 1e-12, "mass");
    }
}
