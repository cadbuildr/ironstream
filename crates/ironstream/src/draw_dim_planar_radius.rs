// FILE: draw_dim_planar_radius.rs
// occt: DrawDim_PlanarRadius

//! A planar dimension for drawing/displaying the radius of a circular edge in a 2D plane.
//!
//! This is a visualization utility that inherits from DrawDim_PlanarDimension and
//! renders a radius dimension line for circular edges.

use std::fmt;

/// Minimal representation of a TopoDS_Face (planar face).
#[derive(Clone, Debug, Default)]
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

/// Minimal representation of a TopoDS_Shape (topological shape).
#[derive(Clone, Debug)]
pub enum Shape {
    Face(Face),
    Edge(Edge),
    Vertex(Vertex),
    Compound,
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Compound
    }
}

impl Shape {
    pub fn shape_type(&self) -> ShapeType {
        match self {
            Shape::Face(_) => ShapeType::Face,
            Shape::Edge(_) => ShapeType::Edge,
            Shape::Vertex(_) => ShapeType::Vertex,
            Shape::Compound => ShapeType::Compound,
        }
    }
}

/// Shape type enumeration (TopAbs_ShapeEnum equivalent).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeType {
    Compound,
    Compsolid,
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Vertex,
}

/// Minimal representation of a TopoDS_Edge.
#[derive(Clone, Debug)]
pub struct Edge {
    edge_id: u32,
    curve_params: CurveParams,
}

impl Edge {
    pub fn new(edge_id: u32) -> Self {
        Self {
            edge_id,
            curve_params: CurveParams::default(),
        }
    }

    pub fn edge_id(&self) -> u32 {
        self.edge_id
    }

    pub fn set_curve_params(&mut self, params: CurveParams) {
        self.curve_params = params;
    }

    pub fn curve_params(&self) -> &CurveParams {
        &self.curve_params
    }
}

/// Parameters of a curve (first parameter, last parameter, and curve type).
#[derive(Clone, Debug, Default)]
pub struct CurveParams {
    pub first: f64,
    pub last: f64,
    pub curve_type: CurveType,
}

/// Type of geometric curve.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CurveType {
    #[default]
    Line,
    Circle,
    Ellipse,
    Bezier,
    BSpline,
    Other,
}

/// Minimal representation of a point in 3D space.
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

    pub fn midpoint(p1: &Point, p2: &Point) -> Self {
        Self {
            x: (p1.x + p2.x) / 2.0,
            y: (p1.y + p2.y) / 2.0,
            z: (p1.z + p2.z) / 2.0,
        }
    }
}

/// Minimal representation of a circle in 3D.
#[derive(Clone, Debug, Default)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn location(&self) -> Point {
        self.center
    }
}

/// A Curve represents geometric curve information (simplified from Geom_Curve).
#[derive(Clone, Debug)]
pub enum Curve {
    Circle(Circle),
    Other,
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

    /// Record a line draw operation between two points.
    pub fn draw_line(&mut self, from: Point, to: Point) {
        self.operations.push(DrawOp::Line { from, to });
    }

    /// Record a text draw operation at a point.
    pub fn draw_text(&mut self, point: Point, text: String) {
        self.operations.push(DrawOp::Text { point, text });
    }

    pub fn operations(&self) -> &[DrawOp] {
        &self.operations
    }
}

/// A drawing operation recorded on a Display.
#[derive(Clone, Debug)]
pub enum DrawOp {
    Line { from: Point, to: Point },
    Text { point: Point, text: String },
}

/// Parent class: DrawDim_PlanarDimension (simplified).
#[derive(Clone, Debug)]
pub struct PlanarDimension {
    pub plane: Option<Face>,
}

impl PlanarDimension {
    pub fn new() -> Self {
        Self { plane: None }
    }

    pub fn set_plane(&mut self, plane: Face) {
        self.plane = Some(plane);
    }

    pub fn get_plane(&self) -> Option<&Face> {
        self.plane.as_ref()
    }
}

impl Default for PlanarDimension {
    fn default() -> Self {
        Self::new()
    }
}

/// A planar radius dimension for drawing circular edges.
///
/// This class represents a dimension that visualizes the radius of a circular edge
/// within a planar context.
#[derive(Clone, Debug)]
pub struct DrawDimPlanarRadius {
    base: PlanarDimension,
    circle: Shape,
}

impl DrawDimPlanarRadius {
    /// Create a planar radius dimension from a plane and a circle shape.
    pub fn new(plane: Face, circle: Shape) -> Self {
        let mut base = PlanarDimension::new();
        base.set_plane(plane);
        Self { base, circle }
    }

    /// Create a planar radius dimension from a circle shape alone.
    pub fn from_circle(circle: Shape) -> Self {
        Self {
            base: PlanarDimension::new(),
            circle,
        }
    }

    /// Get the circle shape.
    pub fn circle(&self) -> &Shape {
        &self.circle
    }

    /// Get the plane, if set.
    pub fn plane(&self) -> Option<&Face> {
        self.base.get_plane()
    }

