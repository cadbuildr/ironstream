extern crate ironstream;
use ironstream::geom_revolve_tool::*;
use std::f64::consts::PI;

#[test]
fn occt_make_revol_smoke() {
    let t = RevolTool::new("profile_0", [0.0, 0.0, 1.0], 2.0 * PI);
    assert!(t.is_done());
    let result = t.build();
    assert!(!result.is_empty());
}

#[test]
fn occt_feature_revol_smoke() {
    let f = FeatRevol::new("top_face", [0.0, 1.0, 0.0]);
    assert!(f.is_done());
    let result = f.build();
    assert!(!result.is_empty());
}
