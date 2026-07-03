// FILE: int_res2d_intersection.rs
// occt: IntRes2d_Intersection

use crate::int_res2d_intersection_point::IntersectionPoint;
use crate::int_res2d_intersection_segment::IntersectionSegment;

/// Base class for 2D curve intersections
#[derive(Clone, Debug)]
pub struct Intersection {
    is_done: bool,
    points: Vec<IntersectionPoint>,
    segments: Vec<IntersectionSegment>,
}

impl Intersection {
    /// Create a new intersection
    pub fn new() -> Self {
        Intersection {
            is_done: false,
            points: Vec::new(),
            segments: Vec::new(),
        }
    }

    /// Check if computation was successful
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Set done flag
    pub fn set_done(&mut self, done: bool) {
        self.is_done = done;
    }

    /// Get number of intersection points
    pub fn nb_points(&self) -> i32 {
        self.points.len() as i32
    }

    /// Get number of intersection segments
    pub fn nb_segments(&self) -> i32 {
        self.segments.len() as i32
    }

    /// Get intersection point at index
    pub fn point(&self, index: i32) -> Option<IntersectionPoint> {
        if index > 0 && index <= self.nb_points() {
            Some(self.points[(index - 1) as usize])
        } else {
            None
        }
    }

    /// Get intersection segment at index
    pub fn segment(&self, index: i32) -> Option<IntersectionSegment> {
        if index > 0 && index <= self.nb_segments() {
            Some(self.segments[(index - 1) as usize])
        } else {
            None
        }
    }

    /// Add an intersection point
    pub fn add_point(&mut self, point: IntersectionPoint) {
        self.points.push(point);
    }

    /// Add an intersection segment
    pub fn add_segment(&mut self, segment: IntersectionSegment) {
        self.segments.push(segment);
    }
}

impl Default for Intersection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let inter = Intersection::new();
        assert!(!inter.is_done());
        assert_eq!(inter.nb_points(), 0);
        assert_eq!(inter.nb_segments(), 0);
    }

    #[test]
    fn test_set_done() {
        let mut inter = Intersection::new();
        inter.set_done(true);
        assert!(inter.is_done());
    }

    #[test]
    fn test_add_point() {
        let mut inter = Intersection::new();
        let t1 = crate::int_res2d_transition::Transition::new();
        let t2 = crate::int_res2d_transition::Transition::new();
        let p = IntersectionPoint::with_values(1.0, 2.0, 0.5, 0.6, t1, t2, false);

        inter.add_point(p);
        assert_eq!(inter.nb_points(), 1);
        assert!(inter.point(1).is_some());
    }

    #[test]
    fn test_add_segment() {
        let mut inter = Intersection::new();
        let seg = IntersectionSegment::infinite(false);

        inter.add_segment(seg);
        assert_eq!(inter.nb_segments(), 1);
        assert!(inter.segment(1).is_some());
    }
}
