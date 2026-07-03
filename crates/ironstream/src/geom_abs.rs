// FILE: geom_abs.rs

// occt: GeomAbs_CurveType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

// occt: GeomAbs_SurfaceType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

// occt: GeomAbs_Shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shape {
    C0,
    G1,
    C1,
    G2,
    C2,
    C3,
    CN,
}

// occt: GeomAbs_BSplKnotDistribution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSplKnotDistribution {
    NonUniform,
    Uniform,
    QuasiUniform,
    PiecewiseBezier,
}

// occt: GeomAbs_IsoType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IsoType {
    IsoU,
    IsoV,
    NoneIso,
}

// occt: GeomAbs_JoinType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoinType {
    Arc,
    Tangent,
    Intersection,
}

impl Shape {
    /// Returns true if this continuity implies at least C0 (position) continuity.
    pub fn is_at_least_c0(&self) -> bool {
        matches!(
            self,
            Shape::C0
                | Shape::G1
                | Shape::C1
                | Shape::G2
                | Shape::C2
                | Shape::C3
                | Shape::CN
        )
    }

    /// Returns true if this continuity implies at least C1 (first derivative) continuity.
    pub fn is_at_least_c1(&self) -> bool {
        matches!(
            self,
            Shape::C1 | Shape::G2 | Shape::C2 | Shape::C3 | Shape::CN
        )
    }

    /// Returns true if this continuity implies at least C2 (second derivative) continuity.
    pub fn is_at_least_c2(&self) -> bool {
        matches!(self, Shape::C2 | Shape::C3 | Shape::CN)
    }

    /// Returns true if this continuity implies at least C3 (third derivative) continuity.
    pub fn is_at_least_c3(&self) -> bool {
        matches!(self, Shape::C3 | Shape::CN)
    }

    /// Returns the geometric continuity order as an integer.
    /// C0=0, G1=1, C1=2, G2=3, C2=4, C3=5, CN=6
    pub fn order(&self) -> u32 {
        match self {
            Shape::C0 => 0,
            Shape::G1 => 1,
            Shape::C1 => 2,
            Shape::G2 => 3,
            Shape::C2 => 4,
            Shape::C3 => 5,
            Shape::CN => 6,
        }
    }
}

impl CurveType {
    /// Returns true if the curve type is an analytic (conic or line) curve.
    pub fn is_analytic(&self) -> bool {
        matches!(
            self,
            CurveType::Line
                | CurveType::Circle
                | CurveType::Ellipse
                | CurveType::Hyperbola
                | CurveType::Parabola
        )
    }

    /// Returns true if the curve type is defined by control points (Bezier or BSpline).
    pub fn is_polynomial(&self) -> bool {
        matches!(self, CurveType::BezierCurve | CurveType::BSplineCurve)
    }
}

impl SurfaceType {
    /// Returns true if the surface type is an analytic surface.
    pub fn is_analytic(&self) -> bool {
        matches!(
            self,
            SurfaceType::Plane
                | SurfaceType::Cylinder
                | SurfaceType::Cone
                | SurfaceType::Sphere
                | SurfaceType::Torus
        )
    }

    /// Returns true if the surface type is defined by control points (Bezier or BSpline).
    pub fn is_polynomial(&self) -> bool {
        matches!(
            self,
            SurfaceType::BezierSurface | SurfaceType::BSplineSurface
        )
    }
}

impl BSplKnotDistribution {
    /// Returns true if the knot distribution is uniform (equal spacing).
    pub fn is_uniform(&self) -> bool {
        matches!(
            self,
            BSplKnotDistribution::Uniform | BSplKnotDistribution::QuasiUniform
        )
    }
}

