// FILE: i_vtk_types.rs
// occt: IVtk_Types

/// Type definitions for IVtk.

/// Unique identifier for a shape.
pub type ShapeId = u32;

/// Unique identifier for a view.
pub type ViewId = u32;

/// Unique identifier for a cell.
pub type CellId = u32;

/// Unique identifier for a vertex.
pub type VertexId = u32;

/// 3D coordinate.
pub type Coord3d = [f64; 3];

/// 2D coordinate.
pub type Coord2d = [f64; 2];

/// RGB color.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Black color.
    pub fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }

    /// White color.
    pub fn white() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    /// Red color.
    pub fn red() -> Self {
        Color { r: 255, g: 0, b: 0 }
    }

    /// Green color.
    pub fn green() -> Self {
        Color { r: 0, g: 255, b: 0 }
    }

    /// Blue color.
    pub fn blue() -> Self {
        Color { r: 0, g: 0, b: 255 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(100, 150, 200);
        assert_eq!(color.r, 100);
        assert_eq!(color.g, 150);
        assert_eq!(color.b, 200);
    }

    #[test]
    fn test_color_presets() {
        assert_eq!(Color::black(), Color::new(0, 0, 0));
        assert_eq!(Color::white(), Color::new(255, 255, 255));
        assert_eq!(Color::red(), Color::new(255, 0, 0));
        assert_eq!(Color::green(), Color::new(0, 255, 0));
        assert_eq!(Color::blue(), Color::new(0, 0, 255));
    }

    #[test]
    fn test_coord3d() {
        let coord: Coord3d = [1.0, 2.0, 3.0];
        assert_eq!(coord[0], 1.0);
        assert_eq!(coord[1], 2.0);
        assert_eq!(coord[2], 3.0);
    }

    #[test]
    fn test_coord2d() {
        let coord: Coord2d = [4.0, 5.0];
        assert_eq!(coord[0], 4.0);
        assert_eq!(coord[1], 5.0);
    }

    #[test]
    fn test_type_aliases() {
        let shape_id: ShapeId = 42;
        let view_id: ViewId = 1;
        let cell_id: CellId = 100;
        let vertex_id: VertexId = 200;

        assert_eq!(shape_id, 42);
        assert_eq!(view_id, 1);
        assert_eq!(cell_id, 100);
        assert_eq!(vertex_id, 200);
    }
}
