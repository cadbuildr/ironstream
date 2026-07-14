extern crate ironstream;
use ironstream::topo_ds_ext::*;

#[test]
fn test_topo_ds_ext_basic() {
    let iter = TopoDsIterator::new("compound_1");
    assert!(iter.initialized());

    let mut list = ListOfShape::new();
    assert!(list.is_empty());

    list.append("shape_a");
    list.append("shape_b");
    assert_eq!(list.size(), 2);
    assert!(!list.is_empty());

    list.prepend("shape_z");
    assert_eq!(list.first().map(|s| s.as_str()), Some("shape_z"));
    assert_eq!(list.last().map(|s| s.as_str()), Some("shape_b"));

    list.remove_first();
    assert_eq!(list.size(), 2);

    let subs = collect_sub_shapes("root_shape");
    assert!(!subs.is_empty());
    assert_eq!(subs[0], "root_shape");
}
