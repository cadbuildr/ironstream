use ironstream::ais_group::*;

#[test]
fn group_add_remove_contains_nb_objects() {
    let mut g = AisGroup::new("Parts");
    g.add(10);
    g.add(20);
    g.add(10); // duplicate — should be ignored
    assert_eq!(g.nb_objects(), 2);
    assert!(g.contains(10));
    assert!(g.contains(20));
    g.remove(10);
    assert!(!g.contains(10));
    assert_eq!(g.nb_objects(), 1);
}

#[test]
fn group_display_erase() {
    let mut g = AisGroup::new("G");
    assert!(g.is_displayed);
    g.erase();
    assert!(!g.is_displayed);
    g.display();
    assert!(g.is_displayed);
}

#[test]
fn group_displayed_ids() {
    let mut g = AisGroup::new("G");
    g.add(1);
    g.add(2);
    let ids = g.displayed_ids();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&1));
    assert!(ids.contains(&2));
}

#[test]
fn multiple_connected_connect_and_instance_transform_translation() {
    let mut mc = AisMultipleConnectedInteractive::new(100);
    let i1 = mc.connect([5.0, 0.0, 0.0]);
    let i2 = mc.connect([0.0, 3.0, 0.0]);
    assert_eq!(mc.nb_instances(), 2);
    let t1 = mc.instance_transform(i1).unwrap();
    assert!((t1[3] - 5.0).abs() < 1e-10);  // translation X in col 3, row 0
    let t2 = mc.instance_transform(i2).unwrap();
    assert!((t2[7] - 3.0).abs() < 1e-10);  // translation Y in col 3, row 1
}

#[test]
fn multiple_connected_remove_instance() {
    let mut mc = AisMultipleConnectedInteractive::new(5);
    let i1 = mc.connect([1.0, 0.0, 0.0]);
    let _i2 = mc.connect([2.0, 0.0, 0.0]);
    mc.remove_instance(i1);
    assert_eq!(mc.nb_instances(), 1);
    assert!(mc.instance_transform(i1).is_none());
}

#[test]
fn group_member_with_transform() {
    let mut g = AisGroup::new("G");
    let m = [1.0,0.0,0.0,0.0, 0.0,1.0,0.0,0.0, 0.0,0.0,1.0,0.0, 0.0,0.0,0.0,1.0];
    g.add_with_transform(42, m);
    assert!(g.contains(42));
    assert!(g.members[0].has_transform());
}
