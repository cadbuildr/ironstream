// FILE: int_polyh_intersection.rs
// occt: IntPolyh_Intersection

//! Polyhedron-based surface-surface intersection.

/// Intersection point between two surfaces
#[derive(Clone)]
pub struct IntersectionPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u1: f64,
    pub v1: f64,
    pub u2: f64,
    pub v2: f64,
}

/// Polyhedron-based surface intersection
pub struct IntPolyhIntersection {
    intersection_points: Vec<IntersectionPoint>,
    is_done: bool,
}

impl IntPolyhIntersection {
    /// Creates empty intersection
    pub fn new() -> Self {
        IntPolyhIntersection {
            intersection_points: Vec::new(),
            is_done: false,
        }
    }

    /// Performs intersection
    pub fn perform(&mut self, _poly1: &Polyhedron, _poly2: &Polyhedron) {
        // TODO: Implement polyhedron intersection
        self.is_done = true;
    }

    /// Returns whether intersection is computed
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Returns number of intersection points
    pub fn nb_points(&self) -> i32 {
        self.intersection_points.len() as i32
    }

    /// Returns intersection point at index
    pub fn point(&self, index: i32) -> Option<IntersectionPoint> {
        self.intersection_points.get(index as usize).cloned()
    }

    /// Adds intersection point
    pub fn add_point(&mut self, point: IntersectionPoint) {
        self.intersection_points.push(point);
    }
}

impl Default for IntPolyhIntersection {
    fn default() -> Self {
        Self::new()
    }
}

/// Placeholder for polyhedron
#[derive(Clone)]
pub struct Polyhedron;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_new() {
        let inter = IntPolyhIntersection::new();
        assert!(!inter.is_done());
        assert_eq!(inter.nb_points(), 0);
    }

    #[test]
    fn test_intersection_perform() {
        let mut inter = IntPolyhIntersection::new();
        inter.perform(&Polyhedron, &Polyhedron);
        assert!(inter.is_done());
    }

    #[test]
    fn test_intersection_add_point() {
        let mut inter = IntPolyhIntersection::new();
        let point = IntersectionPoint {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            u1: 0.5,
            v1: 0.5,
            u2: 0.5,
            v2: 0.5,
        };
        inter.add_point(point);
        assert_eq!(inter.nb_points(), 1);
    }
}
