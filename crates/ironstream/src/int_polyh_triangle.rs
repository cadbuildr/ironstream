// FILE: int_polyh_triangle.rs
// occt: IntPolyh_Triangle

/// Represents a triangle in the polyhedron, referencing three points and three edges.
#[derive(Clone, Debug, Copy)]
pub struct Triangle {
    points: [i32; 3],
    edges: [i32; 3],
    edges_orientations: [i32; 3],
    has_intersection: bool,
    is_intersection_possible: bool,
    is_degenerated: bool,
    deflection: f64,
}

impl Triangle {
    /// Create a default triangle
    pub fn new() -> Self {
        Triangle {
            points: [-1, -1, -1],
            edges: [-1, -1, -1],
            edges_orientations: [0, 0, 0],
            has_intersection: false,
            is_intersection_possible: true,
            is_degenerated: false,
            deflection: 0.0,
        }
    }

    /// Create a triangle with three points
    pub fn with_points(p1: i32, p2: i32, p3: i32) -> Self {
        Triangle {
            points: [p1, p2, p3],
            edges: [-1, -1, -1],
            edges_orientations: [0, 0, 0],
            has_intersection: false,
            is_intersection_possible: true,
            is_degenerated: false,
            deflection: 0.0,
        }
    }

    // Getters for points
    pub fn first_point(&self) -> i32 {
        self.points[0]
    }

    pub fn second_point(&self) -> i32 {
        self.points[1]
    }

    pub fn third_point(&self) -> i32 {
        self.points[2]
    }

    // Getters for edges
    pub fn first_edge(&self) -> i32 {
        self.edges[0]
    }

    pub fn first_edge_orientation(&self) -> i32 {
        self.edges_orientations[0]
    }

    pub fn second_edge(&self) -> i32 {
        self.edges[1]
    }

    pub fn second_edge_orientation(&self) -> i32 {
        self.edges_orientations[1]
    }

    pub fn third_edge(&self) -> i32 {
        self.edges[2]
    }

    pub fn third_edge_orientation(&self) -> i32 {
        self.edges_orientations[2]
    }

    // Getters for flags
    pub fn deflection(&self) -> f64 {
        self.deflection
    }

    pub fn is_intersection_possible(&self) -> bool {
        self.is_intersection_possible
    }

    pub fn has_intersection(&self) -> bool {
        self.has_intersection
    }

    pub fn is_degenerated(&self) -> bool {
        self.is_degenerated
    }

    // Setters for points
    pub fn set_first_point(&mut self, p: i32) {
        self.points[0] = p;
    }

    pub fn set_second_point(&mut self, p: i32) {
        self.points[1] = p;
    }

    pub fn set_third_point(&mut self, p: i32) {
        self.points[2] = p;
    }

    // Setters for edges
    pub fn set_first_edge(&mut self, edge: i32, orientation: i32) {
        self.edges[0] = edge;
        self.edges_orientations[0] = orientation;
    }

    pub fn set_second_edge(&mut self, edge: i32, orientation: i32) {
        self.edges[1] = edge;
        self.edges_orientations[1] = orientation;
    }

    pub fn set_third_edge(&mut self, edge: i32, orientation: i32) {
        self.edges[2] = edge;
        self.edges_orientations[2] = orientation;
    }

    // Setters for flags
    pub fn set_deflection(&mut self, deflection: f64) {
        self.deflection = deflection;
    }

    pub fn set_intersection_possible(&mut self, possible: bool) {
        self.is_intersection_possible = possible;
    }

    pub fn set_intersection(&mut self, has_int: bool) {
        self.has_intersection = has_int;
    }

    pub fn set_degenerated(&mut self, deg: bool) {
        self.is_degenerated = deg;
    }

    /// Get edge number by index (1-3)
    pub fn get_edge_number(&self, edge_index: i32) -> i32 {
        if edge_index >= 1 && edge_index <= 3 {
            self.edges[(edge_index - 1) as usize]
        } else {
            0
        }
    }

    /// Set edge by index (1-3)
    pub fn set_edge(&mut self, edge_index: i32, edge_number: i32) {
        if edge_index >= 1 && edge_index <= 3 {
            self.edges[(edge_index - 1) as usize] = edge_number;
        }
    }

    /// Get edge orientation by index (1-3)
    pub fn get_edge_orientation(&self, edge_index: i32) -> i32 {
        if edge_index >= 1 && edge_index <= 3 {
            self.edges_orientations[(edge_index - 1) as usize]
        } else {
            0
        }
    }

    /// Set edge orientation by index (1-3)
    pub fn set_edge_orientation(&mut self, edge_index: i32, orientation: i32) {
        if edge_index >= 1 && edge_index <= 3 {
            self.edges_orientations[(edge_index - 1) as usize] = orientation;
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tri = Triangle::new();
        assert_eq!(tri.first_point(), -1);
        assert_eq!(tri.second_point(), -1);
        assert_eq!(tri.third_point(), -1);
        assert_eq!(tri.first_edge(), -1);
        assert_eq!(tri.deflection(), 0.0);
        assert!(!tri.has_intersection());
        assert!(tri.is_intersection_possible());
        assert!(!tri.is_degenerated());
    }

    #[test]
    fn test_with_points() {
        let tri = Triangle::with_points(0, 1, 2);
        assert_eq!(tri.first_point(), 0);
        assert_eq!(tri.second_point(), 1);
        assert_eq!(tri.third_point(), 2);
    }

    #[test]
    fn test_edge_setters_getters() {
        let mut tri = Triangle::new();
        tri.set_first_edge(5, 1);
        assert_eq!(tri.first_edge(), 5);
        assert_eq!(tri.first_edge_orientation(), 1);

        tri.set_edge(2, 10);
        assert_eq!(tri.get_edge_number(2), 10);

        tri.set_edge_orientation(2, 2);
        assert_eq!(tri.get_edge_orientation(2), 2);
    }

    #[test]
    fn test_flags() {
        let mut tri = Triangle::new();
        tri.set_deflection(0.5);
        assert_eq!(tri.deflection(), 0.5);

        tri.set_intersection(true);
        assert!(tri.has_intersection());

        tri.set_degenerated(true);
        assert!(tri.is_degenerated());

        tri.set_intersection_possible(false);
        assert!(!tri.is_intersection_possible());
    }
}
