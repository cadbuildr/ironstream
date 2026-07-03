//! Integration tests for `BSplSLib` — tensor-product B-spline surface utilities.
//! All tests import from `ironstream::` only.

use ironstream::bsplslib::{
    build_cache, cache_d0, cache_d1, d0, d1, d2, d3, homogeneous_d0, homogeneous_d1, iso_u,
    iso_v, resolution, reverse_u, reverse_v, BSplSLib,
};
use ironstream::gp::Pnt;
use ironstream::precision;

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

/// Bilinear patch over [0,1]^2 in z=0 plane: corners at (0,0,0), (1,0,0),
/// (0,1,0), (1,1,0). poles[i][j]: i = U index (0..2), j = V index (0..2).
fn bilinear_patch() -> (Vec<f64>, Vec<f64>, Vec<Vec<Pnt>>, Vec<Vec<f64>>) {
    let u_flat = vec![0.0, 0.0, 1.0, 1.0];
    let v_flat = vec![0.0, 0.0, 1.0, 1.0];
    let poles = vec![
        vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)],
        vec![Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)],
    ];
    let weights = vec![vec![1.0f64; 2]; 2];
    (u_flat, v_flat, poles, weights)
}

/// Cubic Bezier (clamped) surface: u and v both degree 3, 4×4 poles on a
/// flat z=0 grid with poles[i][j] = (i/3, j/3, 0).
fn cubic_flat_patch() -> (Vec<f64>, Vec<f64>, Vec<Vec<Pnt>>, Vec<Vec<f64>>) {
    let u_flat = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let v_flat = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let poles: Vec<Vec<Pnt>> = (0..4)
        .map(|i| {
            (0..4)
                .map(|j| Pnt::new(i as f64 / 3.0, j as f64 / 3.0, 0.0))
                .collect()
        })
        .collect();
    let weights = vec![vec![1.0f64; 4]; 4];
    (u_flat, v_flat, poles, weights)
}

/// Tiny helper: assert two f64 values are within `tol`.
fn approx_eq(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{msg}: |{a} - {b}| = {} > {tol}",
        (a - b).abs()
    );
}

/// Assert two Pnt values are within `tol` in each component.
fn pnt_approx_eq(a: Pnt, b: Pnt, tol: f64, msg: &str) {
    approx_eq(a.x, b.x, tol, &format!("{msg} x"));
    approx_eq(a.y, b.y, tol, &format!("{msg} y"));
    approx_eq(a.z, b.z, tol, &format!("{msg} z"));
}

// ---------------------------------------------------------------------------
// Test 1: D0 on bilinear patch — midpoint
// ---------------------------------------------------------------------------

#[test]
fn d0_bilinear_midpoint() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();
    let p = d0(0.5, 0.5, &u_flat, &v_flat, &poles, &weights, 1, 1);
    pnt_approx_eq(p, Pnt::new(0.5, 0.5, 0.0), precision::CONFUSION, "bilinear midpoint");
}

// ---------------------------------------------------------------------------
// Test 2: D0 on bilinear patch — corners interpolate control points
// ---------------------------------------------------------------------------

#[test]
fn d0_bilinear_corners() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();
    let tol = precision::CONFUSION;

    let corners = [
        ((0.0, 0.0), Pnt::new(0.0, 0.0, 0.0)),
        ((0.0, 1.0), Pnt::new(0.0, 1.0, 0.0)),
        ((1.0, 0.0), Pnt::new(1.0, 0.0, 0.0)),
        ((1.0, 1.0), Pnt::new(1.0, 1.0, 0.0)),
    ];
    for ((u, v), expected) in corners {
        let got = d0(u, v, &u_flat, &v_flat, &poles, &weights, 1, 1);
        pnt_approx_eq(got, expected, tol, &format!("corner ({u},{v})"));
    }
}

// ---------------------------------------------------------------------------
// Test 3: D1 — partial derivatives on a flat bilinear patch
// ---------------------------------------------------------------------------

#[test]
fn d1_bilinear_derivatives() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();
    // For a bilinear patch in [0,1]^2 with corners (0,0),(1,0),(0,1),(1,1),
    // the partial derivatives are constant: dS/dU = (1,0,0), dS/dV = (0,1,0).
    let res = d1(0.5, 0.5, &u_flat, &v_flat, &poles, &weights, 1, 1);
    pnt_approx_eq(res.d1u, Pnt::new(1.0, 0.0, 0.0), 1e-6, "d1u");
    pnt_approx_eq(res.d1v, Pnt::new(0.0, 1.0, 0.0), 1e-6, "d1v");
}

