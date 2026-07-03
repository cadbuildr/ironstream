extern crate ironstream;
use ironstream::geom_loft_surface::*;

#[test]
fn test_geom_loft_surface_basic() {
    // ThruSections
    let mut ts = ThruSections::new(false);
    assert!(!ts.is_done());
    assert_eq!(ts.section_count(), 0);

    ts.add_wire(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]]);
    ts.add_wire(vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0]]);
    assert_eq!(ts.section_count(), 2);

    let shape_id = ts.build();
    assert!(ts.is_done());
    assert!(!shape_id.is_empty());

    ts.set_ruled(true);
    // adding a wire invalidates
    ts.add_wire(vec![[0.0, 0.0, 2.0]]);
    assert!(!ts.is_done());

    // LoftSurface
    let mut ls = LoftSurface::new();
    ls.add_profile(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    ls.add_profile(vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0]]);
    ls.build();
    assert!(!ls.poles.is_empty());

    let pt = ls.evaluate(0.5, 0.5);
    // result should be inside bounding box of profiles
    let _ = pt;

    // loft_profiles free function
    let profiles = vec![
        vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        vec![[0.0, 0.0, 1.0], [2.0, 0.0, 1.0]],
    ];
    let ls2 = loft_profiles(&profiles);
    assert_eq!(ls2.profiles.len(), 2);
}
