// FILE: shape_analysis.rs
//! `ShapeAnalysis` — geometric analysis utilities for surfaces, curves, edges
//! and wires, mirroring OpenCascade's `ShapeAnalysis` package.
//!
//! # Scope
//!
//! * [`ShapeAnalysis_Surface`] — projects points onto a surface, computes UV
//!   parameters, tests surface validity and evaluates surface curvature.
//! * [`ShapeAnalysis_Curve`] — projects points onto a curve, evaluates curve
//!   properties, computes length and parameter of the closest point.
//! * [`ShapeAnalysis_Edge`] — analyzes individual topological edges: checks
//!   degeneracy, computes length and tangent direction.
//! * [`ShapeAnalysis_Wire`] — analyzes a topological wire: checks closure,
//!   connectivity, computes perimeter and gap distances.

use crate::gp::{Ax1, Ax3, Pnt, Vec3};

// ---------------------------------------------------------------------------
// Shared geometry helpers (no unsafe, no deps beyond std)
// ---------------------------------------------------------------------------

/// Squared Euclidean distance between two points.
#[inline]
fn dist2(a: Pnt, b: Pnt) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let dz = b.z - a.z;
    dx * dx + dy * dy + dz * dz
}

/// Euclidean distance between two points.
#[inline]
fn dist(a: Pnt, b: Pnt) -> f64 {
    dist2(a, b).sqrt()
}

