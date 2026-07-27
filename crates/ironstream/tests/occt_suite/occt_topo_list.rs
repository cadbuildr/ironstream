use ironstream::topo_list::*;
use ironstream::brep_tool::{TopoShape, TopAbsShapeEnum, TopAbsOrientation};

fn make_shape(id: u32) -> TopoShape {
    TopoShape::new(id, TopAbsShapeEnum::Edge)
}

#[test]
fn topo_list_of_shape_append_prepend_size_first_remove_by_id() {
    let mut l = TopoListOfShape::new();
    assert!(l.is_empty());

    l.append(make_shape(1));
    l.append(make_shape(2));
    l.prepend(make_shape(0));
    assert_eq!(l.size(), 3);
    assert_eq!(l.first().unwrap().shape_id, 0);
    assert_eq!(l.last().unwrap().shape_id, 2);

    // remove_first removes index 0
    l.remove_first();
    assert_eq!(l.size(), 2);
    assert_eq!(l.first().unwrap().shape_id, 1);

    // remove by id
    assert!(l.remove(1));
    assert!(!l.remove(99)); // not found
    assert_eq!(l.size(), 1);
    assert_eq!(l.first().unwrap().shape_id, 2);
}

#[test]
fn topo_seq_of_shape_1_based_value_and_remove() {
    let mut s = TopoSeqOfShape::new();
    s.append(make_shape(10));
    s.append(make_shape(20));
    s.append(make_shape(30));
    assert_eq!(s.length(), 3);

    // 1-based access
    assert_eq!(s.value(1).unwrap().shape_id, 10);
    assert_eq!(s.value(2).unwrap().shape_id, 20);
    assert_eq!(s.value(3).unwrap().shape_id, 30);
    assert!(s.value(0).is_none()); // 0 is out of range

    // remove index 2 (value=20); 30 shifts to index 2
    s.remove(2);
    assert_eq!(s.length(), 2);
    assert_eq!(s.value(1).unwrap().shape_id, 10);
    assert_eq!(s.value(2).unwrap().shape_id, 30);
}

#[test]
fn data_map_shape_shape_bind_find_unbind() {
    let mut m = DataMapShapeShape::new();
    assert!(m.is_empty());

    m.bind(1, make_shape(10));
    m.bind(2, make_shape(20));
    assert!(m.is_bound(1));
    assert!(m.is_bound(2));
    assert!(!m.is_bound(99));
    assert_eq!(m.extent(), 2);

    assert_eq!(m.find(1).unwrap().shape_id, 10);
    assert_eq!(m.find(2).unwrap().shape_id, 20);
    assert!(m.find(99).is_none());

    assert!(m.unbind(1));
    assert!(!m.is_bound(1));
    assert_eq!(m.extent(), 1);

    // double unbind => false
    assert!(!m.unbind(1));
}

#[test]
fn data_map_shape_int_find_and_change_find_or_insert() {
    let mut m = DataMapShapeInt::new();

    m.bind(5, 42);
    assert_eq!(m.find(5), Some(42));
    assert_eq!(m.find(99), None);

    // existing key: returns stored value, does not overwrite
    let v = m.change_find_or_insert(5, 0);
    assert_eq!(v, 42);

    // new key: inserts default
    let v2 = m.change_find_or_insert(6, 99);
    assert_eq!(v2, 99);
    assert_eq!(m.find(6), Some(99));

    assert!(m.unbind(5));
    assert!(!m.is_bound(5));
    assert_eq!(m.extent(), 1);
}

#[test]
fn map_of_oriented_shape_dedup() {
    let mut m = MapOfOrientedShape::new();
    assert!(m.is_empty());

    // first add succeeds
    assert!(m.add(1, TopAbsOrientation::Forward));
    // duplicate add returns false
    assert!(!m.add(1, TopAbsOrientation::Forward));
    // same id, different orientation => allowed
    assert!(m.add(1, TopAbsOrientation::Reversed));
    // different id => allowed
    assert!(m.add(2, TopAbsOrientation::Forward));

    assert_eq!(m.extent(), 3);
    assert!(m.contains(1, TopAbsOrientation::Forward));
    assert!(m.contains(1, TopAbsOrientation::Reversed));
    assert!(m.contains(2, TopAbsOrientation::Forward));
    assert!(!m.contains(2, TopAbsOrientation::Reversed));

    m.clear();
    assert!(m.is_empty());
}
