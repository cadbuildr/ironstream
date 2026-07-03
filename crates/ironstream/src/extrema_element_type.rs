// FILE: extrema_element_type.rs
// occt: Extrema_ElementType

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtremaElementType {
    Point = 0,
    Line = 1,
    Circle = 2,
    Ellipse = 3,
    Hyperbola = 4,
    Parabola = 5,
    BezierCurve = 6,
    BSplineCurve = 7,
    OtherCurve = 8,
    Plane = 9,
    Sphere = 10,
    Cylinder = 11,
    Cone = 12,
    Torus = 13,
    BezierSurface = 14,
    BSplineSurface = 15,
    SurfaceOfRevolution = 16,
    OffsetSurface = 17,
    OtherSurface = 18,
}

impl ExtremaElementType {
    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(ExtremaElementType::Point),
            1 => Some(ExtremaElementType::Line),
            2 => Some(ExtremaElementType::Circle),
            3 => Some(ExtremaElementType::Ellipse),
            4 => Some(ExtremaElementType::Hyperbola),
            5 => Some(ExtremaElementType::Parabola),
            6 => Some(ExtremaElementType::BezierCurve),
            7 => Some(ExtremaElementType::BSplineCurve),
            8 => Some(ExtremaElementType::OtherCurve),
            9 => Some(ExtremaElementType::Plane),
            10 => Some(ExtremaElementType::Sphere),
            11 => Some(ExtremaElementType::Cylinder),
            12 => Some(ExtremaElementType::Cone),
            13 => Some(ExtremaElementType::Torus),
            14 => Some(ExtremaElementType::BezierSurface),
            15 => Some(ExtremaElementType::BSplineSurface),
            16 => Some(ExtremaElementType::SurfaceOfRevolution),
            17 => Some(ExtremaElementType::OffsetSurface),
            18 => Some(ExtremaElementType::OtherSurface),
            _ => None,
        }
    }
    pub fn to_i32(self) -> i32 { self as i32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_values() {
        assert_eq!(ExtremaElementType::Point.to_i32(), 0);
        assert_eq!(ExtremaElementType::Line.to_i32(), 1);
        assert_eq!(ExtremaElementType::BSplineSurface.to_i32(), 15);
    }
    #[test]
    fn test_round_trip() {
        for i in 0..19 {
            if let Some(et) = ExtremaElementType::from_i32(i) {
                assert_eq!(et.to_i32(), i);
            }
        }
    }
}