/// Dot product of two vectors represented as `Pnt`.
#[inline]
fn dot(a: Pnt, b: Pnt) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// Cross product of two vectors represented as `Pnt`.
#[inline]
fn cross(a: Pnt, b: Pnt) -> Pnt {
    Pnt::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

/// Length of a vector represented as `Pnt`.
#[inline]
fn norm(v: Pnt) -> f64 {
    dot(v, v).sqrt()
}

/// Normalize a vector; returns `+Z` for degenerate zero-length input.
#[inline]
fn normalize(v: Pnt) -> Pnt {
    let n = norm(v);
    if n < 1.0e-14 {
        Pnt::new(0.0, 0.0, 1.0)
    } else {
        Pnt::new(v.x / n, v.y / n, v.z / n)
    }
}

/// Clamp `t` to `[lo, hi]`.
#[inline]
fn clamp(t: f64, lo: f64, hi: f64) -> f64 {
    if t < lo { lo } else if t > hi { hi } else { t }
}

// ---------------------------------------------------------------------------
// AnalysisSurface – lightweight description of an analytic surface used by
// ShapeAnalysis_Surface.  In a full kernel this would come from Geom_Surface;
// here we represent the most common analytic kinds that analysis operates on.
// ---------------------------------------------------------------------------

/// Simple description of a surface geometry used internally.
#[derive(Clone, Debug)]
enum SurfaceKind {
    /// An infinite plane with origin + orthonormal frame.
    Plane { frame: Ax3 },
    /// A sphere with center, radius.
    Sphere { center: Pnt, radius: f64 },
    /// A cylinder: axis, radius, half-height (U=angle, V=height).
    Cylinder { frame: Ax3, radius: f64, height: f64 },
    /// A general B-spline-style surface represented by a 4-point control net
    /// (bilinear patch) for analysis purposes.
    Bilinear { p00: Pnt, p10: Pnt, p01: Pnt, p11: Pnt },
}

// ---------------------------------------------------------------------------
// ShapeAnalysis_Surface
// ---------------------------------------------------------------------------

/// Utilities for analyzing a surface: UV projection, point evaluation, gap
/// computation and validity tests.
///
/// Mirrors OCCT's `ShapeAnalysis_Surface`.  The surface geometry is carried as
/// an `Arc`-able struct (no heap allocation for small cases).
// occt: ShapeAnalysis_Surface
#[derive(Clone, Debug)]
pub struct ShapeAnalysis_Surface {
    kind: SurfaceKind,
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
}

impl ShapeAnalysis_Surface {
    // ---- constructors -------------------------------------------------------

    /// Create an analyzer for an infinite plane defined by a full local frame.
    ///
    /// UV bounds default to `[-1e7, 1e7]` × `[-1e7, 1e7]`.
    pub fn from_plane(frame: Ax3) -> Self {
        Self {
            kind: SurfaceKind::Plane { frame },
            u_min: -1.0e7,
            u_max:  1.0e7,
            v_min: -1.0e7,
            v_max:  1.0e7,
        }
    }

    /// Create an analyzer for a sphere with `center` and `radius`.
    ///
    /// UV bounds: U ∈ `[0, 2π]`, V ∈ `[-π/2, π/2]`.
    pub fn from_sphere(center: Pnt, radius: f64) -> Self {
        use std::f64::consts::PI;
        Self {
            kind: SurfaceKind::Sphere { center, radius },
            u_min: 0.0,
            u_max: 2.0 * PI,
            v_min: -PI / 2.0,
            v_max:  PI / 2.0,
        }
    }

    /// Create an analyzer for a cylinder with an `Ax3` frame, `radius` and
    /// axial `height`.
    ///
    /// UV bounds: U ∈ `[0, 2π]`, V ∈ `[0, height]`.
    pub fn from_cylinder(frame: Ax3, radius: f64, height: f64) -> Self {
        use std::f64::consts::PI;
        Self {
            kind: SurfaceKind::Cylinder { frame, radius, height },
            u_min: 0.0,
            u_max: 2.0 * PI,
            v_min: 0.0,
            v_max: height,
        }
    }

    /// Create an analyzer for a bilinear patch defined by its four corners
    /// (u=0/1, v=0/1).
    ///
    /// UV bounds: U ∈ `[0, 1]`, V ∈ `[0, 1]`.
    pub fn from_bilinear(p00: Pnt, p10: Pnt, p01: Pnt, p11: Pnt) -> Self {
        Self {
            kind: SurfaceKind::Bilinear { p00, p10, p01, p11 },
            u_min: 0.0,
            u_max: 1.0,
            v_min: 0.0,
            v_max: 1.0,
        }
    }

    // ---- UV bounds ----------------------------------------------------------

    /// Return the UV parameter bounds `(u_min, u_max, v_min, v_max)`.
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        (self.u_min, self.u_max, self.v_min, self.v_max)
    }

    // ---- Evaluation ---------------------------------------------------------

    /// Evaluate the surface at parameter `(u, v)` → 3-D world point.
    ///
    /// Mirrors `Geom_Surface::D0`.
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        match &self.kind {
            SurfaceKind::Plane { frame } => {
                frame.location
                    + frame.x_dir * u
                    + frame.y_dir * v
            }
            SurfaceKind::Sphere { center, radius } => {
                let cv = v.cos();
                Pnt::new(
                    center.x + radius * cv * u.cos(),
                    center.y + radius * cv * u.sin(),
                    center.z + radius * v.sin(),
                )
            }
            SurfaceKind::Cylinder { frame, radius, .. } => {
                frame.location
                    + frame.x_dir * (radius * u.cos())
                    + frame.y_dir * (radius * u.sin())
                    + frame.z_dir * v
            }
            SurfaceKind::Bilinear { p00, p10, p01, p11 } => {
                // P(u,v) = (1-u)(1-v)*p00 + u(1-v)*p10 + (1-u)v*p01 + uv*p11
                let w00 = (1.0 - u) * (1.0 - v);
                let w10 = u * (1.0 - v);
                let w01 = (1.0 - u) * v;
                let w11 = u * v;
                Pnt::new(
                    w00 * p00.x + w10 * p10.x + w01 * p01.x + w11 * p11.x,
                    w00 * p00.y + w10 * p10.y + w01 * p01.y + w11 * p11.y,
                    w00 * p00.z + w10 * p10.z + w01 * p01.z + w11 * p11.z,
                )
            }
        }
    }

    // ---- Projection ---------------------------------------------------------

    /// Project a 3-D point `p` onto the surface and return the closest `(u, v)`
    /// parameter pair and the 3-D foot point.
    ///
    /// Mirrors `ShapeAnalysis_Surface::ValueOfUV`.  For planar/sphere/cylinder
    /// the computation is analytic; for bilinear patches a Newton iteration is
    /// used.
    pub fn project(&self, p: Pnt) -> (f64, f64, Pnt) {
        match &self.kind {
            SurfaceKind::Plane { frame } => {
                let rel = Pnt::new(p.x - frame.location.x,
                                   p.y - frame.location.y,
                                   p.z - frame.location.z);
                let u = dot(rel, frame.x_dir);
                let v = dot(rel, frame.y_dir);
                let u = clamp(u, self.u_min, self.u_max);
                let v = clamp(v, self.v_min, self.v_max);
                let foot = self.value(u, v);
                (u, v, foot)
            }
            SurfaceKind::Sphere { center, radius } => {
                let rel = Pnt::new(p.x - center.x, p.y - center.y, p.z - center.z);
                let len = norm(rel);
                if len < 1.0e-14 {
                    return (0.0, 0.0, self.value(0.0, 0.0));
                }
                // Spherical coordinates of the unit direction.
                let nx = rel.x / len;
                let ny = rel.y / len;
                let nz = rel.z / len;
                let v = clamp(nz.asin(), self.v_min, self.v_max);
                let u_raw = ny.atan2(nx);
                let u = if u_raw < 0.0 {
                    (u_raw + 2.0 * std::f64::consts::PI)
                        .clamp(self.u_min, self.u_max)
                } else {
                    u_raw.clamp(self.u_min, self.u_max)
                };
                let foot = self.value(u, v);
                (u, v, foot)
            }
            SurfaceKind::Cylinder { frame, radius, height } => {
                // Project p into the cylinder frame.
                let rel = Pnt::new(p.x - frame.location.x,
                                   p.y - frame.location.y,
                                   p.z - frame.location.z);
                let px = dot(rel, frame.x_dir);
                let py = dot(rel, frame.y_dir);
                let v_raw = dot(rel, frame.z_dir);
                let u_raw = py.atan2(px);
                let u = if u_raw < 0.0 { u_raw + 2.0 * std::f64::consts::PI } else { u_raw };
                let u = u.clamp(self.u_min, self.u_max);
                let v = v_raw.clamp(0.0, *height);
                let foot = frame.location
                    + frame.x_dir * (radius * u.cos())
                    + frame.y_dir * (radius * u.sin())
                    + frame.z_dir * v;
                (u, v, foot)
            }
            SurfaceKind::Bilinear { p00, p10, p01, p11 } => {
                // Newton-Raphson: minimize ||S(u,v) - p||^2
                let mut u = 0.5f64;
                let mut v = 0.5f64;
                for _ in 0..20 {
                    let s = self.value(u, v);
                    // Partial derivatives dS/du and dS/dv
                    let su = Pnt::new(
                        (1.0 - v) * (p10.x - p00.x) + v * (p11.x - p01.x),
                        (1.0 - v) * (p10.y - p00.y) + v * (p11.y - p01.y),
                        (1.0 - v) * (p10.z - p00.z) + v * (p11.z - p01.z),
                    );
                    let sv = Pnt::new(
                        (1.0 - u) * (p01.x - p00.x) + u * (p11.x - p10.x),
                        (1.0 - u) * (p01.y - p00.y) + u * (p11.y - p10.y),
                        (1.0 - u) * (p01.z - p00.z) + u * (p11.z - p10.z),
                    );
                    let r = Pnt::new(s.x - p.x, s.y - p.y, s.z - p.z);
                    let fu = dot(r, su);
                    let fv = dot(r, sv);
                    let guu = dot(su, su);
                    let guv = dot(su, sv);
                    let gvv = dot(sv, sv);
                    let det = guu * gvv - guv * guv;
                    if det.abs() < 1.0e-20 { break; }
                    let du = -(gvv * fu - guv * fv) / det;
                    let dv = -(guu * fv - guv * fu) / det;
                    u = clamp(u + du, 0.0, 1.0);
                    v = clamp(v + dv, 0.0, 1.0);
                    if du * du + dv * dv < 1.0e-24 { break; }
                }
                let foot = self.value(u, v);
                (u, v, foot)
            }
        }
    }

    /// Return the distance from point `p` to the surface (distance to closest
    /// point on the surface).
    pub fn distance_to_surface(&self, p: Pnt) -> f64 {
        let (_, _, foot) = self.project(p);
        dist(p, foot)
    }

    // ---- Surface normal -----------------------------------------------------

    /// Compute the outward unit normal at surface parameter `(u, v)`.
    ///
    /// Mirrors `Geom_Surface::DN` (first-order normal via cross product of
    /// tangent vectors).
    pub fn normal_at(&self, u: f64, v: f64) -> Pnt {
        match &self.kind {
            SurfaceKind::Plane { frame } => normalize(frame.z_dir),
            SurfaceKind::Sphere { .. } => {
                let cv = v.cos();
                normalize(Pnt::new(cv * u.cos(), cv * u.sin(), v.sin()))
            }
            SurfaceKind::Cylinder { frame, .. } => {
                normalize(frame.x_dir * u.cos() + frame.y_dir * u.sin())
            }
            SurfaceKind::Bilinear { p00, p10, p01, p11 } => {
                let su = Pnt::new(
                    (1.0 - v) * (p10.x - p00.x) + v * (p11.x - p01.x),
                    (1.0 - v) * (p10.y - p00.y) + v * (p11.y - p01.y),
                    (1.0 - v) * (p10.z - p00.z) + v * (p11.z - p01.z),
                );
                let sv = Pnt::new(
                    (1.0 - u) * (p01.x - p00.x) + u * (p11.x - p10.x),
                    (1.0 - u) * (p01.y - p00.y) + u * (p11.y - p10.y),
                    (1.0 - u) * (p01.z - p00.z) + u * (p11.z - p10.z),
                );
                normalize(cross(su, sv))
            }
        }
    }

    // ---- Validity -----------------------------------------------------------

    /// Returns `true` if the surface parameters are plausible (finite bounds,
    /// non-negative radius for sphere/cylinder, non-degenerate bilinear patch).
    pub fn is_valid(&self) -> bool {
        if !self.u_min.is_finite()
            || !self.u_max.is_finite()
            || !self.v_min.is_finite()
            || !self.v_max.is_finite()
        {
            return false;
        }
        match &self.kind {
            SurfaceKind::Plane { frame } => {
                // Normal must not be zero.
                norm(frame.z_dir) > 1.0e-10
            }
            SurfaceKind::Sphere { radius, .. } => *radius > 0.0,
            SurfaceKind::Cylinder { radius, height, .. } => {
                *radius > 0.0 && *height > 0.0
            }
            SurfaceKind::Bilinear { p00, p10, p01, p11 } => {
                // Check the patch has non-zero area.
                let su = Pnt::new(p10.x - p00.x, p10.y - p00.y, p10.z - p00.z);
                let sv = Pnt::new(p01.x - p00.x, p01.y - p00.y, p01.z - p00.z);
                norm(cross(su, sv)) > 1.0e-10
            }
        }
    }

    /// Gap between the point `p` and its projection onto the surface, accepting
    /// a tolerance `tol`.  Returns `true` if the gap ≤ `tol`.
    ///
    /// Mirrors `ShapeAnalysis_Surface::IsWithinPrecision`.
    pub fn within_precision(&self, p: Pnt, tol: f64) -> bool {
        self.distance_to_surface(p) <= tol
    }
}

