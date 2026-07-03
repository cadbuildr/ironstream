// FILE: gc_make_segment2d.rs
// occt: GC_MakeSegment2d

//! Construction algorithms for 2D line segments.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MakeStatus {
    Ok,
    InsufficientData,
}

/// A 2D line segment between two points.
#[derive(Clone, Debug)]
pub struct Segment2d {
    /// Start point (x, y)
    pub p1: (f64, f64),
    /// End point (x, y)
    pub p2: (f64, f64),
}

impl Segment2d {
    /// Create a segment.
    pub fn new(p1: (f64, f64), p2: (f64, f64)) -> Self {
        Segment2d { p1, p2 }
    }

    /// Get segment length.
    pub fn length(&self) -> f64 {
        let dx = self.p2.0 - self.p1.0;
        let dy = self.p2.1 - self.p1.1;
        (dx * dx + dy * dy).sqrt()
    }

    /// Get midpoint.
    pub fn midpoint(&self) -> (f64, f64) {
        ((self.p1.0 + self.p2.0) / 2.0, (self.p1.1 + self.p2.1) / 2.0)
    }

    /// Evaluate point at parameter t in [0, 1].
    pub fn eval(&self, t: f64) -> (f64, f64) {
        let t_clamped = t.clamp(0.0, 1.0);
        (
            self.p1.0 + t_clamped * (self.p2.0 - self.p1.0),
            self.p1.1 + t_clamped * (self.p2.1 - self.p1.1),
        )
    }

    /// Direction vector (normalized).
    pub fn direction(&self) -> (f64, f64) {
        let dx = self.p2.0 - self.p1.0;
        let dy = self.p2.1 - self.p1.1;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 1e-15 {
            (0.0, 0.0)
        } else {
            (dx / len, dy / len)
        }
    }
}

/// Builder for 2D line segments.
pub struct MakeSegment2d {
    segment: Option<Segment2d>,
    status: MakeStatus,
}

impl MakeSegment2d {
    /// Create from two points.
    pub fn new(p1: (f64, f64), p2: (f64, f64)) -> Self {
        if (p1.0 - p2.0).abs() < 1e-15 && (p1.1 - p2.1).abs() < 1e-15 {
            return MakeSegment2d {
                segment: None,
                status: MakeStatus::InsufficientData,
            };
        }

        MakeSegment2d {
            segment: Some(Segment2d::new(p1, p2)),
            status: MakeStatus::Ok,
        }
    }

    /// Get the constructed segment.
    pub fn value(&self) -> Option<&Segment2d> {
        self.segment.as_ref()
    }

    /// Get status.
    pub fn status(&self) -> MakeStatus {
        self.status
    }

    /// Check if construction succeeded.
    pub fn is_done(&self) -> bool {
        self.status == MakeStatus::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_create() {
        let seg = Segment2d::new((0.0, 0.0), (3.0, 4.0));
        assert!((seg.length() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_make_segment() {
        let maker = MakeSegment2d::new((0.0, 0.0), (1.0, 0.0));
        assert!(maker.is_done());
        assert!(maker.value().is_some());
    }

    #[test]
    fn test_same_point() {
        let maker = MakeSegment2d::new((1.0, 1.0), (1.0, 1.0));
        assert!(!maker.is_done());
    }

    #[test]
    fn test_segment_midpoint() {
        let seg = Segment2d::new((0.0, 0.0), (2.0, 4.0));
        let m = seg.midpoint();
        assert_eq!(m, (1.0, 2.0));
    }

    #[test]
    fn test_segment_eval() {
        let seg = Segment2d::new((0.0, 0.0), (10.0, 0.0));
        let p = seg.eval(0.5);
        assert_eq!(p, (5.0, 0.0));
    }

    #[test]
    fn test_segment_direction() {
        let seg = Segment2d::new((0.0, 0.0), (3.0, 4.0));
        let dir = seg.direction();
        assert!((dir.0 - 0.6).abs() < 1e-9);
        assert!((dir.1 - 0.8).abs() < 1e-9);
    }
}
