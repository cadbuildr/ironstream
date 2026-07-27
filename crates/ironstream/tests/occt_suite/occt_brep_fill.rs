// FILE: tests/occt_brep_fill.rs
//! Integration tests for `ironstream::brep_fill` — the BRepFill surface
//! filling port.
//!
//! Tests exercise the public API exclusively, covering:
//! - [`BRepFillFilling`]          — full build cycle (G0 and G1 constraints)
//! - [`BRepFillCurveConstraint`]  — construction, accessors, sample evaluation
//! - [`BRepFillEdgeFaceAndOrder`] — accessors and average-normal computation
//! - [`BRepFillSectionLaw`]       — section management and interpolation

extern crate ironstream;

use ironstream::brep_fill::{
    BRepFillCurveConstraint, BRepFillEdgeFaceAndOrder, BRepFillFilling, BRepFillSectionLaw,
    ContinuityOrder, FillStatus, SectionLawType,
};
use ironstream::geom_bspline_curve::GeomBSplineCurve;
use ironstream::gp::Pnt;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a degree-1 (linear) B-spline between two endpoints.
fn line_bspline(p0: Pnt, p1: Pnt) -> GeomBSplineCurve {
    let poles = vec![p0, p1];
    let knots = vec![0.0, 1.0];
    let mults = vec![2, 2];
    GeomBSplineCurve::new(&poles, &knots, &mults, 1, false)
}

/// Build a degree-3 (cubic) B-spline through four control points.
fn cubic_bspline(p0: Pnt, p1: Pnt, p2: Pnt, p3: Pnt) -> GeomBSplineCurve {
    let poles = vec![p0, p1, p2, p3];
    let knots = vec![0.0, 1.0];
    let mults = vec![4, 4];
    GeomBSplineCurve::new(&poles, &knots, &mults, 3, false)
}

// ---------------------------------------------------------------------------
// BRepFillCurveConstraint — integration tests
// ---------------------------------------------------------------------------

#[test]
fn curve_constraint_g0_construction_and_order() {
    let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(5.0, 0.0, 0.0));
    let cc = BRepFillCurveConstraint::new_g0(c);
    assert_eq!(cc.order(), ContinuityOrder::G0);
    assert!(cc.is_boundary());
    assert!(cc.normals().is_none());
}

#[test]
fn curve_constraint_g1_has_normals() {
    let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let normals = vec![Pnt::new(0.0, 0.0, 1.0); 5];
    let cc = BRepFillCurveConstraint::new_g1(c, normals);
    assert_eq!(cc.order(), ContinuityOrder::G1);
    let n = cc.normals().expect("normals should be present");
    assert_eq!(n.len(), 5);
    assert!((n[0].z - 1.0).abs() < 1.0e-15);
}

#[test]
fn curve_constraint_g2_nb_points() {
    let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0));
    let normals = vec![Pnt::new(0.0, 1.0, 0.0); 3];
    let cc = BRepFillCurveConstraint::new_g2(c, normals);
    assert_eq!(cc.order(), ContinuityOrder::G2);
    assert_eq!(cc.nb_points(), 8);
}

#[test]
fn curve_constraint_value_at_parameter_endpoints() {
    let p0 = Pnt::new(2.0, 3.0, 4.0);
    let p1 = Pnt::new(7.0, 8.0, 9.0);
    let c = line_bspline(p0, p1);
    let cc = BRepFillCurveConstraint::new_g0(c);
    assert!(cc.value(0.0).distance(p0) < 1.0e-9, "t=0 should equal p0");
    assert!(cc.value(1.0).distance(p1) < 1.0e-9, "t=1 should equal p1");
}

#[test]
fn curve_constraint_tol3d_default_and_setter() {
    let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let mut cc = BRepFillCurveConstraint::new_g0(c);
    let default_tol = cc.tol3d();
    assert!(default_tol > 0.0, "default tolerance should be positive");
    cc.set_tol3d(1.0e-7);
    assert!((cc.tol3d() - 1.0e-7).abs() < 1.0e-15);
}

// ---------------------------------------------------------------------------
// BRepFillEdgeFaceAndOrder — integration tests
// ---------------------------------------------------------------------------

#[test]
fn edge_face_and_order_g0_has_no_normals() {
    let edge = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let efa = BRepFillEdgeFaceAndOrder::new_g0(edge);
    assert_eq!(efa.order(), ContinuityOrder::G0);
    assert!(efa.face_normals().is_empty());
    assert!(efa.average_normal().is_none());
}

#[test]
fn edge_face_and_order_g1_average_normal_correct() {
    let edge = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(3.0, 0.0, 0.0));
    // All normals point in +Z.
    let normals = vec![Pnt::new(0.0, 0.0, 1.0); 4];
    let efa = BRepFillEdgeFaceAndOrder::new(edge, normals, ContinuityOrder::G1);
    let avg = efa.average_normal().expect("should have average normal");
    assert!((avg.x).abs() < 1.0e-9);
    assert!((avg.y).abs() < 1.0e-9);
    assert!((avg.z - 1.0).abs() < 1.0e-9);
}

#[test]
fn edge_face_and_order_edge_ref_accessible() {
    let p0 = Pnt::new(1.0, 2.0, 3.0);
    let p1 = Pnt::new(4.0, 5.0, 6.0);
    let edge = line_bspline(p0, p1);
    let efa = BRepFillEdgeFaceAndOrder::new_g0(edge);
    // Edge start/end should be recoverable.
    let v0 = efa.edge().value(efa.edge().first_parameter());
    let v1 = efa.edge().value(efa.edge().last_parameter());
    assert!(v0.distance(p0) < 1.0e-9);
    assert!(v1.distance(p1) < 1.0e-9);
}