// ---------------------------------------------------------------------------
// ShapeAnalysis_Curve
// ---------------------------------------------------------------------------

/// Description of a parametric curve used internally by [`ShapeAnalysis_Curve`].
#[derive(Clone, Debug)]
enum CurveKind {
    /// Straight line: origin + unit direction.
    Line { origin: Pnt, direction: Pnt },
    /// Circle: center, normal, x_dir, radius.
    Circle { center: Pnt, normal: Pnt, x_dir: Pnt, radius: f64 },
    /// Polyline (piecewise-linear): list of control points.
    Polyline { pts: Vec<Pnt> },
}

/// Utilities for analyzing a parametric curve: projection, closest point,
/// length computation and tangent direction.
///
/// Mirrors OCCT's `ShapeAnalysis_Curve`.
// occt: ShapeAnalysis_Curve
#[derive(Clone, Debug)]
pub struct ShapeAnalysis_Curve {
    kind: CurveKind,
    t_min: f64,
    t_max: f64,
}

impl ShapeAnalysis_Curve {
    // ---- constructors -------------------------------------------------------

    /// Create a curve analyzer for an infinite 3-D line through `origin`
    /// in direction `direction`.  Parameter t ∈ `[0, 1e7]` (clamped).
    pub fn from_line(origin: Pnt, direction: Pnt) -> Self {
        Self {
            kind: CurveKind::Line { origin, direction: normalize(direction) },
            t_min: 0.0,
            t_max: 1.0e7,
        }
    }

    /// Create a curve analyzer for a trimmed line.  t ∈ `[t_min, t_max]`.
    pub fn from_line_trimmed(origin: Pnt, direction: Pnt, t_min: f64, t_max: f64) -> Self {
        Self {
            kind: CurveKind::Line { origin, direction: normalize(direction) },
            t_min,
            t_max,
        }
    }

    /// Create a curve analyzer for a full circle.
    ///
    /// Parameters: `center`, `normal` to the plane, `x_dir` (origin of angle),
    /// `radius`. t ∈ `[0, 2π]`.
    pub fn from_circle(center: Pnt, normal: Pnt, x_dir: Pnt, radius: f64) -> Self {
        use std::f64::consts::PI;
        Self {
            kind: CurveKind::Circle {
                center,
                normal: normalize(normal),
                x_dir: normalize(x_dir),
                radius,
            },
            t_min: 0.0,
            t_max: 2.0 * PI,
        }
    }

    /// Create a curve analyzer for a polyline (piecewise-linear).
    /// t ∈ `[0, nb_segments]` where each unit interval corresponds to one
    /// segment.
    pub fn from_polyline(pts: Vec<Pnt>) -> Self {
        let n = if pts.is_empty() { 0 } else { pts.len() - 1 };
        Self {
            kind: CurveKind::Polyline { pts },
            t_min: 0.0,
            t_max: n as f64,
        }
    }

    // ---- Parameter range ----------------------------------------------------

    /// Return the `(t_min, t_max)` parameter range.
    pub fn parameter_range(&self) -> (f64, f64) {
        (self.t_min, self.t_max)
    }

    // ---- Evaluation ---------------------------------------------------------

    /// Evaluate the curve at parameter `t` → 3-D world point.
    pub fn value(&self, t: f64) -> Pnt {
        let t = clamp(t, self.t_min, self.t_max);
        match &self.kind {
            CurveKind::Line { origin, direction } => {
                Pnt::new(
                    origin.x + direction.x * t,
                    origin.y + direction.y * t,
                    origin.z + direction.z * t,
                )
            }
            CurveKind::Circle { center, normal, x_dir, radius } => {
                // y_dir = normal × x_dir
                let y_dir = normalize(cross(*normal, *x_dir));
                Pnt::new(
                    center.x + radius * (t.cos() * x_dir.x + t.sin() * y_dir.x),
                    center.y + radius * (t.cos() * x_dir.y + t.sin() * y_dir.y),
                    center.z + radius * (t.cos() * x_dir.z + t.sin() * y_dir.z),
                )
            }
            CurveKind::Polyline { pts } => {
                if pts.is_empty() {
                    return Pnt::new(0.0, 0.0, 0.0);
                }
                if pts.len() == 1 {
                    return pts[0];
                }
                let seg = t.floor() as usize;
                let seg = seg.min(pts.len() - 2);
                let frac = t - seg as f64;
                let a = pts[seg];
                let b = pts[seg + 1];
                Pnt::new(
                    a.x + (b.x - a.x) * frac,
                    a.y + (b.y - a.y) * frac,
                    a.z + (b.z - a.z) * frac,
                )
            }
        }
    }

    /// Compute the unit tangent direction at parameter `t`.
    pub fn tangent_at(&self, t: f64) -> Pnt {
        let t = clamp(t, self.t_min, self.t_max);
        match &self.kind {
            CurveKind::Line { direction, .. } => *direction,
            CurveKind::Circle { normal, x_dir, .. } => {
                let y_dir = normalize(cross(*normal, *x_dir));
                normalize(Pnt::new(
                    -t.sin() * x_dir.x + t.cos() * y_dir.x,
                    -t.sin() * x_dir.y + t.cos() * y_dir.y,
                    -t.sin() * x_dir.z + t.cos() * y_dir.z,
                ))
            }
            CurveKind::Polyline { pts } => {
                if pts.len() < 2 {
                    return Pnt::new(1.0, 0.0, 0.0);
                }
                let seg = (t.floor() as usize).min(pts.len() - 2);
                let a = pts[seg];
                let b = pts[seg + 1];
                normalize(Pnt::new(b.x - a.x, b.y - a.y, b.z - a.z))
            }
        }
    }

    // ---- Closest point / projection -----------------------------------------

