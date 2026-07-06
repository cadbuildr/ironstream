// FILE: draw_tr_surf.rs
// occt: DrawTrSurf

//! DrawTrSurf is a utility package for drawing and registering parametric curves and surfaces.
//!
//! It provides static methods to store and retrieve geometric objects (curves, surfaces,
//! points, triangulations, polygons) in a named variable system, with support for
//! displaying them via the Draw interface.

use std::collections::HashMap;
use std::sync::Mutex;
use std::cell::RefCell;

/// A 3D point.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// A 2D point.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// A 3D curve.
#[derive(Clone, Debug, PartialEq)]
pub struct Curve {
    pub curve_id: u32,
}

impl Curve {
    pub fn new(curve_id: u32) -> Self {
        Self { curve_id }
    }
}

/// A 2D curve.
#[derive(Clone, Debug, PartialEq)]
pub struct Curve2d {
    pub curve_id: u32,
}

impl Curve2d {
    pub fn new(curve_id: u32) -> Self {
        Self { curve_id }
    }
}

/// A Bezier curve (3D).
#[derive(Clone, Debug, PartialEq)]
pub struct BezierCurve {
    pub curve_id: u32,
    pub control_points: Vec<Point>,
}

impl BezierCurve {
    pub fn new(curve_id: u32, control_points: Vec<Point>) -> Self {
        Self {
            curve_id,
            control_points,
        }
    }
}

/// A B-spline curve (3D).
#[derive(Clone, Debug, PartialEq)]
pub struct BSplineCurve {
    pub curve_id: u32,
    pub control_points: Vec<Point>,
}

impl BSplineCurve {
    pub fn new(curve_id: u32, control_points: Vec<Point>) -> Self {
        Self {
            curve_id,
            control_points,
        }
    }
}

/// A 2D Bezier curve.
#[derive(Clone, Debug, PartialEq)]
pub struct BezierCurve2d {
    pub curve_id: u32,
    pub control_points: Vec<Point2d>,
}

impl BezierCurve2d {
    pub fn new(curve_id: u32, control_points: Vec<Point2d>) -> Self {
        Self {
            curve_id,
            control_points,
        }
    }
}

/// A 2D B-spline curve.
#[derive(Clone, Debug, PartialEq)]
pub struct BSplineCurve2d {
    pub curve_id: u32,
    pub control_points: Vec<Point2d>,
}

impl BSplineCurve2d {
    pub fn new(curve_id: u32, control_points: Vec<Point2d>) -> Self {
        Self {
            curve_id,
            control_points,
        }
    }
}

/// A 3D surface.
#[derive(Clone, Debug, PartialEq)]
pub struct Surface {
    pub surface_id: u32,
}

impl Surface {
    pub fn new(surface_id: u32) -> Self {
        Self { surface_id }
    }
}

/// A Bezier surface.
#[derive(Clone, Debug, PartialEq)]
pub struct BezierSurface {
    pub surface_id: u32,
}

impl BezierSurface {
    pub fn new(surface_id: u32) -> Self {
        Self { surface_id }
    }
}

/// A B-spline surface.
#[derive(Clone, Debug, PartialEq)]
pub struct BSplineSurface {
    pub surface_id: u32,
}

impl BSplineSurface {
    pub fn new(surface_id: u32) -> Self {
        Self { surface_id }
    }
}

/// A triangulation (mesh).
#[derive(Clone, Debug, PartialEq)]
pub struct Triangulation {
    pub triangulation_id: u32,
    pub vertices: Vec<Point>,
    pub triangles: Vec<[usize; 3]>,
}

impl Triangulation {
    pub fn new(triangulation_id: u32, vertices: Vec<Point>, triangles: Vec<[usize; 3]>) -> Self {
        Self {
            triangulation_id,
            vertices,
            triangles,
        }
    }
}

/// A 3D polygon.
#[derive(Clone, Debug, PartialEq)]
pub struct Polygon3d {
    pub polygon_id: u32,
    pub vertices: Vec<Point>,
}

impl Polygon3d {
    pub fn new(polygon_id: u32, vertices: Vec<Point>) -> Self {
        Self {
            polygon_id,
            vertices,
        }
    }
}

/// A 2D polygon.
#[derive(Clone, Debug, PartialEq)]
pub struct Polygon2d {
    pub polygon_id: u32,
    pub vertices: Vec<Point2d>,
}

impl Polygon2d {
    pub fn new(polygon_id: u32, vertices: Vec<Point2d>) -> Self {
        Self {
            polygon_id,
            vertices,
        }
    }
}

/// Geometric object variants for storage.
#[derive(Clone, Debug, PartialEq)]
pub enum GeometricObject {
    Point(Point),
    Point2d(Point2d),
    Curve(Curve),
    Curve2d(Curve2d),
    BezierCurve(BezierCurve),
    BSplineCurve(BSplineCurve),
    BezierCurve2d(BezierCurve2d),
    BSplineCurve2d(BSplineCurve2d),
    Surface(Surface),
    BezierSurface(BezierSurface),
    BSplineSurface(BSplineSurface),
    Triangulation(Triangulation),
    Polygon3d(Polygon3d),
    Polygon2d(Polygon2d),
}

