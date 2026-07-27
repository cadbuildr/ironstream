extern crate ironstream;
use ironstream::toptools_ext::*;

#[test]
fn test_shape_ref_type_name() {
    let face = ShapeRef::new(1, 3);
    assert_eq!(face.shape_type_name(), "Face");
    let edge = ShapeRef::new(2, 5);
    assert_eq!(edge.shape_type_name(), "Edge");
    assert!(!face.is_null());
}

#[test]
fn test_indexed_map_of_shape_add_and_find() {
    let mut m = IndexedMapOfShape::new();
    let s1 = ShapeRef::new(10, 3);
    let s2 = ShapeRef::new(20, 5);
    let i1 = m.add(s1.clone());
    let i2 = m.add(s2.clone());
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(m.find_index(&s1), Some(1));
    assert_eq!(m.find_from_index(2), Some(&s2));
    // Adding a duplicate must return the existing index
    assert_eq!(m.add(s1.clone()), 1);
    assert_eq!(m.extent(), 2);
}

#[test]
fn test_list_of_shape_append_remove_first() {
    let mut l = ListOfShape::new();
    l.append(ShapeRef::new(1, 3));
    l.append(ShapeRef::new(2, 5));
    l.prepend(ShapeRef::new(0, 6));
    assert_eq!(l.size(), 3);
    let removed = l.remove_first().unwrap();
    assert_eq!(removed.id, 0);
    assert_eq!(l.size(), 2);
}

#[test]
fn test_map_of_shape_set_operations() {
    let mut a = MapOfShape::new();
    let mut b = MapOfShape::new();
    a.add(ShapeRef::new(1, 3));
    a.add(ShapeRef::new(2, 5));
    b.add(ShapeRef::new(2, 5));
    b.add(ShapeRef::new(3, 6));
    let inter = a.intersection(&b);
    assert_eq!(inter.extent(), 1);
    let union = a.union(&b);
    assert_eq!(union.extent(), 3);
    let diff = a.difference(&b);
    assert_eq!(diff.extent(), 1);
}

#[test]
fn test_data_map_shape_shape_bind_find_unbind() {
    let mut m = DataMapOfShapeShape::new();
    let k = ShapeRef::new(1, 3);
    let v = ShapeRef::new(2, 3);
    m.bind(k.clone(), v.clone());
    assert_eq!(m.find(&k), Some(&v));
    assert!(m.is_bound(&k));
    assert!(m.unbind(&k));
    assert!(!m.is_bound(&k));
}

#[test]
fn test_indexed_data_map_of_shape_list() {
    let mut m = IndexedDataMapOfShapeListOfShape::new();
    let face = ShapeRef::new(1, 3);
    let mut edges = ListOfShape::new();
    edges.append(ShapeRef::new(10, 5));
    edges.append(ShapeRef::new(11, 5));
    let idx = m.add(face.clone(), edges);
    assert_eq!(idx, 1);
    assert_eq!(m.extent(), 1);
    let list = m.find_from_key(&face).unwrap();
    assert_eq!(list.size(), 2);
    let key_back = m.key(1).unwrap();
    assert_eq!(key_back.id, 1);
}
