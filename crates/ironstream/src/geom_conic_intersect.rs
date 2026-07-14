// FILE: geom_conic_intersect.rs
// occt: IntAna_QuadQuadGeo, IntAna_Int3Pln, IntAna_IntConicQuad
// occt-ref: IntAna_Curve

use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IntType {
    Empty,
    Point,
    TwoPoints,
    Line,
    Circle,
    Ellipse,
    Hyperbola,
    Parabola,
}

/// Result of a conic/conic or conic/surface intersection.
#[derive(Clone, Debug)]
pub struct IntResult {
    pub kind: IntType,
    pub points: Vec<[f64; 3]>,
    pub nb_solutions: usize,
    pub is_parallel: bool,
}

impl IntResult {
    pub fn empty() -> Self { Self { kind: IntType::Empty, points: Vec::new(), nb_solutions: 0, is_parallel: false } }
    pub fn has_intersection(&self) -> bool { self.nb_solutions > 0 }
}

// occt: IntAna_IntConicQuad // — conic vs planar quadric
pub struct IntConicQuad;

impl IntConicQuad {
    /// Line vs plane: p(t) = origin + t*dir; plane: n·x = d
    pub fn line_plane(origin: [f64; 3], dir: [f64; 3], n: [f64; 3], d: f64) -> IntResult {
        let denom = dot(dir, n);
        if denom.abs() < 1e-14 {
            let mut res = IntResult::empty();
            res.is_parallel = true;
            return res;
        }
        let t = (d - dot(origin, n)) / denom;
        let pt = [origin[0]+t*dir[0], origin[1]+t*dir[1], origin[2]+t*dir[2]];
        IntResult { kind: IntType::Point, points: vec![pt], nb_solutions: 1, is_parallel: false }
    }

    /// Circle vs plane. Circle: center, radius, normal. Plane: plane_n·x = plane_d.
    pub fn circle_plane(center: [f64; 3], radius: f64, circ_n: [f64; 3], plane_n: [f64; 3], plane_d: f64) -> IntResult {
        let cos_angle = dot(circ_n, plane_n).abs();
        if cos_angle > 1.0 - 1e-12 {
            // Circle lies in or parallel to plane
            let on_plane = (dot(center, plane_n) - plane_d).abs() < 1e-10;
            let mut res = IntResult::empty();
            res.is_parallel = true;
            if on_plane { res.kind = IntType::Circle; res.nb_solutions = usize::MAX; }
            return res;
        }
        // General: find distance from circle center to plane, compare with radius
        let dist = (dot(center, plane_n) - plane_d).abs();
        if dist > radius + 1e-12 {
            return IntResult::empty();
        }
        if (dist - radius).abs() < 1e-12 {
            // Tangent: 1 point (approximated)
            return IntResult { kind: IntType::Point, points: vec![center], nb_solutions: 1, is_parallel: false };
        }
        IntResult { kind: IntType::TwoPoints, points: Vec::new(), nb_solutions: 2, is_parallel: false }
    }
}

// occt: IntAna_Int3Pln // — intersection of 3 planes
pub struct Int3Pln;

impl Int3Pln {
    /// Find the intersection point of three planes (n_i · x = d_i).
    pub fn solve(n: [[f64; 3]; 3], d: [f64; 3]) -> Option<[f64; 3]> {
        // Solve 3x3 linear system using Cramer's rule
        let det = det3(n);
        if det.abs() < 1e-14 { return None; }
        let dx = det3([[d[0], n[0][1], n[0][2]], [d[1], n[1][1], n[1][2]], [d[2], n[2][1], n[2][2]]]);
        let dy = det3([[n[0][0], d[0], n[0][2]], [n[1][0], d[1], n[1][2]], [n[2][0], d[2], n[2][2]]]);
        let dz = det3([[n[0][0], n[0][1], d[0]], [n[1][0], n[1][1], d[1]], [n[2][0], n[2][1], d[2]]]);
        Some([dx / det, dy / det, dz / det])
    }
}

// occt: IntAna_QuadQuadGeo // — quadric vs quadric (sphere/plane/cylinder)
pub struct QuadQuadGeo;

impl QuadQuadGeo {
    /// Two spheres intersection: c1, r1; c2, r2 → circle or nothing.
    pub fn sphere_sphere(c1: [f64; 3], r1: f64, c2: [f64; 3], r2: f64) -> IntResult {
        let d = mag(sub(c2, c1));
        if d < 1e-14 {
            return IntResult::empty(); // concentric
        }
        if d > r1 + r2 + 1e-12 || d < (r1 - r2).abs() - 1e-12 {
            return IntResult::empty();
        }
        if (d - (r1 + r2)).abs() < 1e-12 || (d - (r1 - r2).abs()).abs() < 1e-12 {
            return IntResult { kind: IntType::Point, points: Vec::new(), nb_solutions: 1, is_parallel: false };
        }
        IntResult { kind: IntType::Circle, points: Vec::new(), nb_solutions: usize::MAX, is_parallel: false }
    }

    /// Sphere vs plane intersection.
    pub fn sphere_plane(center: [f64; 3], radius: f64, plane_n: [f64; 3], plane_d: f64) -> IntResult {
        let dist = (dot(center, plane_n) - plane_d).abs();
        if dist > radius + 1e-12 { return IntResult::empty(); }
        if (dist - radius).abs() < 1e-12 {
            return IntResult { kind: IntType::Point, points: Vec::new(), nb_solutions: 1, is_parallel: false };
        }
        IntResult { kind: IntType::Circle, points: Vec::new(), nb_solutions: usize::MAX, is_parallel: false }
    }
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]-b[0],a[1]-b[1],a[2]-b[2]] }
fn mag(v: [f64; 3]) -> f64 { (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt() }
fn det3(m: [[f64; 3]; 3]) -> f64 {
    m[0][0]*(m[1][1]*m[2][2]-m[1][2]*m[2][1])
   -m[0][1]*(m[1][0]*m[2][2]-m[1][2]*m[2][0])
   +m[0][2]*(m[1][0]*m[2][1]-m[1][1]*m[2][0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_plane_intersect() {
        let r = IntConicQuad::line_plane([0.0,0.0,0.0],[0.0,0.0,1.0],[0.0,0.0,1.0],5.0);
        assert!(r.has_intersection());
        assert!((r.points[0][2] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn line_plane_parallel() {
        let r = IntConicQuad::line_plane([0.0,0.0,0.0],[1.0,0.0,0.0],[0.0,0.0,1.0],5.0);
        assert!(!r.has_intersection());
        assert!(r.is_parallel);
    }

    #[test]
    fn sphere_sphere_intersect() {
        let r = QuadQuadGeo::sphere_sphere([0.0;3], 5.0, [6.0,0.0,0.0], 5.0);
        assert!(r.has_intersection());
        assert_eq!(r.kind, IntType::Circle);
    }

    #[test]
    fn sphere_sphere_miss() {
        let r = QuadQuadGeo::sphere_sphere([0.0;3], 1.0, [10.0,0.0,0.0], 1.0);
        assert!(!r.has_intersection());
    }

    #[test]
    fn int3pln() {
        let n = [[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]];
        let d = [2.0, 3.0, 4.0];
        let pt = Int3Pln::solve(n, d).unwrap();
        assert!((pt[0]-2.0).abs() < 1e-10);
        assert!((pt[1]-3.0).abs() < 1e-10);
        assert!((pt[2]-4.0).abs() < 1e-10);
    }
}
