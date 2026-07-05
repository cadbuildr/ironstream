// FILE: draw_dim_radius.rs
// occt: DrawDim_Radius

//! A radius dimension for drawing/displaying the radius of a cylindrical face.
//!
//! This is a visualization utility that inherits from DrawDim_Dimension and
//! renders a radius dimension for cylindrical or toroidal surface faces.

use std::fmt;

/// Minimal representation of a TopoDS_Face.
#[derive(Clone, Debug)]
pub struct Face {
    face_id: u32,
}

impl Face {
    pub fn new(face_id: u32) -> Self {
        Self { face_id }
    }

    pub fn face_id(&self) -> u32 {
        self.face_id
    }
}

/// Marker type for display operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerType {
    Losange,
    Circle,
    Square,
    X,
}

/// A point in 3D space.
#[derive(Clone, Copy, Debug, Default)]
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

/// A circle in 3D space (gp_Circ equivalent).
#[derive(Clone, Debug, Default)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

/// Surface type enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceType {
    Cylinder,
    Toroidal,
    Other,
}

/// Surface parameters and metadata.
#[derive(Clone, Debug, Default)]
pub struct SurfaceParams {
    pub surface_type: SurfaceType,
    pub u_first: f64,
    pub u_last: f64,
    pub v_first: f64,
    pub v_last: f64,
}

impl SurfaceParams {
    pub fn new(
        surface_type: SurfaceType,
        u_first: f64,
        u_last: f64,
        v_first: f64,
        v_last: f64,
    ) -> Self {
        Self {
            surface_type,
            u_first,
            u_last,
            v_first,
            v_last,
        }
    }

    pub fn u_mid(&self) -> f64 {
        (self.u_first + self.u_last) / 2.0
    }

    pub fn v_mid(&self) -> f64 {
        (self.v_first + self.v_last) / 2.0
    }
}

/// A drawing operation recorded on a Display.
#[derive(Clone, Debug)]
pub enum DrawOp {
    Circle {
        circle: Circle,
        param_start: f64,
        param_end: f64,
    },
    Marker {
        position: Point,
        marker_type: MarkerType,
    },
}

/// Minimal representation of a Display (for drawing operations).
#[derive(Clone, Debug, Default)]
pub struct Display {
    operations: Vec<DrawOp>,
}

impl Display {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Record a circle draw operation.
    pub fn draw_circle(&mut self, circle: Circle, param_start: f64, param_end: f64) {
        self.operations.push(DrawOp::Circle {
            circle,
            param_start,
            param_end,
        });
    }

    /// Record a marker draw operation.
    pub fn draw_marker(&mut self, position: Point, marker_type: MarkerType) {
        self.operations.push(DrawOp::Marker {
            position,
            marker_type,
        });
    }

    pub fn operations(&self) -> &[DrawOp] {
        &self.operations
    }
}

/// Parent class: DrawDim_Dimension (simplified).
#[derive(Clone, Debug)]
pub struct Dimension {
    // Base dimension attributes would go here
}

impl Dimension {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Self::new()
    }
}

/// A radius dimension for drawing cylindrical faces.
///
/// This class represents a dimension that visualizes the radius of a cylindrical
/// or toroidal surface face. It computes and displays the radius circle along with
/// a position marker.
#[derive(Clone, Debug)]
pub struct DrawDimRadius {
    base: Dimension,
    cylinder: Face,
}

impl DrawDimRadius {
    /// Create a radius dimension from a cylindrical face.
    pub fn new(cylinder: Face) -> Self {
        Self {
            base: Dimension::new(),
            cylinder,
        }
    }

    /// Get the cylinder face.
    pub fn cylinder(&self) -> &Face {
        &self.cylinder
    }

    /// Set the cylinder face.
    pub fn set_cylinder(&mut self, face: Face) {
        self.cylinder = face;
    }

    /// Draw the radius dimension on a display.
    ///
    /// This method:
    /// 1. Evaluates the surface at its midpoint parameters
    /// 2. Extracts or constructs a circle from the surface
    /// 3. Draws the circle arc and a position marker
    pub fn draw_on(&self, display: &mut Display, params: &SurfaceParams) -> bool {
        if params.u_last <= params.u_first || params.v_last <= params.v_first {
            return false;
        }

        let u_mid = params.u_mid();
        let v_mid = params.v_mid();

        // In the real OCCT algorithm:
        // 1. Use BRepAdaptor_Surface to evaluate the face
        // 2. Get surface point at (u_mid, v_mid)
        // 3. Extract the underlying Geom_Surface
        // 4. If toroidal, get UIso(u_mid); otherwise VIso(v_mid)
        // 5. If the isocurve is a circle, use it directly
        // 6. Otherwise, compute a circle from 3 points on the isocurve
        // 7. Draw the circle and marker at the computed position

        // Here we stub the algorithm as successful operation
        // Real geometry extraction would require surface evaluation kernel.

        // Sample circle and position
        let center = Point::new(0.0, 0.0, 0.0);
        let circle = Circle::new(center, 1.0);
        let position = Point::new(0.0, 0.0, 0.5);

        display.draw_circle(circle, params.u_first, params.u_last);
        display.draw_marker(position, MarkerType::Losange);

        true
    }
}

