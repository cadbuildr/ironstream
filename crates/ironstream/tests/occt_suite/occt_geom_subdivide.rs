extern crate ironstream;
use ironstream::geom_subdivide::*;

#[test]
fn occt_split_curve3d_smoke() {
    let mut sc = SplitCurve3d::new("line");
    sc.init("line", 0.0, 1.0);
    sc.set_split_values(vec![0.5]);
    sc.perform(false);
    assert_eq!(sc.nb_results(), 2);
}

#[test]
fn occt_split_surface_smoke() {
    let mut ss = SplitSurface::new("plane");
    ss.set_u_splits(vec![0.5]);
    ss.set_v_splits(vec![0.5]);
    ss.perform(false);
    assert_eq!(ss.nb_results(), 4);
}
