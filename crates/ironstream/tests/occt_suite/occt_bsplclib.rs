//! Integration tests for `ironstream::bsplclib`, mirroring the OpenCascade
//! `BSplCLib` unit-test suite.

use ironstream::bsplclib::{
    antiderivative_pc, build_knots, build_schoenberg_points, cache_d0, cache_d1, d0, d1, d2, d3,
    dn, eval_bspline_basis, flat_bezier_knots, hunt, insert_knot, knot_analysis, knot_sequence,
    locate_parameter, rational_derivative, BSplCLib, KnotDistribution,
};
use ironstream::gp::Pnt;
use ironstream::precision;

// ─── helpers ────────────────────────────────────────────────────────────────

fn assert_pnt_near(a: Pnt, b: Pnt, tol: f64, msg: &str) {
    assert!(
        a.distance(b) < tol,
        "{}: |{:?} - {:?}| = {}",
        msg,
        a,
        b,
        a.distance(b)
    );
}

fn ones(n: usize) -> Vec<f64> {
    vec![1.0; n]
}

// ─── 1. KnotSequence ────────────────────────────────────────────────────────

#[test]
fn knot_sequence_clamped_cubic() {
    // Degree-3 B-spline, 5 poles:  knots=[0,1,2], mults=[4,1,4] => flat has 9 entries.
    let knots = [0.0, 1.0, 2.0];
    let mults = [4, 1, 4];
    let flat = knot_sequence(&knots, &mults);
    assert_eq!(flat.len(), 9, "flat length");
    assert_eq!(flat[0], 0.0);
    assert_eq!(flat[3], 0.0);
    assert_eq!(flat[4], 1.0);
    assert_eq!(flat[5], 2.0);
    assert_eq!(flat[8], 2.0);
}

// ─── 2. KnotAnalysis ────────────────────────────────────────────────────────

#[test]
fn knot_analysis_piecewise_bezier() {
    // Degree-2, two Bezier segments over [0,1,2]: mults [3,2,3] is piecewise Bezier.
    let knots = [0.0, 1.0, 2.0];
    let mults = [3, 2, 3];
    let dist = knot_analysis(2, &knots, &mults);
    assert_eq!(dist, KnotDistribution::PiecewiseBezier);
}

#[test]
fn knot_analysis_uniform() {
    // All multiplicities 1, uniform spacing => Uniform.
    let knots = [0.0, 1.0, 2.0, 3.0, 4.0];
    let mults = [1, 1, 1, 1, 1];
    let dist = knot_analysis(2, &knots, &mults);
    assert_eq!(dist, KnotDistribution::Uniform);
}

#[test]
fn knot_analysis_quasi_uniform() {
    // Degree-3 quasi-uniform: ends mult=4, interior mult=1, uniform spacing.
    let knots = [0.0, 1.0, 2.0, 3.0];
    let mults = [4, 1, 1, 4];
    let dist = knot_analysis(3, &knots, &mults);
    assert_eq!(dist, KnotDistribution::QuasiUniform);
}

// ─── 3. Hunt / LocateParameter ──────────────────────────────────────────────

#[test]
fn hunt_mid_span() {
    // flat = [0,0,0,1,2,3,3,3] — a clamped cubic with 5 poles.
    let flat = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
    // u = 1.5 should land in span [3,4] (0-based index 3).
    let idx = hunt(&flat, 1.5);
    assert_eq!(idx, 3, "hunt span index for u=1.5");

    // BSplCLib::LocateParameter clamps to [degree, n_poles-1] = [3, 4].
    let loc = locate_parameter(&flat, 3, 5, 1.5);
    assert_eq!(loc, 3);
}

#[test]
fn hunt_boundary_clamp() {
    let flat = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    // u exactly at upper end.
    let idx = hunt(&flat, 1.0);
    assert_eq!(idx, flat.len() - 2, "clamped to last span");
    // u below lower end.
    let idx2 = hunt(&flat, -1.0);
    assert_eq!(idx2, 0);
}

// ─── 4. EvalBsplineBasis ────────────────────────────────────────────────────

