extern crate ironstream;
use ironstream::topo_shape_iter::*;

#[test]
fn test_topo_shape_iter_basic() {
    // IndexedMapOfShape
    let mut map = IndexedMapOfShape::new();
    assert_eq!(map.size(), 0);

    let i1 = map.add("shape_a");
    assert_eq!(i1, 1);
    let i2 = map.add("shape_b");
    assert_eq!(i2, 2);

    // idempotent add
    let i1b = map.add("shape_a");
    assert_eq!(i1b, 1);
    assert_eq!(map.size(), 2);

    // find_index
    assert_eq!(map.find_index("shape_a"), Some(1));
    assert_eq!(map.find_index("no_such"), None);

    // find_key (1-based)
    assert_eq!(map.find_key(1), Some(&"shape_a".to_string()));
    assert_eq!(map.find_key(999), None);

    // contains
    assert!(map.contains("shape_b"));
    assert!(!map.contains("shape_z"));

    // clear
    map.clear();
    assert_eq!(map.size(), 0);

    // MapOfShape
    let mut mset = MapOfShape::new();
    assert!(mset.add("s1"));
    assert!(!mset.add("s1")); // duplicate
    assert!(mset.contains("s1"));
    assert_eq!(mset.size(), 1);
    mset.clear();
    assert_eq!(mset.size(), 0);

    // map_shapes_and_ancestors
    let result = map_shapes_and_ancestors("root_shape");
    assert_eq!(result.find_index("root_shape"), Some(1));
}
