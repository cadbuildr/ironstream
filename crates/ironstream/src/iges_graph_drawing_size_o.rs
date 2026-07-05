// FILE: iges_graph_drawing_size_o.rs
// occt: IGESGraph_DrawingSize

/// Represents an IGES Drawing Size entity (Type 406, Form 16).
/// Specifies the drawing size in drawing units.
/// The origin of the drawing is defined to be (0,0) in drawing space.
pub struct IgesGraphDrawingSize {
    nb_property_values: i32,
    x_size: f64,
    y_size: f64,
}

impl IgesGraphDrawingSize {
    /// Creates a new empty DrawingSize entity.
    pub fn new() -> Self {
        IgesGraphDrawingSize {
            nb_property_values: 0,
            x_size: 0.0,
            y_size: 0.0,
        }
    }

    /// Sets the fields of the DrawingSize entity.
    ///
    /// # Arguments
    /// - `nb_props`: Number of property values (should be 2)
    /// - `x_size`: Extent of drawing along positive XD axis
    /// - `y_size`: Extent of drawing along positive YD axis
    pub fn init(&mut self, nb_props: i32, x_size: f64, y_size: f64) {
        self.nb_property_values = nb_props;
        self.x_size = x_size;
        self.y_size = y_size;
    }

    /// Returns the number of property values (NP = 2).
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the extent of drawing along positive XD axis.
    pub fn x_size(&self) -> f64 {
        self.x_size
    }

    /// Returns the extent of drawing along positive YD axis.
    pub fn y_size(&self) -> f64 {
        self.y_size
    }
}

impl Default for IgesGraphDrawingSize {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawing_size_creation() {
        let ds = IgesGraphDrawingSize::new();
        assert_eq!(ds.nb_property_values(), 0);
        assert_eq!(ds.x_size(), 0.0);
        assert_eq!(ds.y_size(), 0.0);
    }

    #[test]
    fn test_drawing_size_init() {
        let mut ds = IgesGraphDrawingSize::new();
        ds.init(2, 100.0, 200.0);
        assert_eq!(ds.nb_property_values(), 2);
        assert_eq!(ds.x_size(), 100.0);
        assert_eq!(ds.y_size(), 200.0);
    }

    #[test]
    fn test_drawing_size_init_different_values() {
        let mut ds = IgesGraphDrawingSize::new();
        ds.init(2, 50.5, 75.25);
        assert_eq!(ds.x_size(), 50.5);
        assert_eq!(ds.y_size(), 75.25);
    }
}
