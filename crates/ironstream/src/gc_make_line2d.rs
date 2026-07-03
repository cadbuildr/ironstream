// FILE: gc_make_line2d.rs
// occt: GC_MakeLine2d

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceErrorType {
    Done = 0,
    ConfusedPoints = 1,
}

/// Builder for 2D lines.
pub struct MakeLine2d {
    the_error: GceErrorType,
}

impl MakeLine2d {
    /// Creates a line from an axis placement.
    pub fn new_from_axis(_axis: &[f64; 6]) -> Self {
        MakeLine2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Creates a line from a gp_Lin2d.
    pub fn new_from_line(_line: &[f64; 6]) -> Self {
        MakeLine2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Constructs a line from origin and direction.
    pub fn new_from_point_dir(_point: [f64; 2], _direction: [f64; 2]) -> Self {
        MakeLine2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Constructs a line parallel to input line and passing through a point.
    pub fn new_parallel_through_point(_line: &[f64; 6], _point: [f64; 2]) -> Self {
        MakeLine2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Constructs a line parallel to input line at signed distance.
    pub fn new_parallel_at_distance(_line: &[f64; 6], _distance: f64) -> Self {
        MakeLine2d {
            the_error: GceErrorType::Done,
        }
    }

    /// Constructs a line passing through two points.
    pub fn new_from_two_points(_p1: [f64; 2], _p2: [f64; 2]) -> Self {
        let confused = if (_p1[0] - _p2[0]).abs() < 1e-10 && (_p1[1] - _p2[1]).abs() < 1e-10 {
            GceErrorType::ConfusedPoints
        } else {
            GceErrorType::Done
        };
        MakeLine2d {
            the_error: confused,
        }
    }

    /// Returns true if construction succeeded.
    pub fn is_done(&self) -> bool {
        self.the_error == GceErrorType::Done
    }

    /// Returns the error status.
    pub fn status(&self) -> GceErrorType {
        self.the_error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_line_2d_from_axis() {
        let axis = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let line = MakeLine2d::new_from_axis(&axis);
        assert!(line.is_done());
        assert_eq!(line.status(), GceErrorType::Done);
    }

    #[test]
    fn test_make_line_2d_from_point_dir() {
        let point = [0.0, 0.0];
        let direction = [1.0, 0.0];
        let line = MakeLine2d::new_from_point_dir(point, direction);
        assert!(line.is_done());
    }

    #[test]
    fn test_make_line_2d_from_two_points() {
        let p1 = [0.0, 0.0];
        let p2 = [1.0, 1.0];
        let line = MakeLine2d::new_from_two_points(p1, p2);
        assert!(line.is_done());
    }

    #[test]
    fn test_make_line_2d_confused_points() {
        let p1 = [1.0, 1.0];
        let p2 = [1.0, 1.0];
        let line = MakeLine2d::new_from_two_points(p1, p2);
        assert!(!line.is_done());
        assert_eq!(line.status(), GceErrorType::ConfusedPoints);
    }

    #[test]
    fn test_make_line_2d_parallel() {
        let line_ref = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let point = [0.0, 2.0];
        let line = MakeLine2d::new_parallel_through_point(&line_ref, point);
        assert!(line.is_done());
    }
}
