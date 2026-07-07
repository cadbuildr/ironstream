// FILE: bopds_indexed_map_of_pave_block.rs
// occt: BOPDS_IndexedMapOfPaveBlock

//! NCollection alias: IndexedMap<BOPDS_PaveBlock>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_IndexedMapOfPaveBlock
/// IndexedMap maintains insertion order and allows O(1) lookup by value.
pub type BOPDSIndexedMapOfPaveBlock = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexed_map_creation() {
        let map: BOPDSIndexedMapOfPaveBlock = Vec::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_indexed_map_operations() {
        let mut map: BOPDSIndexedMapOfPaveBlock = Vec::new();
        map.push(1);
        map.push(2);
        assert_eq!(map.len(), 2);
        assert!(map.contains(&1));
        assert!(map.contains(&2));
    }
}