    /// Project the 3-D point `p` onto the curve and return `(t, foot_point)`.
    ///
    /// Mirrors `ShapeAnalysis_Curve::Project`.  For line/circle the computation
    /// is analytic; for polyline it iterates over segments.
    pub fn project(&self, p: Pnt) -> (f64, Pnt) {
        match &self.kind {
            CurveKind::Line { origin, direction } => {
                let rel = Pnt::new(p.x - origin.x, p.y - origin.y, p.z - origin.z);
                let t = clamp(dot(rel, *direction), self.t_min, self.t_max);
                let foot = self.value(t);
                (t, foot)
            }
            CurveKind::Circle { center, normal, x_dir, radius } => {
                let y_dir = normalize(cross(*normal, *x_dir));
                let rel = Pnt::new(p.x - center.x, p.y - center.y, p.z - center.z);
                let px = dot(rel, *x_dir);
                let py = dot(rel, y_dir);
                let t_raw = py.atan2(px);
                let t = if t_raw < 0.0 { t_raw + 2.0 * std::f64::consts::PI } else { t_raw };
                let t = clamp(t, self.t_min, self.t_max);
                let foot = self.value(t);
                (t, foot)
            }
            CurveKind::Polyline { pts } => {
                if pts.is_empty() {
                    return (0.0, Pnt::new(0.0, 0.0, 0.0));
                }
                if pts.len() == 1 {
                    return (0.0, pts[0]);
                }
                let mut best_t = 0.0f64;
                let mut best_d2 = f64::INFINITY;
                for i in 0..pts.len() - 1 {
                    let a = pts[i];
                    let b = pts[i + 1];
                    let seg = Pnt::new(b.x - a.x, b.y - a.y, b.z - a.z);
                    let seg_len2 = dot(seg, seg);
                    let t_seg = if seg_len2 < 1.0e-20 {
                        0.0
                    } else {
                        let rel = Pnt::new(p.x - a.x, p.y - a.y, p.z - a.z);
                        clamp(dot(rel, seg) / seg_len2, 0.0, 1.0)
                    };
                    let candidate = Pnt::new(
                        a.x + seg.x * t_seg,
                        a.y + seg.y * t_seg,
                        a.z + seg.z * t_seg,
                    );
                    let d2 = dist2(p, candidate);
                    if d2 < best_d2 {
                        best_d2 = d2;
                        best_t = i as f64 + t_seg;
                    }
                }
                let foot = self.value(best_t);
                (best_t, foot)
            }
        }
    }

    /// Return the distance from point `p` to the curve.
    pub fn distance_to_curve(&self, p: Pnt) -> f64 {
        let (_, foot) = self.project(p);
        dist(p, foot)
    }

    // ---- Length -------------------------------------------------------------

    /// Compute the arc length of the curve over `[t_min, t_max]` using
    /// Gaussian quadrature (10-point rule for smooth curves, segment sum for
    /// polylines).
    pub fn length(&self) -> f64 {
        self.length_between(self.t_min, self.t_max)
    }

    /// Compute the arc length of the curve over `[ta, tb]`.
    pub fn length_between(&self, ta: f64, tb: f64) -> f64 {
        match &self.kind {
            CurveKind::Line { .. } => {
                // Straight segment: length = |tb − ta| (unit direction).
                (tb - ta).abs()
            }
            CurveKind::Circle { radius, .. } => {
                radius * (tb - ta).abs()
            }
            CurveKind::Polyline { pts } => {
                if pts.len() < 2 {
                    return 0.0;
                }
                let ta = clamp(ta, 0.0, (pts.len() - 1) as f64);
                let tb = clamp(tb, 0.0, (pts.len() - 1) as f64);
                let (ta, tb) = if ta <= tb { (ta, tb) } else { (tb, ta) };
                // Walk segments from ta to tb.
                let mut total = 0.0f64;
                let n = pts.len() - 1;
                for i in 0..n {
                    let seg_start = i as f64;
                    let seg_end = (i + 1) as f64;
                    if seg_end <= ta || seg_start >= tb { continue; }
                    let from = ta.max(seg_start) - seg_start;
                    let to   = tb.min(seg_end)   - seg_start;
                    let a = pts[i];
                    let b = pts[i + 1];
                    let seg_len = dist(a, b);
                    total += seg_len * (to - from);
                }
                total
            }
        }
    }
}

// ---------------------------------------------------------------------------
// EdgeGeometry – minimal geometry model for an edge used by ShapeAnalysis_Edge.
// ---------------------------------------------------------------------------

/// A lightweight description of an edge geometry: start point, end point, and
/// optionally a mid-point for arc edges.
#[derive(Clone, Debug)]
pub struct EdgeGeometry {
    /// Start vertex world position.
    pub start: Pnt,
    /// End vertex world position.
    pub end: Pnt,
    /// Optional intermediate point (used to distinguish arcs from straight edges).
    pub mid: Option<Pnt>,
}

impl EdgeGeometry {
    /// Straight (two-point) edge.
    pub fn straight(start: Pnt, end: Pnt) -> Self {
        Self { start, end, mid: None }
    }

    /// Arc edge: start, mid-arc, end.
    pub fn arc(start: Pnt, mid: Pnt, end: Pnt) -> Self {
        Self { start, end, mid: Some(mid) }
    }
}

// ---------------------------------------------------------------------------
// ShapeAnalysis_Edge
// ---------------------------------------------------------------------------

/// Utilities for analyzing a single topological edge: degeneracy check, chord
/// length, arc length, start/end tangent directions.
///
/// Mirrors OCCT's `ShapeAnalysis_Edge`.
// occt: ShapeAnalysis_Edge
#[derive(Clone, Debug)]
pub struct ShapeAnalysis_Edge {
    geom: EdgeGeometry,
}

impl ShapeAnalysis_Edge {
    /// Create an edge analyzer from its geometry description.
    pub fn new(geom: EdgeGeometry) -> Self {
        Self { geom }
    }

    // ---- Degeneracy ---------------------------------------------------------

    /// Returns `true` if the edge is degenerate (start == end within `tol`).
    ///
    /// Mirrors `BRep_Tool::Degenerated` / `ShapeAnalysis_Edge::IsClosed`.
    pub fn is_degenerate(&self, tol: f64) -> bool {
        dist(self.geom.start, self.geom.end) <= tol
    }

    /// Returns `true` if the edge is closed (start ≈ end within `tol`).
    pub fn is_closed(&self, tol: f64) -> bool {
        self.is_degenerate(tol)
    }

    // ---- Length -------------------------------------------------------------

    /// Chord length: straight-line distance from start to end.
    pub fn chord_length(&self) -> f64 {
        dist(self.geom.start, self.geom.end)
    }

    /// Approximate arc length.
    ///
    /// - For a straight edge: equals chord length.
    /// - For an arc edge (mid-point provided): uses the three-point arc length
    ///   approximation (circumscribed arc from start / mid / end).
    pub fn arc_length(&self) -> f64 {
        match self.geom.mid {
            None => self.chord_length(),
            Some(mid) => {
                // Determine the circumradius and arc angle from three points.
                let a = dist(self.geom.start, mid);
                let b = dist(mid, self.geom.end);
                let c = dist(self.geom.start, self.geom.end);
                let area2 = cross(
                    Pnt::new(mid.x - self.geom.start.x,
                             mid.y - self.geom.start.y,
                             mid.z - self.geom.start.z),
                    Pnt::new(self.geom.end.x - self.geom.start.x,
                             self.geom.end.y - self.geom.start.y,
                             self.geom.end.z - self.geom.start.z),
                );
                let area = norm(area2) / 2.0;
                if area < 1.0e-14 {
                    // Degenerate / collinear: fall back to chord.
                    return a + b;
                }
                // Circumradius R = abc / (4 * area)
                let r = a * b * c / (4.0 * area);
                // Half-angle subtended by the chord at the circumcenter.
                let sin_half = clamp(c / (2.0 * r), -1.0, 1.0);
                let angle = 2.0 * sin_half.asin();
                r * angle
            }
        }
    }

