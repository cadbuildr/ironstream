extern crate ironstream;
use ironstream::topo_map::*;

#[test]
fn map_of_shape_add_and_contains() {
    let mut m = MapOfShape::new();
    let added = m.add(ShapeKey { id: 42, shape_type: 1 });
    assert!(added);
    assert!(m.contains(42));
    assert_eq!(m.extent(), 1);
}

#[test]
fn map_of_shape_no_duplicate() {
    let mut m = MapOfShape::new();
    m.add(ShapeKey { id: 7, shape_type: 0 });
    let second = m.add(ShapeKey { id: 7, shape_type: 0 });
    assert!(!second);
    assert_eq!(m.extent(), 1);
}

#[test]
fn indexed_map_add_and_find() {
    let mut m = IndexedMapOfShape::new();
    let i1 = m.add(ShapeKey { id: 10, shape_type: 1 });
    let i2 = m.add(ShapeKey { id: 20, shape_type: 2 });
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(m.find_index(10), Some(1));
    assert_eq!(m.find_key(2).unwrap().id, 20);
}

#[test]
fn datamap_shape_shape_bind_find() {
    let mut d = DataMapOfShapeShape::new();
    d.bind(1, 100);
    d.bind(2, 200);
    assert_eq!(d.find(1), Some(100));
    assert!(d.is_bound(2));
    assert!(!d.is_bound(99));
    d.unbind(1);
    assert!(!d.is_bound(1));
    assert_eq!(d.extent(), 1);
}

#[test]
fn datamap_shape_list_of_shape_append() {
    let mut d = DataMapOfShapeListOfShape::new();
    d.append(5, 10);
    d.append(5, 20);
    d.append(5, 30);
    let list = d.find(5).unwrap();
    assert_eq!(list.len(), 3);
    assert_eq!(list[0], 10);
    assert_eq!(d.extent(), 1);
}
