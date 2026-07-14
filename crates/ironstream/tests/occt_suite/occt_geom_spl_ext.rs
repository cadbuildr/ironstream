// FILE: tests/occt_geom_spl_ext.rs
extern crate ironstream;
use ironstream::geom_spl_ext::*;

#[test]
fn bspline_to_bezier_curve_arcs() {
    // degree=3, nb_knots=4 → 4 arcs, each with degree+1=4 poles
    let c = GeomConvertBSplineToBezierCurve::new(1, 3, 4);
    assert_eq!(c.nb_arcs(), 4);
    assert_eq!(c.nb_poles_per_arc(), 4);
    let poles = c.arc_poles(0).unwrap();
    assert_eq!(poles.len(), 4);
}

#[test]
fn arc_poles_out_of_bounds() {
    let c = GeomConvertBSplineToBezierCurve::new(1, 3, 4);
    // Arc index 4 is out of range (0..4)
    assert!(c.arc_poles(4).is_none());
}

#[test]
fn bspline_to_bezier_surface_patches() {
    // 4 u-patches × 2 v-patches = 8 total
    let s = GeomConvertBSplineToBezierSurface::new(2, 3, 3, 4, 2);
    assert_eq!(s.nb_patches(), 8);
    assert!(s.patch_id(0, 0).is_some());
    assert!(s.patch_id(4, 0).is_none());
}

#[test]
fn comp_curve_build() {
    let mut c = GeomConvertCompCurveToBSpline::new(1e-6);
    c.add(10, true);
    c.add(20, true);
    assert_eq!(c.nb_curves(), 2);
    assert!(!c.is_done());
    c.build();
    assert!(c.is_done());
    assert!(c.result_id().is_some());
}

#[test]
fn approx_surface_perform() {
    let mut a = GeomConvertApproxSurface::new(5);
    a.perform();
    assert!(a.is_done());
    // result_id = surface_id + 30000 = 5 + 30000 = 30005
    assert_eq!(a.result_id(), Some(30005));
}
