use ironstream::topo_solid::*;

#[test]
fn shell_new_add_face_set_closed() {
    let mut s = TopoShell::new(1);
    assert!(!s.is_null());
    s.add_face(10);
    s.add_face(20);
    assert_eq!(s.nb_faces(), 2);
    s.set_closed(true);
    assert!(s.is_closed());
}

#[test]
fn solid_add_shell_volume_hint() {
    let mut s = TopoSolid::new(1);
    assert!(!s.is_null());
    s.add_shell(5);
    s.set_volume_hint(1234.5);
    assert_eq!(s.nb_shells(), 1);
    assert!((s.volume_hint() - 1234.5).abs() < 1e-6);
}

#[test]
fn comp_solid_add_solid() {
    let mut cs = TopoCompSolid::new(1);
    assert!(!cs.is_null());
    cs.add_solid(10);
    cs.add_solid(20);
    assert_eq!(cs.nb_solids(), 2);
}

#[test]
fn compound_add_remove_dedup() {
    let mut c = TopoCompound::new(1);
    assert!(!c.is_null());
    c.add(10);
    c.add(20);
    c.add(10); // duplicate — should be ignored
    assert_eq!(c.nb_children(), 2);
    c.remove(10);
    assert_eq!(c.nb_children(), 1);
}

#[test]
fn compound_default_is_null() {
    let c = TopoCompound::default();
    assert!(c.is_null());
    assert!(c.is_empty());
}
