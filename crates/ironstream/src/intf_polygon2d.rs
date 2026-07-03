// FILE: intf_polygon2d.rs
// occt: Intf_Polygon2d

/// 2D polygon for interference analysis
#[derive(Clone, Debug)]
pub struct Polygon2d {
    vertices: Vec<(f64, f64)>,
    is_closed: bool,
}

impl Polygon2d {
    /// Create new 2D polygon
    pub fn new() -> Self {
        Polygon2d {
            vertices: Vec::new(),
            is_closed: false,
        }
    }

    /// Add vertex
    pub fn add_vertex(&mut self, x: f64, y: f64) {
        self.vertices.push((x, y));
    }

    /// Set closed flag
    pub fn set_closed(&mut self, closed: bool) {
        self.is_closed = closed;
    }

    /// Get number of vertices
    pub fn nb_vertices(&self) -> i32 {
        self.vertices.len() as i32
    }

    /// Get vertex at index
    pub fn vertex(&self, index: usize) -> Option<(f64, f64)> {
        self.vertices.get(index).copied()
    }

    /// Check if closed
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }
}

impl Default for Polygon2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let poly = Polygon2d::new();
        assert_eq!(poly.nb_vertices(), 0);
        assert!(!poly.is_closed());
    }

    #[test]
    fn test_add_vertex() {
        let mut poly = Polygon2d::new();
        poly.add_vertex(0.0, 0.0);
        poly.add_vertex(1.0, 0.0);
        poly.add_vertex(1.0, 1.0);
        assert_eq!(poly.nb_vertices(), 3);
    }

    #[test]
    fn test_closed() {
        let mut poly = Polygon2d::new();
        poly.set_closed(true);
        assert!(poly.is_closed());
    }
}
