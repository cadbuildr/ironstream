// FILE: tests/occt_geom2d_conic.rs
extern crate ironstream;
use ironstream::geom2d_conic::{
    Circle2d, Circle3d, Ellipse2d, Geom2dBoundedCurve, Geom2dConic, Geom2dCurve,
    Geom2dGeometry, GeomBoundedCurve, GeomConic, GeomCurve, GeomGeometry, Line2d, Line3d,
};

const EPS: f64 = 1e-10;

fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}

fn approx_eq2(a: [f64; 2], b: [f64; 2], eps: f64) -> bool {
    approx_eq(a[0], b[0], eps) && approx_eq(a[1], b[1], eps)
}

fn approx_eq3(a: [f64; 3], b: [f64; 3], eps: f64) -> bool {
    approx_eq(a[0], b[0], eps) && approx_eq(a[1], b[1], eps) && approx_eq(a[2], b[2], eps)
}

// ---------------------------------------------------------------------------
// Integration: Line2d as Geom2dBoundedCurve
// ---------------------------------------------------------------------------

#[test]
fn integration_line2d_as_bounded_curve() {
    let line = Line2d::new([0.0, 0.0], [6.0, 8.0]);
    let curve: &dyn Geom2dBoundedCurve = &line;
    assert!(approx_eq2(curve.start_point(), [0.0, 0.0], EPS));
    assert!(approx_eq2(curve.end_point(), [6.0, 8.0], EPS));
    // parameter range
    assert!(approx_eq(curve.first_param(), 0.0, EPS));
    assert!(approx_eq(curve.last_param(), 10.0, EPS));
    // midpoint
    let mid = curve.value(5.0);
    assert!(approx_eq2(mid, [3.0, 4.0], EPS));
}

#[test]
fn integration_line2d_d1_derivative_magnitude_is_one() {
    let line = Line2d::new([1.0, -1.0], [1.0 + 3.0_f64.sqrt(), 0.0]);
    let curve: &dyn Geom2dCurve = &line;
    let (_, d) = curve.d1(0.5);
    let mag = (d[0] * d[0] + d[1] * d[1]).sqrt();
    assert!(approx_eq(mag, 1.0, EPS));
}

#[test]
fn integration_line2d_not_periodic_not_closed() {
    let line = Line2d::new([0.0, 0.0], [1.0, 1.0]);
    let curve: &dyn Geom2dCurve = &line;
    assert!(!curve.is_periodic());
    assert!(!curve.is_closed());
    assert!(approx_eq(curve.period(), 0.0, EPS));
}

// ---------------------------------------------------------------------------
// Integration: Circle2d as Geom2dConic
// ---------------------------------------------------------------------------

#[test]
fn integration_circle2d_unit_circle_samples() {
    let c = Circle2d::new([0.0, 0.0], 1.0);
    let conic: &dyn Geom2dConic = &c;

    // 8 equally spaced points should lie on the unit circle
    for i in 0..8 {
        let u = i as f64 * std::f64::consts::PI / 4.0;
        let p = conic.value(u);
        let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
        assert!(
            approx_eq(r, 1.0, EPS),
            "sample {}: r={}",
            i,
            r
        );
    }
}

#[test]
fn integration_circle2d_eccentricity_is_zero() {
    let c = Circle2d::new([5.0, -3.0], 7.0);
    let conic: &dyn Geom2dConic = &c;
    assert!(approx_eq(conic.eccentricity(), 0.0, EPS));
}

#[test]
fn integration_circle2d_location_matches_center() {
    let c = Circle2d::new([2.5, -1.5], 4.0);
    let conic: &dyn Geom2dConic = &c;
    assert!(approx_eq2(conic.location(), [2.5, -1.5], EPS));
}

#[test]
fn integration_circle2d_periodic() {
    let c = Circle2d::new([0.0, 0.0], 3.0);
    let curve: &dyn Geom2dCurve = &c;
    assert!(curve.is_periodic());
    assert!(curve.is_closed());
    assert!(approx_eq(curve.period(), 2.0 * std::f64::consts::PI, EPS));
}

#[test]
fn integration_circle2d_clone_box() {
    let c = Circle2d::new([0.0, 0.0], 1.0);
    let geom: &dyn Geom2dGeometry = &c;
    let _clone = geom.clone_box();
}

