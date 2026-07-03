// FILE: geom2d_tools_ext.rs
// occt: Geom2dConvert_BSplineCurveToBezierCurve, Geom2d_OffsetCurve (more methods),
//       Geom2dAPI_ProjectPointOnCurve

use std::f64::consts::PI;

/// Result of 2D curve-curve intersection (single point).
/// occt: Geom2dAPI_InterCurveCurve (extended)
#[derive(Clone, Copy, Debug)]
pub struct Geom2dIntersectionPoint {
    pub point: [f64; 2],
    pub param1: f64,
    pub param2: f64,
    pub is_tangent: bool,
}

impl Geom2dIntersectionPoint {
    pub fn new(point: [f64; 2], param1: f64, param2: f64) -> Self {
        Self { point, param1, param2, is_tangent: false }
    }

    pub fn point(&self) -> [f64; 2] { self.point }
    pub fn parameter1(&self) -> f64 { self.param1 }
    pub fn parameter2(&self) -> f64 { self.param2 }
}

/// 2D line-line intersection.
/// occt: Geom2dAPI_InterCurveCurve (line specialization)
#[derive(Clone, Debug)]
pub struct Geom2dLineLineIntersect {
    pub line1_origin: [f64; 2],
    pub line1_dir: [f64; 2],
    pub line2_origin: [f64; 2],
    pub line2_dir: [f64; 2],
    pub is_done: bool,
    pub is_parallel: bool,
    pub point: Option<[f64; 2]>,
    pub param1: f64,
    pub param2: f64,
}

fn normalize2d(d: [f64; 2]) -> [f64; 2] {
    let len = (d[0]*d[0]+d[1]*d[1]).sqrt();
    if len > 1e-14 { [d[0]/len, d[1]/len] } else { [1.0, 0.0] }
}

