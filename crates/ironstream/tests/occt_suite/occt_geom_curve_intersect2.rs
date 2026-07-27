extern crate ironstream;
use ironstream::geom_curve_intersect2::*;

#[test]
fn test_geom_curve_intersect2_basic() {
    // IntSS - surface/surface intersection
    let iss = IntSS::new("Plane1", "Plane2", 1e-7);
    assert!(iss.is_done());
    assert_eq!(iss.nb_lines(), 0);  // stub returns empty
    assert!(iss.line(0).is_none());

    // IntCS - curve/surface intersection
    let ics = IntCS::new("Line1", "Plane1", 1e-7);
    assert!(ics.is_done());
    assert_eq!(ics.nb_points(), 0);  // stub returns empty
    assert!(ics.point(0).is_none());
    assert!(ics.curve_param(0).is_none());
    assert!(ics.surface_params(0).is_none());

    // surface_surface_intersect free function
    let lines = surface_surface_intersect("S1", "S2", 1e-6);
    // stub returns empty vec
    assert_eq!(lines.len(), 0);
}
