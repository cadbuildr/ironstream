// FILE: top_ope_b_rep_type_line_curve.rs
// occt: TopOpeBRep_TypeLineCurve

/// Type of line/curve in topological operations.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeLineCurve {
    /// Analytic type
    Analytic = 0,
    /// Restriction type
    Restriction = 1,
    /// Walking type
    Walking = 2,
    /// Line type
    Line = 3,
    /// Circle type
    Circle = 4,
    /// Ellipse type
    Ellipse = 5,
    /// Parabola type
    Parabola = 6,
    /// Hyperbola type
    Hyperbola = 7,
    /// Other type
    OtherType = 8,
}

impl TypeLineCurve {
    /// Convert from raw u32 value.
    pub fn from_raw(value: u32) -> Option<Self> {
        match value {
            0 => Some(TypeLineCurve::Analytic),
            1 => Some(TypeLineCurve::Restriction),
            2 => Some(TypeLineCurve::Walking),
            3 => Some(TypeLineCurve::Line),
            4 => Some(TypeLineCurve::Circle),
            5 => Some(TypeLineCurve::Ellipse),
            6 => Some(TypeLineCurve::Parabola),
            7 => Some(TypeLineCurve::Hyperbola),
            8 => Some(TypeLineCurve::OtherType),
            _ => None,
        }
    }

    /// Convert to raw u32 value.
    pub fn to_raw(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_line_curve_values() {
        assert_eq!(TypeLineCurve::Analytic as u32, 0);
        assert_eq!(TypeLineCurve::Restriction as u32, 1);
        assert_eq!(TypeLineCurve::Walking as u32, 2);
        assert_eq!(TypeLineCurve::Line as u32, 3);
        assert_eq!(TypeLineCurve::Circle as u32, 4);
        assert_eq!(TypeLineCurve::Ellipse as u32, 5);
        assert_eq!(TypeLineCurve::Parabola as u32, 6);
        assert_eq!(TypeLineCurve::Hyperbola as u32, 7);
        assert_eq!(TypeLineCurve::OtherType as u32, 8);
    }

    #[test]
    fn test_from_raw() {
        assert_eq!(TypeLineCurve::from_raw(0), Some(TypeLineCurve::Analytic));
        assert_eq!(TypeLineCurve::from_raw(4), Some(TypeLineCurve::Circle));
        assert_eq!(TypeLineCurve::from_raw(8), Some(TypeLineCurve::OtherType));
        assert_eq!(TypeLineCurve::from_raw(9), None);
    }

    #[test]
    fn test_to_raw() {
        assert_eq!(TypeLineCurve::Line.to_raw(), 3);
        assert_eq!(TypeLineCurve::Circle.to_raw(), 4);
    }
}