impl Geom2dLineLineIntersect {
    pub fn new(o1: [f64;2], d1: [f64;2], o2: [f64;2], d2: [f64;2]) -> Self {
        let nd1 = normalize2d(d1);
        let nd2 = normalize2d(d2);
        // 2D line intersection via Cramer's rule
        // o1 + t1*d1 = o2 + t2*d2
        // d1[0]*t1 - d2[0]*t2 = o2[0]-o1[0]
        // d1[1]*t1 - d2[1]*t2 = o2[1]-o1[1]
        let det = nd1[0]*(-nd2[1]) - nd1[1]*(-nd2[0]);
        let dx = o2[0]-o1[0];
        let dy = o2[1]-o1[1];
        if det.abs() < 1e-14 {
            return Self { line1_origin: o1, line1_dir: nd1, line2_origin: o2, line2_dir: nd2,
                          is_done: true, is_parallel: true, point: None, param1: 0.0, param2: 0.0 };
        }
        let t1 = (dx*(-nd2[1]) - dy*(-nd2[0])) / det;
        let t2 = (nd1[0]*dy - nd1[1]*dx) / det;
        let pt = [o1[0]+nd1[0]*t1, o1[1]+nd1[1]*t1];
        Self { line1_origin: o1, line1_dir: nd1, line2_origin: o2, line2_dir: nd2,
               is_done: true, is_parallel: false, point: Some(pt), param1: t1, param2: t2 }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn is_parallel(&self) -> bool { self.is_parallel }
    pub fn point(&self) -> Option<[f64;2]> { self.point }
    pub fn param1(&self) -> f64 { self.param1 }
}

/// Project a 2D point onto a 2D parametric curve (stub).
/// occt: Geom2dAPI_ProjectPointOnCurve
#[derive(Clone, Debug)]
pub struct Geom2dProjectPointOnCurve {
    pub point: [f64; 2],
    pub curve_id: u32,
    pub first: f64,
    pub last: f64,
    pub is_done: bool,
    pub nb_points: usize,
    pub nearest_point: [f64; 2],
    pub nearest_param: f64,
    pub lower_distance_sq: f64,
}

impl Geom2dProjectPointOnCurve {
    pub fn new(point: [f64;2], curve_id: u32, first: f64, last: f64) -> Self {
        let ok = curve_id > 0 && last >= first;
        let mid = (first + last) / 2.0;
        // Stub: project to (mid, 0) on a unit circle approximation
        let nearest = [mid.cos(), mid.sin()];
        let dx = point[0]-nearest[0]; let dy = point[1]-nearest[1];
        let sq_dist = dx*dx+dy*dy;
        Self {
            point, curve_id, first, last,
            is_done: ok, nb_points: if ok { 1 } else { 0 },
            nearest_point: nearest, nearest_param: mid,
            lower_distance_sq: if ok { sq_dist } else { f64::MAX },
        }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_points(&self) -> usize { self.nb_points }
    pub fn nearest_point(&self) -> [f64; 2] { self.nearest_point }
    pub fn lower_distance_parameter(&self) -> f64 { self.nearest_param }
    pub fn lower_distance(&self) -> f64 { self.lower_distance_sq.sqrt() }
    pub fn point(&self, i: usize) -> Option<[f64;2]> {
        if i == 1 && self.is_done { Some(self.nearest_point) } else { None }
    }
}

/// Convert a bspline curve to a sequence of Bezier segments.
/// occt: Geom2dConvert_BSplineCurveToBezierCurve
#[derive(Clone, Debug)]
pub struct Geom2dBSplineToBezier {
    pub curve_id: u32,
    pub first: f64,
    pub last: f64,
    pub nb_arcs: usize,
    pub is_done: bool,
    pub degree: usize,
}

impl Geom2dBSplineToBezier {
    pub fn new(curve_id: u32, first: f64, last: f64, degree: usize) -> Self {
        let ok = curve_id > 0 && last > first;
        let nb = if ok { ((last - first) / (2.0 * PI / (degree.max(1) as f64))).ceil() as usize + 1 } else { 0 };
        let nb = nb.max(1).min(100);
        Self { curve_id, first, last, nb_arcs: if ok { nb } else { 0 }, is_done: ok, degree: degree.max(1) }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_arcs(&self) -> usize { self.nb_arcs }
    pub fn degree(&self) -> usize { self.degree }

    /// Return stub Bezier arc i (1-based): control points as [f64;2] array.
    pub fn bezier_arc(&self, i: usize) -> Option<Vec<[f64; 2]>> {
        if i == 0 || i > self.nb_arcs || !self.is_done { return None; }
        let t = self.first + (i as f64 - 0.5) * (self.last - self.first) / self.nb_arcs as f64;
        // Stub: return degree+1 unit circle control points
        let mut pts = Vec::new();
        for k in 0..=self.degree {
            let ang = t + k as f64 * 0.1;
            pts.push([ang.cos(), ang.sin()]);
        }
        Some(pts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_line_intersect_basic() {
        let i = Geom2dLineLineIntersect::new(
            [0.0, 0.0], [1.0, 0.0],
            [0.5, -1.0], [0.0, 1.0],
        );
        assert!(i.is_done());
        assert!(!i.is_parallel());
        let pt = i.point().unwrap();
        assert!((pt[0] - 0.5).abs() < 1e-10);
        assert!(pt[1].abs() < 1e-10);
    }

    #[test]
    fn line_line_parallel() {
        let i = Geom2dLineLineIntersect::new(
            [0.0, 0.0], [1.0, 0.0],
            [0.0, 1.0], [1.0, 0.0],
        );
        assert!(i.is_parallel());
        assert!(i.point().is_none());
    }

    #[test]
    fn project_point_on_curve() {
        let p = Geom2dProjectPointOnCurve::new([1.0, 0.0], 1, 0.0, std::f64::consts::PI);
        assert!(p.is_done());
        assert_eq!(p.nb_points(), 1);
        assert!(p.lower_distance() >= 0.0);
    }

    #[test]
    fn bspline_to_bezier() {
        let b = Geom2dBSplineToBezier::new(1, 0.0, std::f64::consts::PI * 2.0, 3);
        assert!(b.is_done());
        assert!(b.nb_arcs() >= 1);
        let arc = b.bezier_arc(1);
        assert!(arc.is_some());
        assert_eq!(arc.unwrap().len(), 4); // degree+1
    }

    #[test]
    fn bspline_to_bezier_invalid() {
        let b = Geom2dBSplineToBezier::new(0, 0.0, 1.0, 3);
        assert!(!b.is_done());
        assert_eq!(b.nb_arcs(), 0);
    }
}
