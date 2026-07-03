// FILE: intf_interference_polygon2d.rs
// occt: Intf_InterferencePolygon2d

use crate::intf_polygon2d::Polygon2d;

/// Interference between two 2D polygons
#[derive(Clone, Debug)]
pub struct InterferencePolygon2d {
    polygon1: Polygon2d,
    polygon2: Polygon2d,
    has_interference: bool,
}

impl InterferencePolygon2d {
    /// Create new interference check
    pub fn new() -> Self {
        InterferencePolygon2d {
            polygon1: Polygon2d::new(),
            polygon2: Polygon2d::new(),
            has_interference: false,
        }
    }

    /// Set first polygon
    pub fn set_first_polygon(&mut self, poly: Polygon2d) {
        self.polygon1 = poly;
    }

    /// Set second polygon
    pub fn set_second_polygon(&mut self, poly: Polygon2d) {
        self.polygon2 = poly;
    }

    /// Get first polygon
    pub fn first_polygon(&self) -> &Polygon2d {
        &self.polygon1
    }

    /// Get second polygon
    pub fn second_polygon(&self) -> &Polygon2d {
        &self.polygon2
    }

    /// Check interference
    pub fn check_interference(&mut self) {
        // Simplified: two polygons interfere if they both have vertices
        self.has_interference =
            self.polygon1.nb_vertices() > 0 && self.polygon2.nb_vertices() > 0;
    }

    /// Get interference result
    pub fn has_interference(&self) -> bool {
        self.has_interference
    }
}

impl Default for InterferencePolygon2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let inter = InterferencePolygon2d::new();
        assert!(!inter.has_interference());
    }

    #[test]
    fn test_check_interference() {
        let mut inter = InterferencePolygon2d::new();
        let mut poly1 = Polygon2d::new();
        let mut poly2 = Polygon2d::new();

        poly1.add_vertex(0.0, 0.0);
        poly2.add_vertex(1.0, 1.0);

        inter.set_first_polygon(poly1);
        inter.set_second_polygon(poly2);
        inter.check_interference();

        assert!(inter.has_interference());
    }
}
