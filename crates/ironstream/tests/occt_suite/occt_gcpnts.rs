// FILE: tests/occt_gcpnts.rs
extern crate ironstream;

use ironstream::gcpnts::{
    GCPntsAbscissaPoint, GCPntsDistribution, GCPntsTangentialDeflection,
    GCPntsUniformAbscissa, GCPntsUniformDeflection,
};

const EPS: f64 = 1e-12;

fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= EPS,
        "{label}: got {a}, expected {b}, diff={}",
        (a - b).abs()
    );
}

#[test]
fn distribution_enum_variants_are_distinct() {
    assert_ne!(GCPntsDistribution::Uniform, GCPntsDistribution::Curvature);
    assert_ne!(GCPntsDistribution::Uniform, GCPntsDistribution::Combined);
    assert_ne!(GCPntsDistribution::Curvature, GCPntsDistribution::Combined);
}

#[test]
fn distribution_enum_copy_and_debug() {
    let d = GCPntsDistribution::Curvature;
    let d2 = d;
    assert_eq!(d, d2);
    let s = format!("{:?}", d);
    assert!(s.contains("Curvature"));
}

#[test]
fn abscissa_point_is_done_and_parameter() {
    let ap = GCPntsAbscissaPoint::new(0.4, 0.1, 1e-6);
    assert!(ap.is_done());
    near(ap.parameter().unwrap(), 0.5, "abscissa u0+abscissa");
}

#[test]
fn abscissa_point_zero_abscissa() {
    let ap = GCPntsAbscissaPoint::new(0.0, 0.75, 1e-6);
    near(ap.parameter().unwrap(), 0.75, "zero abscissa => same as u0");
}

#[test]
fn uniform_abscissa_five_points_on_unit_interval() {
    let ua = GCPntsUniformAbscissa::new(0.0, 1.0, 5);
    assert!(ua.is_done());
    assert_eq!(ua.nb_points(), 5);
    near(ua.parameter(1).unwrap(), 0.00, "param[1]");
    near(ua.parameter(2).unwrap(), 0.25, "param[2]");
    near(ua.parameter(3).unwrap(), 0.50, "param[3]");
    near(ua.parameter(4).unwrap(), 0.75, "param[4]");
    near(ua.parameter(5).unwrap(), 1.00, "param[5]");
}

#[test]
fn uniform_abscissa_out_of_range_indices() {
    let ua = GCPntsUniformAbscissa::new(0.0, 1.0, 3);
    assert!(ua.parameter(0).is_none(), "index 0 is out of range (1-based)");
    assert!(ua.parameter(4).is_none(), "index 4 exceeds nb_points=3");
}

#[test]
fn uniform_abscissa_zero_points_not_done() {
    let ua = GCPntsUniformAbscissa::new(0.0, 1.0, 0);
    assert!(!ua.is_done());
    assert_eq!(ua.nb_points(), 0);
}

#[test]
fn uniform_abscissa_two_points_endpoints_only() {
    let ua = GCPntsUniformAbscissa::new(2.0, 8.0, 2);
    assert_eq!(ua.nb_points(), 2);
    near(ua.parameter(1).unwrap(), 2.0, "first");
    near(ua.parameter(2).unwrap(), 8.0, "last");
}

#[test]
fn uniform_deflection_intervals_and_params() {
    // span=1.0, deflection=0.25 => ceil(4) intervals => 5 points
    let ud = GCPntsUniformDeflection::new(0.0, 1.0, 0.25);
    assert!(ud.is_done());
    assert_eq!(ud.nb_points(), 5);
    near(ud.parameter(1).unwrap(), 0.00, "first");
    near(ud.parameter(5).unwrap(), 1.00, "last");
    near(ud.deflection(), 0.25, "deflection accessor");
}

#[test]
fn uniform_deflection_non_integer_span() {
    // span=1.0, deflection=0.3 => ceil(3.333) = 4 intervals => 5 points
    let ud = GCPntsUniformDeflection::new(0.0, 1.0, 0.3);
    assert_eq!(ud.nb_points(), 5);
    near(ud.parameter(1).unwrap(), 0.0, "first");
    near(ud.parameter(5).unwrap(), 1.0, "last");
}

#[test]
fn tangential_deflection_uses_smaller_value() {
    // angular=0.5, curvature=0.2, span=1.0 => effective=0.2, ceil(5) intervals => 6 points
    let td = GCPntsTangentialDeflection::new(0.0, 1.0, 0.5, 0.2);
    assert!(td.is_done());
    assert_eq!(td.nb_points(), 6);
    near(td.angular_deflection(), 0.5, "angular accessor");
    near(td.curvature_deflection(), 0.2, "curvature accessor");
}

#[test]
fn tangential_deflection_endpoint_coverage() {
    let td = GCPntsTangentialDeflection::new(1.0, 3.0, 0.4, 0.4);
    let n = td.nb_points();
    assert!(n >= 2);
    near(td.parameter(1).unwrap(), 1.0, "first param");
    near(td.parameter(n).unwrap(), 3.0, "last param");
}

#[test]
fn tangential_deflection_oob_index() {
    let td = GCPntsTangentialDeflection::new(0.0, 1.0, 0.5, 0.5);
    assert!(td.parameter(0).is_none());
    assert!(td.parameter(td.nb_points() + 1).is_none());
}
