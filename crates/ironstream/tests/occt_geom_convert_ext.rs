extern crate ironstream;

use ironstream::geom_convert_ext::{ApproxSurface, BSplineSurfaceToBezier, CompBezierToBSpline};

#[test]
fn comp_bezier_empty() {
    let cb = CompBezierToBSpline::new(3);
    assert!(!cb.is_done());
    assert_eq!(cb.nb_poles(), 0);
}

#[test]
fn comp_bezier_single_cubic_arc() {
    let mut cb = CompBezierToBSpline::new(3);
    cb.add_arc(vec![[0.0,0.0,0.0],[1.0,1.0,0.0],[2.0,1.0,0.0],[3.0,0.0,0.0]]);
    assert!(cb.is_done());
    assert_eq!(cb.nb_poles(), 4);
    assert_eq!(cb.poles().len(), 4);
}

#[test]
fn comp_bezier_poles_no_duplicate_junction() {
    let mut cb = CompBezierToBSpline::new(2);
    cb.add_arc(vec![[0.0,0.0,0.0],[1.0,1.0,0.0],[2.0,0.0,0.0]]);
    cb.add_arc(vec![[2.0,0.0,0.0],[3.0,-1.0,0.0],[4.0,0.0,0.0]]);
    // 2 arcs of degree 2 => 2*2+1 = 5 merged poles
    assert_eq!(cb.poles().len(), 5);
}

#[test]
fn approx_surface_done_and_error() {
    let mut ap = ApproxSurface::new(3, 3, 1, 1, 1e-4, 10, 10);
    assert!(!ap.is_done());
    ap.perform();
    assert!(ap.is_done());
    assert!(ap.max_error() < ap.tol);
}

#[test]
fn bspline_to_bezier_patch_count() {
    let b = BSplineSurfaceToBezier::new(3, 3, 2, 3);
    assert_eq!(b.nb_patches(), 6);
}
