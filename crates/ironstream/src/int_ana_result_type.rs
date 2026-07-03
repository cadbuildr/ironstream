// FILE: int_ana_result_type.rs
// occt: IntAna_ResultType

/// Type of result from intersection computation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntAnaResultType {
    /// A single point intersection.
    Point = 0,
    /// A line intersection.
    Line = 1,
    /// A circle intersection.
    Circle = 2,
    /// A point and a circle intersection.
    PointAndCircle = 3,
    /// An ellipse intersection.
    Ellipse = 4,
    /// A parabola intersection.
    Parabola = 5,
    /// A hyperbola intersection.
    Hyperbola = 6,
    /// No intersection (empty result).
    Empty = 7,
    /// Both objects are identical.
    Same = 8,
    /// No geometric solution.
    NoGeometricSolution = 9,
}

impl IntAnaResultType {
    /// Convert from i32 to enum.
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(IntAnaResultType::Point),
            1 => Some(IntAnaResultType::Line),
            2 => Some(IntAnaResultType::Circle),
            3 => Some(IntAnaResultType::PointAndCircle),
            4 => Some(IntAnaResultType::Ellipse),
            5 => Some(IntAnaResultType::Parabola),
            6 => Some(IntAnaResultType::Hyperbola),
            7 => Some(IntAnaResultType::Empty),
            8 => Some(IntAnaResultType::Same),
            9 => Some(IntAnaResultType::NoGeometricSolution),
            _ => None,
        }
    }

    /// Convert enum to i32.
    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_type_values() {
        assert_eq!(IntAnaResultType::Point.to_i32(), 0);
        assert_eq!(IntAnaResultType::Line.to_i32(), 1);
        assert_eq!(IntAnaResultType::Circle.to_i32(), 2);
        assert_eq!(IntAnaResultType::NoGeometricSolution.to_i32(), 9);
    }

    #[test]
    fn test_result_type_from_i32() {
        assert_eq!(IntAnaResultType::from_i32(0), Some(IntAnaResultType::Point));
        assert_eq!(IntAnaResultType::from_i32(5), Some(IntAnaResultType::Parabola));
        assert_eq!(IntAnaResultType::from_i32(10), None);
    }

    #[test]
    fn test_result_type_equality() {
        let rt1 = IntAnaResultType::Circle;
        let rt2 = IntAnaResultType::Circle;
        let rt3 = IntAnaResultType::Line;

        assert_eq!(rt1, rt2);
        assert_ne!(rt1, rt3);
    }
}
