// FILE: bopds_data_map_of_integer_list_of_pave_block.rs
// occt: BOPDS_DataMapOfIntegerListOfPaveBlock

//! NCollection alias: DataMap<int, BOPDS_ListOfPaveBlock>
//! Deprecated type for backward compatibility.

use std::collections::HashMap;

/// Deprecated: BOPDS_DataMapOfIntegerListOfPaveBlock
/// Use `std::collections::HashMap<i32, Vec<T>>` directly instead.
pub type BOPDSDataMapOfIntegerListOfPaveBlock = HashMap<i32, Vec<u32>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map: BOPDSDataMapOfIntegerListOfPaveBlock = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_insert_retrieve() {
        let mut map: BOPDSDataMapOfIntegerListOfPaveBlock = HashMap::new();
        let key = 42;
        let value = vec![1, 2, 3];
        map.insert(key, value.clone());
        assert_eq!(map.get(&key), Some(&value));
    }
}
