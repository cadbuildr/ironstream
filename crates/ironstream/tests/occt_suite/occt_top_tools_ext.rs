// FILE: tests/occt_top_tools_ext.rs
extern crate ironstream;

use ironstream::top_tools_ext::{
    ListOfShape, SequenceOfShape, DataMapOfShapeShape, DataMapOfShapeListOfShape,
};

#[test]
fn list_of_shape_prepend_append_contains() {
    let mut l = ListOfShape::new();
    assert!(l.is_empty());
    l.append(10);
    l.append(20);
    l.prepend(5);
    assert_eq!(l.size(), 3);
    assert_eq!(l.first(), Some(5));
    assert_eq!(l.last(), Some(20));
    assert!(l.contains(10));
    assert!(!l.contains(99));
}

#[test]
fn list_of_shape_remove_first_and_clear() {
    let mut l = ListOfShape::new();
    l.append(1);
    l.append(2);
    l.append(3);
    l.remove_first();
    assert_eq!(l.first(), Some(2));
    assert_eq!(l.size(), 2);
    l.clear();
    assert!(l.is_empty());
}

#[test]
fn list_of_shape_append_list() {
    let mut l1 = ListOfShape::new();
    l1.append(1);
    let mut l2 = ListOfShape::new();
    l2.append(2);
    l2.append(3);
    l1.append_list(&l2);
    assert_eq!(l1.size(), 3);
    assert_eq!(l1.as_slice(), &[1, 2, 3]);
}

#[test]
fn sequence_of_shape_one_based_access() {
    let mut s = SequenceOfShape::new();
    s.append(100);
    s.append(200);
    s.append(300);
    assert_eq!(s.length(), 3);
    assert_eq!(s.value(1), Some(100));
    assert_eq!(s.value(3), Some(300));
    assert_eq!(s.value(0), None);
    assert_eq!(s.value(4), None);
    s.set_value(2, 999);
    assert_eq!(s.value(2), Some(999));
    s.remove(1);
    assert_eq!(s.length(), 2);
    assert_eq!(s.first(), Some(999));
}

#[test]
fn data_map_of_shape_shape_bind_find_unbind() {
    let mut m = DataMapOfShapeShape::new();
    assert!(m.is_empty());
    m.bind(1, 101);
    m.bind(2, 202);
    assert_eq!(m.size(), 2);
    assert_eq!(m.find(1), Some(101));
    assert!(m.is_bound(2));
    assert!(!m.is_bound(99));
    assert!(m.unbind(1));
    assert!(!m.is_bound(1));
    m.clear();
    assert!(m.is_empty());
}

#[test]
fn data_map_of_shape_list_of_shape() {
    let mut m = DataMapOfShapeListOfShape::new();
    m.add_to(1, 10);
    m.add_to(1, 20);
    m.bind(2, vec![30, 40, 50]);
    assert_eq!(m.size(), 2);
    assert_eq!(m.find(1).unwrap().len(), 2);
    assert_eq!(m.find(2).unwrap(), &[30, 40, 50]);
    assert!(m.is_bound(1));
    assert!(m.unbind(2));
    assert!(!m.is_bound(2));
}