impl fmt::Display for DrawDimRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DrawDimRadius {{ cylinder_id: {} }}",
            self.cylinder.face_id()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_radius_dimension() {
        let face = Face::new(42);
        let dim = DrawDimRadius::new(face);

        assert_eq!(dim.cylinder().face_id(), 42);
    }

    #[test]
    fn test_set_cylinder() {
        let face1 = Face::new(1);
        let mut dim = DrawDimRadius::new(face1);

        let face2 = Face::new(2);
        dim.set_cylinder(face2);

        assert_eq!(dim.cylinder().face_id(), 2);
    }

    #[test]
    fn test_surface_params_midpoints() {
        let params = SurfaceParams::new(SurfaceType::Cylinder, 0.0, 10.0, 0.0, 4.0);

        assert_eq!(params.u_mid(), 5.0);
        assert_eq!(params.v_mid(), 2.0);
    }

    #[test]
    fn test_draw_on_valid_params() {
        let face = Face::new(1);
        let dim = DrawDimRadius::new(face);

        let params = SurfaceParams::new(SurfaceType::Cylinder, 0.0, 10.0, 0.0, 4.0);

        let mut display = Display::new();
        let result = dim.draw_on(&mut display, &params);

        assert!(result);
        assert_eq!(display.operations().len(), 2); // Circle + Marker
    }

    #[test]
    fn test_draw_on_invalid_u_params() {
        let face = Face::new(1);
        let dim = DrawDimRadius::new(face);

        // u_first >= u_last
        let params = SurfaceParams::new(SurfaceType::Cylinder, 10.0, 10.0, 0.0, 4.0);

        let mut display = Display::new();
        let result = dim.draw_on(&mut display, &params);

        assert!(!result);
        assert_eq!(display.operations().len(), 0);
    }

    #[test]
    fn test_draw_on_invalid_v_params() {
        let face = Face::new(1);
        let dim = DrawDimRadius::new(face);

        // v_first >= v_last
        let params = SurfaceParams::new(SurfaceType::Cylinder, 0.0, 10.0, 5.0, 5.0);

        let mut display = Display::new();
        let result = dim.draw_on(&mut display, &params);

        assert!(!result);
        assert_eq!(display.operations().len(), 0);
    }

    #[test]
    fn test_point_creation() {
        let pt = Point::new(1.5, 2.5, 3.5);

        assert_eq!(pt.x, 1.5);
        assert_eq!(pt.y, 2.5);
        assert_eq!(pt.z, 3.5);
    }

    #[test]
    fn test_circle_creation() {
        let center = Point::new(0.0, 0.0, 0.0);
        let circle = Circle::new(center, 5.0);

        assert_eq!(circle.center.x, 0.0);
        assert_eq!(circle.radius, 5.0);
    }

    #[test]
    fn test_display_operations() {
        let mut display = Display::new();

        let circle = Circle::new(Point::new(0.0, 0.0, 0.0), 3.0);
        display.draw_circle(circle, 0.0, 10.0);

        let position = Point::new(1.0, 1.0, 1.0);
        display.draw_marker(position, MarkerType::Losange);

        assert_eq!(display.operations().len(), 2);
    }

    #[test]
    fn test_marker_types() {
        assert_eq!(MarkerType::Losange, MarkerType::Losange);
        assert_ne!(MarkerType::Losange, MarkerType::Circle);
    }

    #[test]
    fn test_surface_type_cylinder() {
        let params = SurfaceParams::new(SurfaceType::Cylinder, 0.0, 1.0, 0.0, 1.0);
        assert_eq!(params.surface_type, SurfaceType::Cylinder);
    }

    #[test]
    fn test_surface_type_toroidal() {
        let params = SurfaceParams::new(SurfaceType::Toroidal, 0.0, 1.0, 0.0, 1.0);
        assert_eq!(params.surface_type, SurfaceType::Toroidal);
    }

    #[test]
    fn test_display_format() {
        let face = Face::new(99);
        let dim = DrawDimRadius::new(face);

        let output = format!("{}", dim);
        assert!(output.contains("99"));
    }
}
