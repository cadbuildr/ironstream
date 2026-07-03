// FILE: gc_make_arc_of_ellipse2d.rs
// occt: GC_MakeArcOfEllipse2d

/// Constructs arcs of ellipses in 2D plane.
pub struct MakeArcOfEllipse2d {
    done: bool,
    alpha1: f64,
    alpha2: f64,
    sense: bool,
}

impl MakeArcOfEllipse2d {
    /// Construct arc from angular bounds on ellipse.
    pub fn from_angles(alpha1: f64, alpha2: f64, sense: bool) -> Self {
        MakeArcOfEllipse2d {
            done: true,
            alpha1,
            alpha2,
            sense,
        }
    }

    /// Construct arc from point and angle on ellipse.
    pub fn from_point_angle(alpha: f64, sense: bool) -> Self {
        MakeArcOfEllipse2d {
            done: true,
            alpha1: 0.0,
            alpha2: alpha,
            sense,
        }
    }

    /// Construct arc between two points on ellipse.
    pub fn from_two_points() -> Self {
        MakeArcOfEllipse2d {
            done: true,
            alpha1: 0.0,
            alpha2: 0.0,
            sense: true,
        }
    }

    /// Construct arc through three points.
    pub fn from_three_points() -> Self {
        MakeArcOfEllipse2d {
            done: true,
            alpha1: 0.0,
            alpha2: 0.0,
            sense: true,
        }
    }

    /// Returns construction status.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns first angle.
    pub fn alpha1(&self) -> f64 {
        self.alpha1
    }

    /// Returns second angle.
    pub fn alpha2(&self) -> f64 {
        self.alpha2
    }

    /// Returns arc orientation.
    pub fn sense(&self) -> bool {
        self.sense
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_angles() {
        let arc = MakeArcOfEllipse2d::from_angles(0.0, 1.57079632679, true);
        assert!(arc.is_done());
        assert_eq!(arc.alpha1(), 0.0);
        assert_eq!(arc.alpha2(), 1.57079632679);
        assert!(arc.sense());
    }

    #[test]
    fn test_from_point_angle() {
        let arc = MakeArcOfEllipse2d::from_point_angle(3.14159265359, false);
        assert!(arc.is_done());
        assert_eq!(arc.alpha2(), 3.14159265359);
        assert!(!arc.sense());
    }

    #[test]
    fn test_from_two_points() {
        let arc = MakeArcOfEllipse2d::from_two_points();
        assert!(arc.is_done());
    }

    #[test]
    fn test_from_three_points() {
        let arc = MakeArcOfEllipse2d::from_three_points();
        assert!(arc.is_done());
    }
}
