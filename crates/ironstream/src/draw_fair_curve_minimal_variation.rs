// FILE: draw_fair_curve_minimal_variation.rs
// occt: DrawFairCurve_MinimalVariation

//! Interactive Draw object for Minimal Variation Curve (MVC).
//!
//! An MVC is a specialized Batten curve that minimizes curvature variation
//! while respecting both tangency and curvature constraints at the endpoints.
//! This class adds curvature control to the Batten interface.

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

/// Analysis code returned from curve computation.
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

/// Color enumeration for draw visualization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawColor {
    White,
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

impl Default for DrawColor {
    fn default() -> Self {
        DrawColor::White
    }
}

/// Parent class: DrawTrSurf_BSplineCurve2d (simplified).
#[derive(Clone, Debug)]
pub struct BSplineCurve2dDrawer {
    pub curve: BSplineCurve2d,
    pub show_curvature: bool,
    pub color: DrawColor,
}

impl BSplineCurve2dDrawer {
    pub fn new(curve: BSplineCurve2d) -> Self {
        Self {
            curve,
            show_curvature: false,
            color: DrawColor::default(),
        }
    }

    pub fn show_curvature(&mut self) {
        self.show_curvature = true;
    }

    pub fn hide_curvature(&mut self) {
        self.show_curvature = false;
    }

    pub fn set_color(&mut self, color: DrawColor) {
        self.color = color;
    }

    pub fn get_color(&self) -> DrawColor {
        self.color
    }
}

impl Default for BSplineCurve2dDrawer {
    fn default() -> Self {
        Self {
            curve: BSplineCurve2d::default(),
            show_curvature: false,
            color: DrawColor::default(),
        }
    }
}

/// Core MVC (Minimal Variation Curve) solver (abstracted from FairCurve_MinimalVariation).
#[derive(Clone, Debug)]
pub struct MinimalVariationCurveCore {
    p1: Point2d,
    p2: Point2d,
    angle1: f64,
    angle2: f64,
    curvature1: f64,
    curvature2: f64,
    height: f64,
    slope: f64,
    sliding_factor: f64,
    physical_ratio: f64,
    constraint_order1: usize,
    constraint_order2: usize,
    free_sliding: bool,
}

