// FILE: top_ope_b_rep_point2d.rs
// occt: TopOpeBRep_Point2d

use crate::top_ope_b_rep_p2_dstatus::P2Dstatus;

/// 2D point in topological operations.
pub struct Point2d {
    pint: Option<(f64, f64)>,
    is_vertex1: bool,
    is_vertex2: bool,
    parameter1: f64,
    parameter2: f64,
    is_point_of_segment: bool,
    segment_ancestors: Option<(usize, usize)>,
    status: P2Dstatus,
    index: usize,
    pnt: (f64, f64, f64),
    pnt2d: (f64, f64),
    keep: bool,
    tolerance: f64,
}

impl Point2d {
    /// Create a new Point2d.
    pub fn new() -> Self {
        Point2d {
            pint: None,
            is_vertex1: false,
            is_vertex2: false,
            parameter1: 0.0,
            parameter2: 0.0,
            is_point_of_segment: false,
            segment_ancestors: None,
            status: P2Dstatus::Unknown,
            index: 0,
            pnt: (0.0, 0.0, 0.0),
            pnt2d: (0.0, 0.0),
            keep: false,
            tolerance: 0.0,
        }
    }

    /// Set the 2D intersection point.
    pub fn set_pint(&mut self, x: f64, y: f64) {
        self.pint = Some((x, y));
    }

    /// Check if this point has a pint.
    pub fn has_pint(&self) -> bool {
        self.pint.is_some()
    }

    /// Get the 2D intersection point.
    pub fn pint(&self) -> Option<(f64, f64)> {
        self.pint
    }

    /// Set if this is a vertex.
    pub fn set_is_vertex(&mut self, i: usize, b: bool) {
        match i {
            0 => self.is_vertex1 = b,
            1 => self.is_vertex2 = b,
            _ => {}
        }
    }

    /// Check if this is a vertex.
    pub fn is_vertex(&self, i: usize) -> bool {
        match i {
            0 => self.is_vertex1,
            1 => self.is_vertex2,
            _ => false,
        }
    }

    /// Set parameter.
    pub fn set_parameter(&mut self, i: usize, p: f64) {
        match i {
            0 => self.parameter1 = p,
            1 => self.parameter2 = p,
            _ => {}
        }
    }

    /// Get parameter.
    pub fn parameter(&self, i: usize) -> f64 {
        match i {
            0 => self.parameter1,
            1 => self.parameter2,
            _ => 0.0,
        }
    }

    /// Set if this is a point of segment.
    pub fn set_is_point_of_segment(&mut self, b: bool) {
        self.is_point_of_segment = b;
    }

    /// Check if this is a point of segment.
    pub fn is_point_of_segment(&self) -> bool {
        self.is_point_of_segment
    }

    /// Set segment ancestors.
    pub fn set_segment_ancestors(&mut self, ip1: usize, ip2: usize) {
        self.segment_ancestors = Some((ip1, ip2));
    }

    /// Get segment ancestors.
    pub fn segment_ancestors(&self) -> Option<(usize, usize)> {
        self.segment_ancestors
    }

    /// Set status.
    pub fn set_status(&mut self, s: P2Dstatus) {
        self.status = s;
    }

    /// Get status.
    pub fn status(&self) -> P2Dstatus {
        self.status
    }

    /// Set index.
    pub fn set_index(&mut self, x: usize) {
        self.index = x;
    }

    /// Get index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Set 3D point value.
    pub fn set_value(&mut self, x: f64, y: f64, z: f64) {
        self.pnt = (x, y, z);
    }

    /// Get 3D point value.
    pub fn value(&self) -> (f64, f64, f64) {
        self.pnt
    }

    /// Set 2D point value.
    pub fn set_value2d(&mut self, u: f64, v: f64) {
        self.pnt2d = (u, v);
    }

    /// Get 2D point value.
    pub fn value2d(&self) -> (f64, f64) {
        self.pnt2d
    }

    /// Set keep flag.
    pub fn set_keep(&mut self, b: bool) {
        self.keep = b;
    }

    /// Get keep flag.
    pub fn keep(&self) -> bool {
        self.keep
    }

    /// Set tolerance.
    pub fn set_tolerance(&mut self, t: f64) {
        self.tolerance = t;
    }

    /// Get tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

impl Default for Point2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pt = Point2d::new();
        assert!(!pt.has_pint());
        assert!(!pt.keep());
    }

    #[test]
    fn test_set_pint() {
        let mut pt = Point2d::new();
        pt.set_pint(1.0, 2.0);
        assert!(pt.has_pint());
        assert_eq!(pt.pint(), Some((1.0, 2.0)));
    }

    #[test]
    fn test_is_vertex() {
        let mut pt = Point2d::new();
        pt.set_is_vertex(0, true);
        assert!(pt.is_vertex(0));
        assert!(!pt.is_vertex(1));
    }

    #[test]
    fn test_parameters() {
        let mut pt = Point2d::new();
        pt.set_parameter(0, 1.5);
        pt.set_parameter(1, 2.5);
        assert_eq!(pt.parameter(0), 1.5);
        assert_eq!(pt.parameter(1), 2.5);
    }
}
