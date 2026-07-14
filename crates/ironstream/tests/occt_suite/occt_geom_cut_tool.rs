extern crate ironstream;
use ironstream::geom_cut_tool::*;

#[test]
fn occt_cut_smoke() {
    let a = vec![[0.0, 0.0, 0.0], [5.0, 0.0, 0.0]];
    let b = vec![[3.0, -1.0, -1.0], [6.0, 1.0, 1.0]];
    let mut c = CutOperation::new(a, b);
    c.build();
    assert!(c.is_done());
}

#[test]
fn occt_fuse_smoke() {
    let a = vec![[0.0, 0.0, 0.0]];
    let b = vec![[10.0, 0.0, 0.0]];
    let mut f = FuseOperation::new(a, b);
    f.build();
    assert_eq!(f.result_pts.len(), 2);
}
