// FILE: int_res2d_intersection_segment.rs
// occt: IntRes2d_IntersectionSegment

use crate::int_res2d_intersection_point::IntersectionPoint;

/// Represents an intersection segment between two 2D curves
#[derive(Clone, Debug, Copy)]
pub struct IntersectionSegment {
    first_point: Option<IntersectionPoint>,
    last_point: Option<IntersectionPoint>,
    is_opposite: bool,
    is_infinite: bool,
}

impl IntersectionSegment {
    /// Create an empty intersection segment
    pub fn new() -> Self {
        IntersectionSegment {
            first_point: None,
            last_point: None,
            is_opposite: false,
            is_infinite: true,
        }
    }

    /// Create segment with two points
    pub fn with_two_points(p1: IntersectionPoint, p2: IntersectionPoint, oppos: bool, reverse: bool) -> Self {
        let _ = reverse; // used in original but not stored
        IntersectionSegment {
            first_point: Some(p1),
            last_point: Some(p2),
            is_opposite: oppos,
            is_infinite: false,
        }
    }

    /// Create segment with one point (semi-infinite)
    pub fn with_one_point(p: IntersectionPoint, is_first: bool, oppos: bool, reverse: bool) -> Self {
        let _ = reverse; // used in original but not stored
        if is_first {
            IntersectionSegment {
                first_point: Some(p),
                last_point: None,
                is_opposite: oppos,
                is_infinite: false,
            }
        } else {
            IntersectionSegment {
                first_point: None,
                last_point: Some(p),
                is_opposite: oppos,
                is_infinite: false,
            }
        }
    }

    /// Create infinite segment
    pub fn infinite(oppos: bool) -> Self {
        IntersectionSegment {
            first_point: None,
            last_point: None,
            is_opposite: oppos,
            is_infinite: true,
        }
    }

    /// Check if opposite orientation
    pub fn is_opposite(&self) -> bool {
        self.is_opposite
    }

    /// Check if has first point
    pub fn has_first_point(&self) -> bool {
        self.first_point.is_some()
    }

    /// Check if has last point
    pub fn has_last_point(&self) -> bool {
        self.last_point.is_some()
    }

    /// Get first point if exists
    pub fn first_point(&self) -> Option<IntersectionPoint> {
        self.first_point
    }

    /// Get last point if exists
    pub fn last_point(&self) -> Option<IntersectionPoint> {
        self.last_point
    }

    /// Check if segment is infinite
    pub fn is_infinite(&self) -> bool {
        self.is_infinite
    }
}

impl Default for IntersectionSegment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::int_res2d_position::Position;

    #[test]
    fn test_new() {
        let seg = IntersectionSegment::new();
        assert!(!seg.has_first_point());
        assert!(!seg.has_last_point());
        assert!(seg.is_infinite());
    }

    #[test]
    fn test_infinite() {
        let seg = IntersectionSegment::infinite(true);
        assert!(seg.is_infinite());
        assert!(seg.is_opposite());
    }

    #[test]
    fn test_with_two_points() {
        let t1 = crate::int_res2d_transition::Transition::undecided(Position::Middle);
        let t2 = crate::int_res2d_transition::Transition::undecided(Position::Middle);
        let p1 = IntersectionPoint::with_values(0.0, 0.0, 0.5, 0.5, t1, t2, false);
        let p2 = IntersectionPoint::with_values(1.0, 1.0, 0.7, 0.7, t1, t2, false);

        let seg = IntersectionSegment::with_two_points(p1, p2, false, false);
        assert!(seg.has_first_point());
        assert!(seg.has_last_point());
        assert!(!seg.is_infinite());
    }

    #[test]
    fn test_with_one_point_first() {
        let t1 = crate::int_res2d_transition::Transition::undecided(Position::Middle);
        let t2 = crate::int_res2d_transition::Transition::undecided(Position::Middle);
        let p = IntersectionPoint::with_values(0.0, 0.0, 0.5, 0.5, t1, t2, false);

        let seg = IntersectionSegment::with_one_point(p, true, false, false);
        assert!(seg.has_first_point());
        assert!(!seg.has_last_point());
    }

    #[test]
    fn test_with_one_point_last() {
        let t1 = crate::int_res2d_transition::Transition::undecided(Position::Middle);
        let t2 = crate::int_res2d_transition::Transition::undecided(Position::Middle);
        let p = IntersectionPoint::with_values(1.0, 1.0, 0.7, 0.7, t1, t2, false);

        let seg = IntersectionSegment::with_one_point(p, false, true, false);
        assert!(!seg.has_first_point());
        assert!(seg.has_last_point());
        assert!(seg.is_opposite());
    }
}
