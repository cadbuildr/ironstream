// FILE: tests/occt_geom_bezier_surface.rs
extern crate ironstream;
use ironstream::geom_bezier_surface::GeomBezierSurface;

// ---------------------------------------------------------------------------
// Integration tests for GeomBezierSurface
// These mirror what OCCT's Geom_BezierSurface tests cover.
// ---------------------------------------------------------------------------

/// Build a flat bilinear patch in the XY plane.
///
/// Poles (u=row, v=col):
///   (0,0,0) (1,0,0)
///   (0,1,0) (1,1,0)
fn make_bilinear() -> GeomBezierSurface {
    GeomBezierSurface::from_owned(
        vec![
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
        ],
        None,
    )
}

#[test]
fn test_degrees() {
    let s = make_bilinear();
    assert_eq!(s.u_degree(), 1);
    assert_eq!(s.v_degree(), 1);
    assert_eq!(s.nb_u_poles(), 2);
    assert_eq!(s.nb_v_poles(), 2);
}

#[test]
fn test_is_not_periodic() {
    let s = make_bilinear();
    assert!(!s.is_u_periodic());
    assert!(!s.is_v_periodic());
}

#[test]
fn test_non_rational_weights() {
    let s = make_bilinear();
    assert!(!s.is_rational());
    for i in 1..=2 {
        for j in 1..=2 {
            assert_eq!(s.weight(i, j), 1.0);
        }
    }
}

#[test]
fn test_pole_access() {
    let s = make_bilinear();
    let p = s.pole(1, 1);
    assert!((p[0]).abs() < 1e-12 && (p[1]).abs() < 1e-12);
    let p = s.pole(2, 2);
    assert!((p[0] - 1.0).abs() < 1e-12 && (p[1] - 1.0).abs() < 1e-12);
}

#[test]
fn test_value_all_corners() {
    let s = make_bilinear();
    let expected = [
        (0.0, 0.0, [0.0, 0.0, 0.0f64]),
        (0.0, 1.0, [1.0, 0.0, 0.0]),
        (1.0, 0.0, [0.0, 1.0, 0.0]),
        (1.0, 1.0, [1.0, 1.0, 0.0]),
    ];
    for (u, v, exp) in &expected {
        let p = s.value(*u, *v);
        assert!((p[0] - exp[0]).abs() < 1e-12, "corner ({},{}) x: {} != {}", u, v, p[0], exp[0]);
        assert!((p[1] - exp[1]).abs() < 1e-12, "corner ({},{}) y: {} != {}", u, v, p[1], exp[1]);
        assert!((p[2] - exp[2]).abs() < 1e-12);
    }
}

#[test]
fn test_value_interior_points() {
    let s = make_bilinear();
    // S(u,v) = (v, u, 0) for this patch.
    let samples = [(0.25, 0.75), (0.5, 0.5), (0.1, 0.9), (0.8, 0.3)];
    for (u, v) in &samples {
        let p = s.value(*u, *v);
        // x coordinate comes from v interpolation.
        assert!((p[0] - v).abs() < 1e-12, "x at ({},{}) = {}, expected {}", u, v, p[0], v);
        // y coordinate comes from u interpolation.
        assert!((p[1] - u).abs() < 1e-12, "y at ({},{}) = {}, expected {}", u, v, p[1], u);
        assert!((p[2]).abs() < 1e-12);
    }
}

#[test]
fn test_d1_partial_derivatives() {
    let s = make_bilinear();
    // dS/du = (0,1,0), dS/dv = (1,0,0) everywhere on this patch.
    let samples = [(0.0, 0.0), (0.5, 0.5), (1.0, 1.0), (0.3, 0.7)];
    for (u, v) in &samples {
        let (pt, du, dv) = s.d1(*u, *v);
        assert!((du[0]).abs() < 1e-10, "du.x at ({},{})", u, v);
        assert!((du[1] - 1.0).abs() < 1e-10, "du.y at ({},{}): {}", u, v, du[1]);
        assert!((du[2]).abs() < 1e-10);
        assert!((dv[0] - 1.0).abs() < 1e-10, "dv.x at ({},{}): {}", u, v, dv[0]);
        assert!((dv[1]).abs() < 1e-10, "dv.y at ({},{})", u, v);
        assert!((dv[2]).abs() < 1e-10);
        // Point should satisfy S(u,v) = (v,u,0).
        assert!((pt[0] - v).abs() < 1e-10);
        assert!((pt[1] - u).abs() < 1e-10);
    }
}