// ---------------------------------------------------------------------------
// Test 4: D2 on cubic flat patch — second derivative relates to curvature
// ---------------------------------------------------------------------------

#[test]
fn d2_cubic_flat_second_deriv_z() {
    let (u_flat, v_flat, poles, weights) = cubic_flat_patch();
    // The patch is flat (z=0), so the normal direction second derivatives
    // should be ~zero. The main partials describe the linear mapping.
    let res = d2(0.5, 0.5, &u_flat, &v_flat, &poles, &weights, 3, 3);
    // z-component of all derivatives must be zero for a flat z=0 patch.
    approx_eq(res.point.z, 0.0, precision::CONFUSION, "D2 pt z");
    approx_eq(res.d1u.z, 0.0, 1e-9, "D2 d1u z");
    approx_eq(res.d1v.z, 0.0, 1e-9, "D2 d1v z");
    approx_eq(res.d2u.z, 0.0, 1e-8, "D2 d2u z");
    approx_eq(res.d2v.z, 0.0, 1e-8, "D2 d2v z");
    approx_eq(res.d2uv.z, 0.0, 1e-8, "D2 d2uv z");
}

// ---------------------------------------------------------------------------
// Test 5: D3 on cubic flat patch — point at midpoint
// ---------------------------------------------------------------------------

#[test]
fn d3_cubic_flat_midpoint() {
    let (u_flat, v_flat, poles, weights) = cubic_flat_patch();
    let res = d3(0.5, 0.5, &u_flat, &v_flat, &poles, &weights, 3, 3);
    // Midpoint of [0,1]^2 cubic Bezier on a regular grid (0..1/3..2/3..1) → (0.5, 0.5, 0).
    pnt_approx_eq(res.point, Pnt::new(0.5, 0.5, 0.0), precision::CONFUSION, "D3 midpoint");
    // z-components of all derivatives are zero.
    approx_eq(res.d1u.z, 0.0, 1e-9, "D3 d1u z");
    approx_eq(res.d1v.z, 0.0, 1e-9, "D3 d1v z");
}

// ---------------------------------------------------------------------------
// Test 6: BuildCache + CacheD0 agrees with D0
// ---------------------------------------------------------------------------

#[test]
fn build_cache_and_cache_d0() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();
    let cache = build_cache(0.3, 0.7, &u_flat, &v_flat, &poles, &weights, 1, 1);
    // CacheD0 must agree with D0 at the same point.
    let samples = [(0.1, 0.2), (0.3, 0.7), (0.9, 0.05), (0.5, 0.5)];
    for (u, v) in samples {
        if u >= cache.u_min && u <= cache.u_max && v >= cache.v_min && v <= cache.v_max {
            let direct = d0(u, v, &u_flat, &v_flat, &poles, &weights, 1, 1);
            let from_cache = cache_d0(u, v, &cache);
            pnt_approx_eq(from_cache, direct, 1e-9, &format!("cache_d0 ({u},{v})"));
        }
    }
}

// ---------------------------------------------------------------------------
// Test 7: CacheD1 point agrees with D0
// ---------------------------------------------------------------------------

#[test]
fn cache_d1_point_matches_d0() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();
    let cache = build_cache(0.5, 0.5, &u_flat, &v_flat, &poles, &weights, 1, 1);
    let res = cache_d1(0.5, 0.5, &cache);
    let direct = d0(0.5, 0.5, &u_flat, &v_flat, &poles, &weights, 1, 1);
    pnt_approx_eq(res.point, direct, precision::CONFUSION, "cache_d1 point");
    // Derivatives on the bilinear [0,1]^2 patch: dS/dU = (1,0,0), dS/dV = (0,1,0).
    pnt_approx_eq(res.d1u, Pnt::new(1.0, 0.0, 0.0), 1e-4, "cache_d1 d1u");
    pnt_approx_eq(res.d1v, Pnt::new(0.0, 1.0, 0.0), 1e-4, "cache_d1 d1v");
}

// ---------------------------------------------------------------------------
// Test 8: Iso U — V-direction curve at u=0 matches first column of poles
// ---------------------------------------------------------------------------

#[test]
fn iso_u_first_column() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();
    // The U-iso at u=0 should return a V-curve that passes through
    // poles[0][0]=(0,0,0) and poles[0][1]=(0,1,0).
    let iso = iso_u(0.0, &u_flat, &v_flat, &poles, &weights, 1, 1);

    // The iso-curve has the same degree as V (=1) and nv poles.
    assert_eq!(iso.degree, 1, "iso_u degree");
    assert_eq!(iso.poles.len(), 2, "iso_u poles count");

    // Evaluate the iso curve at v=0 and v=1.
    let p0 = ironstream::bsplines::BSplineCurve::new(
        iso.degree,
        iso.flat_knots.clone(),
        iso.poles.clone(),
        None,
    );
    pnt_approx_eq(p0.value(0.0), Pnt::new(0.0, 0.0, 0.0), precision::CONFUSION, "iso_u v=0");
    pnt_approx_eq(p0.value(1.0), Pnt::new(0.0, 1.0, 0.0), precision::CONFUSION, "iso_u v=1");
}