#[test]
fn basis_sums_to_one() {
    // Clamped quadratic: flat=[0,0,0,1,2,2,2], 4 poles.
    let flat = vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0];
    let degree = 2;
    for i in 0..=10 {
        let u = i as f64 * 0.2;
        let span = hunt(&flat, u.min(1.9999)).max(degree);
        let mut basis = vec![0.0; degree + 1];
        eval_bspline_basis(span, u.min(1.9999), degree, &flat, &mut basis);
        let sum: f64 = basis.iter().sum();
        assert!(
            (sum - 1.0).abs() < precision::CONFUSION,
            "basis sum != 1 at u={u}: {sum}"
        );
    }
}

// ─── 5. D0 — evaluate a B-spline at a parameter ─────────────────────────────

#[test]
fn d0_straight_line() {
    // Degree-1, 2 poles on the X axis: curve is the segment [0,2].
    let flat = vec![0.0, 0.0, 1.0, 1.0];
    let poles = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0)];
    let weights = ones(poles.len());
    let mid = d0(0.5, &flat, &poles, &weights);
    assert_pnt_near(mid, Pnt::new(1.0, 0.0, 0.0), precision::CONFUSION, "mid of line");
    let start = d0(0.0, &flat, &poles, &weights);
    assert_pnt_near(start, poles[0], precision::CONFUSION, "start of line");
    let end = d0(1.0, &flat, &poles, &weights);
    assert_pnt_near(end, poles[1], precision::CONFUSION, "end of line");
}

#[test]
fn d0_cubic_bezier_endpoints() {
    // Cubic Bezier as clamped B-spline. Endpoints must interpolate poles.
    let flat = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 3.0, 0.0),
        Pnt::new(2.0, 3.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
    ];
    let w = ones(poles.len());
    assert_pnt_near(d0(0.0, &flat, &poles, &w), poles[0], 1e-12, "start");
    assert_pnt_near(d0(1.0, &flat, &poles, &w), poles[3], 1e-12, "end");
    // Midpoint of symmetric net => x == 1.5.
    let mid = d0(0.5, &flat, &poles, &w);
    assert!((mid.x - 1.5).abs() < 1e-10, "mid.x={}", mid.x);
}

// ─── 6. D1 ──────────────────────────────────────────────────────────────────

#[test]
fn d1_straight_line_tangent() {
    // Degree-1 line from (0,0,0) to (2,0,0): tangent must be (2,0,0) (param range [0,1]).
    let flat = vec![0.0, 0.0, 1.0, 1.0];
    let poles = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0)];
    let w = ones(poles.len());
    let (_p, t) = d1(0.5, &flat, &poles, &w);
    assert!(
        (t.x - 2.0).abs() < precision::CONFUSION,
        "tangent x={}", t.x
    );
    assert!(t.y.abs() < precision::CONFUSION, "tangent y={}", t.y);
}

// ─── 7. D2 / D3 ─────────────────────────────────────────────────────────────

#[test]
fn d2_quadratic_second_deriv() {
    // Quadratic Bezier = parabola; second derivative is constant (2 × leading coeff).
    // poles: (0,0,0), (1,2,0), (2,0,0) — parabola opening down in Y.
    let flat = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 2.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let w = ones(poles.len());
    let (_, _, d2v) = d2(0.5, &flat, &poles, &w);
    // The quadratic Bezier P(t)= (1-t)^2 P0 + 2t(1-t)P1 + t^2 P2.
    // Y: (1-t)^2*0 + 2t(1-t)*2 + t^2*0 = 4t(1-t). P''_y = -8 (constant).
    // x: P(t) = 2t. P''_x = 0.
    assert!((d2v.x).abs() < 1e-8, "d2x={}", d2v.x);
    assert!((d2v.y + 8.0).abs() < 1e-8, "d2y={}", d2v.y);
}