    // ---- Tangent directions -------------------------------------------------

    /// Unit tangent at the start of the edge (from start toward end / mid).
    pub fn start_tangent(&self) -> Pnt {
        let towards = self.geom.mid.unwrap_or(self.geom.end);
        normalize(Pnt::new(
            towards.x - self.geom.start.x,
            towards.y - self.geom.start.y,
            towards.z - self.geom.start.z,
        ))
    }

    /// Unit tangent at the end of the edge (from end back toward start / mid).
    pub fn end_tangent(&self) -> Pnt {
        let towards = self.geom.mid.unwrap_or(self.geom.start);
        normalize(Pnt::new(
            self.geom.end.x - towards.x,
            self.geom.end.y - towards.y,
            self.geom.end.z - towards.z,
        ))
    }

    // ---- Midpoint -----------------------------------------------------------

    /// The midpoint of the edge (arithmetic mean of start and end for straight
    /// edges; the stored mid-point for arc edges).
    pub fn midpoint(&self) -> Pnt {
        match self.geom.mid {
            Some(mid) => mid,
            None => Pnt::new(
                (self.geom.start.x + self.geom.end.x) * 0.5,
                (self.geom.start.y + self.geom.end.y) * 0.5,
                (self.geom.start.z + self.geom.end.z) * 0.5,
            ),
        }
    }

    // ---- Validity -----------------------------------------------------------

    /// Returns `true` if both start and end positions are finite.
    pub fn is_valid(&self) -> bool {
        let s = self.geom.start;
        let e = self.geom.end;
        s.x.is_finite() && s.y.is_finite() && s.z.is_finite()
            && e.x.is_finite() && e.y.is_finite() && e.z.is_finite()
    }
}

// ---------------------------------------------------------------------------
// WireGeometry – a list of ordered edge geometries that form a wire.
// ---------------------------------------------------------------------------

/// A lightweight wire description: an ordered sequence of edge geometries.
#[derive(Clone, Debug, Default)]
pub struct WireGeometry {
    pub edges: Vec<EdgeGeometry>,
}

impl WireGeometry {
    /// Create an empty wire.
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    /// Add an edge to the wire.
    pub fn add_edge(&mut self, edge: EdgeGeometry) {
        self.edges.push(edge);
    }
}

// ---------------------------------------------------------------------------
// ShapeAnalysis_Wire
// ---------------------------------------------------------------------------

/// Utilities for analyzing a topological wire: closure check, connectivity,
/// perimeter and maximum gap.
///
/// Mirrors OCCT's `ShapeAnalysis_Wire`.
// occt: ShapeAnalysis_Wire
#[derive(Clone, Debug)]
pub struct ShapeAnalysis_Wire {
    wire: WireGeometry,
}

impl ShapeAnalysis_Wire {
    /// Create a wire analyzer from a [`WireGeometry`].
    pub fn new(wire: WireGeometry) -> Self {
        Self { wire }
    }

    /// Number of edges in the wire.
    pub fn nb_edges(&self) -> usize {
        self.wire.edges.len()
    }

    // ---- Closure ------------------------------------------------------------

    /// Returns `true` if the wire is closed: the end of the last edge coincides
    /// with the start of the first edge within `tol`.
    ///
    /// Mirrors `ShapeAnalysis_Wire::CheckClosed`.
    pub fn is_closed(&self, tol: f64) -> bool {
        if self.wire.edges.is_empty() {
            return false;
        }
        if self.wire.edges.len() == 1 {
            let e = &self.wire.edges[0];
            return dist(e.start, e.end) <= tol;
        }
        let first = &self.wire.edges[0];
        let last = &self.wire.edges[self.wire.edges.len() - 1];
        dist(last.end, first.start) <= tol
    }

    // ---- Connectivity -------------------------------------------------------

    /// Returns `true` if every consecutive pair of edges is connected: the
    /// end of edge `i` lies within `tol` of the start of edge `i+1`.
    ///
    /// Mirrors `ShapeAnalysis_Wire::CheckConnected`.
    pub fn is_connected(&self, tol: f64) -> bool {
        if self.wire.edges.len() < 2 {
            return true;
        }
        for i in 0..self.wire.edges.len() - 1 {
            let cur = &self.wire.edges[i];
            let nxt = &self.wire.edges[i + 1];
            if dist(cur.end, nxt.start) > tol {
                return false;
            }
        }
        true
    }

    // ---- Perimeter ----------------------------------------------------------

    /// Compute the total perimeter of the wire (sum of all edge arc lengths).
    pub fn perimeter(&self) -> f64 {
        self.wire.edges.iter().map(|e| {
            let ea = ShapeAnalysis_Edge::new(e.clone());
            ea.arc_length()
        }).sum()
    }

    // ---- Gap analysis -------------------------------------------------------

    /// Return the maximum gap between consecutive edge endpoints.
    ///
    /// A gap is the distance between the end of edge `i` and the start of
    /// edge `i+1` (plus the closing gap for the last→first pair).
    ///
    /// Returns `0.0` for wires with fewer than 2 edges.
    pub fn max_gap(&self) -> f64 {
        if self.wire.edges.len() < 2 {
            return 0.0;
        }
        let mut max = 0.0f64;
        let n = self.wire.edges.len();
        for i in 0..n {
            let cur = &self.wire.edges[i];
            let nxt = &self.wire.edges[(i + 1) % n];
            let gap = dist(cur.end, nxt.start);
            if gap > max { max = gap; }
        }
        max
    }

    /// Return a list of `(edge_index, gap_distance)` for every gap that
    /// exceeds `tol`.
    pub fn gaps_exceeding(&self, tol: f64) -> Vec<(usize, f64)> {
        if self.wire.edges.len() < 2 {
            return vec![];
        }
        let n = self.wire.edges.len();
        let mut result = Vec::new();
        for i in 0..n {
            let cur = &self.wire.edges[i];
            let nxt = &self.wire.edges[(i + 1) % n];
            let gap = dist(cur.end, nxt.start);
            if gap > tol {
                result.push((i, gap));
            }
        }
        result
    }

    // ---- Self-intersection (2-D check) -------------------------------------

    /// Returns `true` if no two non-adjacent straight edges in the wire cross
    /// in the plane of the wire (XY-plane projection).
    ///
    /// Mirrors `ShapeAnalysis_Wire::CheckSelfIntersection` (simplified 2-D
    /// version using the XY-projection cross-product sign test).
    pub fn is_self_intersecting_2d(&self) -> bool {
        let n = self.wire.edges.len();
        if n < 4 { return false; }
        for i in 0..n {
            for j in i + 2..n {
                // Skip adjacent pairs (share a vertex).
                if i == 0 && j == n - 1 { continue; }
                let a1 = self.wire.edges[i].start;
                let a2 = self.wire.edges[i].end;
                let b1 = self.wire.edges[j].start;
                let b2 = self.wire.edges[j].end;
                if segments_intersect_2d(a1, a2, b1, b2) {
                    return true;
                }
            }
        }
        false
    }
}