// ---------------------------------------------------------------------------
// Integration: Ellipse2d as Geom2dConic
// ---------------------------------------------------------------------------

#[test]
fn integration_ellipse2d_points_satisfy_equation() {
    // x^2/a^2 + y^2/b^2 = 1 for axis-aligned ellipse
    let a = 5.0_f64;
    let b = 3.0_f64;
    let e = Ellipse2d::new([0.0, 0.0], a, b, 0.0);
    let conic: &dyn Geom2dConic = &e;

    for i in 0..16 {
        let u = i as f64 * std::f64::consts::PI / 8.0;
        let p = conic.value(u);
        let lhs = (p[0] / a) * (p[0] / a) + (p[1] / b) * (p[1] / b);
        assert!(approx_eq(lhs, 1.0, EPS), "u={}: lhs={}", u, lhs);
    }
}

#[test]
fn integration_ellipse2d_eccentricity_known_value() {
    // a=5, b=4 => e = sqrt(1 - 16/25) = sqrt(9/25) = 3/5 = 0.6
    let e = Ellipse2d::new([0.0, 0.0], 5.0, 4.0, 0.0);
    let conic: &dyn Geom2dConic = &e;
    assert!(approx_eq(conic.eccentricity(), 0.6, EPS));
}

#[test]
fn integration_ellipse2d_d1_numerical_fd() {
    let e = Ellipse2d::new([0.0, 0.0], 4.0, 2.0, 0.3);
    let curve: &dyn Geom2dCurve = &e;
    let h = 1e-6;
    for i in 0..8 {
        let u = i as f64 * 0.5;
        let (_, d) = curve.d1(u);
        let p_plus = curve.value(u + h);
        let p_minus = curve.value(u - h);
        let fd = [
            (p_plus[0] - p_minus[0]) / (2.0 * h),
            (p_plus[1] - p_minus[1]) / (2.0 * h),
        ];
        assert!(
            approx_eq(d[0], fd[0], 1e-4),
            "i={} u={}: d[0]={} fd[0]={}",
            i,
            u,
            d[0],
            fd[0]
        );
        assert!(
            approx_eq(d[1], fd[1], 1e-4),
            "i={} u={}: d[1]={} fd[1]={}",
            i,
            u,
            d[1],
            fd[1]
        );
    }
}

#[test]
fn integration_ellipse2d_is_periodic_closed() {
    let e = Ellipse2d::new([1.0, 1.0], 3.0, 2.0, 0.0);
    let curve: &dyn Geom2dCurve = &e;
    assert!(curve.is_periodic());
    assert!(curve.is_closed());
}

// ---------------------------------------------------------------------------
// Integration: Line3d as GeomBoundedCurve
// ---------------------------------------------------------------------------

#[test]
fn integration_line3d_bounded_curve() {
    let l = Line3d::new([0.0, 0.0, 0.0], [0.0, 0.0, 5.0]);
    let curve: &dyn GeomBoundedCurve = &l;
    assert!(approx_eq3(curve.start_point(), [0.0, 0.0, 0.0], EPS));
    assert!(approx_eq3(curve.end_point(), [0.0, 0.0, 5.0], EPS));
    let mid = curve.value(2.5);
    assert!(approx_eq3(mid, [0.0, 0.0, 2.5], EPS));
}

#[test]
fn integration_line3d_d1_unit_direction() {
    let l = Line3d::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let curve: &dyn GeomCurve = &l;
    let (_, d) = curve.d1(0.0);
    let mag = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    assert!(approx_eq(mag, 1.0, EPS));
}

#[test]
fn integration_line3d_not_periodic() {
    let l = Line3d::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let curve: &dyn GeomCurve = &l;
    assert!(!curve.is_periodic());
}

#[test]
fn integration_line3d_clone_box() {
    let l = Line3d::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let geom: &dyn GeomGeometry = &l;
    let _clone = geom.clone_box();
}

// ---------------------------------------------------------------------------
// Integration: Circle3d as GeomConic
// ---------------------------------------------------------------------------

