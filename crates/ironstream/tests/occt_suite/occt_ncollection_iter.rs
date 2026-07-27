// Integration tests for ncollection_iter module
use ironstream::ncollection_iter::*;

#[test]
fn indexed_map_add_returns_one_based_index_and_deduplicates() {
    let mut m: IndexedMap<String> = IndexedMap::new();
    let i1 = m.add("alpha".to_string());
    let i2 = m.add("beta".to_string());
    let i3 = m.add("alpha".to_string()); // duplicate
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(i3, 1); // returns existing index
    assert_eq!(m.size(), 2);
    assert_eq!(m.find_index(&"beta".to_string()), 2);
    assert_eq!(m.find_key(1), Some(&"alpha".to_string()));
    assert!(m.contains(&"alpha".to_string()));
}

#[test]
fn indexed_map_remove_last() {
    let mut m: IndexedMap<u32> = IndexedMap::new();
    m.add(10);
    m.add(20);
    assert_eq!(m.size(), 2);
    m.remove_last();
    assert_eq!(m.size(), 1);
    assert!(!m.contains(&20));
    assert!(m.contains(&10));
}

#[test]
fn indexed_data_map_add_find_and_update() {
    let mut m: IndexedDataMap<String, f64> = IndexedDataMap::new();
    let i1 = m.add("x".to_string(), 1.5);
    let i2 = m.add("y".to_string(), 2.5);
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(m.find_from_key(&"x".to_string()), Some(&1.5));
    assert_eq!(m.find_from_index(2), Some(&2.5));
    // Duplicate key updates value
    m.add("x".to_string(), 9.9);
    assert_eq!(m.size(), 2); // still 2 entries
    assert_eq!(m.find_from_key(&"x".to_string()), Some(&9.9));
}

#[test]
fn indexed_data_map_change_from_index() {
    let mut m: IndexedDataMap<u32, String> = IndexedDataMap::new();
    m.add(1, "first".to_string());
    if let Some(v) = m.change_from_index(1) {
        *v = "modified".to_string();
    }
    assert_eq!(m.find_from_index(1), Some(&"modified".to_string()));
}

#[test]
fn ncollection_list_append_prepend_remove_first() {
    let mut l: NCollectionList<i32> = NCollectionList::new();
    l.append(10);
    l.append(20);
    l.prepend(5);
    assert_eq!(l.size(), 3);
    assert_eq!(l.first(), Some(&5));
    assert_eq!(l.last(), Some(&20));
    l.remove_first();
    assert_eq!(l.first(), Some(&10));
    assert_eq!(l.size(), 2);
}

#[test]
fn ncollection_sequence_one_based_access_and_remove() {
    let mut s: NCollectionSequence<f64> = NCollectionSequence::new();
    s.append(1.0);
    s.append(2.0);
    s.append(3.0);
    assert_eq!(s.length(), 3);
    assert_eq!(s.value(1), Some(&1.0));
    assert_eq!(s.value(3), Some(&3.0));
    assert_eq!(s.value(0), None); // 0 is out of range
    s.remove(2); // remove middle
    assert_eq!(s.length(), 2);
    assert_eq!(s.value(2), Some(&3.0)); // 3.0 shifted to index 2
}