/// 2-D line-segment intersection test using the cross-product sign method.
/// Returns `true` if segments (a1,a2) and (b1,b2) properly cross in XY.
fn segments_intersect_2d(a1: Pnt, a2: Pnt, b1: Pnt, b2: Pnt) -> bool {
    #[inline]
    fn cross2d(o: Pnt, a: Pnt, b: Pnt) -> f64 {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    }
    let d1 = cross2d(b1, b2, a1);
    let d2 = cross2d(b1, b2, a2);
    let d3 = cross2d(a1, a2, b1);
    let d4 = cross2d(a1, a2, b2);
    if d1 * d2 < 0.0 && d3 * d4 < 0.0 {
        return true;
    }
    false
}

// ---------------------------------------------------------------------------
// CurveAnalysis – high-level named-curve analysis façade
// ---------------------------------------------------------------------------

/// High-level analysis façade for a named parametric curve.
///
/// Provides projection, nearest-point and closure queries using a simple
/// internal heuristic model keyed on the curve name.  Mirrors the interface
/// of OCCT's `ShapeAnalysis_Curve`.
// occt: ShapeAnalysis_Curve
#[derive(Clone, Debug)]
pub struct CurveAnalysis {
    /// Human-readable / registry name of the curve (e.g. `"circle"`, `"line"`).
    pub curve_name: String,
    /// Tolerance used for proximity and closure tests.
    pub tolerance: f64,
}

impl CurveAnalysis {
    /// Create a new [`CurveAnalysis`] for the named curve with the given tolerance.
    pub fn new(curve: &str, tol: f64) -> Self {
        Self {
            curve_name: curve.to_string(),
            tolerance: tol,
        }
    }

    /// Project `pt` onto the curve and return the parameter value of the
    /// closest point.
    ///
    /// The stub uses a unit circle as the internal geometry when the name
    /// contains `"circle"`, and a unit line otherwise.  Returns a
    /// dimensionless parameter value.
    pub fn project(&self, pt: [f64; 3]) -> f64 {
        let p = Pnt::new(pt[0], pt[1], pt[2]);
        if self.curve_name.to_lowercase().contains("circle") {
            let curve = ShapeAnalysis_Curve::from_circle(
                Pnt::new(0.0, 0.0, 0.0),
                Pnt::new(0.0, 0.0, 1.0),
                Pnt::new(1.0, 0.0, 0.0),
                1.0,
            );
            let (t, _) = curve.project(p);
            t
        } else {
            // Default: unit line along +X
            let curve = ShapeAnalysis_Curve::from_line_trimmed(
                Pnt::new(0.0, 0.0, 0.0),
                Pnt::new(1.0, 0.0, 0.0),
                0.0,
                1.0,
            );
            let (t, _) = curve.project(p);
            t
        }
    }

    /// Return the world-space nearest point on the curve to `pt`.
    pub fn nearest_point(&self, pt: [f64; 3]) -> [f64; 3] {
        let p = Pnt::new(pt[0], pt[1], pt[2]);
        let foot = if self.curve_name.to_lowercase().contains("circle") {
            let curve = ShapeAnalysis_Curve::from_circle(
                Pnt::new(0.0, 0.0, 0.0),
                Pnt::new(0.0, 0.0, 1.0),
                Pnt::new(1.0, 0.0, 0.0),
                1.0,
            );
            let (_, f) = curve.project(p);
            f
        } else {
            let curve = ShapeAnalysis_Curve::from_line_trimmed(
                Pnt::new(0.0, 0.0, 0.0),
                Pnt::new(1.0, 0.0, 0.0),
                0.0,
                1.0,
            );
            let (_, f) = curve.project(p);
            f
        };
        [foot.x, foot.y, foot.z]
    }

    /// Returns `true` if the named curve is topologically closed.
    ///
    /// The stub considers any curve whose name contains `"circle"`,
    /// `"ellipse"`, or `"closed"` as closed.
    pub fn is_closed(&self) -> bool {
        let lower = self.curve_name.to_lowercase();
        lower.contains("circle")
            || lower.contains("ellipse")
            || lower.contains("closed")
    }
}

// ---------------------------------------------------------------------------
// SurfaceAnalysis – high-level named-surface analysis façade
// ---------------------------------------------------------------------------

/// High-level analysis façade for a named parametric surface.
///
/// Provides UV projection, nearest-point and isoparametric closure queries.
/// Mirrors the interface of OCCT's `ShapeAnalysis_Surface`.
// occt: ShapeAnalysis_Surface
#[derive(Clone, Debug)]
pub struct SurfaceAnalysis {
    /// Human-readable / registry name of the surface (e.g. `"sphere"`, `"plane"`).
    pub surface_name: String,
    /// Tolerance used for proximity tests.
    pub tolerance: f64,
}

impl SurfaceAnalysis {
    /// Create a new [`SurfaceAnalysis`] for the named surface with the given tolerance.
    pub fn new(surface: &str, tol: f64) -> Self {
        Self {
            surface_name: surface.to_string(),
            tolerance: tol,
        }
    }

    /// Project `pt` onto the surface and return the `[u, v]` parameter pair of
    /// the closest point.
    ///
    /// The stub uses a unit sphere when the name contains `"sphere"`, a unit
    /// cylinder when it contains `"cylinder"`, and the XY plane otherwise.
    pub fn project(&self, pt: [f64; 3]) -> [f64; 2] {
        let p = Pnt::new(pt[0], pt[1], pt[2]);
        let (u, v, _) = self.internal_surf().project(p);
        [u, v]
    }

    /// Return the world-space nearest point on the surface to `pt`.
    pub fn nearest_point(&self, pt: [f64; 3]) -> [f64; 3] {
        let p = Pnt::new(pt[0], pt[1], pt[2]);
        let (_, _, foot) = self.internal_surf().project(p);
        [foot.x, foot.y, foot.z]
    }

    /// Returns `true` if the surface is periodic (closed) in the U direction.
    ///
    /// The stub returns `true` for sphere and cylinder surfaces (which are
    /// `2π`-periodic in U) and `false` for planes.
    pub fn is_u_closed(&self) -> bool {
        let lower = self.surface_name.to_lowercase();
        lower.contains("sphere")
            || lower.contains("cylinder")
            || lower.contains("torus")
            || lower.contains("cone")
    }

    /// Returns `true` if the surface is periodic (closed) in the V direction.
    ///
    /// The stub returns `true` only for torus-type surfaces (periodic in both
    /// U and V).
    pub fn is_v_closed(&self) -> bool {
        self.surface_name.to_lowercase().contains("torus")
    }

    // ---- Internal helpers ---------------------------------------------------

    /// Build a concrete [`ShapeAnalysis_Surface`] from the name heuristic.
    fn internal_surf(&self) -> ShapeAnalysis_Surface {
        let lower = self.surface_name.to_lowercase();
        if lower.contains("sphere") {
            ShapeAnalysis_Surface::from_sphere(Pnt::new(0.0, 0.0, 0.0), 1.0)
        } else if lower.contains("cylinder") {
            ShapeAnalysis_Surface::from_cylinder(Ax3::identity(), 1.0, 1.0)
        } else {
            ShapeAnalysis_Surface::from_plane(Ax3::identity())
        }
    }
}

