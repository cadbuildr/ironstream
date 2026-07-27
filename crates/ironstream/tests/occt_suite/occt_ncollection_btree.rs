use ironstream::ncollection_btree::*;

#[test]
fn indexed_map_add_find_dedup_find_key_find_index() {
    let mut m = NCollIndexedMap::new();
    let i1 = m.add(100);
    let i2 = m.add(200);
    let i3 = m.add(100); // duplicate
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(i3, 1); // same index
    assert_eq!(m.size(), 2);
    assert_eq!(m.find_key(1), Some(100));
    assert_eq!(m.find_key(2), Some(200));
    assert_eq!(m.find_index(200), 2);
    assert_eq!(m.find_index(999), 0);
}

#[test]
fn indexed_map_remove_last() {
    let mut m = NCollIndexedMap::new();
    m.add(1);
    m.add(2);
    m.add(3);
    m.remove_last();
    assert_eq!(m.size(), 2);
    assert!(!m.contains(3));
    assert!(m.contains(1));
    assert!(m.contains(2));
}

#[test]
fn indexed_data_map_add_update_find() {
    let mut m = NCollIndexedDataMap::new();
    m.add(10, 100);
    m.add(20, 200);
    m.add(10, 999); // update existing
    assert_eq!(m.find(10), Some(999));
    assert_eq!(m.find(20), Some(200));
    assert_eq!(m.size(), 2);
    assert_eq!(m.key_from_index(1), Some(10));
    assert_eq!(m.find_from_index(2), Some(200));
}

#[test]
fn data_map_bind_find_unbind() {
    let mut m = NCollDataMap::new();
    m.bind(1, 100);
    m.bind(2, 200);
    assert_eq!(m.find(1), Some(100));
    assert!(m.is_bound(2));
    m.unbind(1);
    assert!(!m.is_bound(1));
    assert_eq!(m.size(), 1);
}

#[test]
fn map_set_add_remove() {
    let mut m = NCollMap::new();
    assert!(m.add(5));
    assert!(!m.add(5)); // already present
    assert!(m.contains(5));
    assert_eq!(m.size(), 1);
    m.remove(5);
    assert!(!m.contains(5));
    assert!(m.is_empty());
}