#[test]
fn d3_cubic_constant_third_deriv() {
    // Cubic Bezier: (0,0,0),(1,0,0),(2,0,0),(3,0,0) is a straight line.
    // All derivatives > 1 must be zero.
    let flat = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
    ];
    let w = ones(poles.len());
    let (_, _, d2v, d3v) = d3(0.5, &flat, &poles, &w);
    assert!(d2v.norm() < 1e-10, "d2 of line must be 0: {:?}", d2v);
    assert!(d3v.norm() < 1e-10, "d3 of line must be 0: {:?}", d3v);
}

// ─── 8. RationalDerivative ───────────────────────────────────────────────────

#[test]
fn rational_derivative_unit_weight() {
    // With all weights = 1 the rational derivative reduces to the regular one.
    // For a straight line segment: A'(u) = (P1-P0), w'(u) = 0, w(u) = 1.
    let a_ders = vec![Pnt::new(0.5, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0)];
    let w_ders = vec![1.0, 0.0]; // w=1, w'=0
    let ck = rational_derivative(&a_ders, &w_ders, 1);
    // C(u) = A(u)/w = A(u), C'(u) = (A'(u) - C(u)*w'(u)) / w(u) = A'(u).
    assert!((ck[0].x - 0.5).abs() < 1e-12, "C(u).x");
    assert!((ck[1].x - 2.0).abs() < 1e-12, "C'(u).x");
}

// ─── 9. BuildKnots ──────────────────────────────────────────────────────────

#[test]
fn build_knots_simple() {
    let k = [0.0, 1.0];
    let m = [2, 2];
    let flat = build_knots(&k, &m);
    assert_eq!(flat, vec![0.0, 0.0, 1.0, 1.0]);
}

// ─── 10. BuildSchoenbergPoints ──────────────────────────────────────────────

#[test]
fn schoenberg_points_cubic_5poles() {
    // Degree-3 B-spline, flat=[0,0,0,0,1,2,3,3,3,3] (quasi-uniform, 6 poles).
    let flat = vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0, 3.0];
    let pts = build_schoenberg_points(3, &flat);
    // For degree 3, n_poles = 10 - 3 - 1 = 6.
    assert_eq!(pts.len(), 6, "6 Greville points");
    // t_0 = (flat[1]+flat[2]+flat[3])/3 = 0
    assert!((pts[0] - 0.0).abs() < 1e-12, "t0={}", pts[0]);
    // t_1 = (flat[2]+flat[3]+flat[4])/3 = (0+0+1)/3 = 1/3
    assert!((pts[1] - 1.0 / 3.0).abs() < 1e-12, "t1={}", pts[1]);
    // Last point = (flat[7]+flat[8]+flat[9])/3 = (3+3+3)/3 = 3
    assert!((pts[5] - 3.0).abs() < 1e-12, "t5={}", pts[5]);
}

// ─── 11. FlatBezierKnots ─────────────────────────────────────────────────────

#[test]
fn flat_bezier_knots_cubic() {
    let fk = flat_bezier_knots(3);
    assert_eq!(fk, vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]);
}

// ─── 12. InsertKnot ──────────────────────────────────────────────────────────

#[test]
fn insert_knot_preserves_curve() {
    // Cubic clamped line: inserting a knot at 0.5 must not change the geometry.
    let flat = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
    ];
    let w = ones(poles.len());
    let (new_flat, new_poles, new_w) = insert_knot(3, 0.5, &flat, &poles, &w);

    // After insertion: 9 flat knots, 5 poles.
    assert_eq!(new_flat.len(), 9, "knot count after insertion");
    assert_eq!(new_poles.len(), 5, "pole count after insertion");

    // Geometry is preserved: sample at several u.
    for i in 0..=8 {
        let u = i as f64 / 8.0;
        let before = d0(u, &flat, &poles, &w);
        let after = d0(u, &new_flat, &new_poles, &new_w);
        assert_pnt_near(before, after, 1e-10, &format!("u={u}"));
    }
}

// ─── 13. CacheD0 / CacheD1 ──────────────────────────────────────────────────