#[test]
fn integration_circle3d_xy_plane_samples() {
    let r = 4.0;
    let c = Circle3d::new([0.0, 0.0, 0.0], r, [0.0, 0.0, 1.0]);
    let conic: &dyn GeomConic = &c;

    for i in 0..12 {
        let u = i as f64 * std::f64::consts::PI / 6.0;
        let p = conic.value(u);
        let dist = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
        assert!(approx_eq(dist, r, EPS), "i={}: dist={}", i, dist);
        // z component should be 0 (in XY plane)
        assert!(approx_eq(p[2], 0.0, EPS), "i={}: p[2]={}", i, p[2]);
    }
}

#[test]
fn integration_circle3d_eccentricity_zero() {
    let c = Circle3d::new([1.0, 2.0, 3.0], 7.0, [0.0, 1.0, 0.0]);
    let conic: &dyn GeomConic = &c;
    assert!(approx_eq(conic.eccentricity(), 0.0, EPS));
}

#[test]
fn integration_circle3d_location() {
    let c = Circle3d::new([3.0, -2.0, 1.0], 5.0, [0.0, 0.0, 1.0]);
    let conic: &dyn GeomConic = &c;
    assert!(approx_eq3(conic.location(), [3.0, -2.0, 1.0], EPS));
}

#[test]
fn integration_circle3d_d1_tangent_orthogonal_to_radius_and_normal() {
    let c = Circle3d::new([0.0, 0.0, 0.0], 2.0, [0.0, 0.0, 1.0]);
    let conic: &dyn GeomConic = &c;

    let u = 1.3;
    let (pt, d) = conic.d1(u);
    // tangent orthogonal to radius
    let dot_r = pt[0] * d[0] + pt[1] * d[1] + pt[2] * d[2];
    assert!(approx_eq(dot_r, 0.0, EPS));
    // tangent orthogonal to normal [0,0,1]
    assert!(approx_eq(d[2], 0.0, EPS));
}

#[test]
fn integration_circle3d_tilted_plane() {
    // Circle in the XZ plane (normal = [0, 1, 0])
    let r = 3.0;
    let c = Circle3d::new([0.0, 0.0, 0.0], r, [0.0, 1.0, 0.0]);
    let conic: &dyn GeomConic = &c;

    for i in 0..8 {
        let u = i as f64 * std::f64::consts::PI / 4.0;
        let p = conic.value(u);
        // y component should be 0
        assert!(approx_eq(p[1], 0.0, EPS), "i={}: p[1]={}", i, p[1]);
        // distance from origin == radius
        let dist = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
        assert!(approx_eq(dist, r, EPS), "i={}: dist={}", i, dist);
    }
}

#[test]
fn integration_circle3d_periodic_closed() {
    let c = Circle3d::new([0.0, 0.0, 0.0], 1.0, [0.0, 0.0, 1.0]);
    let curve: &dyn GeomCurve = &c;
    assert!(curve.is_periodic());
    assert!(curve.is_closed());
    assert!(approx_eq(curve.period(), 2.0 * std::f64::consts::PI, EPS));
}

// ---------------------------------------------------------------------------
// Trait object polymorphism: heterogeneous collection
// ---------------------------------------------------------------------------

#[test]
fn integration_heterogeneous_2d_curves() {
    let curves: Vec<Box<dyn Geom2dCurve>> = vec![
        Box::new(Line2d::new([0.0, 0.0], [1.0, 0.0])),
        Box::new(Circle2d::new([0.0, 0.0], 1.0)),
        Box::new(Ellipse2d::new([0.0, 0.0], 3.0, 2.0, 0.0)),
    ];
    for curve in &curves {
        let p0 = curve.value(curve.first_param());
        // just ensure we get a valid point (no NaN/inf)
        assert!(p0[0].is_finite(), "p0.x is not finite");
        assert!(p0[1].is_finite(), "p0.y is not finite");
    }
}

#[test]
fn integration_heterogeneous_3d_curves() {
    let curves: Vec<Box<dyn GeomCurve>> = vec![
        Box::new(Line3d::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0])),
        Box::new(Circle3d::new([0.0, 0.0, 0.0], 1.0, [0.0, 0.0, 1.0])),
    ];
    for curve in &curves {
        let p0 = curve.value(curve.first_param());
        assert!(p0[0].is_finite());
        assert!(p0[1].is_finite());
        assert!(p0[2].is_finite());
    }
}
