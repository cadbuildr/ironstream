extern crate ironstream;
use ironstream::topo_seq::*;

#[test]
fn test_list_of_shape_operations() {
    let mut l = ListOfShape::new();
    l.append(ShapeHandle::new(1, 3));
    l.append(ShapeHandle::new(2, 3));
    l.prepend(ShapeHandle::new(0, 3));
    assert_eq!(l.size(), 3);
    assert_eq!(l.first().unwrap().id, 0);
    assert_eq!(l.last().unwrap().id, 2);
    l.remove_first();
    assert_eq!(l.size(), 2);
}

#[test]
fn test_sequence_of_shape() {
    let mut s = SequenceOfShape::new();
    s.append(ShapeHandle::new(10, 1));
    s.append(ShapeHandle::new(20, 1));
    s.append(ShapeHandle::new(30, 1));
    assert_eq!(s.length(), 3);
    s.exchange(0, 2);
    assert_eq!(s.value(0).unwrap().id, 30);
    s.remove(0);
    assert_eq!(s.length(), 2);
}

#[test]
fn test_map_of_shape_uniqueness() {
    let mut m = MapOfShape::new();
    let s = ShapeHandle::new(1, 0);
    assert!(m.add(s.clone()));
    assert!(!m.add(s.clone())); // duplicate
    assert_eq!(m.size(), 1);
    assert!(m.contains(&s));
    m.remove(&s);
    assert!(m.is_empty());
}

#[test]
fn test_indexed_map_of_shape() {
    let mut m = IndexedMapOfShape::new();
    let i0 = m.add(ShapeHandle::new(5, 3));
    let i1 = m.add(ShapeHandle::new(6, 3));
    let i0b = m.add(ShapeHandle::new(5, 3)); // duplicate
    assert_eq!(i0, i0b);
    assert_ne!(i0, i1);
    assert_eq!(m.extent(), 2);
    assert_eq!(m.find_key(i0).unwrap().id, 5);
    assert!(m.find_index(&ShapeHandle::new(6, 3)).is_some());
}

#[test]
fn test_data_map_of_shape_shape() {
    let mut m = DataMapOfShapeShape::new();
    let k = ShapeHandle::new(1, 3);
    let v = ShapeHandle::new(2, 4);
    m.bind(k.clone(), v.clone());
    assert!(m.is_bound(&k));
    assert_eq!(m.find(&k).unwrap().id, 2);
    m.unbind(&k);
    assert!(!m.is_bound(&k));
}
