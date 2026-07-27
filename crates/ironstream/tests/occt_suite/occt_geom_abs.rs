// FILE: tests/occt_geom_abs.rs
extern crate ironstream;
use ironstream::geom_abs::*;

#[test]
fn curve_type_all_nine_variants() {
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
    assert_eq!(variants.len(), 9);
    // All distinct
    for i in 0..variants.len() {
        for j in 0..variants.len() {
            if i == j {
                assert_eq!(variants[i], variants[j], "variant {} should equal itself", i);
            } else {
                assert_ne!(variants[i], variants[j], "variants {} and {} should differ", i, j);
            }
        }
    }
}

#[test]
fn surface_type_all_eleven_variants() {
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
    assert_eq!(variants.len(), 11);
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
fn shape_seven_levels_strict_order() {
    // OCCT defines 7 continuity levels in strictly increasing order
    assert!(Shape::C0 < Shape::G1);
    assert!(Shape::G1 < Shape::C1);
    assert!(Shape::C1 < Shape::G2);
    assert!(Shape::G2 < Shape::C2);
    assert!(Shape::C2 < Shape::C3);
    assert!(Shape::C3 < Shape::CN);
}

#[test]
fn shape_order_discriminants_strictly_increasing() {
    let orders = [
        Shape::C0.order(),
        Shape::G1.order(),
        Shape::C1.order(),
        Shape::G2.order(),
        Shape::C2.order(),
        Shape::C3.order(),
        Shape::CN.order(),
    ];
    for i in 1..orders.len() {
        assert!(
            orders[i] > orders[i - 1],
            "order[{}]={} should be > order[{}]={}",
            i,
            orders[i],
            i - 1,
            orders[i - 1]
        );
    }
    assert_eq!(orders[0], 0);
    assert_eq!(orders[6], 6);
}

#[test]
fn shape_c_continuity_implications() {
    // C3 implies C2 implies C1 implies C0
    assert!(Shape::C3.is_at_least_c3());
    assert!(Shape::C3.is_at_least_c2());
    assert!(Shape::C3.is_at_least_c1());
    assert!(Shape::C3.is_at_least_c0());

    // C2 implies C1, C0 but NOT C3
    assert!(!Shape::C2.is_at_least_c3());
    assert!(Shape::C2.is_at_least_c2());
    assert!(Shape::C2.is_at_least_c1());
    assert!(Shape::C2.is_at_least_c0());

    // C1 does not imply C2 or C3
    assert!(!Shape::C1.is_at_least_c3());
    assert!(!Shape::C1.is_at_least_c2());
    assert!(Shape::C1.is_at_least_c1());
    assert!(Shape::C1.is_at_least_c0());

    // C0 only implies C0
    assert!(!Shape::C0.is_at_least_c3());
    assert!(!Shape::C0.is_at_least_c2());
    assert!(!Shape::C0.is_at_least_c1());
    assert!(Shape::C0.is_at_least_c0());

    // CN (infinite) implies everything
    assert!(Shape::CN.is_at_least_c3());
    assert!(Shape::CN.is_at_least_c2());
    assert!(Shape::CN.is_at_least_c1());
    assert!(Shape::CN.is_at_least_c0());
}

#[test]
fn shape_geometric_continuity_g1_g2() {
    // G1 is geometric C1 — implies C0 but not C1 (parametric)
    assert!(Shape::G1.is_at_least_c0());
    assert!(!Shape::G1.is_at_least_c1());
    assert!(!Shape::G1.is_at_least_c2());

    // G2 is geometric C2 — implies C0 and C1 (parametric) but not C2
    assert!(Shape::G2.is_at_least_c0());
    assert!(Shape::G2.is_at_least_c1());
    assert!(!Shape::G2.is_at_least_c2());
}

#[test]
fn bspl_knot_distribution_four_variants() {
    let variants = [
        BSplKnotDistribution::NonUniform,
        BSplKnotDistribution::Uniform,
        BSplKnotDistribution::QuasiUniform,
        BSplKnotDistribution::PiecewiseBezier,
    ];
    assert_eq!(variants.len(), 4);
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
fn bspl_knot_uniform_classification() {
    assert!(!BSplKnotDistribution::NonUniform.is_uniform());
    assert!(BSplKnotDistribution::Uniform.is_uniform());
    assert!(BSplKnotDistribution::QuasiUniform.is_uniform());
    assert!(!BSplKnotDistribution::PiecewiseBezier.is_uniform());
}

#[test]
fn iso_type_three_variants() {
    assert_ne!(IsoType::IsoU, IsoType::IsoV);
    assert_ne!(IsoType::IsoU, IsoType::NoneIso);
    assert_ne!(IsoType::IsoV, IsoType::NoneIso);
}

#[test]
fn iso_type_is_iso_predicate() {
    assert!(IsoType::IsoU.is_iso());
    assert!(IsoType::IsoV.is_iso());
    assert!(!IsoType::NoneIso.is_iso());
}

#[test]
fn join_type_three_variants() {
    assert_ne!(JoinType::Arc, JoinType::Tangent);
    assert_ne!(JoinType::Arc, JoinType::Intersection);
    assert_ne!(JoinType::Tangent, JoinType::Intersection);
}

#[test]
fn all_enums_are_copy_clone() {
    // Verify Copy trait: assignment does not move
    let c = CurveType::Hyperbola;
    let c2 = c;
    assert_eq!(c, c2);

    let s = SurfaceType::Torus;
    let s2 = s;
    assert_eq!(s, s2);

    let sh = Shape::G2;
    let sh2 = sh;
    assert_eq!(sh, sh2);

    let k = BSplKnotDistribution::PiecewiseBezier;
    let k2 = k;
    assert_eq!(k, k2);

    let i = IsoType::IsoV;
    let i2 = i;
    assert_eq!(i, i2);

    let j = JoinType::Intersection;
    let j2 = j;
    assert_eq!(j, j2);
}

#[test]
fn all_enums_debug_format_non_empty() {
    let items: Vec<String> = vec![
        format!("{:?}", CurveType::Line),
        format!("{:?}", CurveType::BSplineCurve),
        format!("{:?}", SurfaceType::RevolutionSurface),
        format!("{:?}", SurfaceType::OtherSurface),
        format!("{:?}", Shape::C0),
        format!("{:?}", Shape::CN),
        format!("{:?}", BSplKnotDistribution::NonUniform),
        format!("{:?}", BSplKnotDistribution::QuasiUniform),
        format!("{:?}", IsoType::IsoU),
        format!("{:?}", IsoType::NoneIso),
        format!("{:?}", JoinType::Arc),
        format!("{:?}", JoinType::Tangent),
    ];
    for item in &items {
        assert!(!item.is_empty(), "Debug output should not be empty");
    }
    assert_eq!(items[0], "Line");
    assert_eq!(items[1], "BSplineCurve");
    assert_eq!(items[2], "RevolutionSurface");
    assert_eq!(items[4], "C0");
    assert_eq!(items[5], "CN");
}

#[test]
fn curve_type_analytic_vs_polynomial_partition() {
    let analytic = [
        CurveType::Line,
        CurveType::Circle,
        CurveType::Ellipse,
        CurveType::Hyperbola,
        CurveType::Parabola,
    ];
    let polynomial = [CurveType::BezierCurve, CurveType::BSplineCurve];
    let other = [CurveType::OffsetCurve, CurveType::OtherCurve];

    for ct in &analytic {
        assert!(ct.is_analytic(), "{:?} should be analytic", ct);
        assert!(!ct.is_polynomial(), "{:?} should not be polynomial", ct);
    }
    for ct in &polynomial {
        assert!(!ct.is_analytic(), "{:?} should not be analytic", ct);
        assert!(ct.is_polynomial(), "{:?} should be polynomial", ct);
    }
    for ct in &other {
        assert!(!ct.is_analytic(), "{:?} should not be analytic", ct);
        assert!(!ct.is_polynomial(), "{:?} should not be polynomial", ct);
    }
}

#[test]
fn surface_type_analytic_vs_polynomial_partition() {
    let analytic = [
        SurfaceType::Plane,
        SurfaceType::Cylinder,
        SurfaceType::Cone,
        SurfaceType::Sphere,
        SurfaceType::Torus,
    ];
    let polynomial = [SurfaceType::BezierSurface, SurfaceType::BSplineSurface];
    let other = [
        SurfaceType::RevolutionSurface,
        SurfaceType::ExtrusionSurface,
        SurfaceType::OffsetSurface,
        SurfaceType::OtherSurface,
    ];

    for st in &analytic {
        assert!(st.is_analytic(), "{:?} should be analytic", st);
        assert!(!st.is_polynomial(), "{:?} should not be polynomial", st);
    }
    for st in &polynomial {
        assert!(!st.is_analytic(), "{:?} should not be analytic", st);
        assert!(st.is_polynomial(), "{:?} should be polynomial", st);
    }
    for st in &other {
        assert!(!st.is_analytic(), "{:?} should not be analytic", st);
        assert!(!st.is_polynomial(), "{:?} should not be polynomial", st);
    }
}

#[test]
fn shape_cn_is_greatest() {
    let all = [
        Shape::C0,
        Shape::G1,
        Shape::C1,
        Shape::G2,
        Shape::C2,
        Shape::C3,
    ];
    for s in &all {
        assert!(*s < Shape::CN, "{:?} should be less than CN", s);
    }
}

#[test]
fn shape_c0_is_least() {
    let all = [
        Shape::G1,
        Shape::C1,
        Shape::G2,
        Shape::C2,
        Shape::C3,
        Shape::CN,
    ];
    for s in &all {
        assert!(Shape::C0 < *s, "C0 should be less than {:?}", s);
    }
}

#[test]
fn hash_consistency_curve_type() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(CurveType::Line);
    set.insert(CurveType::Circle);
    set.insert(CurveType::Line); // duplicate
    assert_eq!(set.len(), 2);
}

#[test]
fn hash_consistency_surface_type() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(SurfaceType::Plane);
    set.insert(SurfaceType::Sphere);
    set.insert(SurfaceType::Plane); // duplicate
    assert_eq!(set.len(), 2);
}

#[test]
fn hash_consistency_shape() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Shape::C0);
    set.insert(Shape::C1);
    set.insert(Shape::C0); // duplicate
    assert_eq!(set.len(), 2);
}

#[test]
fn hash_consistency_iso_type() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(IsoType::IsoU);
    set.insert(IsoType::IsoV);
    set.insert(IsoType::NoneIso);
    set.insert(IsoType::IsoU); // duplicate
    assert_eq!(set.len(), 3);
}

#[test]
fn hash_consistency_join_type() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(JoinType::Arc);
    set.insert(JoinType::Tangent);
    set.insert(JoinType::Intersection);
    set.insert(JoinType::Arc); // duplicate
    assert_eq!(set.len(), 3);
}
