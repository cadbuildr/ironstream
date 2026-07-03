extern crate ironstream;
use ironstream::geom_approx_ext::*;

#[test]
fn fit_and_divide_poles_knots() {
    let pts = vec![[0.0,0.0,0.0],[1.0,1.0,0.0],[2.0,0.0,0.0],[3.0,1.0,0.0]];
    let mut f = FitAndDivide::new(pts, 3, 1e-5);
    f.set_max_segments(8);
    f.perform();
    assert!(f.is_done());
    assert_eq!(f.nb_poles(), 4);
    assert_eq!(f.nb_knots(), 4);
    assert!((f.error()).abs() < 1e-10);
}

#[test]
fn same_parameter_arc_length_reparameterization() {
    let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[3.0,0.0,0.0]];
    let s = SameParameter::new(pts, 1e-6);
    assert!(s.is_done());
    assert!((s.reparameterized[0]).abs() < 1e-10);
    assert!((s.reparameterized[2] - 1.0).abs() < 1e-10);
}

#[test]
fn curve_on_surface_lift_uv_to_3d() {
    let uv = vec![[1.0, 2.0], [3.0, 4.0]];
    let c = CurveOnSurface::new(uv, 1);
    assert!(c.is_done());
    assert_eq!(c.nb_poles(), 2);
    assert!((c.result_pts[0][0] - 1.0).abs() < 1e-10);
    assert!((c.result_pts[1][1] - 4.0).abs() < 1e-10);
    assert!(c.max_error_3d() < 1e-8);
}

#[test]
fn parameterization_uniform() {
    let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0],[3.0,0.0,0.0]];
    let params = ParametrizationType::Uniform.compute_params(&pts);
    assert_eq!(params.len(), 4);
    assert!((params[0]).abs() < 1e-10);
    assert!((params[3] - 1.0).abs() < 1e-10);
}

#[test]
fn parameterization_chord_length() {
    let pts = vec![[0.0,0.0,0.0],[2.0,0.0,0.0],[4.0,0.0,0.0]];
    let params = ParametrizationType::ChordLength.compute_params(&pts);
    assert!((params[0]).abs() < 1e-10);
    assert!((params[1] - 0.5).abs() < 1e-10);
    assert!((params[2] - 1.0).abs() < 1e-10);
}