// ---------------------------------------------------------------------------
// Test 9: Iso V — U-direction curve at v=1 matches last row of poles
// ---------------------------------------------------------------------------

#[test]
fn iso_v_last_row() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();
    // The V-iso at v=1 gives the U-direction curve at the top edge.
    // Poles at j=1: (0,1,0) and (1,1,0).
    let iso = iso_v(1.0, &u_flat, &v_flat, &poles, &weights, 1, 1);

    assert_eq!(iso.degree, 1, "iso_v degree");
    assert_eq!(iso.poles.len(), 2, "iso_v poles count");

    let crv = ironstream::bsplines::BSplineCurve::new(
        iso.degree,
        iso.flat_knots.clone(),
        iso.poles.clone(),
        None,
    );
    pnt_approx_eq(crv.value(0.0), Pnt::new(0.0, 1.0, 0.0), precision::CONFUSION, "iso_v u=0");
    pnt_approx_eq(crv.value(1.0), Pnt::new(1.0, 1.0, 0.0), precision::CONFUSION, "iso_v u=1");
}

// ---------------------------------------------------------------------------
// Test 10: Reverse U — D0 on reversed surface equals original at (1-u, v)
// ---------------------------------------------------------------------------

#[test]
fn reverse_u_d0_consistency() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();

    let (new_u_flat, new_poles, new_weights) = reverse_u(&u_flat, &poles, &weights, 1);

    // After U-reversal with range [0,1], point at u_rev should equal original at (1-u).
    let samples = [0.0f64, 0.25, 0.5, 0.75, 1.0];
    for &u in &samples {
        let original = d0(u, 0.5, &u_flat, &v_flat, &poles, &weights, 1, 1);
        let reversed = d0(1.0 - u, 0.5, &new_u_flat, &v_flat, &new_poles, &new_weights, 1, 1);
        pnt_approx_eq(original, reversed, precision::CONFUSION, &format!("reverse_u u={u}"));
    }
}

// ---------------------------------------------------------------------------
// Test 11: Reverse V — D0 on reversed surface equals original at (u, 1-v)
// ---------------------------------------------------------------------------

#[test]
fn reverse_v_d0_consistency() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();

    let (new_v_flat, new_poles, new_weights) = reverse_v(&v_flat, &poles, &weights, 1);

    let samples = [0.0f64, 0.3, 0.5, 0.8, 1.0];
    for &v in &samples {
        let original = d0(0.5, v, &u_flat, &v_flat, &poles, &weights, 1, 1);
        let reversed = d0(0.5, 1.0 - v, &u_flat, &new_v_flat, &new_poles, &new_weights, 1, 1);
        pnt_approx_eq(original, reversed, precision::CONFUSION, &format!("reverse_v v={v}"));
    }
}

// ---------------------------------------------------------------------------
// Test 12: Resolution returns sensible parametric tolerances
// ---------------------------------------------------------------------------

#[test]
fn resolution_bilinear() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();
    let tol3d = 1e-3;
    let (u_tol, v_tol) = resolution(&u_flat, &v_flat, &poles, &weights, 1, 1, tol3d);
    // For the unit bilinear patch, max |dS/dU| ≈ 1 and max |dS/dV| ≈ 1,
    // so tolerances should be close to tol3d.
    assert!(u_tol > 0.0, "u_tol must be positive");
    assert!(v_tol > 0.0, "v_tol must be positive");
    // Should be within an order of magnitude of tol3d.
    assert!(u_tol < tol3d * 100.0, "u_tol {u_tol} unreasonably large");
    assert!(v_tol < tol3d * 100.0, "v_tol {v_tol} unreasonably large");
}

// ---------------------------------------------------------------------------
// Test 13: HomogeneousD0 — weight and homogeneous point recover D0
// ---------------------------------------------------------------------------