#[test]
fn test_quadratic_bezier_curve_surface() {
    // Pure quadratic in u (degree 2), constant in v (degree 0).
    // This tests a 3×1 grid:
    //   Row0: [(0,0,0)]
    //   Row1: [(1,2,0)]
    //   Row2: [(2,0,0)]
    let poles = vec![
        vec![[0.0, 0.0, 0.0]],
        vec![[1.0, 2.0, 0.0]],
        vec![[2.0, 0.0, 0.0]],
    ];
    let s = GeomBezierSurface::from_owned(poles, None);
    assert_eq!(s.u_degree(), 2);
    assert_eq!(s.v_degree(), 0);

    // At u=0.5: B_{0,2}(0.5)=0.25, B_{1,2}(0.5)=0.5, B_{2,2}(0.5)=0.25
    // x = 0.25*0 + 0.5*1 + 0.25*2 = 1.0
    // y = 0.25*0 + 0.5*2 + 0.25*0 = 1.0
    let p = s.value(0.5, 0.0);
    assert!((p[0] - 1.0).abs() < 1e-12, "x = {}", p[0]);
    assert!((p[1] - 1.0).abs() < 1e-12, "y = {}", p[1]);

    // Derivative at u=0.5: d/du of quadratic = 2*[(1,2,0)-(0,0,0)]*(1-0.5) + 2*[(2,0,0)-(1,2,0)]*0.5
    //   = (1,2,0) + (1,-2,0) = (2,0,0)
    let (_, du, _) = s.d1(0.5, 0.0);
    assert!((du[0] - 2.0).abs() < 1e-9, "du.x = {}", du[0]);
    assert!((du[1]).abs() < 1e-9, "du.y = {}", du[1]);
}

#[test]
fn test_rational_surface_evaluation() {
    // Quarter circle approximation in v using rational weights.
    // Use a 2×3 rational surface (linear in u, quadratic in v).
    // In v: w=(1,1/sqrt(2),1), poles = (1,0), (1,1), (0,1) scaled by w.
    let w_mid = 1.0f64 / 2.0f64.sqrt();
    let poles = vec![
        vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
        vec![[1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]],
    ];
    let weights = vec![
        vec![1.0, w_mid, 1.0],
        vec![1.0, w_mid, 1.0],
    ];
    let s = GeomBezierSurface::from_owned(poles, Some(weights));
    assert!(s.is_rational());

    // At v=0, any u: should be (1,0,z).
    let p0 = s.value(0.0, 0.0);
    assert!((p0[0] - 1.0).abs() < 1e-12, "p0.x = {}", p0[0]);
    assert!((p0[1]).abs() < 1e-12, "p0.y = {}", p0[1]);

    // At v=1, any u: should be (0,1,z).
    let p1 = s.value(0.0, 1.0);
    assert!((p1[0]).abs() < 1e-12, "p1.x = {}", p1[0]);
    assert!((p1[1] - 1.0).abs() < 1e-12, "p1.y = {}", p1[1]);

    // At v=0.5: should approximate a point on the unit circle.
    // For a rational quadratic: S(0.5) ≈ (cos(π/4), sin(π/4)) = (1/√2, 1/√2).
    let p_mid = s.value(0.5, 0.5);
    let expected = 1.0f64 / 2.0f64.sqrt();
    assert!((p_mid[0] - expected).abs() < 1e-12, "x = {} expected {}", p_mid[0], expected);
    assert!((p_mid[1] - expected).abs() < 1e-12, "y = {} expected {}", p_mid[1], expected);
}

#[test]
fn test_increase_u_degree_preserves_shape() {
    let s_orig = make_bilinear();
    let mut s = make_bilinear();
    s.increase_u_degree(3);
    assert_eq!(s.u_degree(), 3);
    assert_eq!(s.nb_u_poles(), 4);

    for u in [0.0, 0.1, 0.3, 0.5, 0.7, 0.9, 1.0] {
        for v in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let p1 = s_orig.value(u, v);
            let p2 = s.value(u, v);
            assert!((p1[0] - p2[0]).abs() < 1e-9, "x at ({},{}): {} vs {}", u, v, p1[0], p2[0]);
            assert!((p1[1] - p2[1]).abs() < 1e-9, "y at ({},{}): {} vs {}", u, v, p1[1], p2[1]);
        }
    }
}

