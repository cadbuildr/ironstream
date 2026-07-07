// FILE: bopds_data_map_of_pave_block_common_block.rs
// occt: BOPDS_DataMapOfPaveBlockCommonBlock

//! NCollection alias: DataMap<BOPDS_PaveBlock, BOPDS_CommonBlock>
//! Deprecated type for backward compatibility.

use std::collections::HashMap;

/// Deprecated: BOPDS_DataMapOfPaveBlockCommonBlock
pub type BOPDSDataMapOfPaveBlockCommonBlock = HashMap<u32, u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map: BOPDSDataMapOfPaveBlockCommonBlock = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_insert() {
        let mut map: BOPDSDataMapOfPaveBlockCommonBlock = HashMap::new();
        map.insert(1, 2);
        assert_eq!(map.get(&1), Some(&2));
    }
}
