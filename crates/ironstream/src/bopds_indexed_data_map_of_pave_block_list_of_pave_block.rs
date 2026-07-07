// FILE: bopds_indexed_data_map_of_pave_block_list_of_pave_block.rs
// occt: BOPDS_IndexedDataMapOfPaveBlockListOfPaveBlock

//! NCollection alias: IndexedDataMap<BOPDS_PaveBlock, List<BOPDS_PaveBlock>>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_IndexedDataMapOfPaveBlockListOfPaveBlock
pub type BOPDSIndexedDataMapOfPaveBlockListOfPaveBlock = Vec<(u32, Vec<u32>)>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexed_map_creation() {
        let map: BOPDSIndexedDataMapOfPaveBlockListOfPaveBlock = Vec::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_indexed_map_insert() {
        let mut map: BOPDSIndexedDataMapOfPaveBlockListOfPaveBlock = Vec::new();
        map.push((1, vec![2, 3, 4]));
        assert_eq!(map.len(), 1);
    }
}