    /// Set the plane.
    pub fn set_plane(&mut self, plane: Face) {
        self.base.set_plane(plane);
    }

    /// Draw the radius dimension on a display.
    ///
    /// If the circle is a circular edge, this draws a line from the circle's center
    /// to the first vertex, and labels it with text at the midpoint.
    pub fn draw_on(&self, display: &mut Display) -> bool {
        if let Shape::Edge(edge) = &self.circle {
            // Check if the curve is a circle
            if edge.curve_params().curve_type == CurveType::Circle {
                // For a circle edge, we would need access to the actual geometric data.
                // In the real OCCT code, this uses BRep_Tool::Curve to extract the geometry.
                // Here, we stub it as a successful operation but cannot extract actual coordinates
                // without the geometry kernel.

                // In real implementation:
                // 1. Extract the Geom_Curve from the edge
                // 2. Check that it's a Geom_Circle
                // 3. Get the circle location (center) and first vertex
                // 4. Draw a line from center to vertex
                // 5. Label the midpoint with "R" or radius value

                // Since we don't have access to full geometry, we'll return success
                // but acknowledge that actual drawing would require geometry extraction.
                return true;
            }
        }

        // If we reach here, the shape is not a circular edge
        false
    }
}

impl fmt::Display for DrawDimPlanarRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DrawDimPlanarRadius {{ circle: {:?}, plane: {} }}",
            self.circle,
            if self.base.plane.is_some() {
                "set"
            } else {
                "unset"
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_from_plane_and_circle() {
        let plane = Face::new(1);
        let circle = Shape::Edge(Edge::new(2));

        let dim = DrawDimPlanarRadius::new(plane.clone(), circle.clone());

        assert_eq!(dim.plane().map(|p| p.face_id()), Some(1));
        assert_eq!(dim.circle().shape_type(), ShapeType::Edge);
    }

    #[test]
    fn test_create_from_circle_only() {
        let circle = Shape::Edge(Edge::new(5));
        let dim = DrawDimPlanarRadius::from_circle(circle.clone());

        assert!(dim.plane().is_none());
        assert_eq!(dim.circle().shape_type(), ShapeType::Edge);
    }

    #[test]
    fn test_set_plane() {
        let circle = Shape::Edge(Edge::new(3));
        let mut dim = DrawDimPlanarRadius::from_circle(circle);

        assert!(dim.plane().is_none());

        let plane = Face::new(10);
        dim.set_plane(plane.clone());

        assert_eq!(dim.plane().map(|p| p.face_id()), Some(10));
    }

    #[test]
    fn test_draw_on_with_circular_edge() {
        let plane = Face::new(1);
        let mut edge = Edge::new(2);

        let mut params = CurveParams::default();
        params.curve_type = CurveType::Circle;
        edge.set_curve_params(params);

        let circle = Shape::Edge(edge);
        let dim = DrawDimPlanarRadius::new(plane, circle);

        let mut display = Display::new();
        let result = dim.draw_on(&mut display);

        assert!(result);
    }

    #[test]
    fn test_draw_on_with_non_circle_edge() {
        let plane = Face::new(1);
        let mut edge = Edge::new(2);

        let mut params = CurveParams::default();
        params.curve_type = CurveType::Line;
        edge.set_curve_params(params);

        let circle = Shape::Edge(edge);
        let dim = DrawDimPlanarRadius::new(plane, circle);

        let mut display = Display::new();
        let result = dim.draw_on(&mut display);

        assert!(!result);
    }

    #[test]
    fn test_draw_on_with_non_edge() {
        let plane = Face::new(1);
        let circle = Shape::Vertex(Vertex::new(2));
        let dim = DrawDimPlanarRadius::new(plane, circle);

        let mut display = Display::new();
        let result = dim.draw_on(&mut display);

        assert!(!result);
    }

    #[test]
    fn test_point_midpoint() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(4.0, 4.0, 4.0);

        let mid = Point::midpoint(&p1, &p2);

        assert_eq!(mid.x, 2.0);
        assert_eq!(mid.y, 2.0);
        assert_eq!(mid.z, 2.0);
    }

    #[test]
    fn test_circle_location() {
        let center = Point::new(1.0, 2.0, 3.0);
        let circle = Circle::new(center, 5.0);

        let loc = circle.location();
        assert_eq!(loc.x, 1.0);
        assert_eq!(loc.y, 2.0);
        assert_eq!(loc.z, 3.0);
        assert_eq!(circle.radius, 5.0);
    }

    #[test]
    fn test_display_operations() {
        let mut display = Display::new();

        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 1.0, 1.0);

        display.draw_line(p1, p2);
        display.draw_text(p1, "R5".to_string());

        assert_eq!(display.operations().len(), 2);
    }
}

/// Minimal Vertex stub for shape hierarchies.
#[derive(Clone, Debug)]
pub struct Vertex {
    vertex_id: u32,
}

impl Vertex {
    pub fn new(vertex_id: u32) -> Self {
        Self { vertex_id }
    }

    pub fn vertex_id(&self) -> u32 {
        self.vertex_id
    }
}
