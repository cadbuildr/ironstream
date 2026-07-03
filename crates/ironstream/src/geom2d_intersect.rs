// FILE: rust/ironstream/crates/ironstream/src/geom2d_intersect.rs

// occt: Geom2dInt_GInter
/// Represents a single intersection point between two 2D curves, holding the
/// parameters on each curve and the geometric location of the intersection.
#[derive(Clone, Debug)]
pub struct IntersectionPoint2d {
    /// Parameter on the first curve at the intersection.
    pub param1: f64,
    /// Parameter on the second curve at the intersection.
    pub param2: f64,
    /// Geometric location of the intersection point in 2D.
    pub point: [f64; 2],
    /// Positional tolerance associated with this intersection point.
    pub tolerance: f64,
}

impl IntersectionPoint2d {
    /// Construct an intersection point from curve parameters `u`, `v` and 2D
    /// coordinates `pt`.  The tolerance defaults to `1e-7`.
    pub fn new(u: f64, v: f64, pt: [f64; 2]) -> Self {
        Self {
            param1: u,
            param2: v,
            point: pt,
            tolerance: 1e-7,
        }
    }
}

// occt: Geom2dInt_GInter
/// General 2D curve–curve intersection, modelled after `Geom2dInt_GInter`.
///
/// `perform` accepts curve descriptors as opaque strings; concrete
/// implementations would accept typed curve handles.  The stub records the
/// call and leaves the result set empty, marking the operation as done.
pub struct CurveIntersect2d {
    /// Intersection points found by `perform`.
    pub points: Vec<IntersectionPoint2d>,
    /// Working tolerance passed at construction time.
    pub tolerance: f64,
    done: bool,
}

impl CurveIntersect2d {
    /// Create a new intersection engine with the given positional tolerance.
    pub fn new(tol: f64) -> Self {
        Self {
            points: Vec::new(),
            tolerance: tol,
            done: false,
        }
    }

    /// Execute the intersection between two curves described by `c1` and `c2`.
    ///
    /// In the stub the descriptors are accepted but not parsed; no intersection
    /// points are produced.  The `done` flag is set so that downstream code
    /// that calls `is_done` before `point` behaves correctly.
    pub fn perform(&mut self, _c1: &str, _c2: &str) {
        self.points.clear();
        self.done = true;
    }

    /// Number of intersection points found after `perform`.
    pub fn num_points(&self) -> usize {
        self.points.len()
    }

    /// Return the intersection point at index `i`, or `None` if out of range.
    pub fn point(&self, i: usize) -> Option<&IntersectionPoint2d> {
        self.points.get(i)
    }

    /// Returns `true` once `perform` has been called.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl Default for CurveIntersect2d {
    fn default() -> Self {
        Self::new(1e-7)
    }
}

// occt: IntCurve_IntConicConic
/// Analytic intersection of two conics in 2D, modelled after
/// `IntCurve_IntConicConic`.
///
/// Supported combinations:
/// - circle / circle  via `circle_circle`
/// - line / circle    via `line_circle`
pub struct ConicIntersect {
    /// Intersection points produced by the most recent query.
    pub points: Vec<IntersectionPoint2d>,
}

