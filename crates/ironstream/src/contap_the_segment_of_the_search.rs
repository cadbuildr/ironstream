// FILE: contap_the_segment_of_the_search.rs
// occt: Contap_TheSegmentOfTheSearch

use crate::contap_the_path_point_of_the_search::ContapThePathPointOfTheSearch;

/// A segment of a search curve on a surface, bounded by optional limit points.
#[derive(Clone)]
pub struct ContapTheSegmentOfTheSearch {
    arc_index: Option<i32>,  // Arc/curve index
    has_first_point: bool,
    first_point: ContapThePathPointOfTheSearch,
    has_last_point: bool,
    last_point: ContapThePathPointOfTheSearch,
}

impl ContapTheSegmentOfTheSearch {
    /// Create an empty segment.
    pub fn new() -> Self {
        ContapTheSegmentOfTheSearch {
            arc_index: None,
            has_first_point: false,
            first_point: ContapThePathPointOfTheSearch::new(),
            has_last_point: false,
            last_point: ContapThePathPointOfTheSearch::new(),
        }
    }

    /// Set the arc (curve) associated with this segment.
    pub fn set_value(&mut self, arc_idx: i32) {
        self.arc_index = Some(arc_idx);
        self.has_first_point = false;
        self.has_last_point = false;
    }

    /// Set the first (lower parameter) limit point.
    pub fn set_limit_point_first(&mut self, point: ContapThePathPointOfTheSearch) {
        self.first_point = point;
        self.has_first_point = true;
    }

    /// Set the last (higher parameter) limit point.
    pub fn set_limit_point_last(&mut self, point: ContapThePathPointOfTheSearch) {
        self.last_point = point;
        self.has_last_point = true;
    }

    /// Get the arc index.
    pub fn curve(&self) -> Option<i32> {
        self.arc_index
    }

    /// Check if there is a first (lower) limit point.
    pub fn has_first_point(&self) -> bool {
        self.has_first_point
    }

    /// Get the first limit point.
    pub fn first_point(&self) -> Option<&ContapThePathPointOfTheSearch> {
        if self.has_first_point {
            Some(&self.first_point)
        } else {
            None
        }
    }

    /// Check if there is a last (upper) limit point.
    pub fn has_last_point(&self) -> bool {
        self.has_last_point
    }

    /// Get the last limit point.
    pub fn last_point(&self) -> Option<&ContapThePathPointOfTheSearch> {
        if self.has_last_point {
            Some(&self.last_point)
        } else {
            None
        }
    }
}

impl Default for ContapTheSegmentOfTheSearch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let seg = ContapTheSegmentOfTheSearch::new();
        assert!(!seg.has_first_point());
        assert!(!seg.has_last_point());
        assert_eq!(seg.curve(), None);
    }

    #[test]
    fn test_set_value() {
        let mut seg = ContapTheSegmentOfTheSearch::new();
        seg.set_value(5);
        assert_eq!(seg.curve(), Some(5));
    }

    #[test]
    fn test_set_limit_points() {
        let mut seg = ContapTheSegmentOfTheSearch::new();
        let pt1 = ContapThePathPointOfTheSearch::from_arc(0.0, 0.0, 0.0, 1e-6, 1, 0.0);
        let pt2 = ContapThePathPointOfTheSearch::from_arc(1.0, 1.0, 1.0, 1e-6, 1, 1.0);

        seg.set_limit_point_first(pt1);
        seg.set_limit_point_last(pt2);

        assert!(seg.has_first_point());
        assert!(seg.has_last_point());
        assert!(seg.first_point().is_some());
        assert!(seg.last_point().is_some());
    }

    #[test]
    fn test_default() {
        let seg = ContapTheSegmentOfTheSearch::default();
        assert!(!seg.has_first_point());
    }
}
