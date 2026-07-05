// FILE: bopds_map_of_common_block.rs
// occt: BOPDS_MapOfCommonBlock

//! NCollection alias: Map<BOPDS_CommonBlock>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_MapOfCommonBlock
pub type BOPDSMapOfCommonBlock = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map: BOPDSMapOfCommonBlock = Vec::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_add() {
        let mut map: BOPDSMapOfCommonBlock = Vec::new();
        map.push(1);
        map.push(2);
        assert_eq!(map.len(), 2);
        assert!(map.contains(&1));
    }
}
