// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_approx.rs
//! Integration tests for `ironstream::geom_approx` — stub B-spline
//! approximation types mirroring OpenCascade's `GeomConvert_ApproxCurve` and
//! `GeomConvert_ApproxSurface`.

extern crate ironstream;
use ironstream::geom_approx::*;

// ─────────────────────────── GeomApproxCurve ─────────────────────────────────

#[test]
fn curve_new_not_done() {
    let c = GeomApproxCurve::new();
    assert!(!c.is_done(), "freshly constructed curve must not be done");
    assert_eq!(c.nb_poles(), 0);
    assert_eq!(c.degree(), 0);
}

#[test]
fn curve_default_not_done() {
    let c: GeomApproxCurve = Default::default();
    assert!(!c.is_done());
}

#[test]
fn curve_perform_marks_done() {
    let mut c = GeomApproxCurve::new();
    c.perform(5, 3, 1e-4);
    assert!(c.is_done(), "perform must set is_done = true");
}

#[test]
fn curve_perform_stores_degree() {
    let mut c = GeomApproxCurve::new();
    c.perform(4, 2, 0.01);
    assert_eq!(c.degree(), 2);
}

#[test]
fn curve_perform_stores_nb_poles() {
    let mut c = GeomApproxCurve::new();
    c.perform(7, 3, 1e-6);
    assert_eq!(c.nb_poles(), 7);
}

#[test]
fn curve_max_error_is_tolerance_times_tenth() {
    let tol = 1e-3;
    let mut c = GeomApproxCurve::new();
    c.perform(5, 3, tol);
    let expected = tol * 0.1;
    assert!(
        (c.max_error() - expected).abs() < 1e-18,
        "max_error should be tolerance*0.1, got {}",
        c.max_error()
    );
}

#[test]
fn curve_poles_are_equidistant_along_x() {
    let nb = 5;
    let mut c = GeomApproxCurve::new();
    c.perform(nb, 3, 1e-6);
    for i in 0..nb {
        let expected_x = i as f64 / (nb - 1) as f64;
        let pole = c.pole(i);
        assert!(
            (pole[0] - expected_x).abs() < 1e-14,
            "pole[{i}].x = {} expected {}",
            pole[0],
            expected_x
        );
        assert_eq!(pole[1], 0.0, "pole[{i}].y must be 0");
        assert_eq!(pole[2], 0.0, "pole[{i}].z must be 0");
    }
}

#[test]
fn curve_single_pole_at_origin() {
    let mut c = GeomApproxCurve::new();
    c.perform(1, 0, 0.5);
    assert!(c.is_done());
    assert_eq!(c.pole(0), [0.0, 0.0, 0.0]);
}

#[test]
fn curve_two_poles() {
    let mut c = GeomApproxCurve::new();
    c.perform(2, 1, 1e-5);
    assert_eq!(c.nb_poles(), 2);
    assert!((c.pole(0)[0] - 0.0).abs() < 1e-14);
    assert!((c.pole(1)[0] - 1.0).abs() < 1e-14);
}

#[test]
fn curve_knot_vector_starts_at_zero_ends_at_one() {
    let mut c = GeomApproxCurve::new();
    c.perform(5, 3, 1e-6);
    assert_eq!(c.knot(0), 0.0, "first knot must be 0");
    let last_idx = c.knots.len() - 1;
    assert_eq!(c.knot(last_idx), 1.0, "last knot must be 1");
}

#[test]
fn curve_knot_multiplicity_clamped_no_interior() {
    // degree 3, 4 poles → no interior knots
    let mut c = GeomApproxCurve::new();
    c.perform(4, 3, 1e-6);
    assert_eq!(c.knots.len(), 2, "should have exactly start + end knot");
    assert_eq!(c.mults[0], 4, "start mult = degree + 1");
    assert_eq!(c.mults[1], 4, "end mult = degree + 1");
}

#[test]
fn curve_knot_interior_multiplicity_one() {
    // degree 2, 6 poles → n_interior = 6 - 3 = 3
    let mut c = GeomApproxCurve::new();
    c.perform(6, 2, 1e-6);
    // knots: [0, k1, k2, k3, 1]  (3 interior)
    assert_eq!(c.knots.len(), 5);
    for &m in &c.mults[1..c.mults.len() - 1] {
        assert_eq!(m, 1, "interior knot mult must be 1");
    }
}

#[test]
fn curve_knot_method_matches_vec() {
    let mut c = GeomApproxCurve::new();
    c.perform(5, 3, 1e-5);
    for (i, &kv) in c.knots.iter().enumerate() {
        assert_eq!(c.knot(i), kv);
    }
}

// ─────────────────────────── GeomApproxSurface ───────────────────────────────

#[test]
fn surface_new_not_done() {
    let s = GeomApproxSurface::new();
    assert!(!s.is_done(), "freshly constructed surface must not be done");
    assert_eq!(s.nb_u_poles(), 0);
    assert_eq!(s.nb_v_poles(), 0);
    assert_eq!(s.u_degree(), 0);
    assert_eq!(s.v_degree(), 0);
}

#[test]
fn surface_default_not_done() {
    let s: GeomApproxSurface = Default::default();
    assert!(!s.is_done());
}

#[test]
fn surface_perform_marks_done() {
    let mut s = GeomApproxSurface::new();
    s.perform(4, 5, 3, 2, 1e-4);
    assert!(s.is_done());
}

#[test]
fn surface_perform_stores_nb_u_poles() {
    let mut s = GeomApproxSurface::new();
    s.perform(6, 8, 3, 2, 1e-4);
    assert_eq!(s.nb_u_poles(), 6);
}

#[test]
fn surface_perform_stores_nb_v_poles() {
    let mut s = GeomApproxSurface::new();
    s.perform(6, 8, 3, 2, 1e-4);
    assert_eq!(s.nb_v_poles(), 8);
}

#[test]
fn surface_perform_stores_u_degree() {
    let mut s = GeomApproxSurface::new();
    s.perform(5, 5, 3, 2, 1e-4);
    assert_eq!(s.u_degree(), 3);
}

#[test]
fn surface_perform_stores_v_degree() {
    let mut s = GeomApproxSurface::new();
    s.perform(5, 5, 3, 2, 1e-4);
    assert_eq!(s.v_degree(), 2);
}

#[test]
fn surface_max_error_is_tolerance_times_tenth() {
    let tol = 2e-5;
    let mut s = GeomApproxSurface::new();
    s.perform(4, 4, 2, 2, tol);
    let expected = tol * 0.1;
    assert!(
        (s.max_error() - expected).abs() < 1e-20,
        "max_error should be tolerance*0.1, got {}",
        s.max_error()
    );
}

#[test]
fn surface_perform_overwrites_previous_call() {
    let mut s = GeomApproxSurface::new();
    s.perform(4, 4, 2, 2, 1e-3);
    s.perform(10, 12, 3, 4, 5e-6);
    assert_eq!(s.nb_u_poles(), 10);
    assert_eq!(s.nb_v_poles(), 12);
    assert_eq!(s.u_degree(), 3);
    assert_eq!(s.v_degree(), 4);
    assert!((s.max_error() - 5e-7).abs() < 1e-20);
}

#[test]
fn surface_zero_tolerance_gives_zero_error() {
    let mut s = GeomApproxSurface::new();
    s.perform(4, 4, 2, 2, 0.0);
    assert_eq!(s.max_error(), 0.0);
}
