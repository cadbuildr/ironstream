// FILE: draw_fair_curve_batten.rs
// occt: DrawFairCurve_Batten

//! Interactive Draw object for displaying and manipulating a Batten curve.
//!
//! A Batten is a fair curve that passes through boundary points with controlled
//! tangency angles and sliding factors. This class wraps a FairCurve_Batten object
//! for interactive manipulation and visualization via the Draw interface.

use std::fmt;

/// A 2D point in the plane.
#[derive(Clone, Copy, Debug, Default)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// A 2D B-spline curve.
#[derive(Clone, Debug)]
pub struct BSplineCurve2d {
    pub control_points: Vec<Point2d>,
    pub knots: Vec<f64>,
    pub degree: usize,
}

impl BSplineCurve2d {
    pub fn new(control_points: Vec<Point2d>, knots: Vec<f64>, degree: usize) -> Self {
        Self {
            control_points,
            knots,
            degree,
        }
    }

    pub fn default_from_points(points: Vec<Point2d>) -> Self {
        let n = points.len();
        Self {
            control_points: points,
            knots: vec![0.0; n + 2],
            degree: 3,
        }
    }
}

impl Default for BSplineCurve2d {
    fn default() -> Self {
        Self {
            control_points: Vec::new(),
            knots: Vec::new(),
            degree: 3,
        }
    }
}

/// Analysis code returned from Batten computation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnalysisCode {
    Success = 0,
    InvalidInput = 1,
    DivergentSolver = 2,
    MaxIterationsReached = 3,
}

impl Default for AnalysisCode {
    fn default() -> Self {
        AnalysisCode::Success
    }
}

/// Parent class: DrawTrSurf_BSplineCurve2d (simplified).
#[derive(Clone, Debug)]
pub struct BSplineCurve2dDrawer {
    pub curve: BSplineCurve2d,
    pub show_curvature: bool,
}

impl BSplineCurve2dDrawer {
    pub fn new(curve: BSplineCurve2d) -> Self {
        Self {
            curve,
            show_curvature: false,
        }
    }

    pub fn show_curvature(&mut self) {
        self.show_curvature = true;
    }

    pub fn hide_curvature(&mut self) {
        self.show_curvature = false;
    }
}

impl Default for BSplineCurve2dDrawer {
    fn default() -> Self {
        Self {
            curve: BSplineCurve2d::default(),
            show_curvature: false,
        }
    }
}

/// Core Batten curve solver (abstracted from FairCurve_Batten).
#[derive(Clone, Debug)]
pub struct BattenCurveCore {
    p1: Point2d,
    p2: Point2d,
    angle1: f64,
    angle2: f64,
    height: f64,
    slope: f64,
    sliding_factor: f64,
    constraint_order1: usize,
    constraint_order2: usize,
    free_sliding: bool,
}

impl BattenCurveCore {
    pub fn new(p1: Point2d, p2: Point2d) -> Self {
        Self {
            p1,
            p2,
            angle1: 0.0,
            angle2: 0.0,
            height: 0.0,
            slope: 0.0,
            sliding_factor: 1.0,
            constraint_order1: 0,
            constraint_order2: 0,
            free_sliding: true,
        }
    }

    pub fn set_p1(&mut self, p: Point2d) {
        self.p1 = p;
    }

    pub fn set_p2(&mut self, p: Point2d) {
        self.p2 = p;
    }

    pub fn set_angle1(&mut self, angle: f64) {
        self.angle1 = angle;
    }

    pub fn set_angle2(&mut self, angle: f64) {
        self.angle2 = angle;
    }

    pub fn get_angle1(&self) -> f64 {
        self.angle1
    }

    pub fn get_angle2(&self) -> f64 {
        self.angle2
    }

    pub fn set_constraint_order1(&mut self, order: usize) {
        self.constraint_order1 = order;
    }

    pub fn set_constraint_order2(&mut self, order: usize) {
        self.constraint_order2 = order;
    }

    pub fn get_constraint_order1(&self) -> usize {
        self.constraint_order1
    }

    pub fn get_constraint_order2(&self) -> usize {
        self.constraint_order2
    }

    pub fn set_height(&mut self, h: f64) {
        self.height = h;
    }

    pub fn set_slope(&mut self, s: f64) {
        self.slope = s;
    }

    pub fn set_sliding_factor(&mut self, factor: f64) {
        self.sliding_factor = factor.max(0.0);
    }

