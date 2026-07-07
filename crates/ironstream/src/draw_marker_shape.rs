// FILE: draw_marker_shape.rs
// occt: Draw_MarkerShape

//! Enumeration of marker shapes for drawing in the Draw application.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrawMarkerShape {
    /// Square marker
    Square = 0,
    /// Diamond/Losange marker
    Losange = 1,
    /// X marker
    X = 2,
    /// Plus marker
    Plus = 3,
    /// Circle marker (not sensitive to zoom)
    Circle = 4,
    /// Circle marker (sensitive to zoom)
    CircleZoom = 5,
}

impl DrawMarkerShape {
    /// Convert from integer representation
    pub fn from_int(value: u32) -> Option<Self> {
        match value {
            0 => Some(DrawMarkerShape::Square),
            1 => Some(DrawMarkerShape::Losange),
            2 => Some(DrawMarkerShape::X),
            3 => Some(DrawMarkerShape::Plus),
            4 => Some(DrawMarkerShape::Circle),
            5 => Some(DrawMarkerShape::CircleZoom),
            _ => None,
        }
    }

    /// Convert to integer representation
    pub fn to_int(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_shape_values() {
        assert_eq!(DrawMarkerShape::Square.to_int(), 0);
        assert_eq!(DrawMarkerShape::Losange.to_int(), 1);
        assert_eq!(DrawMarkerShape::X.to_int(), 2);
        assert_eq!(DrawMarkerShape::Plus.to_int(), 3);
        assert_eq!(DrawMarkerShape::Circle.to_int(), 4);
        assert_eq!(DrawMarkerShape::CircleZoom.to_int(), 5);
    }

    #[test]
    fn test_marker_shape_from_int() {
        assert_eq!(DrawMarkerShape::from_int(0), Some(DrawMarkerShape::Square));
        assert_eq!(DrawMarkerShape::from_int(4), Some(DrawMarkerShape::Circle));
        assert_eq!(DrawMarkerShape::from_int(6), None);
    }
}