impl IsoType {
    /// Returns true if this is a parametric iso-curve (not NoneIso).
    pub fn is_iso(&self) -> bool {
        !matches!(self, IsoType::NoneIso)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_type_variants_are_distinct() {
        let variants = [
            CurveType::Line,
            CurveType::Circle,
            CurveType::Ellipse,
            CurveType::Hyperbola,
            CurveType::Parabola,
            CurveType::BezierCurve,
            CurveType::BSplineCurve,
            CurveType::OffsetCurve,
            CurveType::OtherCurve,
        ];
        for i in 0..variants.len() {
            for j in 0..variants.len() {
                if i == j {
                    assert_eq!(variants[i], variants[j]);
                } else {
                    assert_ne!(variants[i], variants[j]);
                }
            }
        }
    }

    #[test]
    fn surface_type_variants_are_distinct() {
        let variants = [
            SurfaceType::Plane,
            SurfaceType::Cylinder,
            SurfaceType::Cone,
            SurfaceType::Sphere,
            SurfaceType::Torus,
            SurfaceType::BezierSurface,
            SurfaceType::BSplineSurface,
            SurfaceType::RevolutionSurface,
            SurfaceType::ExtrusionSurface,
            SurfaceType::OffsetSurface,
            SurfaceType::OtherSurface,
        ];
        for i in 0..variants.len() {
            for j in 0..variants.len() {
                if i == j {
                    assert_eq!(variants[i], variants[j]);
                } else {
                    assert_ne!(variants[i], variants[j]);
                }
            }
        }
    }

    #[test]
    fn shape_ordering_is_correct() {
        assert!(Shape::C0 < Shape::G1);
        assert!(Shape::G1 < Shape::C1);
        assert!(Shape::C1 < Shape::G2);
        assert!(Shape::G2 < Shape::C2);
        assert!(Shape::C2 < Shape::C3);
        assert!(Shape::C3 < Shape::CN);
    }

    #[test]
    fn shape_order_values() {
        assert_eq!(Shape::C0.order(), 0);
        assert_eq!(Shape::G1.order(), 1);
        assert_eq!(Shape::C1.order(), 2);
        assert_eq!(Shape::G2.order(), 3);
        assert_eq!(Shape::C2.order(), 4);
        assert_eq!(Shape::C3.order(), 5);
        assert_eq!(Shape::CN.order(), 6);
    }

    #[test]
    fn shape_is_at_least_c0() {
        assert!(Shape::C0.is_at_least_c0());
        assert!(Shape::G1.is_at_least_c0());
        assert!(Shape::C1.is_at_least_c0());
        assert!(Shape::G2.is_at_least_c0());
        assert!(Shape::C2.is_at_least_c0());
        assert!(Shape::C3.is_at_least_c0());
        assert!(Shape::CN.is_at_least_c0());
    }

    #[test]
    fn shape_is_at_least_c1() {
        assert!(!Shape::C0.is_at_least_c1());
        assert!(!Shape::G1.is_at_least_c1());
        assert!(Shape::C1.is_at_least_c1());
        assert!(Shape::G2.is_at_least_c1());
        assert!(Shape::C2.is_at_least_c1());
        assert!(Shape::C3.is_at_least_c1());
        assert!(Shape::CN.is_at_least_c1());
    }

    #[test]
    fn shape_is_at_least_c2() {
        assert!(!Shape::C0.is_at_least_c2());
        assert!(!Shape::G1.is_at_least_c2());
        assert!(!Shape::C1.is_at_least_c2());
        assert!(!Shape::G2.is_at_least_c2());
        assert!(Shape::C2.is_at_least_c2());
        assert!(Shape::C3.is_at_least_c2());
        assert!(Shape::CN.is_at_least_c2());
    }

    #[test]
    fn shape_is_at_least_c3() {
        assert!(!Shape::C0.is_at_least_c3());
        assert!(!Shape::G1.is_at_least_c3());
        assert!(!Shape::C1.is_at_least_c3());
        assert!(!Shape::G2.is_at_least_c3());
        assert!(!Shape::C2.is_at_least_c3());
        assert!(Shape::C3.is_at_least_c3());
        assert!(Shape::CN.is_at_least_c3());
    }

    #[test]
    fn curve_type_is_analytic() {
        assert!(CurveType::Line.is_analytic());
        assert!(CurveType::Circle.is_analytic());
        assert!(CurveType::Ellipse.is_analytic());
        assert!(CurveType::Hyperbola.is_analytic());
        assert!(CurveType::Parabola.is_analytic());
        assert!(!CurveType::BezierCurve.is_analytic());
        assert!(!CurveType::BSplineCurve.is_analytic());
        assert!(!CurveType::OffsetCurve.is_analytic());
        assert!(!CurveType::OtherCurve.is_analytic());
    }

    #[test]
    fn curve_type_is_polynomial() {
        assert!(!CurveType::Line.is_polynomial());
        assert!(!CurveType::Circle.is_polynomial());
        assert!(!CurveType::Ellipse.is_polynomial());
        assert!(!CurveType::Hyperbola.is_polynomial());
        assert!(!CurveType::Parabola.is_polynomial());
        assert!(CurveType::BezierCurve.is_polynomial());
        assert!(CurveType::BSplineCurve.is_polynomial());
        assert!(!CurveType::OffsetCurve.is_polynomial());
        assert!(!CurveType::OtherCurve.is_polynomial());
    }

    #[test]
    fn surface_type_is_analytic() {
        assert!(SurfaceType::Plane.is_analytic());
        assert!(SurfaceType::Cylinder.is_analytic());
        assert!(SurfaceType::Cone.is_analytic());
        assert!(SurfaceType::Sphere.is_analytic());
        assert!(SurfaceType::Torus.is_analytic());
        assert!(!SurfaceType::BezierSurface.is_analytic());
        assert!(!SurfaceType::BSplineSurface.is_analytic());
        assert!(!SurfaceType::RevolutionSurface.is_analytic());
        assert!(!SurfaceType::ExtrusionSurface.is_analytic());
        assert!(!SurfaceType::OffsetSurface.is_analytic());
        assert!(!SurfaceType::OtherSurface.is_analytic());
    }

    #[test]
    fn surface_type_is_polynomial() {
        assert!(!SurfaceType::Plane.is_polynomial());
        assert!(!SurfaceType::Cylinder.is_polynomial());
        assert!(!SurfaceType::Cone.is_polynomial());
        assert!(!SurfaceType::Sphere.is_polynomial());
        assert!(!SurfaceType::Torus.is_polynomial());
        assert!(SurfaceType::BezierSurface.is_polynomial());
        assert!(SurfaceType::BSplineSurface.is_polynomial());
        assert!(!SurfaceType::RevolutionSurface.is_polynomial());
        assert!(!SurfaceType::ExtrusionSurface.is_polynomial());
        assert!(!SurfaceType::OffsetSurface.is_polynomial());
        assert!(!SurfaceType::OtherSurface.is_polynomial());
    }

    #[test]
    fn bspl_knot_distribution_variants_distinct() {
        assert_ne!(
            BSplKnotDistribution::NonUniform,
            BSplKnotDistribution::Uniform
        );
        assert_ne!(
            BSplKnotDistribution::Uniform,
            BSplKnotDistribution::QuasiUniform
        );
        assert_ne!(
            BSplKnotDistribution::QuasiUniform,
            BSplKnotDistribution::PiecewiseBezier
        );
        assert_ne!(
            BSplKnotDistribution::NonUniform,
            BSplKnotDistribution::PiecewiseBezier
        );
    }

    #[test]
    fn bspl_knot_distribution_is_uniform() {
        assert!(!BSplKnotDistribution::NonUniform.is_uniform());
        assert!(BSplKnotDistribution::Uniform.is_uniform());
        assert!(BSplKnotDistribution::QuasiUniform.is_uniform());
        assert!(!BSplKnotDistribution::PiecewiseBezier.is_uniform());
    }

    #[test]
    fn iso_type_variants_distinct() {
        assert_ne!(IsoType::IsoU, IsoType::IsoV);
        assert_ne!(IsoType::IsoU, IsoType::NoneIso);
        assert_ne!(IsoType::IsoV, IsoType::NoneIso);
    }

    #[test]
    fn iso_type_is_iso() {
        assert!(IsoType::IsoU.is_iso());
        assert!(IsoType::IsoV.is_iso());
        assert!(!IsoType::NoneIso.is_iso());
    }

    #[test]
    fn join_type_variants_distinct() {
        assert_ne!(JoinType::Arc, JoinType::Tangent);
        assert_ne!(JoinType::Arc, JoinType::Intersection);
        assert_ne!(JoinType::Tangent, JoinType::Intersection);
    }

    #[test]
    fn enums_are_copy() {
        let ct = CurveType::Circle;
        let ct2 = ct;
        assert_eq!(ct, ct2);

        let st = SurfaceType::Sphere;
        let st2 = st;
        assert_eq!(st, st2);

        let sh = Shape::C2;
        let sh2 = sh;
        assert_eq!(sh, sh2);

        let kd = BSplKnotDistribution::Uniform;
        let kd2 = kd;
        assert_eq!(kd, kd2);

        let it = IsoType::IsoU;
        let it2 = it;
        assert_eq!(it, it2);

        let jt = JoinType::Arc;
        let jt2 = jt;
        assert_eq!(jt, jt2);
    }

    #[test]
    fn enums_support_debug_format() {
        assert_eq!(format!("{:?}", CurveType::Line), "Line");
        assert_eq!(format!("{:?}", SurfaceType::Plane), "Plane");
        assert_eq!(format!("{:?}", Shape::C0), "C0");
        assert_eq!(format!("{:?}", Shape::CN), "CN");
        assert_eq!(format!("{:?}", BSplKnotDistribution::PiecewiseBezier), "PiecewiseBezier");
        assert_eq!(format!("{:?}", IsoType::NoneIso), "NoneIso");
        assert_eq!(format!("{:?}", JoinType::Intersection), "Intersection");
    }

    #[test]
    fn shape_equality_reflexive() {
        let all = [
            Shape::C0,
            Shape::G1,
            Shape::C1,
            Shape::G2,
            Shape::C2,
            Shape::C3,
            Shape::CN,
        ];
        for s in &all {
            assert_eq!(*s, *s);
        }
    }

    #[test]
    fn shape_min_max() {
        let shapes = [
            Shape::C0,
            Shape::G1,
            Shape::C1,
            Shape::G2,
            Shape::C2,
            Shape::C3,
            Shape::CN,
        ];
        let min = shapes.iter().copied().min().unwrap();
        let max = shapes.iter().copied().max().unwrap();
        assert_eq!(min, Shape::C0);
        assert_eq!(max, Shape::CN);
    }

    #[test]
    fn curve_type_clone_eq() {
        let original = CurveType::BSplineCurve;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn surface_type_clone_eq() {
        let original = SurfaceType::BSplineSurface;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn join_type_clone_eq() {
        let original = JoinType::Tangent;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn bspl_all_variants_covered() {
        let variants = [
            BSplKnotDistribution::NonUniform,
            BSplKnotDistribution::Uniform,
            BSplKnotDistribution::QuasiUniform,
            BSplKnotDistribution::PiecewiseBezier,
        ];
        assert_eq!(variants.len(), 4);
        for v in &variants {
            let _ = v.is_uniform();
        }
    }
}
