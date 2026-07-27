// FILE: tests/occt_ncollection_extra.rs
extern crate ironstream;

use ironstream::ncollection_extra::{
    NcArray2, NcIndexedDataMap, NcMap, NcSequence, NcSparseArray, NcUBTree, UBTreeNode,
};

#[test]
fn nc_sequence_append_prepend_exchange() {
    let mut seq: NcSequence<i32> = NcSequence::new();
    seq.append(10);
    seq.append(20);
    seq.prepend(5);
    assert_eq!(seq.length(), 3);
    assert_eq!(seq.value(0), Some(&5));
    assert_eq!(seq.first(), Some(&5));
    assert_eq!(seq.last(), Some(&20));
    seq.exchange(0, 2);
    assert_eq!(seq.value(0), Some(&20));
    assert_eq!(seq.value(2), Some(&5));
}

#[test]
fn nc_array2_set_value_and_value() {
    let mut arr: NcArray2<f64> = NcArray2::new(1, 3, 1, 3);
    assert_eq!(arr.nb_rows(), 3);
    assert_eq!(arr.nb_cols(), 3);
    arr.set_value(2, 2, 99.0);
    assert!((arr.value(2, 2).unwrap() - 99.0).abs() < 1e-10);
    // out of bounds returns None
    assert!(arr.value(0, 0).is_none());
    assert!(arr.value(4, 4).is_none());
}

#[test]
fn nc_map_intersection() {
    let mut m1: NcMap<i32> = NcMap::new();
    m1.add(1); m1.add(2); m1.add(3);
    let mut m2: NcMap<i32> = NcMap::new();
    m2.add(2); m2.add(3); m2.add(4);
    let inter = m1.intersection(&m2);
    assert_eq!(inter.extent(), 2);
    assert!(inter.contains(&2));
    assert!(inter.contains(&3));
    assert!(!inter.contains(&1));
}

#[test]
fn nc_sparse_array_has_value_min_index() {
    let mut arr: NcSparseArray<String> = NcSparseArray::new();
    arr.set_value(100, "hello".to_string());
    arr.set_value(9999, "world".to_string());
    assert!(arr.has_value(100));
    assert!(!arr.has_value(50));
    assert_eq!(arr.extent(), 2);
    assert_eq!(arr.min_index(), Some(100));
    assert_eq!(arr.max_index(), Some(9999));
    arr.unset_value(100);
    assert!(!arr.has_value(100));
}

#[test]
fn nc_ubtree_select() {
    let mut tree = NcUBTree::new();
    tree.add(UBTreeNode::new(0, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]));
    tree.add(UBTreeNode::new(1, [5.0, 5.0, 5.0], [6.0, 6.0, 6.0]));
    tree.add(UBTreeNode::new(2, [0.5, 0.5, 0.5], [2.0, 2.0, 2.0]));

    // select overlapping with [0.5, 0.5, 0.5] to [1.5, 1.5, 1.5]
    let hits = tree.select([0.5, 0.5, 0.5], [1.5, 1.5, 1.5]);
    assert!(hits.contains(&0), "node 0 should be selected");
    assert!(hits.contains(&2), "node 2 should be selected");
    assert!(!hits.contains(&1), "node 1 should not be selected");
}
