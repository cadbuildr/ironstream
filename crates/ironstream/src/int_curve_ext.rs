// FILE: int_curve_ext.rs
//
// Analytic conic intersection stubs modelled after OCCT IntCurve and IntAna packages.

// occt: IntCurve_IntConicConic
/// Analytic intersection of two 2-D conics (line/line, line/circle, circle/circle).
pub struct ConicConicIntersect {
    pub points: Vec<[f64; 2]>,
    pub params1: Vec<f64>,
    pub params2: Vec<f64>,
}

impl ConicConicIntersect {
    /// Create an empty intersection result.
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            params1: Vec::new(),
            params2: Vec::new(),
        }
    }

    /// Intersect two lines given in parametric form:
    ///   L1: p1 + t * d1
    ///   L2: p2 + s * d2
    ///
    /// Solves the 2×2 Cramer system.  Returns 0 solutions when lines are
    /// parallel (cross-product magnitude < 1e-14) and 1 solution otherwise.
    pub fn line_line(p1: [f64; 2], d1: [f64; 2], p2: [f64; 2], d2: [f64; 2]) -> Self {
        let cross = d1[0] * d2[1] - d1[1] * d2[0];
        if cross.abs() < 1e-14 {
            return Self::new();
        }
        let rx = p2[0] - p1[0];
        let ry = p2[1] - p1[1];
        let t = (rx * d2[1] - ry * d2[0]) / cross;
        let s = (rx * d1[1] - ry * d1[0]) / cross;
        let ix = p1[0] + t * d1[0];
        let iy = p1[1] + t * d1[1];
        Self {
            points: vec![[ix, iy]],
            params1: vec![t],
            params2: vec![s],
        }
    }

    /// Intersect a line (pt + t * dir) with a circle (center, r).
    ///
    /// Substitutes the line parametrization into the circle equation and
    /// solves the resulting quadratic.  Returns 0, 1, or 2 solutions.
    /// param1 entries are line parameters t; param2 entries are angles on
    /// the circle (atan2 of the intersection point relative to center).
    pub fn line_circle(pt: [f64; 2], dir: [f64; 2], center: [f64; 2], r: f64) -> Self {
        let fx = pt[0] - center[0];
        let fy = pt[1] - center[1];
        let a = dir[0] * dir[0] + dir[1] * dir[1];
        let b = 2.0 * (fx * dir[0] + fy * dir[1]);
        let c = fx * fx + fy * fy - r * r;
        let disc = b * b - 4.0 * a * c;

        let tol = 1e-14 * r.max(1.0) * r.max(1.0);
        if disc < -tol {
            return Self::new();
        }

        let mut result = Self::new();
        if disc.abs() <= tol {
            let t = -b / (2.0 * a);
            let ix = pt[0] + t * dir[0];
            let iy = pt[1] + t * dir[1];
            let ang = (iy - center[1]).atan2(ix - center[0]);
            result.points.push([ix, iy]);
            result.params1.push(t);
            result.params2.push(ang);
        } else {
            let sq = disc.sqrt();
            let t1 = (-b - sq) / (2.0 * a);
            let t2 = (-b + sq) / (2.0 * a);
            let ix1 = pt[0] + t1 * dir[0];
            let iy1 = pt[1] + t1 * dir[1];
            let ang1 = (iy1 - center[1]).atan2(ix1 - center[0]);
            result.points.push([ix1, iy1]);
            result.params1.push(t1);
            result.params2.push(ang1);
            let ix2 = pt[0] + t2 * dir[0];
            let iy2 = pt[1] + t2 * dir[1];
            let ang2 = (iy2 - center[1]).atan2(ix2 - center[0]);
            result.points.push([ix2, iy2]);
            result.params1.push(t2);
            result.params2.push(ang2);
        }
        result
    }

    /// Intersect two circles (c1, r1) and (c2, r2).
    ///
    /// Uses the radical-axis approach:
    ///   d  = distance between centres
    ///   a  = (r1² - r2² + d²) / (2 d)   — distance from c1 to radical axis
    ///   h² = r1² - a²                     — half-chord length
    ///
    /// Returns 0 solutions when circles do not intersect or are identical;
    /// 1 solution (tangent) when h ≈ 0; 2 solutions otherwise.
    /// param1 / param2 store the angle of each intersection point as seen
    /// from c1 and c2 respectively.
    pub fn circle_circle(c1: [f64; 2], r1: f64, c2: [f64; 2], r2: f64) -> Self {
        let dx = c2[0] - c1[0];
        let dy = c2[1] - c1[1];
        let d2 = dx * dx + dy * dy;
        let d = d2.sqrt();

        let tol = 1e-12 * (r1 + r2).max(1.0);
        if d < tol {
            // Concentric circles — no analytic intersection (identical not returned here)
            return Self::new();
        }
        if d > r1 + r2 + tol || d < (r1 - r2).abs() - tol {
            // Circles too far apart or one inside the other
            return Self::new();
        }

        let a = (r1 * r1 - r2 * r2 + d2) / (2.0 * d);
        let h2 = r1 * r1 - a * a;
        if h2 < 0.0 {
            return Self::new();
        }

        // Midpoint along the line joining the centres
        let mx = c1[0] + (a / d) * dx;
        let my = c1[1] + (a / d) * dy;

        let mut result = Self::new();
        if h2 <= tol * tol {
            // Tangent — single intersection
            let ang1 = (my - c1[1]).atan2(mx - c1[0]);
            let ang2 = (my - c2[1]).atan2(mx - c2[0]);
            result.points.push([mx, my]);
            result.params1.push(ang1);
            result.params2.push(ang2);
        } else {
            let h = h2.sqrt();
            // Perpendicular direction (unit)
            let px = -dy / d;
            let py = dx / d;
            let x1 = mx + h * px;
            let y1 = my + h * py;
            let x2 = mx - h * px;
            let y2 = my - h * py;
            let ang1a = (y1 - c1[1]).atan2(x1 - c1[0]);
            let ang2a = (y1 - c2[1]).atan2(x1 - c2[0]);
            result.points.push([x1, y1]);
            result.params1.push(ang1a);
            result.params2.push(ang2a);
            let ang1b = (y2 - c1[1]).atan2(x2 - c1[0]);
            let ang2b = (y2 - c2[1]).atan2(x2 - c2[0]);
            result.points.push([x2, y2]);
            result.params1.push(ang1b);
            result.params2.push(ang2b);
        }
        result
    }

    /// Number of intersection points found.
    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    /// Return the i-th intersection point (0-based), or None if out of range.
    pub fn point(&self, i: usize) -> Option<[f64; 2]> {
        self.points.get(i).copied()
    }
}