impl ConicIntersect {
    /// Create an empty intersector.
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Compute intersections between two circles.
    ///
    /// Each circle is encoded as `[cx, cy, r]`.
    ///
    /// Algorithm: subtract the two circle equations to get the radical line,
    /// then substitute back into the first circle equation.
    pub fn circle_circle(c1: [f64; 3], c2: [f64; 3]) -> ConicIntersect {
        let [cx1, cy1, r1] = c1;
        let [cx2, cy2, r2] = c2;

        let dx = cx2 - cx1;
        let dy = cy2 - cy1;
        let dist = (dx * dx + dy * dy).sqrt();

        // Degenerate: coincident centres.
        if dist < 1e-14 {
            return ConicIntersect { points: Vec::new() };
        }

        // Check radial separation.
        if dist > r1 + r2 + 1e-12 || dist < (r1 - r2).abs() - 1e-12 {
            return ConicIntersect { points: Vec::new() };
        }

        // Distance along the line of centres from c1 to the radical line.
        let a = (r1 * r1 - r2 * r2 + dist * dist) / (2.0 * dist);
        let h_sq = r1 * r1 - a * a;

        // Midpoint on radical line.
        let mx = cx1 + a * dx / dist;
        let my = cy1 + a * dy / dist;

        if h_sq < 0.0 {
            return ConicIntersect { points: Vec::new() };
        }

        let mut pts = Vec::new();
        if h_sq.abs() < 1e-14 {
            // Tangent — one intersection point.
            // param1 = angle on c1, param2 = angle on c2.
            let ang1 = (my - cy1).atan2(mx - cx1);
            let ang2 = (my - cy2).atan2(mx - cx2);
            pts.push(IntersectionPoint2d::new(ang1, ang2, [mx, my]));
        } else {
            let h = h_sq.sqrt();
            let px = h * dy / dist;
            let py = h * dx / dist;

            let ix1 = mx + px;
            let iy1 = my - py;
            let ix2 = mx - px;
            let iy2 = my + py;

            let ang1a = (iy1 - cy1).atan2(ix1 - cx1);
            let ang1b = (iy1 - cy2).atan2(ix1 - cx2);
            pts.push(IntersectionPoint2d::new(ang1a, ang1b, [ix1, iy1]));

            let ang2a = (iy2 - cy1).atan2(ix2 - cx1);
            let ang2b = (iy2 - cy2).atan2(ix2 - cx2);
            pts.push(IntersectionPoint2d::new(ang2a, ang2b, [ix2, iy2]));
        }

        ConicIntersect { points: pts }
    }

    /// Compute intersections between a finite line segment and a circle.
    ///
    /// `line` is two endpoints `[[x0,y0],[x1,y1]]`.
    /// `circle` is `[cx, cy, r]`.
    ///
    /// `param1` is the parametric value `t ∈ [0,1]` along the segment;
    /// `param2` is the angle on the circle in radians.
    pub fn line_circle(line: [[f64; 2]; 2], circle: [f64; 3]) -> ConicIntersect {
        let [p0, p1] = line;
        let [cx, cy, r] = circle;

        let ldx = p1[0] - p0[0];
        let ldy = p1[1] - p0[1];
        let len_sq = ldx * ldx + ldy * ldy;

        if len_sq < 1e-28 {
            // Degenerate segment (zero length).
            return ConicIntersect { points: Vec::new() };
        }

        let fx = p0[0] - cx;
        let fy = p0[1] - cy;

        let a = len_sq;
        let b = 2.0 * (fx * ldx + fy * ldy);
        let c = fx * fx + fy * fy - r * r;
        let disc = b * b - 4.0 * a * c;

        let tol_sq = 1e-12 * r.max(1.0) * r.max(1.0);
        if disc < -tol_sq {
            return ConicIntersect { points: Vec::new() };
        }

        let mut pts = Vec::new();

        let push = |t: f64, out: &mut Vec<IntersectionPoint2d>| {
            // Only accept t in [0, 1] (i.e. within the segment).
            if t < -1e-9 || t > 1.0 + 1e-9 {
                return;
            }
            let t = t.clamp(0.0, 1.0);
            let ix = p0[0] + t * ldx;
            let iy = p0[1] + t * ldy;
            let ang = (iy - cy).atan2(ix - cx);
            out.push(IntersectionPoint2d::new(t, ang, [ix, iy]));
        };

        if disc.abs() <= tol_sq {
            let t = -b / (2.0 * a);
            push(t, &mut pts);
        } else {
            let sq = disc.sqrt();
            let t1 = (-b - sq) / (2.0 * a);
            let t2 = (-b + sq) / (2.0 * a);
            push(t1, &mut pts);
            push(t2, &mut pts);
        }

        ConicIntersect { points: pts }
    }
}

