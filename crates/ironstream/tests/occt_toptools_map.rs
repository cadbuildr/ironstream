use ironstream::toptools_map::*;

#[test]
fn map_of_shape_add_remove_contains() {
    let mut m = TopToolsMapOfShape::new();
    assert!(m.add(1));
    assert!(m.add(2));
    assert!(!m.add(1)); // duplicate
    assert_eq!(m.extent(), 2);
    assert!(m.contains(1));
    m.remove(1);
    assert!(!m.contains(1));
    assert_eq!(m.extent(), 1);
}

#[test]
fn map_of_shape_unite_intersect() {
    let mut a = TopToolsMapOfShape::new();
    let mut b = TopToolsMapOfShape::new();
    a.add(1); a.add(2); a.add(3);
    b.add(2); b.add(4);
    a.unite(&b);
    assert_eq!(a.extent(), 4);
    // a = {1,2,3,4}, b = {2,4} → intersect gives {2,4}
    a.intersect(&b);
    assert_eq!(a.extent(), 2);
    assert!(a.contains(2));
    assert!(a.contains(4));
    assert!(!a.contains(1));
}

#[test]
fn data_map_of_shape_bind_find_unbind() {
    let mut m = TopToolsDataMapOfShapeShape::new();
    m.bind(1, 100);
    m.bind(2, 200);
    assert_eq!(m.find(1), Some(100));
    assert!(m.is_bound(2));
    m.unbind(1);
    assert!(!m.is_bound(1));
    assert_eq!(m.extent(), 1);
}

#[test]
fn indexed_map_of_shape_add_find_key_find_index() {
    let mut m = TopToolsIndexedMapOfShape::new();
    let i1 = m.add(10);
    let i2 = m.add(20);
    let i3 = m.add(10); // duplicate
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(i3, 1);
    assert_eq!(m.find_key(1), Some(10));
    assert_eq!(m.find_index(20), 2);
    assert_eq!(m.find_index(999), 0);
}

#[test]
fn list_of_shape_append_prepend_first_last_remove_first() {
    let mut l = TopToolsListOfShape::new();
    l.append(1);
    l.append(2);
    l.prepend(0);
    assert_eq!(l.extent(), 3);
    assert_eq!(l.first(), Some(0));
    assert_eq!(l.last(), Some(2));
    let removed = l.remove_first();
    assert_eq!(removed, Some(0));
    assert_eq!(l.extent(), 2);
    assert_eq!(l.first(), Some(1));
}