    pub fn get_sliding_factor(&self) -> f64 {
        self.sliding_factor
    }

    pub fn set_free_sliding(&mut self, free: bool) {
        self.free_sliding = free;
    }

    /// Compute the Batten curve with given parameters.
    /// Returns an analysis code and generates a B-spline curve.
    pub fn compute(&self, _max_iterations: usize, _tolerance: f64) -> (AnalysisCode, BSplineCurve2d) {
        // In a real implementation, this would solve the fair curve optimization problem:
        // - Create a curve passing through p1 and p2
        // - Apply tangency constraints if angles are specified
        // - Apply sliding constraint if not free_sliding
        // - Minimize curvature variation (batten fairness criterion)
        // - Solve the constrained optimization

        // For this stub implementation, we return a simple linear interpolation
        let curve = BSplineCurve2d::new(vec![self.p1, self.p2], vec![0.0, 1.0], 1);

        (AnalysisCode::Success, curve)
    }

    pub fn curve(&self) -> BSplineCurve2d {
        let (_, curve) = self.compute(50, 1.0e-2);
        curve
    }
}

impl Default for BattenCurveCore {
    fn default() -> Self {
        Self::new(Point2d::default(), Point2d::default())
    }
}

/// A Batten curve for interactive fair curve design.
///
/// Inherits from DrawTrSurf_BSplineCurve2d and wraps a FairCurve_Batten object.
/// Provides interactive control over:
/// - Endpoint positions
/// - Tangency angles at endpoints
/// - Height and slope parameters
/// - Sliding factor (constraint movement along a baseline)
#[derive(Clone, Debug)]
pub struct DrawFairCurveBatten {
    base: BSplineCurve2dDrawer,
    batten: BattenCurveCore,
}

impl DrawFairCurveBatten {
    /// Create a new Batten interactive object from a FairCurve_Batten.
    pub fn new(p1: Point2d, p2: Point2d) -> Self {
        let batten = BattenCurveCore::new(p1, p2);
        let curve = batten.curve();
        let mut base = BSplineCurve2dDrawer::new(curve);
        base.show_curvature();

        Self { base, batten }
    }

    /// Recompute the curve after parameter changes.
    pub fn compute(&mut self) {
        let (_, curve) = self.batten.compute(50, 1.0e-2);
        self.base.curve = curve;
    }

    /// Set the position of endpoint (Side 1 or 2).
    pub fn set_point(&mut self, side: usize, point: Point2d) {
        if side == 1 {
            self.batten.set_p1(point);
        } else {
            self.batten.set_p2(point);
        }
        self.compute();
    }

    /// Set the tangency angle (in radians) at an endpoint.
    pub fn set_angle(&mut self, side: usize, angle: f64) {
        if side == 1 {
            self.batten.set_angle1(angle * std::f64::consts::PI / 180.0);
            if self.batten.get_constraint_order1() == 0 {
                self.batten.set_constraint_order1(1);
            }
        } else {
            self.batten.set_angle2(angle * std::f64::consts::PI / 180.0);
            if self.batten.get_constraint_order2() == 0 {
                self.batten.set_constraint_order2(1);
            }
        }
        self.compute();
    }

    /// Set the sliding factor (constrained movement length).
    pub fn set_sliding(&mut self, length: f64) {
        self.batten.set_free_sliding(false);
        self.batten.set_sliding_factor(length);
        self.compute();
    }

    /// Set the height parameter.
    pub fn set_height(&mut self, height: f64) {
        self.batten.set_height(height);
        self.compute();
    }

    /// Set the slope parameter.
    pub fn set_slope(&mut self, slope: f64) {
        self.batten.set_slope(slope);
        self.compute();
    }

    /// Get the tangency angle at an endpoint (in radians).
    pub fn get_angle(&self, side: usize) -> f64 {
        if side == 1 {
            self.batten.get_angle1()
        } else {
            self.batten.get_angle2()
        }
    }

    /// Get the sliding factor.
    pub fn get_sliding(&self) -> f64 {
        self.batten.get_sliding_factor()
    }

    /// Free the sliding constraint (allow arbitrary movement).
    pub fn free_sliding(&mut self) {
        self.batten.set_free_sliding(true);
        self.compute();
    }

    /// Free the tangency constraint at an endpoint.
    pub fn free_angle(&mut self, side: usize) {
        if side == 1 {
            self.batten.set_constraint_order1(0);
        } else {
            self.batten.set_constraint_order2(0);
        }
        self.compute();
    }