// ---------------------------------------------------------------------------
// BRepFillSectionLaw — integration tests
// ---------------------------------------------------------------------------

#[test]
fn section_law_empty_is_constant() {
    let law = BRepFillSectionLaw::new(SectionLawType::NoContact);
    assert_eq!(law.nb_law(), 0);
    assert!(law.is_constant());
}

#[test]
fn section_law_two_identical_sections_is_constant() {
    let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let c1 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let mut law = BRepFillSectionLaw::new(SectionLawType::Contact);
    law.add_section(c0, 0.0);
    law.add_section(c1, 1.0);
    assert!(law.is_constant());
}

#[test]
fn section_law_two_different_sections_not_constant() {
    let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let c1 = line_bspline(Pnt::new(0.0, 0.0, 5.0), Pnt::new(1.0, 0.0, 5.0));
    let mut law = BRepFillSectionLaw::new(SectionLawType::ContactOnBorder);
    law.add_section(c0, 0.0);
    law.add_section(c1, 1.0);
    assert!(!law.is_constant());
}

#[test]
fn section_law_section_at_midpoint_interpolates_z() {
    let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let c1 = line_bspline(Pnt::new(0.0, 0.0, 4.0), Pnt::new(1.0, 0.0, 4.0));
    let mut law = BRepFillSectionLaw::new(SectionLawType::NoContact);
    law.add_section(c0, 0.0);
    law.add_section(c1, 1.0);
    let mid = law.section_at(0.5).expect("section_at should return Some");
    // Z should be halfway between 0 and 4 → 2.
    assert!((mid.z - 2.0).abs() < 1.0e-9, "Z should be ~2, got {}", mid.z);
}

#[test]
fn section_law_periodic_flag() {
    let mut law = BRepFillSectionLaw::new(SectionLawType::Contact);
    assert!(!law.is_periodic());
    law.set_periodic(true);
    assert!(law.is_periodic());
}

// ---------------------------------------------------------------------------
// BRepFillFilling — integration tests
// ---------------------------------------------------------------------------

#[test]
fn filling_default_status_not_built() {
    let filler = BRepFillFilling::default();
    assert_eq!(filler.status(), FillStatus::NotBuilt);
    assert!(!filler.is_done());
    assert_eq!(filler.nb_constraints(), 0);
}

#[test]
fn filling_too_few_boundaries_gives_failed_status() {
    let mut filler = BRepFillFilling::default();
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.build();
    assert_eq!(filler.status(), FillStatus::Failed);
}

#[test]
fn filling_four_g0_square_boundaries_produces_valid_surface() {
    let mut filler = BRepFillFilling::default();
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.build();
    assert!(filler.is_done(), "expected Done, got {:?}", filler.status());
    let surf = filler.surface();
    assert!(surf.nb_u_poles() >= 3);
    assert!(surf.nb_v_poles() >= 3);
}

#[test]
fn filling_g0_error_finite_after_build() {
    let mut filler = BRepFillFilling::default();
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.build();
    assert!(filler.is_done());
    assert!(filler.g0_error().is_finite());
    assert!(filler.g0_error() >= 0.0);
}

#[test]
fn filling_g1_constraint_shifts_inner_poles_in_z() {
    let mut filler = BRepFillFilling::default();
    // Bottom edge with upward G1 normal.
    let normals = vec![Pnt::new(0.0, 0.0, 1.0); 4];
    filler.add_constraint(BRepFillCurveConstraint::new_g1(
        line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)),
        normals,
    ));
    filler.add_edge(
        line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.build();
    assert!(filler.is_done());
    let surf = filler.surface();
    // The inner poles should have been nudged in Z by the G1 correction.
    let inner = surf.pole(2, 2);
    assert!(inner.z.abs() > 0.0, "G1 inner pole Z should be non-zero; got {:?}", inner);
}

#[test]
fn filling_three_boundary_triangular_succeeds() {
    let mut filler = BRepFillFilling::default();
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(2.0, 0.0, 0.0), Pnt::new(1.0, 2.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 2.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.build();
    assert!(filler.is_done(), "triangular fill should succeed");
}

#[test]
fn filling_point_constraint_is_registered() {
    let mut filler = BRepFillFilling::default();
    assert_eq!(filler.nb_point_constraints(), 0);
    filler.add_point_constraint(Pnt::new(0.5, 0.5, 0.1));
    assert_eq!(filler.nb_point_constraints(), 1);
    filler.add_point_constraint(Pnt::new(0.25, 0.75, 0.2));
    assert_eq!(filler.nb_point_constraints(), 2);
}

#[test]
fn filling_custom_params_respected() {
    let mut filler = BRepFillFilling::new(
        4,    // degree
        8,    // nb_pts_on_curve
        3,    // nb_iter
        false,
        1.0e-6,
        1.0e-5,
        0.001,
        0.05,
        10,
        5,
    );
    assert_eq!(filler.degree(), 4);
    // Build with four boundary curves.
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.build();
    assert!(filler.is_done());
}

#[test]
fn filling_curved_boundary_builds_correctly() {
    let mut filler = BRepFillFilling::default();
    // Bottom edge: curved (cubic) boundary.
    let curved = cubic_bspline(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.33, 0.0, 0.2),
        Pnt::new(0.66, 0.0, 0.2),
        Pnt::new(1.0, 0.0, 0.0),
    );
    filler.add_edge(curved, ContinuityOrder::G0, true);
    filler.add_edge(
        line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.add_edge(
        line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)),
        ContinuityOrder::G0,
        true,
    );
    filler.build();
    assert!(filler.is_done());
    let surf = filler.surface();
    // Result surface should span a plausible region.
    assert!(surf.nb_u_poles() >= 3);
    assert!(surf.nb_v_poles() >= 3);
}
