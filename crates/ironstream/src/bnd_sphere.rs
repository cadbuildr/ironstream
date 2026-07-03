//! `Bnd_Sphere` — a bounding sphere of a geometric entity (triangle, segment
//! of a line, or any other shape), mirroring OpenCascade's `Bnd_Sphere` class.
//!
//! Faithful, dependency-free reproduction of OCCT's
//! `src/FoundationClasses/TKMath/Bnd/Bnd_Sphere.{hxx,cxx}`. The public API and
//! numeric semantics match the C++ original; we build on [`crate::gp::Pnt`]
//! (which plays the role of OCCT's `gp_XYZ`).

use crate::gp::Pnt;

/// `Bnd_Sphere` — a bounding sphere: a center, a radius, a validity flag, and
/// the `(U, V)` parameters of the entity it bounds.
///
/// `Pnt` here stands in for OCCT's `gp_XYZ`; `square_modulus` maps to
/// `Pnt::square_distance` from the origin / `Pnt::dot(self)`.
// occt: Bnd_Sphere
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BndSphere {
    center: Pnt,
    radius: f64,
    is_valid: bool,
    u: i32,
    v: i32,
}

impl Default for BndSphere {
    fn default() -> Self {
        Self::new()
    }
}

impl BndSphere {
    /// Empty constructor — center at origin, zero radius, invalid, `U = V = 0`.
    ///
    /// Mirrors `Bnd_Sphere::Bnd_Sphere()`.
    #[inline]
    pub fn new() -> Self {
        Self {
            center: Pnt::new(0.0, 0.0, 0.0),
            radius: 0.0,
            is_valid: false,
            u: 0,
            v: 0,
        }
    }

    /// Constructor of a definite sphere.
    ///
    /// Mirrors `Bnd_Sphere::Bnd_Sphere(const gp_XYZ&, double, int, int)`.
    /// Note: like OCCT, the sphere is created **invalid** (`is_valid == false`).
    #[inline]
    pub fn with_center(center: Pnt, radius: f64, u: i32, v: i32) -> Self {
        Self {
            center,
            radius,
            is_valid: false,
            u,
            v,
        }
    }

    /// Returns the `U` parameter on shape. Mirrors `U()`.
    #[inline]
    pub fn u(&self) -> i32 {
        self.u
    }

    /// Returns the `V` parameter on shape. Mirrors `V()`.
    #[inline]
    pub fn v(&self) -> i32 {
        self.v
    }

    /// Returns the validity status, indicating that this sphere corresponds to
    /// a real entity. Mirrors `IsValid()`.
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Sets the validity status. Mirrors `SetValid(bool)`.
    #[inline]
    pub fn set_valid(&mut self, is_valid: bool) {
        self.is_valid = is_valid;
    }

    /// Returns the center of the sphere. Mirrors `Center()`.
    #[inline]
    pub fn center(&self) -> Pnt {
        self.center
    }

    /// Returns the radius value. Mirrors `Radius()`.
    #[inline]
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Calculates the minimal and maximal **squared** distances from `xyz` to
    /// the sphere, returned as `(min, max)`.
    ///
    /// Mirrors `Bnd_Sphere::SquareDistances`:
    /// `max = |xyz - center|^2`; `min = max < r^2 ? 0 : max - r^2`; then
    /// `max += r^2`.
    #[inline]
    pub fn square_distances(&self, xyz: Pnt) -> (f64, f64) {
        let mut max = (xyz - self.center).square_distance(Pnt::origin());
        let rad_sq = self.radius * self.radius;
        let min = if max < rad_sq { 0.0 } else { max - rad_sq };
        max += rad_sq;
        (min, max)
    }

    /// Calculates the minimal and maximal distances from `xyz` to the sphere,
    /// returned as `(min, max)`.
    ///
    /// Mirrors `Bnd_Sphere::Distances`:
    /// `max = |xyz - center|`; `min = max - r < 0 ? 0 : max - r`; then
    /// `max += r`.
    #[inline]
    pub fn distances(&self, xyz: Pnt) -> (f64, f64) {
        let mut max = (xyz - self.center).norm();
        let min = if max - self.radius < 0.0 {
            0.0
        } else {
            max - self.radius
        };
        max += self.radius;
        (min, max)
    }

    /// Projects a point onto the entity, returning
    /// `(proj_node, dist, inside, success)`.
    ///
    /// Mirrors `Bnd_Sphere::Project`: the projection is always the center, the
    /// distance is `|node - center|`, `inside` is always `true`, and the call
    /// always succeeds.
    #[inline]
    pub fn project(&self, node: Pnt) -> (Pnt, f64, bool, bool) {
        let proj_node = self.center;
        let dist = (node - proj_node).norm();
        let inside = true;
        (proj_node, dist, inside, true)
    }

    /// Distance from `node` to the center. Mirrors `Distance`.
    #[inline]
    pub fn distance(&self, node: Pnt) -> f64 {
        (node - self.center).norm()
    }

    /// Squared distance from `node` to the center. Mirrors `SquareDistance`.
    #[inline]
    pub fn square_distance(&self, node: Pnt) -> f64 {
        (node - self.center).square_distance(Pnt::origin())
    }

    /// Enlarges this sphere to also contain `other`.
    ///
    /// Mirrors `Bnd_Sphere::Add`:
    /// - if this sphere is not yet initialised (`radius < 0`), become `other`;
    /// - if `other` encloses this, become `other`;
    /// - if this encloses `other`, stay unchanged;
    /// - otherwise grow to the minimal enclosing sphere along the center line,
    ///   and mark the result invalid.
    pub fn add(&mut self, other: &BndSphere) {
        if self.radius < 0.0 {
            // not initialised yet
            *self = *other;
            return;
        }

        let dist = (self.center - other.center).norm();
        if self.radius + dist <= other.radius {
            // the other sphere is larger and encloses this
            *self = *other;
            return;
        }

        if other.radius + dist <= self.radius {
            return; // this sphere encloses other
        }

        // expansion
        let df_r = (dist + self.radius + other.radius) * 0.5;
        let param_on_diam = (df_r - self.radius) / dist;
        self.center = self.center * (1.0 - param_on_diam) + other.center * param_on_diam;
        self.radius = df_r;
        self.is_valid = false;
    }

    /// Returns `true` if `other` lies entirely outside this sphere (the two
    /// bounding spheres do not intersect).
    ///
    /// Mirrors `Bnd_Sphere::IsOut(const Bnd_Sphere&)`.
    #[inline]
    pub fn is_out_sphere(&self, other: &BndSphere) -> bool {
        (self.center - other.center).square_distance(Pnt::origin())
            > (self.radius + other.radius) * (self.radius + other.radius)
    }

    /// Returns `true` if the point `xyz` is out of reach given the running
    /// `max_dist`. If the point is reachable and this sphere is valid, tightens
    /// `max_dist` to this sphere's maximal distance.
    ///
    /// Mirrors `Bnd_Sphere::IsOut(const gp_XYZ&, double&)`.
    #[inline]
    pub fn is_out_point(&self, xyz: Pnt, max_dist: &mut f64) -> bool {
        let (cur_min_dist, cur_max_dist) = self.distances(xyz);
        if cur_min_dist > *max_dist {
            return true;
        }
        if self.is_valid && cur_max_dist < *max_dist {
            *max_dist = cur_max_dist;
        }
        false
    }

    /// Returns the squared extent (diameter squared): `4 * r^2`.
    ///
    /// Mirrors `Bnd_Sphere::SquareExtent`.
    #[inline]
    pub fn square_extent(&self) -> f64 {
        4.0 * self.radius * self.radius
    }
}