/// Global parameters for DrawTrSurf display.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawTrSurfParams {
    pub u_samples: usize,
    pub v_samples: usize,
    pub discretization_mode: DiscretizationMode,
}

impl DrawTrSurfParams {
    pub fn new() -> Self {
        Self {
            u_samples: 50,
            v_samples: 50,
            discretization_mode: DiscretizationMode::Parametric,
        }
    }
}

impl Default for DrawTrSurfParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Discretization mode for curves and surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiscretizationMode {
    Parametric,
    Adaptive,
    Uniform,
}

/// A Draw Interpretor stub for command registration.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawInterpretor {
    commands: Vec<String>,
}

impl DrawInterpretor {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn commands(&self) -> &[String] {
        &self.commands
    }
}

impl Default for DrawInterpretor {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-local storage for DrawTrSurf variables and parameters.
thread_local! {
    static DRAW_VARIABLES: RefCell<HashMap<String, GeometricObject>> = RefCell::new(HashMap::new());
    static DRAW_PARAMETERS: RefCell<DrawTrSurfParams> = RefCell::new(DrawTrSurfParams::new());
}

/// DrawTrSurf utility class for managing drawable geometry.
pub struct DrawTrSurf;

impl DrawTrSurf {
    /// Set a point in a named variable.
    pub fn set_point(name: &str, point: Point) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::Point(point));
        });
    }

    /// Set a 2D point in a named variable.
    pub fn set_point2d(name: &str, point: Point2d) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::Point2d(point));
        });
    }

    /// Set a 3D curve in a named variable.
    pub fn set_curve(name: &str, curve: Curve) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::Curve(curve));
        });
    }

    /// Set a 2D curve in a named variable.
    pub fn set_curve2d(name: &str, curve: Curve2d) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::Curve2d(curve));
        });
    }

    /// Set a Bezier curve in a named variable.
    pub fn set_bezier_curve(name: &str, curve: BezierCurve) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::BezierCurve(curve));
        });
    }

    /// Set a B-spline curve in a named variable.
    pub fn set_bspline_curve(name: &str, curve: BSplineCurve) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::BSplineCurve(curve));
        });
    }

    /// Set a 2D Bezier curve in a named variable.
    pub fn set_bezier_curve2d(name: &str, curve: BezierCurve2d) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::BezierCurve2d(curve));
        });
    }

    /// Set a 2D B-spline curve in a named variable.
    pub fn set_bspline_curve2d(name: &str, curve: BSplineCurve2d) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::BSplineCurve2d(curve));
        });
    }

    /// Set a surface in a named variable.
    pub fn set_surface(name: &str, surface: Surface) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::Surface(surface));
        });
    }

    /// Set a Bezier surface in a named variable.
    pub fn set_bezier_surface(name: &str, surface: BezierSurface) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::BezierSurface(surface));
        });
    }

    /// Set a B-spline surface in a named variable.
    pub fn set_bspline_surface(name: &str, surface: BSplineSurface) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::BSplineSurface(surface));
        });
    }

    /// Set a triangulation in a named variable.
    pub fn set_triangulation(name: &str, triangulation: Triangulation) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::Triangulation(triangulation));
        });
    }

    /// Set a 3D polygon in a named variable.
    pub fn set_polygon3d(name: &str, polygon: Polygon3d) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::Polygon3d(polygon));
        });
    }

    /// Set a 2D polygon in a named variable.
    pub fn set_polygon2d(name: &str, polygon: Polygon2d) {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().insert(name.to_string(), GeometricObject::Polygon2d(polygon));
        });
    }

    /// Get a geometric object by name.
    pub fn get(name: &str) -> Option<GeometricObject> {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow().get(name).cloned()
        })
    }

    /// Get a point by name.
    pub fn get_point(name: &str) -> Option<Point> {
        DRAW_VARIABLES.with(|vars| {
            if let Some(GeometricObject::Point(p)) = vars.borrow().get(name) {
                Some(*p)
            } else {
                None
            }
        })
    }

    /// Get a 2D point by name.
    pub fn get_point2d(name: &str) -> Option<Point2d> {
        DRAW_VARIABLES.with(|vars| {
            if let Some(GeometricObject::Point2d(p)) = vars.borrow().get(name) {
                Some(*p)
            } else {
                None
            }
        })
    }

    /// Get the global DrawTrSurf parameters.
    pub fn parameters() -> DrawTrSurfParams {
        DRAW_PARAMETERS.with(|params| {
            params.borrow().clone()
        })
    }

    /// Set the global DrawTrSurf parameters.
    pub fn set_parameters(params: DrawTrSurfParams) {
        DRAW_PARAMETERS.with(|global_params| {
            *global_params.borrow_mut() = params;
        });
    }

    /// Clear all registered variables.
    pub fn clear_all() {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow_mut().clear();
        });
    }

    /// Get number of registered variables.
    pub fn variable_count() -> usize {
        DRAW_VARIABLES.with(|vars| {
            vars.borrow().len()
        })
    }

    /// Register basic Draw commands (stub implementation).
    pub fn basic_commands(_interpretor: &mut DrawInterpretor) {
        // In a real implementation, this would register TCL commands
        // for drawing curves, surfaces, triangulations, etc.
        // Example commands: trsurf, bezier, bspline, polygons, etc.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_point() {
        DrawTrSurf::clear_all();

        let point = Point::new(1.0, 2.0, 3.0);
        DrawTrSurf::set_point("test_pt", point);

        let retrieved = DrawTrSurf::get_point("test_pt");
        assert_eq!(retrieved, Some(point));
    }

    #[test]
    fn test_set_and_get_point2d() {
        DrawTrSurf::clear_all();

        let point = Point2d::new(4.5, 5.5);
        DrawTrSurf::set_point2d("test_pt2d", point);

        let retrieved = DrawTrSurf::get_point2d("test_pt2d");
        assert_eq!(retrieved, Some(point));
    }

    #[test]
    fn test_set_bezier_curve() {
        DrawTrSurf::clear_all();

        let control_points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(2.0, 0.0, 0.0),
        ];
        let curve = BezierCurve::new(1, control_points);

        DrawTrSurf::set_bezier_curve("bezier", curve);

        let retrieved = DrawTrSurf::get("bezier");
        assert!(matches!(retrieved, Some(GeometricObject::BezierCurve(_))));
    }

    #[test]
    fn test_set_bspline_curve2d() {
        DrawTrSurf::clear_all();

        let control_points = vec![Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.0)];
        let curve = BSplineCurve2d::new(1, control_points);

        DrawTrSurf::set_bspline_curve2d("bspline2d", curve);

        let retrieved = DrawTrSurf::get("bspline2d");
        assert!(matches!(retrieved, Some(GeometricObject::BSplineCurve2d(_))));
    }

    #[test]
    fn test_set_surface() {
        DrawTrSurf::clear_all();

        let surface = Surface::new(10);
        DrawTrSurf::set_surface("surf", surface);

        let retrieved = DrawTrSurf::get("surf");
        assert!(matches!(retrieved, Some(GeometricObject::Surface(_))));
    }

    #[test]
    fn test_set_triangulation() {
        DrawTrSurf::clear_all();

        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let triangles = vec![[0, 1, 2]];
        let triangulation = Triangulation::new(1, vertices, triangles);

        DrawTrSurf::set_triangulation("tri", triangulation);

        let retrieved = DrawTrSurf::get("tri");
        assert!(matches!(retrieved, Some(GeometricObject::Triangulation(_))));
    }

    #[test]
    fn test_set_polygon3d() {
        DrawTrSurf::clear_all();

        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.5, 1.0, 0.0),
        ];
        let polygon = Polygon3d::new(1, vertices);

        DrawTrSurf::set_polygon3d("poly", polygon);

        let retrieved = DrawTrSurf::get("poly");
        assert!(matches!(retrieved, Some(GeometricObject::Polygon3d(_))));
    }

    #[test]
    fn test_set_polygon2d() {
        DrawTrSurf::clear_all();

        let vertices = vec![Point2d::new(0.0, 0.0), Point2d::new(1.0, 0.0)];
        let polygon = Polygon2d::new(1, vertices);

        DrawTrSurf::set_polygon2d("poly2d", polygon);

        let retrieved = DrawTrSurf::get("poly2d");
        assert!(matches!(retrieved, Some(GeometricObject::Polygon2d(_))));
    }

    #[test]
    fn test_parameters() {
        let params = DrawTrSurf::parameters();
        assert_eq!(params.u_samples, 50);
        assert_eq!(params.v_samples, 50);
    }

    #[test]
    fn test_set_parameters() {
        let mut params = DrawTrSurfParams::new();
        params.u_samples = 100;
        params.v_samples = 100;

        DrawTrSurf::set_parameters(params);

        let retrieved = DrawTrSurf::parameters();
        assert_eq!(retrieved.u_samples, 100);
        assert_eq!(retrieved.v_samples, 100);
    }

    #[test]
    fn test_variable_count() {
        DrawTrSurf::clear_all();

        let point = Point::new(0.0, 0.0, 0.0);
        DrawTrSurf::set_point("pt1", point);
        DrawTrSurf::set_point("pt2", point);

        let count = DrawTrSurf::variable_count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_clear_all() {
        let point = Point::new(0.0, 0.0, 0.0);
        DrawTrSurf::set_point("test", point);

        assert!(DrawTrSurf::variable_count() > 0);

        DrawTrSurf::clear_all();
        assert_eq!(DrawTrSurf::variable_count(), 0);
    }

    #[test]
    fn test_get_nonexistent() {
        DrawTrSurf::clear_all();

        let result = DrawTrSurf::get("nonexistent");
        assert_eq!(result, None);
    }
}