impl Default for ConicConicIntersect {
    fn default() -> Self {
        Self::new()
    }
}

// occt: IntAna_Curve
/// Stub for analytic curve–curve intersection in 3-D (IntAna_Curve family).
///
/// Intersection points are stored as 3-D coordinates `[x, y, z]`.
/// `perform` accepts symbolic curve-type strings (e.g. "line", "circle",
/// "cylinder", "sphere") and a tolerance; the actual solver is a stub that
/// leaves the result empty.  Callers that need real analytic 3-D intersections
/// should call the OCCT backend or fill in the match arms below.
pub struct AnaCurveIntersect {
    pub points: Vec<[f64; 3]>,
}

impl AnaCurveIntersect {
    /// Create an empty intersection result.
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Attempt analytic intersection of two named curve types within `tol`.
    ///
    /// Recognised pair names (order-independent):
    ///   "line"/"line", "line"/"circle", "circle"/"circle"
    /// All other pairs are left as stubs returning an empty result.
    pub fn perform(&mut self, c1: &str, c2: &str, _tol: f64) {
        self.points.clear();
        // Stub: real analytic 3-D intersection logic would branch here.
        // Match known type pairs so callers can identify unimplemented paths.
        match (c1, c2) {
            ("line", "line") | ("line", "circle") | ("circle", "line")
            | ("circle", "circle") => {
                // Defer to 2-D helpers or a future 3-D implementation.
                // For now, leave result empty.
            }
            _ => {
                // Unknown pair — no-op stub.
            }
        }
    }

    /// Number of intersection points found after `perform`.
    pub fn nb_points(&self) -> usize {
        self.points.len()
    }
}