    /// Get the underlying B-spline curve.
    pub fn curve(&self) -> &BSplineCurve2d {
        &self.base.curve
    }

    /// Dump curve information to a string.
    pub fn dump(&self) -> String {
        format!(
            "DrawFairCurveBatten {{\n  p1: ({}, {}),\n  p2: ({}, {}),\n  angle1: {},\n  angle2: {},\n  height: {},\n  slope: {},\n  sliding_factor: {},\n  free_sliding: {}\n}}",
            self.batten.p1.x,
            self.batten.p1.y,
            self.batten.p2.x,
            self.batten.p2.y,
            self.batten.angle1,
            self.batten.angle2,
            self.batten.height,
            self.batten.slope,
            self.batten.sliding_factor,
            self.batten.free_sliding
        )
    }
}

impl fmt::Display for DrawFairCurveBatten {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dump())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_batten() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let batten = DrawFairCurveBatten::new(p1, p2);

        assert_eq!(batten.batten.p1.x, 0.0);
        assert_eq!(batten.batten.p2.x, 10.0);
        assert!(batten.base.show_curvature);
    }

    #[test]
    fn test_set_point() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut batten = DrawFairCurveBatten::new(p1, p2);

        let new_p1 = Point2d::new(1.0, 1.0);
        batten.set_point(1, new_p1);

        assert_eq!(batten.batten.p1.x, 1.0);
        assert_eq!(batten.batten.p1.y, 1.0);
    }

    #[test]
    fn test_set_angle() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut batten = DrawFairCurveBatten::new(p1, p2);

        batten.set_angle(1, 45.0);

        let angle_rad = 45.0 * std::f64::consts::PI / 180.0;
        assert!((batten.batten.angle1 - angle_rad).abs() < 1e-10);
        assert_eq!(batten.batten.constraint_order1, 1);
    }

    #[test]
    fn test_set_sliding() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut batten = DrawFairCurveBatten::new(p1, p2);

        batten.set_sliding(2.5);

        assert_eq!(batten.batten.sliding_factor, 2.5);
        assert!(!batten.batten.free_sliding);
    }

    #[test]
    fn test_set_height() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut batten = DrawFairCurveBatten::new(p1, p2);

        batten.set_height(3.0);

        assert_eq!(batten.batten.height, 3.0);
    }

    #[test]
    fn test_set_slope() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut batten = DrawFairCurveBatten::new(p1, p2);

        batten.set_slope(0.5);

        assert_eq!(batten.batten.slope, 0.5);
    }

    #[test]
    fn test_get_angle() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut batten = DrawFairCurveBatten::new(p1, p2);

        batten.set_angle(1, 30.0);

        let angle = batten.get_angle(1);
        let expected = 30.0 * std::f64::consts::PI / 180.0;
        assert!((angle - expected).abs() < 1e-10);
    }

    #[test]
    fn test_get_sliding() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut batten = DrawFairCurveBatten::new(p1, p2);

        batten.set_sliding(5.0);

        assert_eq!(batten.get_sliding(), 5.0);
    }

    #[test]
    fn test_free_sliding() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut batten = DrawFairCurveBatten::new(p1, p2);

        batten.set_sliding(2.0);
        assert!(!batten.batten.free_sliding);

        batten.free_sliding();
        assert!(batten.batten.free_sliding);
    }

    #[test]
    fn test_free_angle() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut batten = DrawFairCurveBatten::new(p1, p2);

        batten.set_angle(1, 45.0);
        assert_eq!(batten.batten.constraint_order1, 1);

        batten.free_angle(1);
        assert_eq!(batten.batten.constraint_order1, 0);
    }

    #[test]
    fn test_point2d() {
        let p = Point2d::new(3.5, 4.5);

        assert_eq!(p.x, 3.5);
        assert_eq!(p.y, 4.5);
    }

    #[test]
    fn test_bspline_curve2d_creation() {
        let points = vec![Point2d::new(0.0, 0.0), Point2d::new(10.0, 5.0)];
        let curve = BSplineCurve2d::default_from_points(points);

        assert_eq!(curve.control_points.len(), 2);
        assert_eq!(curve.degree, 3);
    }

    #[test]
    fn test_dump_string() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let batten = DrawFairCurveBatten::new(p1, p2);
        let dump = batten.dump();

        assert!(dump.contains("p1"));
        assert!(dump.contains("p2"));
        assert!(dump.contains("10"));
    }
}
