// FILE: bopds_data_map_of_pave_block_list_of_pave_block.rs
// occt: BOPDS_DataMapOfPaveBlockListOfPaveBlock

//! NCollection alias: DataMap<BOPDS_PaveBlock, List<BOPDS_PaveBlock>>
//! Deprecated type for backward compatibility.

use std::collections::HashMap;

/// Deprecated: BOPDS_DataMapOfPaveBlockListOfPaveBlock
pub type BOPDSDataMapOfPaveBlockListOfPaveBlock = HashMap<u32, Vec<u32>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map: BOPDSDataMapOfPaveBlockListOfPaveBlock = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_insert() {
        let mut map: BOPDSDataMapOfPaveBlockListOfPaveBlock = HashMap::new();
        map.insert(1, vec![2, 3, 4]);
        assert_eq!(map.get(&1), Some(&vec![2, 3, 4]));
    }
}