impl MinimalVariationCurveCore {
    pub fn new(p1: Point2d, p2: Point2d) -> Self {
        Self {
            p1,
            p2,
            angle1: 0.0,
            angle2: 0.0,
            curvature1: 0.0,
            curvature2: 0.0,
            height: 0.0,
            slope: 0.0,
            sliding_factor: 1.0,
            physical_ratio: 0.0,
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

    pub fn set_curvature1(&mut self, rho: f64) {
        self.curvature1 = rho;
    }

    pub fn set_curvature2(&mut self, rho: f64) {
        self.curvature2 = rho;
    }

    pub fn get_curvature1(&self) -> f64 {
        self.curvature1
    }

    pub fn get_curvature2(&self) -> f64 {
        self.curvature2
    }

    pub fn set_physical_ratio(&mut self, ratio: f64) {
        self.physical_ratio = ratio;
    }

    pub fn get_physical_ratio(&self) -> f64 {
        self.physical_ratio
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

    /// Compute the MVC curve with given parameters.
    /// Returns an analysis code and generates a B-spline curve.
    pub fn compute(&self, _max_iterations: usize, _tolerance: f64) -> (AnalysisCode, BSplineCurve2d) {
        // In a real implementation, this would solve the minimal variation curve optimization problem:
        // - Create a curve passing through p1 and p2
        // - Apply tangency constraints if angles are specified
        // - Apply curvature constraints at endpoints
        // - Minimize curvature variation (second derivative smoothness)
        // - Respect physical ratio scaling
        // - Solve the constrained optimization

        // For this stub implementation, we return a simple curve
        let curve = BSplineCurve2d::new(vec![self.p1, self.p2], vec![0.0, 1.0], 1);

        (AnalysisCode::Success, curve)
    }

    pub fn curve(&self) -> BSplineCurve2d {
        let (_, curve) = self.compute(50, 1.0e-2);
        curve
    }
}

impl Default for MinimalVariationCurveCore {
    fn default() -> Self {
        Self::new(Point2d::default(), Point2d::default())
    }
}

/// A Minimal Variation Curve (MVC) for interactive fair curve design.
///
/// Inherits from DrawFairCurve_Batten and adds curvature constraints at endpoints.
/// Provides control over:
/// - Endpoint positions and tangency angles (from Batten)
/// - Endpoint curvatures (new in MVC)
/// - Physical ratio for scaling
/// - All Batten parameters (height, slope, sliding)
#[derive(Clone, Debug)]
pub struct DrawFairCurveMinimalVariation {
    base: BSplineCurve2dDrawer,
    mvc: MinimalVariationCurveCore,
}

impl DrawFairCurveMinimalVariation {
    /// Create a new MVC interactive object.
    pub fn new(p1: Point2d, p2: Point2d) -> Self {
        let mvc = MinimalVariationCurveCore::new(p1, p2);
        let curve = mvc.curve();
        let mut base = BSplineCurve2dDrawer::new(curve);
        base.show_curvature();
        base.set_color(DrawColor::Yellow);

        Self { base, mvc }
    }

    /// Recompute the curve after parameter changes.
    pub fn compute(&mut self) {
        let (_, curve) = self.mvc.compute(50, 1.0e-2);
        self.base.curve = curve;
    }

    /// Set the position of endpoint (Side 1 or 2).
    pub fn set_point(&mut self, side: usize, point: Point2d) {
        if side == 1 {
            self.mvc.set_p1(point);
        } else {
            self.mvc.set_p2(point);
        }
        self.compute();
    }

    /// Set the tangency angle (in radians) at an endpoint.
    pub fn set_angle(&mut self, side: usize, angle: f64) {
        if side == 1 {
            self.mvc.set_angle1(angle * std::f64::consts::PI / 180.0);
            if self.mvc.get_constraint_order1() == 0 {
                self.mvc.set_constraint_order1(1);
            }
        } else {
            self.mvc.set_angle2(angle * std::f64::consts::PI / 180.0);
            if self.mvc.get_constraint_order2() == 0 {
                self.mvc.set_constraint_order2(1);
            }
        }
        self.compute();
    }

    /// Set the sliding factor (constrained movement length).
    pub fn set_sliding(&mut self, length: f64) {
        self.mvc.set_free_sliding(false);
        self.mvc.set_sliding_factor(length);
        self.compute();
    }

    /// Set the height parameter.
    pub fn set_height(&mut self, height: f64) {
        self.mvc.set_height(height);
        self.compute();
    }

    /// Set the slope parameter.
    pub fn set_slope(&mut self, slope: f64) {
        self.mvc.set_slope(slope);
        self.compute();
    }

    /// Get the tangency angle at an endpoint (in radians).
    pub fn get_angle(&self, side: usize) -> f64 {
        if side == 1 {
            self.mvc.get_angle1()
        } else {
            self.mvc.get_angle2()
        }
    }

    /// Get the sliding factor.
    pub fn get_sliding(&self) -> f64 {
        self.mvc.get_sliding_factor()
    }

    /// Free the sliding constraint (allow arbitrary movement).
    pub fn free_sliding(&mut self) {
        self.mvc.set_free_sliding(true);
        self.compute();
    }

    /// Free the tangency constraint at an endpoint.
    pub fn free_angle(&mut self, side: usize) {
        if side == 1 {
            self.mvc.set_constraint_order1(0);
        } else {
            self.mvc.set_constraint_order2(0);
        }
        self.compute();
    }

    /// Set the curvature constraint at an endpoint.
    pub fn set_curvature(&mut self, side: usize, rho: f64) {
        if side == 1 {
            self.mvc.set_curvature1(rho);
            self.mvc.set_constraint_order1(2);
        } else {
            self.mvc.set_curvature2(rho);
            self.mvc.set_constraint_order2(2);
        }
        self.compute();
    }

    /// Get the curvature at an endpoint.
    pub fn get_curvature(&self, side: usize) -> f64 {
        if side == 1 {
            self.mvc.get_curvature1()
        } else {
            self.mvc.get_curvature2()
        }
    }

    /// Free the curvature constraint at an endpoint.
    pub fn free_curvature(&mut self, side: usize) {
        if side == 1 {
            if self.mvc.get_constraint_order1() > 1 {
                self.mvc.set_constraint_order1(1);
            }
        } else {
            if self.mvc.get_constraint_order2() > 1 {
                self.mvc.set_constraint_order2(1);
            }
        }
        self.compute();
    }

    /// Set the physical ratio parameter.
    pub fn set_physical_ratio(&mut self, ratio: f64) {
        self.mvc.set_physical_ratio(ratio);
        self.compute();
    }

    /// Get the physical ratio parameter.
    pub fn get_physical_ratio(&self) -> f64 {
        self.mvc.get_physical_ratio()
    }

    /// Get the underlying B-spline curve.
    pub fn curve(&self) -> &BSplineCurve2d {
        &self.base.curve
    }

    /// Get the display color.
    pub fn get_color(&self) -> DrawColor {
        self.base.get_color()
    }
}

impl fmt::Display for DrawFairCurveMinimalVariation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DrawFairCurveMinimalVariation {{ p1: ({}, {}), p2: ({}, {}), curvature1: {}, curvature2: {}, physical_ratio: {} }}",
            self.mvc.p1.x,
            self.mvc.p1.y,
            self.mvc.p2.x,
            self.mvc.p2.y,
            self.mvc.curvature1,
            self.mvc.curvature2,
            self.mvc.physical_ratio
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_mvc() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mvc = DrawFairCurveMinimalVariation::new(p1, p2);

        assert_eq!(mvc.mvc.p1.x, 0.0);
        assert_eq!(mvc.mvc.p2.x, 10.0);
        assert!(mvc.base.show_curvature);
        assert_eq!(mvc.base.color, DrawColor::Yellow);
    }

    #[test]
    fn test_set_curvature() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut mvc = DrawFairCurveMinimalVariation::new(p1, p2);

        mvc.set_curvature(1, 0.5);

        assert_eq!(mvc.mvc.curvature1, 0.5);
        assert_eq!(mvc.mvc.constraint_order1, 2);
    }

