use ironstream::brep_locs::*;

#[test]
fn displacement_identity_is_identity_and_apply() {
    let d = BrepLocsDisplacement::identity();
    assert!(d.is_identity());
    let p = [3.0, 5.0, 7.0];
    let q = d.apply(p);
    assert!((q[0] - 3.0).abs() < 1e-14);
    assert!((q[1] - 5.0).abs() < 1e-14);
    assert!((q[2] - 7.0).abs() < 1e-14);
}

#[test]
fn displacement_from_translation_apply() {
    let d = BrepLocsDisplacement::from_translation(10.0, -3.0, 2.0);
    assert!(!d.is_identity());
    let p = d.apply([0.0, 0.0, 0.0]);
    assert!((p[0] - 10.0).abs() < 1e-14);
    assert!((p[1] + 3.0).abs() < 1e-14);
    assert!((p[2] - 2.0).abs() < 1e-14);
    assert_eq!(d.translation(), [10.0, -3.0, 2.0]);
}

#[test]
fn displacement_rotation_z_quarter_turn() {
    use std::f64::consts::PI;
    let d = BrepLocsDisplacement::from_rotation_z(PI / 2.0);
    // [1,0,0] -> [0,1,0]
    let p = d.apply([1.0, 0.0, 0.0]);
    assert!(p[0].abs() < 1e-10);
    assert!((p[1] - 1.0).abs() < 1e-10);
    assert!(p[2].abs() < 1e-14);
}

#[test]
fn displacement_compose() {
    let t1 = BrepLocsDisplacement::from_translation(3.0, 0.0, 0.0);
    let t2 = BrepLocsDisplacement::from_translation(0.0, 4.0, 0.0);
    let t = t1.compose(&t2);
    let p = t.apply([0.0, 0.0, 0.0]);
    assert!((p[0] - 3.0).abs() < 1e-14);
    assert!((p[1] - 4.0).abs() < 1e-14);
}

#[test]
fn brep_locs_stack_push_and_composed() {
    let mut s = BrepLocsStack::new();
    assert!(s.is_empty());
    s.push(BrepLocsDisplacement::from_translation(1.0, 0.0, 0.0));
    s.push(BrepLocsDisplacement::from_translation(0.0, 2.0, 0.0));
    assert_eq!(s.depth(), 2);
    let c = s.composed();
    let p = c.apply([0.0, 0.0, 0.0]);
    assert!((p[0] - 1.0).abs() < 1e-14);
    assert!((p[1] - 2.0).abs() < 1e-14);
}

#[test]
fn brep_gprop_surface_perform_is_done_mass() {
    let mut g = BrepGPropSurface::new(8);
    assert!(!g.is_done());
    g.perform(1e-6);
    assert!(g.is_done());
    assert!((g.mass() - 8.0).abs() < 1e-10);
    let com = g.centre_of_mass();
    assert_eq!(com, [0.0, 0.0, 0.0]);
}
