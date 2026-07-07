// FILE: mesh_vs_data_map_of_integer_boolean.rs
// occt: MeshVS_DataMapOfIntegerBoolean, MeshVS_DataMapIteratorOfDataMapOfIntegerBoolean

use std::collections::HashMap;

/// Deprecated typedef alias for backward compatibility.
/// A data map from integer to boolean values.
///
/// Original OCCT: `NCollection_DataMap<int, bool>`
pub type MeshVsDataMapOfIntegerBoolean = HashMap<i32, bool>;

/// Deprecated typedef alias for the iterator.
/// Original OCCT: `NCollection_DataMap<int, bool>::Iterator`
pub type MeshVsDataMapIteratorOfDataMapOfIntegerBoolean =
    std::collections::hash_map::IntoIter<i32, bool>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_map_creation() {
        let map: MeshVsDataMapOfIntegerBoolean = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_data_map_insert_and_retrieve() {
        let mut map: MeshVsDataMapOfIntegerBoolean = HashMap::new();
        map.insert(1, true);
        map.insert(2, false);
        map.insert(3, true);

        assert_eq!(map.get(&1), Some(&true));
        assert_eq!(map.get(&2), Some(&false));
        assert_eq!(map.get(&3), Some(&true));
        assert_eq!(map.get(&4), None);
    }

    #[test]
    fn test_data_map_size() {
        let mut map: MeshVsDataMapOfIntegerBoolean = HashMap::new();
        assert_eq!(map.len(), 0);

        map.insert(10, true);
        map.insert(20, false);
        assert_eq!(map.len(), 2);

        map.remove(&10);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_data_map_iteration() {
        let mut map: MeshVsDataMapOfIntegerBoolean = HashMap::new();
        map.insert(1, true);
        map.insert(2, false);
        map.insert(3, true);

        let mut collected: Vec<(i32, bool)> = map.into_iter().collect();
        collected.sort_by_key(|(k, _)| *k);

        assert_eq!(collected, vec![(1, true), (2, false), (3, true)]);
    }
}
