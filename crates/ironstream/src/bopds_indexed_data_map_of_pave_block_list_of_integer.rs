// FILE: bopds_indexed_data_map_of_pave_block_list_of_integer.rs
// occt: BOPDS_IndexedDataMapOfPaveBlockListOfInteger

//! NCollection alias: IndexedDataMap<BOPDS_PaveBlock, List<int>>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_IndexedDataMapOfPaveBlockListOfInteger
/// IndexedDataMap is similar to HashMap but maintains insertion order.
pub type BOPDSIndexedDataMapOfPaveBlockListOfInteger = Vec<(u32, Vec<i32>)>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexed_map_creation() {
        let map: BOPDSIndexedDataMapOfPaveBlockListOfInteger = Vec::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_indexed_map_insert() {
        let mut map: BOPDSIndexedDataMapOfPaveBlockListOfInteger = Vec::new();
        map.push((1, vec![10, 20]));
        assert_eq!(map.len(), 1);
        assert_eq!(map[0], (1, vec![10, 20]));
    }
}