impl Default for ConicIntersect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- IntersectionPoint2d ---

    #[test]
    fn intersection_point_stores_params_and_coords() {
        let p = IntersectionPoint2d::new(0.25, 0.75, [1.0, 2.0]);
        assert_eq!(p.param1, 0.25);
        assert_eq!(p.param2, 0.75);
        assert_eq!(p.point, [1.0, 2.0]);
        assert!((p.tolerance - 1e-7).abs() < 1e-15);
    }

    // --- CurveIntersect2d ---

    #[test]
    fn curve_intersect_not_done_before_perform() {
        let ci = CurveIntersect2d::new(1e-6);
        assert!(!ci.is_done());
        assert_eq!(ci.num_points(), 0);
        assert!(ci.point(0).is_none());
    }

    #[test]
    fn curve_intersect_done_after_perform() {
        let mut ci = CurveIntersect2d::new(1e-6);
        ci.perform("circle", "line");
        assert!(ci.is_done());
    }

    #[test]
    fn curve_intersect_default_tolerance() {
        let ci = CurveIntersect2d::default();
        assert!((ci.tolerance - 1e-7).abs() < 1e-15);
    }

    // --- ConicIntersect: circle / circle ---

    #[test]
    fn circle_circle_no_intersection_too_far() {
        let r = ConicIntersect::circle_circle([0.0, 0.0, 1.0], [10.0, 0.0, 1.0]);
        assert!(r.points.is_empty());
    }

    #[test]
    fn circle_circle_no_intersection_one_inside_other() {
        let r = ConicIntersect::circle_circle([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        assert!(r.points.is_empty());
    }

    #[test]
    fn circle_circle_tangent_one_point() {
        // Two circles touching externally at (1, 0).
        let r = ConicIntersect::circle_circle([0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
        assert_eq!(r.points.len(), 1);
        let p = &r.points[0];
        assert!((p.point[0] - 1.0).abs() < 1e-9);
        assert!(p.point[1].abs() < 1e-9);
    }

    #[test]
    fn circle_circle_two_points_symmetric() {
        // Unit circles centred at (-0.5, 0) and (0.5, 0); they intersect at
        // (0, ±√(3)/2).
        let r = ConicIntersect::circle_circle([-0.5, 0.0, 1.0], [0.5, 0.0, 1.0]);
        assert_eq!(r.points.len(), 2);
        let expected_y = (3.0_f64 / 4.0).sqrt();
        for p in &r.points {
            assert!(p.point[0].abs() < 1e-9);
            assert!((p.point[1].abs() - expected_y).abs() < 1e-9);
        }
    }

    #[test]
    fn circle_circle_coincident_centres_no_result() {
        let r = ConicIntersect::circle_circle([0.0, 0.0, 1.0], [0.0, 0.0, 2.0]);
        assert!(r.points.is_empty());
    }

    // --- ConicIntersect: line / circle ---

    #[test]
    fn line_circle_two_intersections() {
        // Horizontal line through centre of unit circle.
        let r = ConicIntersect::line_circle([[-2.0, 0.0], [2.0, 0.0]], [0.0, 0.0, 1.0]);
        assert_eq!(r.points.len(), 2);
        // Both points should lie on y = 0, x = ±1.
        for p in &r.points {
            assert!(p.point[1].abs() < 1e-9);
            assert!((p.point[0].abs() - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn line_circle_tangent_one_point() {
        // Horizontal line tangent to unit circle at (0, 1).
        let r = ConicIntersect::line_circle([[-2.0, 1.0], [2.0, 1.0]], [0.0, 0.0, 1.0]);
        assert_eq!(r.points.len(), 1);
        let p = &r.points[0];
        assert!(p.point[0].abs() < 1e-6);
        assert!((p.point[1] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn line_circle_no_intersection() {
        // Line far above unit circle.
        let r = ConicIntersect::line_circle([[-2.0, 3.0], [2.0, 3.0]], [0.0, 0.0, 1.0]);
        assert!(r.points.is_empty());
    }

    #[test]
    fn line_circle_segment_does_not_reach() {
        // Segment from (-3, 0) to (-2, 0) does not reach the unit circle.
        let r = ConicIntersect::line_circle([[-3.0, 0.0], [-2.0, 0.0]], [0.0, 0.0, 1.0]);
        assert!(r.points.is_empty());
    }

    #[test]
    fn line_circle_param_in_unit_interval() {
        let r = ConicIntersect::line_circle([[-2.0, 0.0], [2.0, 0.0]], [0.0, 0.0, 1.0]);
        for p in &r.points {
            assert!(p.param1 >= 0.0 && p.param1 <= 1.0);
        }
    }
}
