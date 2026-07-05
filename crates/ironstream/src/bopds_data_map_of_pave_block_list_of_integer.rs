// FILE: bopds_data_map_of_pave_block_list_of_integer.rs
// occt: BOPDS_DataMapOfPaveBlockListOfInteger

//! NCollection alias: DataMap<BOPDS_PaveBlock, List<int>>
//! Deprecated type for backward compatibility.

use std::collections::HashMap;

/// Deprecated: BOPDS_DataMapOfPaveBlockListOfInteger
pub type BOPDSDataMapOfPaveBlockListOfInteger = HashMap<u32, Vec<i32>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map: BOPDSDataMapOfPaveBlockListOfInteger = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_insert_retrieve() {
        let mut map: BOPDSDataMapOfPaveBlockListOfInteger = HashMap::new();
        map.insert(1, vec![10, 20, 30]);
        assert_eq!(map.get(&1), Some(&vec![10, 20, 30]));
    }
}