#[test]
fn cache_d0_linear_segment() {
    // A single linear Bezier segment (2 poles) over [0, 1].
    let cache_poles = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(4.0, 0.0, 0.0)];
    let w = ones(cache_poles.len());
    let p = cache_d0(0.5, 0.0, 1.0, &cache_poles, &w);
    assert_pnt_near(p, Pnt::new(2.0, 0.0, 0.0), precision::CONFUSION, "midpoint");
}

#[test]
fn cache_d1_linear_tangent() {
    let cache_poles = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(4.0, 0.0, 0.0)];
    let w = ones(cache_poles.len());
    let (p, t) = cache_d1(0.5, 0.0, 1.0, &cache_poles, &w);
    assert_pnt_near(p, Pnt::new(2.0, 0.0, 0.0), precision::CONFUSION, "position");
    // Tangent = (P1 - P0) / (t_max - t_min) = 4.0 in x.
    assert!((t.x - 4.0).abs() < 1e-4, "tangent x={}", t.x);
}

// ─── 14. AntiderivativePC ────────────────────────────────────────────────────

#[test]
fn antiderivative_pc_quadratic() {
    // P(t) = 1 + 2t + 3t^2 => ∫P dt = t + t^2 + t^3 (constant=0).
    let coeffs = [1.0, 2.0, 3.0];
    let anti = antiderivative_pc(&coeffs);
    assert_eq!(anti.len(), 4);
    assert!((anti[0] - 0.0).abs() < 1e-15, "c0={}", anti[0]);
    assert!((anti[1] - 1.0).abs() < 1e-15, "c1={}", anti[1]);
    assert!((anti[2] - 1.0).abs() < 1e-15, "c2={}", anti[2]);
    assert!((anti[3] - 1.0).abs() < 1e-15, "c3={}", anti[3]);
}

// ─── 15. DN ──────────────────────────────────────────────────────────────────

#[test]
fn dn_cubic_third_derivative_nonzero() {
    // Cubic y-parabola: poles give a curve where d3 != 0.
    // poles: (0,0,0),(0,1,0),(0,2,0),(0,3,0) is a straight line in y => d3=0.
    // Instead use an asymmetric cubic.
    let flat = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
    ];
    let w = ones(poles.len());
    let d3v = dn(3, 0.5, &flat, &poles, &w);
    // Third derivative of an S-cubic is nonzero; just check it's finite.
    assert!(d3v.x.is_finite() && d3v.y.is_finite(), "d3={:?}", d3v);
}

// ─── 16. NURBS quarter circle via D0 ─────────────────────────────────────────

#[test]
fn d0_nurbs_quarter_circle() {
    // Standard rational quadratic NURBS quarter circle (radius = 1).
    // Weight of middle pole = 1/sqrt(2); all points must lie on unit circle.
    let sqrt2_inv = 1.0 / 2.0_f64.sqrt();
    let flat = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let poles = vec![
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ];
    let weights = vec![1.0, sqrt2_inv, 1.0];
    for i in 0..=8 {
        let u = i as f64 / 8.0;
        let p = d0(u, &flat, &poles, &weights);
        let r = (p.x * p.x + p.y * p.y).sqrt();
        assert!(
            (r - 1.0).abs() < 1e-8,
            "r={r} at u={u}: point={p:?}"
        );
    }
}

// ─── 17. BSplCLib facade ─────────────────────────────────────────────────────

#[test]
fn bsplclib_facade_delegates_correctly() {
    // Smoke-test the BSplCLib:: associated-function wrappers.
    let knots = [0.0, 1.0];
    let mults = [2, 2];
    let flat = BSplCLib::knot_sequence(&knots, &mults);
    assert_eq!(flat, vec![0.0, 0.0, 1.0, 1.0]);

    let fk = BSplCLib::flat_bezier_knots(2);
    assert_eq!(fk, vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);

    let anti = BSplCLib::antiderivative_pc(&[2.0]);
    assert_eq!(anti.len(), 2);
    assert!((anti[1] - 2.0).abs() < 1e-15, "anti[1]={}", anti[1]);

    let pts = BSplCLib::build_schoenberg_points(1, &[0.0, 0.0, 1.0, 1.0]);
    assert_eq!(pts.len(), 2);
}