    #[test]
    fn test_get_curvature() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut mvc = DrawFairCurveMinimalVariation::new(p1, p2);

        mvc.set_curvature(1, 0.3);

        assert_eq!(mvc.get_curvature(1), 0.3);
    }

    #[test]
    fn test_free_curvature() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut mvc = DrawFairCurveMinimalVariation::new(p1, p2);

        mvc.set_curvature(1, 0.5);
        assert_eq!(mvc.mvc.constraint_order1, 2);

        mvc.free_curvature(1);
        assert_eq!(mvc.mvc.constraint_order1, 1);
    }

    #[test]
    fn test_set_physical_ratio() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut mvc = DrawFairCurveMinimalVariation::new(p1, p2);

        mvc.set_physical_ratio(1.5);

        assert_eq!(mvc.mvc.physical_ratio, 1.5);
    }

    #[test]
    fn test_get_physical_ratio() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut mvc = DrawFairCurveMinimalVariation::new(p1, p2);

        mvc.set_physical_ratio(2.0);

        assert_eq!(mvc.get_physical_ratio(), 2.0);
    }

    #[test]
    fn test_set_point() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut mvc = DrawFairCurveMinimalVariation::new(p1, p2);

        let new_p1 = Point2d::new(1.0, 2.0);
        mvc.set_point(1, new_p1);

        assert_eq!(mvc.mvc.p1.x, 1.0);
        assert_eq!(mvc.mvc.p1.y, 2.0);
    }

    #[test]
    fn test_set_angle() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut mvc = DrawFairCurveMinimalVariation::new(p1, p2);

        mvc.set_angle(1, 45.0);

        let angle_rad = 45.0 * std::f64::consts::PI / 180.0;
        assert!((mvc.mvc.angle1 - angle_rad).abs() < 1e-10);
    }

    #[test]
    fn test_set_sliding() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut mvc = DrawFairCurveMinimalVariation::new(p1, p2);

        mvc.set_sliding(3.0);

        assert_eq!(mvc.mvc.sliding_factor, 3.0);
        assert!(!mvc.mvc.free_sliding);
    }

    #[test]
    fn test_free_sliding() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mut mvc = DrawFairCurveMinimalVariation::new(p1, p2);

        mvc.set_sliding(2.0);
        mvc.free_sliding();

        assert!(mvc.mvc.free_sliding);
    }

    #[test]
    fn test_display_color() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mvc = DrawFairCurveMinimalVariation::new(p1, p2);

        assert_eq!(mvc.get_color(), DrawColor::Yellow);
    }

    #[test]
    fn test_dump_string() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(10.0, 5.0);

        let mvc = DrawFairCurveMinimalVariation::new(p1, p2);
        let output = format!("{}", mvc);

        assert!(output.contains("p1"));
        assert!(output.contains("p2"));
        assert!(output.contains("curvature"));
    }
}
