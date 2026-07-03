extern crate ironstream;
use ironstream::tobj_model::*;

#[test]
fn test_model_create_and_find_object() {
    let mut m = TObjModel::new("Assembly");
    let id = m.create_object("Bolt");
    assert_eq!(m.nb_objects(), 1);
    let obj = m.find_object("Bolt").unwrap();
    assert_eq!(obj.id, id);
    assert_eq!(obj.name, "Bolt");
}

#[test]
fn test_model_remove_object() {
    let mut m = TObjModel::new("M");
    let id = m.create_object("Part");
    assert!(m.remove_object(id));
    assert_eq!(m.nb_objects(), 0);
    assert!(!m.names.is_bound("Part"));
}

#[test]
fn test_tobj_object_children_and_references() {
    let mut obj = TObjObject::new(1, "Assembly");
    obj.add_child(10);
    obj.add_child(11);
    assert_eq!(obj.nb_children(), 2);
    obj.add_reference(20);
    assert!(obj.has_reference(20));
    obj.remove_reference(20);
    assert!(!obj.has_reference(20));
}

#[test]
fn test_tobj_partition_add_remove_contains() {
    let mut p = TObjPartition::new(1, "Group");
    p.add_object(5);
    p.add_object(6);
    p.add_object(7);
    assert_eq!(p.nb_objects(), 3);
    assert!(p.contains(6));
    p.remove_object(6);
    assert!(!p.contains(6));
    assert_eq!(p.nb_objects(), 2);
}

#[test]
fn test_name_container_bind_find_unbind() {
    let mut nc = TNameContainer::new();
    nc.bind("Wheel", 100);
    nc.bind("Axle", 200);
    assert_eq!(nc.find("Wheel"), Some(100));
    assert!(nc.is_bound("Axle"));
    assert!(nc.unbind("Axle"));
    assert!(!nc.is_bound("Axle"));
    assert_eq!(nc.nb_names(), 1);
}

#[test]
fn test_tobj_object_iterator_counts_live_objects() {
    let mut m = TObjModel::new("M");
    m.create_object("A");
    m.create_object("B");
    m.create_object("C");
    let mut it = TObjObjectIterator::new(&m, None);
    let mut count = 0;
    while it.more() {
        count += 1;
        it.next();
    }
    assert_eq!(count, 3);
}
