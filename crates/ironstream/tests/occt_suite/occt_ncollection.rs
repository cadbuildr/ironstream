// FILE: tests/occt_ncollection.rs
extern crate ironstream;

use ironstream::ncollection::{
    NCollectionArray1, NCollectionList, NCollectionMap, NCollectionSequence,
};

#[test]
fn list_empty_on_creation() {
    let list: NCollectionList<f64> = NCollectionList::new();
    assert!(list.is_empty());
    assert_eq!(list.size(), 0);
    assert_eq!(list.first(), None);
    assert_eq!(list.last(), None);
}

#[test]
fn list_append_prepend_order() {
    let mut list = NCollectionList::new();
    list.append(2);
    list.append(3);
    list.prepend(1);
    assert_eq!(list.size(), 3);
    assert_eq!(list.first(), Some(&1));
    assert_eq!(list.last(), Some(&3));
}

#[test]
fn list_remove_first_drains() {
    let mut list = NCollectionList::new();
    list.append(10);
    list.append(20);
    list.append(30);
    assert_eq!(list.remove_first(), Some(10));
    assert_eq!(list.remove_first(), Some(20));
    assert_eq!(list.remove_first(), Some(30));
    assert_eq!(list.remove_first(), None);
    assert!(list.is_empty());
}

#[test]
fn list_clear_resets() {
    let mut list = NCollectionList::new();
    list.append("a");
    list.append("b");
    list.clear();
    assert!(list.is_empty());
    assert_eq!(list.first(), None);
}

#[test]
fn list_iter_order() {
    let mut list = NCollectionList::new();
    for i in 0..5u32 {
        list.append(i);
    }
    let values: Vec<u32> = list.iter().copied().collect();
    assert_eq!(values, vec![0, 1, 2, 3, 4]);
}

#[test]
fn sequence_one_based_indexing() {
    let mut seq = NCollectionSequence::new();
    seq.append(100);
    seq.append(200);
    seq.append(300);
    assert_eq!(seq.length(), 3);
    assert_eq!(seq.value(1), Some(&100));
    assert_eq!(seq.value(2), Some(&200));
    assert_eq!(seq.value(3), Some(&300));
    assert_eq!(seq.value(0), None);
    assert_eq!(seq.value(4), None);
}

#[test]
fn sequence_set_value_mutates_in_place() {
    let mut seq = NCollectionSequence::new();
    seq.append(1);
    seq.append(2);
    seq.append(3);
    seq.set_value(2, 99);
    assert_eq!(seq.value(1), Some(&1));
    assert_eq!(seq.value(2), Some(&99));
    assert_eq!(seq.value(3), Some(&3));
}

#[test]
fn sequence_remove_middle() {
    let mut seq = NCollectionSequence::new();
    seq.append(10);
    seq.append(20);
    seq.append(30);
    seq.remove(2);
    assert_eq!(seq.length(), 2);
    assert_eq!(seq.value(1), Some(&10));
    assert_eq!(seq.value(2), Some(&30));
}

#[test]
fn sequence_prepend_shifts() {
    let mut seq = NCollectionSequence::new();
    seq.append(3);
    seq.prepend(2);
    seq.prepend(1);
    assert_eq!(seq.first(), Some(&1));
    assert_eq!(seq.last(), Some(&3));
    assert_eq!(seq.length(), 3);
}

#[test]
fn sequence_clear() {
    let mut seq: NCollectionSequence<i64> = NCollectionSequence::new();
    seq.append(1);
    seq.append(2);
    seq.clear();
    assert!(seq.is_empty());
    assert_eq!(seq.first(), None);
    assert_eq!(seq.last(), None);
}

#[test]
fn map_bind_new_and_replacement() {
    let mut map = NCollectionMap::new();
    assert!(map.bind("alpha", 1));
    assert!(map.bind("beta", 2));
    assert!(!map.bind("alpha", 42));
    assert_eq!(map.find(&"alpha"), Some(&42));
    assert_eq!(map.extent(), 2);
}

#[test]
fn map_contains_and_unbind() {
    let mut map: NCollectionMap<u32, u32> = NCollectionMap::new();
    map.bind(10, 100);
    assert!(map.contains(&10));
    assert!(!map.contains(&99));
    assert!(map.unbind(&10));
    assert!(!map.contains(&10));
    assert!(!map.unbind(&10));
}

#[test]
fn map_find_missing_returns_none() {
    let map: NCollectionMap<i32, i32> = NCollectionMap::new();
    assert_eq!(map.find(&0), None);
}

#[test]
fn map_clear_empties() {
    let mut map = NCollectionMap::new();
    map.bind(1, "one");
    map.bind(2, "two");
    map.clear();
    assert!(map.is_empty());
    assert_eq!(map.extent(), 0);
}

#[test]
fn array1_default_fill_and_bounds() {
    let arr: NCollectionArray1<i32> = NCollectionArray1::new(1, 5);
    assert_eq!(arr.lower(), 1);
    assert_eq!(arr.upper(), 5);
    assert_eq!(arr.size(), 5);
    for idx in 1..=5 {
        assert_eq!(arr.value(idx), Some(&0));
    }
    assert_eq!(arr.value(0), None);
    assert_eq!(arr.value(6), None);
}

#[test]
fn array1_set_and_init() {
    let mut arr: NCollectionArray1<f64> = NCollectionArray1::new(0, 3);
    arr.set_value(0, 1.0);
    arr.set_value(1, 2.0);
    arr.set_value(2, 3.0);
    arr.set_value(3, 4.0);
    assert_eq!(arr.value(2), Some(&3.0));
    arr.init(9.9);
    for idx in 0..=3 {
        assert_eq!(arr.value(idx), Some(&9.9));
    }
}