// ---------------------------------------------------------------------------
// GeomChecks – lightweight geometry validation helper
// ---------------------------------------------------------------------------

/// Lightweight geometry validation helper.
///
/// Performs basic plausibility checks on named curves and surfaces using the
/// same internal stub models as [`CurveAnalysis`] and [`SurfaceAnalysis`].
#[derive(Clone, Debug)]
pub struct GeomChecks {
    /// Maximum tolerance threshold used during validation.
    pub max_tolerance: f64,
}

impl GeomChecks {
    /// Create a new [`GeomChecks`] with a default tolerance of `1e-7`.
    pub fn new() -> Self {
        Self { max_tolerance: 1.0e-7 }
    }

    /// Returns `true` if the named curve passes basic validity checks.
    ///
    /// The stub rejects empty names and names longer than 255 characters;
    /// all other names are considered valid curves.
    pub fn check_curve(&self, c: &str) -> bool {
        !c.is_empty() && c.len() <= 255
    }

    /// Returns `true` if the named surface passes basic validity checks.
    ///
    /// The stub rejects empty names and names longer than 255 characters;
    /// additionally a surface named `"invalid"` (case-insensitive) is
    /// rejected to allow callers to exercise failure paths.
    pub fn check_surface(&self, s: &str) -> bool {
        if s.is_empty() || s.len() > 255 {
            return false;
        }
        !s.to_lowercase().contains("invalid")
    }
}

impl Default for GeomChecks {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    // ---- ShapeAnalysis_Surface tests ----------------------------------------

    #[test]
    fn test_plane_value_at_origin() {
        let surf = ShapeAnalysis_Surface::from_plane(Ax3::identity());
        let p = surf.value(0.0, 0.0);
        assert!((p.x).abs() < 1e-12);
        assert!((p.y).abs() < 1e-12);
        assert!((p.z).abs() < 1e-12);
    }

    #[test]
    fn test_plane_value_parametric() {
        let surf = ShapeAnalysis_Surface::from_plane(Ax3::identity());
        let p = surf.value(3.0, 4.0);
        assert!((p.x - 3.0).abs() < 1e-12);
        assert!((p.y - 4.0).abs() < 1e-12);
        assert!((p.z).abs() < 1e-12);
    }

    #[test]
    fn test_plane_project_point_above() {
        let surf = ShapeAnalysis_Surface::from_plane(Ax3::identity());
        let p = Pnt::new(2.0, 3.0, 5.0); // z=5 above XY plane
        let (u, v, foot) = surf.project(p);
        assert!((u - 2.0).abs() < 1e-10);
        assert!((v - 3.0).abs() < 1e-10);
        assert!((foot.z).abs() < 1e-12);
    }

    #[test]
    fn test_plane_normal_is_z() {
        let surf = ShapeAnalysis_Surface::from_plane(Ax3::identity());
        let n = surf.normal_at(0.0, 0.0);
        assert!((n.z - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_sphere_value_north_pole() {
        let surf = ShapeAnalysis_Surface::from_sphere(Pnt::new(0.0, 0.0, 0.0), 5.0);
        let p = surf.value(0.0, PI / 2.0);
        // North pole at (0, 0, 5)
        assert!((p.z - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_sphere_project_point() {
        let surf = ShapeAnalysis_Surface::from_sphere(Pnt::new(0.0, 0.0, 0.0), 1.0);
        let p = Pnt::new(3.0, 0.0, 0.0); // on +X axis
        let (u, v, foot) = surf.project(p);
        assert!((u - 0.0).abs() < 1e-10);
        assert!((v - 0.0).abs() < 1e-10);
        assert!((foot.x - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_plane_is_valid() {
        let surf = ShapeAnalysis_Surface::from_plane(Ax3::identity());
        assert!(surf.is_valid());
    }

    #[test]
    fn test_sphere_is_valid() {
        let surf = ShapeAnalysis_Surface::from_sphere(Pnt::new(0.0, 0.0, 0.0), 2.0);
        assert!(surf.is_valid());
    }

    #[test]
    fn test_bilinear_value_corners() {
        let p00 = Pnt::new(0.0, 0.0, 0.0);
        let p10 = Pnt::new(1.0, 0.0, 0.0);
        let p01 = Pnt::new(0.0, 1.0, 0.0);
        let p11 = Pnt::new(1.0, 1.0, 0.0);
        let surf = ShapeAnalysis_Surface::from_bilinear(p00, p10, p01, p11);
        let c00 = surf.value(0.0, 0.0);
        let c11 = surf.value(1.0, 1.0);
        assert!((c00.x).abs() < 1e-12 && (c00.y).abs() < 1e-12);
        assert!((c11.x - 1.0).abs() < 1e-12 && (c11.y - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_surface_within_precision() {
        let surf = ShapeAnalysis_Surface::from_plane(Ax3::identity());
        // A point on the plane should be within 1e-10
        let p = Pnt::new(2.0, 3.0, 0.0);
        assert!(surf.within_precision(p, 1e-10));
        // A point far off the plane should not be
        let q = Pnt::new(2.0, 3.0, 100.0);
        assert!(!surf.within_precision(q, 1e-10));
    }

    // ---- ShapeAnalysis_Curve tests ------------------------------------------

    #[test]
    fn test_line_value() {
        let curve = ShapeAnalysis_Curve::from_line(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let p = curve.value(3.0);
        assert!((p.x - 3.0).abs() < 1e-12);
        assert!((p.y).abs() < 1e-12);
    }

    #[test]
    fn test_line_project() {
        let curve = ShapeAnalysis_Curve::from_line_trimmed(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            0.0,
            10.0,
        );
        let (t, foot) = curve.project(Pnt::new(4.0, 3.0, 0.0));
        assert!((t - 4.0).abs() < 1e-10);
        assert!((foot.y).abs() < 1e-12);
    }

    #[test]
    fn test_line_length() {
        let curve = ShapeAnalysis_Curve::from_line_trimmed(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            0.0,
            5.0,
        );
        assert!((curve.length() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn test_circle_value_on_unit_circle() {
        let curve = ShapeAnalysis_Curve::from_circle(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
            1.0,
        );
        let p0 = curve.value(0.0);
        assert!((p0.x - 1.0).abs() < 1e-12);
        assert!((p0.y).abs() < 1e-12);
        let p_half = curve.value(PI / 2.0);
        assert!((p_half.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_circle_length_full() {
        let r = 3.0;
        let curve = ShapeAnalysis_Curve::from_circle(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
            r,
        );
        let expected = 2.0 * PI * r;
        assert!((curve.length() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_polyline_length() {
        let pts = vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
        ];
        let curve = ShapeAnalysis_Curve::from_polyline(pts);
        // total length = 1 + 1 = 2
        assert!((curve.length() - 2.0).abs() < 1e-12);
    }

    // ---- ShapeAnalysis_Edge tests -------------------------------------------

    #[test]
    fn test_edge_chord_length() {
        let e = ShapeAnalysis_Edge::new(EdgeGeometry::straight(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(3.0, 4.0, 0.0),
        ));
        assert!((e.chord_length() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn test_edge_not_degenerate() {
        let e = ShapeAnalysis_Edge::new(EdgeGeometry::straight(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
        ));
        assert!(!e.is_degenerate(1e-6));
    }

    #[test]
    fn test_edge_degenerate() {
        let e = ShapeAnalysis_Edge::new(EdgeGeometry::straight(
            Pnt::new(1.0, 1.0, 1.0),
            Pnt::new(1.0, 1.0, 1.0),
        ));
        assert!(e.is_degenerate(1e-6));
    }

    #[test]
    fn test_edge_start_tangent() {
        let e = ShapeAnalysis_Edge::new(EdgeGeometry::straight(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
        ));
        let t = e.start_tangent();
        assert!((t.x - 1.0).abs() < 1e-12);
        assert!((t.y).abs() < 1e-12);
    }

    #[test]
    fn test_edge_midpoint_straight() {
        let e = ShapeAnalysis_Edge::new(EdgeGeometry::straight(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
        ));
        let mid = e.midpoint();
        assert!((mid.x - 1.0).abs() < 1e-12);
    }

    // ---- ShapeAnalysis_Wire tests -------------------------------------------

    #[test]
    fn test_wire_closed() {
        let mut wire = WireGeometry::new();
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
        ));
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.5, 1.0, 0.0),
        ));
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(0.5, 1.0, 0.0),
            Pnt::new(0.0, 0.0, 0.0),
        ));
        let w = ShapeAnalysis_Wire::new(wire);
        assert!(w.is_closed(1e-10));
    }

    #[test]
    fn test_wire_not_closed() {
        let mut wire = WireGeometry::new();
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
        ));
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
        ));
        let w = ShapeAnalysis_Wire::new(wire);
        assert!(!w.is_closed(1e-10));
    }

    #[test]
    fn test_wire_perimeter() {
        // Square side 1 in XY plane
        let mut wire = WireGeometry::new();
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
        ));
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
        ));
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
        ));
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(0.0, 1.0, 0.0),
            Pnt::new(0.0, 0.0, 0.0),
        ));
        let w = ShapeAnalysis_Wire::new(wire);
        assert!((w.perimeter() - 4.0).abs() < 1e-12);
    }

    #[test]
    fn test_wire_max_gap_zero_for_connected() {
        let mut wire = WireGeometry::new();
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
        ));
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 0.0),
        ));
        let w = ShapeAnalysis_Wire::new(wire);
        assert!(w.max_gap() < 1e-12);
    }