impl Default for AnaCurveIntersect {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience free function: intersect two 2-D circles and return
/// the list of intersection points.
///
/// Delegates to `ConicConicIntersect::circle_circle`.
pub fn circle_circle_intersect(c1: [f64; 2], r1: f64, c2: [f64; 2], r2: f64) -> Vec<[f64; 2]> {
    ConicConicIntersect::circle_circle(c1, r1, c2, r2).points
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- ConicConicIntersect: line / line ---

    #[test]
    fn line_line_axes_intersect_at_origin() {
        let r = ConicConicIntersect::line_line([0.0, 0.0], [1.0, 0.0], [0.0, 0.0], [0.0, 1.0]);
        assert_eq!(r.nb_points(), 1);
        let p = r.point(0).unwrap();
        assert!(p[0].abs() < 1e-12);
        assert!(p[1].abs() < 1e-12);
    }

    #[test]
    fn line_line_parallel_no_intersection() {
        let r = ConicConicIntersect::line_line([0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 0.0]);
        assert_eq!(r.nb_points(), 0);
    }

    #[test]
    fn line_line_diagonal() {
        // y = x  (p=[0,0], d=[1,1])  and  y = -x + 4  (p=[4,0], d=[-1,1])
        // Intersection at (2, 2)
        let r = ConicConicIntersect::line_line([0.0, 0.0], [1.0, 1.0], [4.0, 0.0], [-1.0, 1.0]);
        assert_eq!(r.nb_points(), 1);
        let p = r.point(0).unwrap();
        assert!((p[0] - 2.0).abs() < 1e-10);
        assert!((p[1] - 2.0).abs() < 1e-10);
    }

    // --- ConicConicIntersect: line / circle ---

    #[test]
    fn line_circle_secant_two_points() {
        // x-axis through unit circle centred at origin → hits (-1,0) and (1,0)
        let r = ConicConicIntersect::line_circle([0.0, 0.0], [1.0, 0.0], [0.0, 0.0], 1.0);
        assert_eq!(r.nb_points(), 2);
    }

    #[test]
    fn line_circle_tangent_one_point() {
        // Horizontal line y=1, unit circle at origin → tangent at (0,1)
        let r = ConicConicIntersect::line_circle([0.0, 1.0], [1.0, 0.0], [0.0, 0.0], 1.0);
        assert_eq!(r.nb_points(), 1);
        let p = r.point(0).unwrap();
        assert!(p[0].abs() < 1e-10);
        assert!((p[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn line_circle_no_intersection() {
        // Line y = 2, unit circle at origin → miss
        let r = ConicConicIntersect::line_circle([0.0, 2.0], [1.0, 0.0], [0.0, 0.0], 1.0);
        assert_eq!(r.nb_points(), 0);
    }

    // --- ConicConicIntersect: circle / circle ---

    #[test]
    fn circle_circle_two_intersections() {
        // Unit circles centred at (0,0) and (1,0)
        let r = ConicConicIntersect::circle_circle([0.0, 0.0], 1.0, [1.0, 0.0], 1.0);
        assert_eq!(r.nb_points(), 2);
        // Both points should lie on both circles
        for p in &r.points {
            let d1 = (p[0] * p[0] + p[1] * p[1]).sqrt();
            let d2 = ((p[0] - 1.0) * (p[0] - 1.0) + p[1] * p[1]).sqrt();
            assert!((d1 - 1.0).abs() < 1e-10, "not on circle 1: d={d1}");
            assert!((d2 - 1.0).abs() < 1e-10, "not on circle 2: d={d2}");
        }
    }

    #[test]
    fn circle_circle_tangent_one_point() {
        // Circles of radius 1 centred at (0,0) and (2,0) — externally tangent at (1,0)
        let r = ConicConicIntersect::circle_circle([0.0, 0.0], 1.0, [2.0, 0.0], 1.0);
        assert_eq!(r.nb_points(), 1);
        let p = r.point(0).unwrap();
        assert!((p[0] - 1.0).abs() < 1e-10);
        assert!(p[1].abs() < 1e-10);
    }

    #[test]
    fn circle_circle_no_intersection_apart() {
        // Circles too far apart
        let r = ConicConicIntersect::circle_circle([0.0, 0.0], 1.0, [5.0, 0.0], 1.0);
        assert_eq!(r.nb_points(), 0);
    }

    #[test]
    fn circle_circle_no_intersection_contained() {
        // One circle fully inside the other
        let r = ConicConicIntersect::circle_circle([0.0, 0.0], 5.0, [0.0, 0.0], 1.0);
        assert_eq!(r.nb_points(), 0);
    }

    // --- AnaCurveIntersect ---

    #[test]
    fn ana_curve_new_is_empty() {
        let aci = AnaCurveIntersect::new();
        assert_eq!(aci.nb_points(), 0);
    }

    #[test]
    fn ana_curve_perform_stub_clears_points() {
        let mut aci = AnaCurveIntersect::new();
        aci.points.push([1.0, 2.0, 3.0]);
        aci.perform("line", "circle", 1e-7);
        // Stub empties the result
        assert_eq!(aci.nb_points(), 0);
    }

    // --- free function ---

    #[test]
    fn free_fn_circle_circle_intersect() {
        let pts = circle_circle_intersect([0.0, 0.0], 1.0, [1.0, 0.0], 1.0);
        assert_eq!(pts.len(), 2);
    }
}
