extern crate ironstream;
use ironstream::geom_merge::*;

#[test]
fn occt_comp_curve_smoke() {
    let mut cc = CompCurveToBSpline::new(1);
    cc.add("seg_a", true);
    assert!(cc.is_done());
}

#[test]
fn occt_merge_algo_smoke() {
    let mut m = MergeAlgo::new();
    m.add_shape("face1");
    let r = m.perform();
    assert!(r.contains("face1"));
    assert!(m.is_done());
}