#[test]
fn test_increase_v_degree_preserves_shape() {
    let s_orig = make_bilinear();
    let mut s = make_bilinear();
    s.increase_v_degree(2);
    assert_eq!(s.v_degree(), 2);
    assert_eq!(s.nb_v_poles(), 3);

    for u in [0.0, 0.25, 0.5, 0.75, 1.0] {
        for v in [0.0, 0.2, 0.5, 0.8, 1.0] {
            let p1 = s_orig.value(u, v);
            let p2 = s.value(u, v);
            assert!((p1[0] - p2[0]).abs() < 1e-9);
            assert!((p1[1] - p2[1]).abs() < 1e-9);
        }
    }
}

#[test]
fn test_insert_u_pole_row_count() {
    let mut s = make_bilinear();
    let row = [[0.0, 0.5, 0.0], [1.0, 0.5, 0.0]];
    s.insert_u_pole_row(1, &row, None);
    assert_eq!(s.nb_u_poles(), 3);
    // The inserted row is at index 1 (0-based: 0).
    let p = s.pole(1, 1);
    assert!((p[1] - 0.5).abs() < 1e-12, "y = {}", p[1]);
}

#[test]
fn test_insert_v_pole_col_count() {
    let mut s = make_bilinear();
    let col = [[0.5, 0.0, 0.0], [0.5, 1.0, 0.0]];
    s.insert_v_pole_col(2, &col, None);
    assert_eq!(s.nb_v_poles(), 3);
    let p = s.pole(1, 2);
    assert!((p[0] - 0.5).abs() < 1e-12, "x = {}", p[0]);
}

#[test]
fn test_z_coordinate_flat_surface() {
    // Verify the z=0 plane is preserved for all evaluation points.
    let s = make_bilinear();
    for u in [0.0, 0.1, 0.2, 0.5, 0.8, 1.0] {
        for v in [0.0, 0.1, 0.2, 0.5, 0.8, 1.0] {
            let p = s.value(u, v);
            assert!((p[2]).abs() < 1e-13, "z != 0 at ({},{}): z={}", u, v, p[2]);
        }
    }
}

#[test]
fn test_cubic_bezier_surface_midpoint() {
    // Cubic surface in u, linear in v.
    // Standard cubic Bézier: (0,0), (1,3), (2,3), (3,0) — tent.
    let poles = vec![
        vec![[0.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        vec![[1.0, 3.0, 0.0], [1.0, 3.0, 1.0]],
        vec![[2.0, 3.0, 0.0], [2.0, 3.0, 1.0]],
        vec![[3.0, 0.0, 0.0], [3.0, 0.0, 1.0]],
    ];
    let s = GeomBezierSurface::from_owned(poles, None);
    assert_eq!(s.u_degree(), 3);

    // At u=0.5, v=0: cubic Bézier midpoint.
    // B_0(0.5)=1/8, B_1(0.5)=3/8, B_2(0.5)=3/8, B_3(0.5)=1/8
    // x = (1/8)*0 + (3/8)*1 + (3/8)*2 + (1/8)*3 = 0 + 3/8 + 6/8 + 3/8 = 12/8 = 1.5
    // y = (1/8)*0 + (3/8)*3 + (3/8)*3 + (1/8)*0 = 9/8 + 9/8 = 18/8 = 2.25
    let p = s.value(0.5, 0.0);
    assert!((p[0] - 1.5).abs() < 1e-12, "x = {}", p[0]);
    assert!((p[1] - 2.25).abs() < 1e-12, "y = {}", p[1]);
    assert!((p[2]).abs() < 1e-12);

    // Same at v=1.
    let p2 = s.value(0.5, 1.0);
    assert!((p2[0] - 1.5).abs() < 1e-12);
    assert!((p2[1] - 2.25).abs() < 1e-12);
    assert!((p2[2] - 1.0).abs() < 1e-12);
}

#[test]
fn test_new_constructor_matches_from_owned() {
    let p00 = [0.0f64, 0.0, 0.0];
    let p01 = [1.0f64, 0.0, 0.0];
    let p10 = [0.0f64, 1.0, 0.0];
    let p11 = [1.0f64, 1.0, 0.0];

    let s_new = GeomBezierSurface::from_owned(
        vec![vec![p00, p01], vec![p10, p11]],
        None,
    );
    let s_owned = make_bilinear();

    for u in [0.0, 0.5, 1.0] {
        for v in [0.0, 0.5, 1.0] {
            let p1 = s_new.value(u, v);
            let p2 = s_owned.value(u, v);
            assert!((p1[0] - p2[0]).abs() < 1e-12);
            assert!((p1[1] - p2[1]).abs() < 1e-12);
        }
    }
}
