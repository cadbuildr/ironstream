extern crate ironstream;
use ironstream::geom_map_tools::*;

#[test]
fn test_geom_map_tools_basic() {
    // IndexedDataMap
    let mut map = IndexedDataMap::new();
    let idx1 = map.add("shapeA", vec!["shapeB".to_string(), "shapeC".to_string()]);
    let idx2 = map.add("shapeB", vec!["shapeA".to_string()]);
    assert_eq!(idx1, 1);
    assert_eq!(idx2, 2);

    // find_from_key
    let v = map.find_from_key("shapeA").unwrap();
    assert_eq!(v[0], "shapeB");

    // find_from_index (1-based)
    let v2 = map.find_from_index(2).unwrap();
    assert_eq!(v2[0], "shapeA");

    // find_key
    let k = map.find_key(1).unwrap();
    assert_eq!(k, "shapeA");

    // overwrite existing key
    let idx_again = map.add("shapeA", vec!["shapeD".to_string()]);
    assert_eq!(idx_again, 1);
    assert_eq!(map.find_from_key("shapeA").unwrap()[0], "shapeD");

    // DataMap
    let mut dm: DataMap<String, i32> = DataMap::new();
    dm.bind("k1".to_string(), 42);
    assert!(dm.is_bound(&"k1".to_string()));
    assert_eq!(dm.find(&"k1".to_string()), Some(&42));
    assert_eq!(dm.size(), 1);
    dm.unbind(&"k1".to_string());
    assert!(!dm.is_bound(&"k1".to_string()));
}
