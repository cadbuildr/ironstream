// FILE: int_res2d_intersection_point.rs
// occt: IntRes2d_IntersectionPoint

use crate::int_res2d_transition::Transition;

/// Represents an intersection point between two 2D curves
#[derive(Clone, Debug, Copy)]
pub struct IntersectionPoint {
    point_x: f64,
    point_y: f64,
    param_c1: f64,
    param_c2: f64,
    trans1: Transition,
    trans2: Transition,
    reversed_flag: bool,
}

impl IntersectionPoint {
    /// Create an empty intersection point
    pub fn new() -> Self {
        IntersectionPoint {
            point_x: 0.0,
            point_y: 0.0,
            param_c1: 0.0,
            param_c2: 0.0,
            trans1: Transition::new(),
            trans2: Transition::new(),
            reversed_flag: false,
        }
    }

    /// Create intersection point with values
    pub fn with_values(
        px: f64,
        py: f64,
        uc1: f64,
        uc2: f64,
        trans1: Transition,
        trans2: Transition,
        reversed: bool,
    ) -> Self {
        IntersectionPoint {
            point_x: px,
            point_y: py,
            param_c1: uc1,
            param_c2: uc2,
            trans1,
            trans2,
            reversed_flag: reversed,
        }
    }

    /// Set values
    pub fn set_values(
        &mut self,
        px: f64,
        py: f64,
        uc1: f64,
        uc2: f64,
        trans1: Transition,
        trans2: Transition,
        reversed: bool,
    ) {
        self.point_x = px;
        self.point_y = py;
        self.param_c1 = uc1;
        self.param_c2 = uc2;
        self.trans1 = trans1;
        self.trans2 = trans2;
        self.reversed_flag = reversed;
    }

    /// Get intersection point (x, y)
    pub fn point(&self) -> (f64, f64) {
        (self.point_x, self.point_y)
    }

    /// Get parameter on curve 1
    pub fn param_on_first_curve(&self) -> f64 {
        self.param_c1
    }

    /// Get parameter on curve 2
    pub fn param_on_second_curve(&self) -> f64 {
        self.param_c2
    }

    /// Get transition on first curve
    pub fn transition_on_first_curve(&self) -> Transition {
        self.trans1
    }

    /// Get transition on second curve
    pub fn transition_on_second_curve(&self) -> Transition {
        self.trans2
    }

    /// Check if reversed flag is set
    pub fn is_reversed(&self) -> bool {
        self.reversed_flag
    }
}

impl Default for IntersectionPoint {
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
        let ip = IntersectionPoint::new();
        assert_eq!(ip.point(), (0.0, 0.0));
        assert_eq!(ip.param_on_first_curve(), 0.0);
        assert_eq!(ip.param_on_second_curve(), 0.0);
        assert!(!ip.is_reversed());
    }

    #[test]
    fn test_with_values() {
        let t1 = Transition::undecided(Position::Middle);
        let t2 = Transition::undecided(Position::Middle);
        let ip = IntersectionPoint::with_values(1.0, 2.0, 0.5, 0.6, t1, t2, false);

        assert_eq!(ip.point(), (1.0, 2.0));
        assert_eq!(ip.param_on_first_curve(), 0.5);
        assert_eq!(ip.param_on_second_curve(), 0.6);
    }

    #[test]
    fn test_set_values() {
        let mut ip = IntersectionPoint::new();
        let t1 = Transition::undecided(Position::Head);
        let t2 = Transition::undecided(Position::End);
        ip.set_values(3.0, 4.0, 0.7, 0.8, t1, t2, true);

        assert_eq!(ip.point(), (3.0, 4.0));
        assert_eq!(ip.param_on_first_curve(), 0.7);
        assert!(ip.is_reversed());
    }
}
