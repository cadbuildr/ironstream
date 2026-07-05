// FILE: draw_marker2_d.rs
// occt: Draw_Marker2D

//! A drawable 2D marker for the Draw application.

/// Represents a 2D marker drawable
pub struct DrawMarker2D {
    /// Position X coordinate
    x: f64,
    /// Position Y coordinate
    y: f64,
    /// Marker shape
    shape: DrawMarkerShape,
    /// Marker color
    color: u32,
    /// Marker size
    size: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawMarkerShape {
    /// Square marker
    Square = 0,
    /// Diamond marker
    Losange = 1,
    /// X marker
    X = 2,
    /// Plus marker
    Plus = 3,
    /// Circle marker
    Circle = 4,
    /// Circle marker with zoom sensitivity
    CircleZoom = 5,
}

impl DrawMarker2D {
    /// Create a new 2D marker
    pub fn new(x: f64, y: f64, shape: DrawMarkerShape, color: u32, size: f64) -> Self {
        DrawMarker2D {
            x,
            y,
            shape,
            color,
            size,
        }
    }

    /// Get the position
    pub fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    /// Set the position
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Get the marker shape
    pub fn shape(&self) -> DrawMarkerShape {
        self.shape
    }

    /// Set the marker shape
    pub fn set_shape(&mut self, shape: DrawMarkerShape) {
        self.shape = shape;
    }

    /// Get the color
    pub fn color(&self) -> u32 {
        self.color
    }

    /// Set the color
    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    /// Get the size
    pub fn size(&self) -> f64 {
        self.size
    }

    /// Set the size
    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker2d_creation() {
        let marker = DrawMarker2D::new(10.0, 20.0, DrawMarkerShape::Square, 0xFF0000, 5.0);
        assert_eq!(marker.position(), (10.0, 20.0));
        assert_eq!(marker.shape(), DrawMarkerShape::Square);
        assert_eq!(marker.color(), 0xFF0000);
        assert_eq!(marker.size(), 5.0);
    }

    #[test]
    fn test_marker2d_position() {
        let mut marker = DrawMarker2D::new(0.0, 0.0, DrawMarkerShape::Circle, 0x00FF00, 3.0);
        marker.set_position(5.0, 10.0);
        assert_eq!(marker.position(), (5.0, 10.0));
    }

    #[test]
    fn test_marker2d_all_shapes() {
        let shapes = vec![
            DrawMarkerShape::Square,
            DrawMarkerShape::Losange,
            DrawMarkerShape::X,
            DrawMarkerShape::Plus,
            DrawMarkerShape::Circle,
            DrawMarkerShape::CircleZoom,
        ];

        for shape in shapes {
            let marker = DrawMarker2D::new(0.0, 0.0, shape, 0xFFFFFF, 5.0);
            assert_eq!(marker.shape(), shape);
        }
    }
}
