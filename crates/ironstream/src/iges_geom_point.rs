// FILE: iges_geom_point.rs
// occt: IGESGeom_Point

/// Defines IGESPoint, Type <116> Form <0> in package IGESGeom.
/// A point entity with optional display symbol defined by a subfigure.
#[derive(Clone, Debug)]
pub struct Point {
    /// Coordinates of the point (x, y, z)
    coordinates: [f64; 3],
    /// Optional display symbol (subfigure definition)
    /// Represented as a string identifier for the symbol
    display_symbol: Option<String>,
    /// Type number for IGES (always 116 for Point)
    entity_type: i32,
    /// Form number (always 0 for Point)
    form: i32,
}

impl Point {
    /// Creates a new Point entity.
    pub fn new() -> Self {
        Point {
            coordinates: [0.0, 0.0, 0.0],
            display_symbol: None,
            entity_type: 116,
            form: 0,
        }
    }

    /// Initializes the point with coordinates and optional symbol.
    pub fn init(&mut self, coordinates: [f64; 3], symbol: Option<String>) {
        self.coordinates = coordinates;
        self.display_symbol = symbol;
        self.entity_type = 116;
        self.form = 0;
    }

    /// Returns the coordinates of the point.
    pub fn value(&self) -> [f64; 3] {
        self.coordinates
    }

    /// Returns the coordinates after applying a transformation matrix.
    /// For this port, we return the coordinates directly as transformation
    /// would be applied externally.
    pub fn transformed_value(&self) -> [f64; 3] {
        // TODO: Apply transformation matrix if present
        self.coordinates
    }

    /// Returns true if a display symbol exists.
    pub fn has_display_symbol(&self) -> bool {
        self.display_symbol.is_some()
    }

    /// Returns the display symbol if it exists.
    pub fn display_symbol(&self) -> Option<&str> {
        self.display_symbol.as_deref()
    }

    /// Returns the entity type number (always 116 for Point).
    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }

    /// Returns the form number (always 0 for Point).
    pub fn form_number(&self) -> i32 {
        self.form
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_point() {
        let point = Point::new();
        assert_eq!(point.value(), [0.0, 0.0, 0.0]);
        assert!(!point.has_display_symbol());
        assert_eq!(point.entity_type(), 116);
        assert_eq!(point.form_number(), 0);
    }

    #[test]
    fn test_init_point_without_symbol() {
        let mut point = Point::new();
        point.init([1.5, 2.5, 3.5], None);
        assert_eq!(point.value(), [1.5, 2.5, 3.5]);
        assert!(!point.has_display_symbol());
        assert_eq!(point.entity_type(), 116);
    }

    #[test]
    fn test_init_point_with_symbol() {
        let mut point = Point::new();
        point.init([0.0, 0.0, 0.0], Some("SYMBOL_001".to_string()));
        assert_eq!(point.value(), [0.0, 0.0, 0.0]);
        assert!(point.has_display_symbol());
        assert_eq!(point.display_symbol(), Some("SYMBOL_001"));
    }

    #[test]
    fn test_transformed_value() {
        let mut point = Point::new();
        point.init([1.0, 2.0, 3.0], None);
        // Without transformation applied, transformed value equals original
        assert_eq!(point.transformed_value(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_symbol_handling() {
        let mut point = Point::new();
        assert!(!point.has_display_symbol());
        assert_eq!(point.display_symbol(), None);

        point.init([0.0, 0.0, 0.0], Some("CROSS".to_string()));
        assert!(point.has_display_symbol());
        assert_eq!(point.display_symbol(), Some("CROSS"));
    }
}
