// FILE: rw_mesh_shape_iterator.rs
// occt: RWMesh_ShapeIterator

//! This is a virtual base class for other shape iterators.
//! Provides an abstract interface for iterating over the elements of a shape.
//! It defines a set of methods that must be implemented by derived classes
//! to handle specific types of shapes and their elements.

use std::collections::HashMap;

/// Shape explorer type enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeType {
    Vertex,
    Edge,
    Face,
    Wire,
    Shell,
    Solid,
    CompSolid,
    Compound,
}

/// Style information for presentation
#[derive(Clone, Debug, Default)]
pub struct StyleInfo;

/// Color in RGBA format
#[derive(Clone, Debug, Default)]
pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Transform matrix placeholder
#[derive(Clone, Debug, Default)]
pub struct TransformMatrix;

/// Location information
#[derive(Clone, Debug, Default)]
pub struct Location;

/// Point in 3D space
#[derive(Clone, Debug)]
pub struct Point3D;

/// Direction in 3D space
#[derive(Clone, Debug)]
pub struct Direction3D;

/// Shape representation
#[derive(Clone, Debug)]
pub struct Shape;

/// Base iterator for shape traversal
pub struct ShapeIterator {
    /// Shape explorer
    shape_explorer: Option<Shape>,
    /// Current location
    location: Location,
    /// Current transformation
    trsf: TransformMatrix,
    /// Style for current shape
    style: StyleInfo,
    /// Current shape color
    color: ColorRGBA,
    /// Shape type being explored
    shape_type: ShapeType,
    /// Whether shape has a color
    has_color: bool,
    /// Map of shapes to styles
    styles: HashMap<String, StyleInfo>,
    /// Default style
    def_style: StyleInfo,
    /// Flag to map colors
    to_map_colors: bool,
}

impl ShapeIterator {
    /// Main constructor
    pub fn new(
        shape_type_find: ShapeType,
        shape_type_avoid: ShapeType,
        to_map_colors: bool,
        style: StyleInfo,
    ) -> Self {
        ShapeIterator {
            shape_explorer: None,
            location: Location::default(),
            trsf: TransformMatrix::default(),
            style,
            color: ColorRGBA::default(),
            shape_type: shape_type_find,
            has_color: false,
            styles: HashMap::new(),
            def_style: StyleInfo::default(),
            to_map_colors,
        }
    }

    /// Auxiliary constructor with label
    pub fn with_label(
        shape_type_find: ShapeType,
        shape_type_avoid: ShapeType,
        to_map_colors: bool,
        style: StyleInfo,
    ) -> Self {
        Self::new(shape_type_find, shape_type_avoid, to_map_colors, style)
    }

    /// Get the explored shape
    pub fn explored_shape(&self) -> Option<&Shape> {
        self.shape_explorer.as_ref()
    }

    /// Get the current style
    pub fn style(&self) -> &StyleInfo {
        &self.style
    }

    /// Check if shape has a color
    pub fn has_color(&self) -> bool {
        self.has_color
    }

    /// Get the current color
    pub fn color(&self) -> &ColorRGBA {
        &self.color
    }

    /// Get the node with applied transformation
    pub fn node_transformed(&self, node_index: i32) -> Option<Point3D> {
        self.node(node_index)
    }

    /// Get the shape type
    pub fn shape_type(&self) -> ShapeType {
        self.shape_type
    }

    /// Get the location
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Get the transformation
    pub fn trsf(&self) -> &TransformMatrix {
        &self.trsf
    }

    /// Dispatch shape styles
    pub fn dispatch_styles(&mut self, style: StyleInfo) {
        if self.to_map_colors {
            self.style = style;
        }
    }

    /// Reset shape information
    pub fn reset_shape(&mut self) {
        self.has_color = false;
        self.color = ColorRGBA::default();
        self.style = StyleInfo::default();
    }

    /// Initialize shape properties (protected)
    pub fn init_shape(&mut self) {
        // Initialize shape from explorer position
    }

    // Virtual methods that must be implemented by subclasses

    /// Return the current shape
    pub fn shape(&self) -> Option<&Shape> {
        None // To be overridden
    }

    /// Check if more elements are available
    pub fn more(&self) -> bool {
        false // To be overridden
    }

    /// Move to next element
    pub fn next(&mut self) {
        // To be overridden
    }

    /// Check if data is empty
    pub fn is_empty(&self) -> bool {
        true // To be overridden
    }

    /// Lower element index
    pub fn elem_lower(&self) -> i32 {
        1 // To be overridden
    }

    /// Upper element index
    pub fn elem_upper(&self) -> i32 {
        0 // To be overridden
    }

    /// Get number of nodes
    pub fn nb_nodes(&self) -> i32 {
        0 // To be overridden
    }

    /// Lower node index
    pub fn node_lower(&self) -> i32 {
        1 // To be overridden
    }

    /// Upper node index
    pub fn node_upper(&self) -> i32 {
        0 // To be overridden
    }

    /// Get node (to be overridden by subclasses)
    pub fn node(&self, _node_index: i32) -> Option<Point3D> {
        None // To be overridden
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_iterator_creation() {
        let iterator = ShapeIterator::new(
            ShapeType::Face,
            ShapeType::Vertex,
            false,
            StyleInfo::default(),
        );

        assert_eq!(iterator.shape_type(), ShapeType::Face);
        assert!(!iterator.has_color());
        assert!(!iterator.more());
        assert!(iterator.is_empty());
    }

    #[test]
    fn test_shape_type_enum() {
        assert_eq!(ShapeType::Vertex, ShapeType::Vertex);
        assert_ne!(ShapeType::Face, ShapeType::Edge);
    }

    #[test]
    fn test_default_bounds() {
        let iterator = ShapeIterator::new(
            ShapeType::Face,
            ShapeType::Vertex,
            false,
            StyleInfo::default(),
        );

        assert_eq!(iterator.elem_lower(), 1);
        assert_eq!(iterator.elem_upper(), 0);
        assert_eq!(iterator.node_lower(), 1);
        assert_eq!(iterator.node_upper(), 0);
    }

    #[test]
    fn test_reset_shape() {
        let mut iterator = ShapeIterator::new(
            ShapeType::Face,
            ShapeType::Vertex,
            false,
            StyleInfo::default(),
        );

        iterator.reset_shape();
        assert!(!iterator.has_color());
    }

    #[test]
    fn test_with_label_constructor() {
        let iterator = ShapeIterator::with_label(
            ShapeType::Edge,
            ShapeType::Vertex,
            true,
            StyleInfo::default(),
        );

        assert_eq!(iterator.shape_type(), ShapeType::Edge);
    }

    #[test]
    fn test_location_default() {
        let location = Location::default();
        // Should create a default location
        let _ = location;
    }

    #[test]
    fn test_color_rgba_default() {
        let color = ColorRGBA::default();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 0);
    }
}
