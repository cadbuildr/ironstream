extern crate ironstream;
use ironstream::geom_fuse::*;

#[test]
fn test_geom_fuse_basic() {
    let mut b = BoolOpBuilder::fuse("box1", "box2");
    let r = b.build();
    assert!(b.is_done());
    assert!(!r.is_empty());

    let mut c = BoolOpBuilder::cut("box1", "box2");
    let r2 = c.build();
    assert!(!r2.is_empty());

    let mut co = BoolOpBuilder::common("box1", "box2")
        .with_fuzzy(1e-5);
    let r3 = co.build();
    assert!(!r3.is_empty());

    let rf = fuse("a", "b");
    assert!(!rf.is_empty());
    let rc = cut("a", "b");
    assert!(!rc.is_empty());
    let rco = common("a", "b");
    assert!(!rco.is_empty());
}
