extern crate ironstream;
use ironstream::select3d_filter::*;

#[test]
fn occt_type_filter_smoke() {
    let f = TypeFilter::new(3);
    assert!(f.is_ok(3));
    assert!(!f.is_ok(2));
}

#[test]
fn occt_and_filter_smoke() {
    let mut af = AndFilter::new();
    af.add(Box::new(TypeFilter::new(0)));
    assert!(af.is_ok(0));
    assert!(!af.is_ok(1));
}