#[test]
fn homogeneous_d0_agrees_with_d0() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();
    let samples = [(0.0, 0.0), (0.5, 0.5), (1.0, 1.0), (0.3, 0.7)];
    for (u, v) in samples {
        let (hom_pt, w) = homogeneous_d0(u, v, &u_flat, &v_flat, &poles, &weights, 1, 1);
        let w = if w.abs() < 1e-15 { 1.0 } else { w };
        let cart_from_hom = hom_pt * (1.0 / w);
        let direct = d0(u, v, &u_flat, &v_flat, &poles, &weights, 1, 1);
        pnt_approx_eq(cart_from_hom, direct, precision::CONFUSION,
                      &format!("homogeneous_d0 ({u},{v})"));
    }
}

// ---------------------------------------------------------------------------
// Test 14: HomogeneousD1 — Cartesian D1 recovers from homogeneous form
// ---------------------------------------------------------------------------

#[test]
fn homogeneous_d1_agrees_with_d1() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();
    let u = 0.4;
    let v = 0.6;

    let hres = homogeneous_d1(u, v, &u_flat, &v_flat, &poles, &weights, 1, 1);
    let w0: f64 = if hres.weight.abs() < 1e-15 {
        1.0
    } else {
        hres.weight
    };

    // Cartesian point from homogeneous: P = hom_point / w.
    let cart_pt = hres.hom_point * (1.0 / w0);
    let direct = d0(u, v, &u_flat, &v_flat, &poles, &weights, 1, 1);
    pnt_approx_eq(cart_pt, direct, precision::CONFUSION, "hom_d1 point");

    // Cartesian D1U via quotient rule: (A'_u - w'_u * P) / w.
    let d1u_cart = (hres.d1u_hom - direct * hres.w_du) * (1.0 / w0);
    let d1res = d1(u, v, &u_flat, &v_flat, &poles, &weights, 1, 1);
    pnt_approx_eq(d1u_cart, d1res.d1u, 1e-9, "hom_d1 d1u");

    // Cartesian D1V.
    let d1v_cart = (hres.d1v_hom - direct * hres.w_dv) * (1.0 / w0);
    pnt_approx_eq(d1v_cart, d1res.d1v, 1e-9, "hom_d1 d1v");
}

// ---------------------------------------------------------------------------
// Test 15: BSplSLib facade — D0 via associated function
// ---------------------------------------------------------------------------

#[test]
fn facade_d0() {
    let (u_flat, v_flat, poles, weights) = bilinear_patch();
    let p = BSplSLib::d0(0.5, 0.5, &u_flat, &v_flat, &poles, &weights, 1, 1);
    pnt_approx_eq(p, Pnt::new(0.5, 0.5, 0.0), precision::CONFUSION, "facade D0");
}

// ---------------------------------------------------------------------------
// Test 16: D0 on a cubic patch with a lift (z-parabola)
// ---------------------------------------------------------------------------

#[test]
fn d0_cubic_lifted_patch() {
    // Bicubic patch lifted in z so that z(u,v) = u*(1-u)*v*(1-v) roughly.
    // We use a simple parabolic lift: the middle poles are z=0.25.
    let u_flat = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let v_flat = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    // 4×4 pole grid: z = 0 on boundary, z = 0.25 in interior.
    let mut poles: Vec<Vec<Pnt>> = (0..4)
        .map(|i| {
            (0..4)
                .map(|j| {
                    let x = i as f64 / 3.0;
                    let y = j as f64 / 3.0;
                    let z = if i > 0 && i < 3 && j > 0 && j < 3 {
                        0.25
                    } else {
                        0.0
                    };
                    Pnt::new(x, y, z)
                })
                .collect()
        })
        .collect();
    // Boundary poles at corners: exactly (0,0,0) etc.
    poles[0][0] = Pnt::new(0.0, 0.0, 0.0);
    poles[3][0] = Pnt::new(1.0, 0.0, 0.0);
    poles[0][3] = Pnt::new(0.0, 1.0, 0.0);
    poles[3][3] = Pnt::new(1.0, 1.0, 0.0);

    let weights = vec![vec![1.0f64; 4]; 4];

    // Corners should interpolate.
    let tol = precision::CONFUSION;
    let c00 = d0(0.0, 0.0, &u_flat, &v_flat, &poles, &weights, 3, 3);
    pnt_approx_eq(c00, Pnt::new(0.0, 0.0, 0.0), tol, "lifted corner 00");
    let c11 = d0(1.0, 1.0, &u_flat, &v_flat, &poles, &weights, 3, 3);
    pnt_approx_eq(c11, Pnt::new(1.0, 1.0, 0.0), tol, "lifted corner 11");

    // Midpoint should have z > 0 due to the lift.
    let mid = d0(0.5, 0.5, &u_flat, &v_flat, &poles, &weights, 3, 3);
    assert!(mid.z > 0.0, "midpoint z={} should be > 0 with lift", mid.z);
}