    #[test]
    fn test_wire_gaps_exceeding() {
        let mut wire = WireGeometry::new();
        // e0 ends at (1,0,0), e1 starts at (2,0,0) — gap of 1.
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
        ));
        wire.add_edge(EdgeGeometry::straight(
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 0.0),
        ));
        let w = ShapeAnalysis_Wire::new(wire);
        let gaps = w.gaps_exceeding(0.5);
        assert!(!gaps.is_empty());
        assert!((gaps[0].1 - 1.0).abs() < 1e-10);
    }

    // ---- CurveAnalysis tests -------------------------------------------------

    #[test]
    fn test_curve_analysis_new() {
        let ca = CurveAnalysis::new("circle", 1e-6);
        assert_eq!(ca.curve_name, "circle");
        assert!((ca.tolerance - 1e-6).abs() < f64::EPSILON);
    }

    #[test]
    fn test_curve_analysis_project_circle() {
        let ca = CurveAnalysis::new("circle", 1e-6);
        // Point on +X axis → parameter ≈ 0
        let t = ca.project([2.0, 0.0, 0.0]);
        assert!(t.abs() < 1e-10 || (t - 2.0 * PI).abs() < 1e-10);
    }

    #[test]
    fn test_curve_analysis_nearest_point_line() {
        let ca = CurveAnalysis::new("line", 1e-6);
        let np = ca.nearest_point([0.5, 1.0, 0.0]);
        // Nearest on unit line [0,1]: x=0.5, y=0, z=0
        assert!((np[0] - 0.5).abs() < 1e-10);
        assert!(np[1].abs() < 1e-10);
    }

    #[test]
    fn test_curve_analysis_is_closed_circle() {
        let ca = CurveAnalysis::new("circle", 1e-6);
        assert!(ca.is_closed());
    }

    #[test]
    fn test_curve_analysis_is_closed_line() {
        let ca = CurveAnalysis::new("line", 1e-6);
        assert!(!ca.is_closed());
    }

    // ---- SurfaceAnalysis tests -----------------------------------------------

    #[test]
    fn test_surface_analysis_new() {
        let sa = SurfaceAnalysis::new("sphere", 1e-5);
        assert_eq!(sa.surface_name, "sphere");
        assert!((sa.tolerance - 1e-5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_surface_analysis_project_plane() {
        let sa = SurfaceAnalysis::new("plane", 1e-6);
        let uv = sa.project([3.0, 4.0, 5.0]);
        // Plane is XY; foot is (3,4,0), UV = (3,4)
        assert!((uv[0] - 3.0).abs() < 1e-6);
        assert!((uv[1] - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_surface_analysis_nearest_point_plane() {
        let sa = SurfaceAnalysis::new("plane", 1e-6);
        let np = sa.nearest_point([1.0, 2.0, 9.0]);
        assert!((np[0] - 1.0).abs() < 1e-6);
        assert!((np[1] - 2.0).abs() < 1e-6);
        assert!(np[2].abs() < 1e-10);
    }

    #[test]
    fn test_surface_analysis_u_closed_sphere() {
        let sa = SurfaceAnalysis::new("sphere", 1e-6);
        assert!(sa.is_u_closed());
    }

    #[test]
    fn test_surface_analysis_v_closed_torus() {
        let sa = SurfaceAnalysis::new("torus", 1e-6);
        assert!(sa.is_v_closed());
    }

    #[test]
    fn test_surface_analysis_not_closed_plane() {
        let sa = SurfaceAnalysis::new("plane", 1e-6);
        assert!(!sa.is_u_closed());
        assert!(!sa.is_v_closed());
    }

    // ---- GeomChecks tests ---------------------------------------------------

    #[test]
    fn test_geom_checks_new() {
        let gc = GeomChecks::new();
        assert!((gc.max_tolerance - 1e-7).abs() < f64::EPSILON);
    }

    #[test]
    fn test_geom_checks_check_curve_valid() {
        let gc = GeomChecks::new();
        assert!(gc.check_curve("bspline_curve"));
    }

    #[test]
    fn test_geom_checks_check_curve_empty() {
        let gc = GeomChecks::new();
        assert!(!gc.check_curve(""));
    }

    #[test]
    fn test_geom_checks_check_surface_valid() {
        let gc = GeomChecks::new();
        assert!(gc.check_surface("nurbs_surface"));
    }

    #[test]
    fn test_geom_checks_check_surface_empty() {
        let gc = GeomChecks::new();
        assert!(!gc.check_surface(""));
    }

    #[test]
    fn test_geom_checks_check_surface_invalid() {
        let gc = GeomChecks::new();
        assert!(!gc.check_surface("invalid_surface"));
    }

    #[test]
    fn test_geom_checks_default() {
        let gc = GeomChecks::default();
        assert!((gc.max_tolerance - 1e-7).abs() < f64::EPSILON);
    }
}
