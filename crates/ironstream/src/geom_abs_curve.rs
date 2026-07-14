// FILE: geom_abs_curve.rs
// occt-ref: GeomAbs_CurveType, GeomAbs_SurfaceType, GeomAbs_Shape, GeomAbs_JoinType

// occt-ref: GeomAbs_CurveType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CurveType {
    Line,
    Circle,
    Ellipse,
    Hyperbola,
    Parabola,
    BezierCurve,
    BSplineCurve,
    OffsetCurve,
    OtherCurve,
}

impl CurveType {
    pub fn is_conic(&self) -> bool {
        matches!(self, Self::Circle | Self::Ellipse | Self::Hyperbola | Self::Parabola)
    }

    pub fn is_bounded(&self) -> bool {
        matches!(self, Self::BezierCurve | Self::BSplineCurve)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Line => "Line",
            Self::Circle => "Circle",
            Self::Ellipse => "Ellipse",
            Self::Hyperbola => "Hyperbola",
            Self::Parabola => "Parabola",
            Self::BezierCurve => "BezierCurve",
            Self::BSplineCurve => "BSplineCurve",
            Self::OffsetCurve => "OffsetCurve",
            Self::OtherCurve => "OtherCurve",
        }
    }
}

// occt-ref: GeomAbs_SurfaceType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SurfaceType {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    BezierSurface,
    BSplineSurface,
    RevolutionSurface,
    ExtrusionSurface,
    OffsetSurface,
    OtherSurface,
}

impl SurfaceType {
    pub fn is_elementary(&self) -> bool {
        matches!(self, Self::Plane | Self::Cylinder | Self::Cone | Self::Sphere | Self::Torus)
    }

    pub fn is_swept(&self) -> bool {
        matches!(self, Self::RevolutionSurface | Self::ExtrusionSurface)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Plane => "Plane",
            Self::Cylinder => "Cylinder",
            Self::Cone => "Cone",
            Self::Sphere => "Sphere",
            Self::Torus => "Torus",
            Self::BezierSurface => "BezierSurface",
            Self::BSplineSurface => "BSplineSurface",
            Self::RevolutionSurface => "RevolutionSurface",
            Self::ExtrusionSurface => "ExtrusionSurface",
            Self::OffsetSurface => "OffsetSurface",
            Self::OtherSurface => "OtherSurface",
        }
    }
}

// occt-ref: GeomAbs_Shape // (continuity)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShapeContinuity {
    C0,  // geometric continuity (position)
    G1,  // tangential continuity
    C1,  // first derivative continuity
    G2,  // curvature continuity
    C2,  // second derivative continuity
    C3,  // third derivative continuity
    CN,  // n-th derivative continuity
}

impl ShapeContinuity {
    pub fn order(&self) -> i32 {
        match self { Self::C0 => 0, Self::G1 => 1, Self::C1 => 1,
                     Self::G2 => 2, Self::C2 => 2, Self::C3 => 3, Self::CN => 99 }
    }

    pub fn is_geometric(&self) -> bool {
        matches!(self, Self::G1 | Self::G2)
    }
}

// occt-ref: GeomAbs_JoinType // (fillet/chamfer join)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinType {
    Arc,
    Tangent,
    Intersection,
}

// occt-ref: GeomAbs_IsoType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsoType {
    IsoU,
    IsoV,
    NoneIso,
}

// occt-ref: GeomAbs_BSplKnotDistribution
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KnotDistribution {
    NonUniform,
    Uniform,
    QuasiUniform,
    PiecewiseBezier,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_type_conic() {
        assert!(CurveType::Circle.is_conic());
        assert!(!CurveType::Line.is_conic());
    }

    #[test]
    fn surface_type_elementary() {
        assert!(SurfaceType::Sphere.is_elementary());
        assert!(!SurfaceType::OffsetSurface.is_elementary());
    }

    #[test]
    fn shape_continuity_order() {
        assert_eq!(ShapeContinuity::C2.order(), 2);
        assert!(ShapeContinuity::C1 < ShapeContinuity::C2);
    }

    #[test]
    fn curve_type_name() {
        assert_eq!(CurveType::BSplineCurve.name(), "BSplineCurve");
    }

    #[test]
    fn surface_type_swept() {
        assert!(SurfaceType::RevolutionSurface.is_swept());
        assert!(!SurfaceType::Plane.is_swept());
    }
}
