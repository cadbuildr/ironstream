// FILE: bopds_map_of_pave_block.rs
// occt: BOPDS_MapOfPaveBlock

//! NCollection alias: Map<BOPDS_PaveBlock>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_MapOfPaveBlock
pub type BOPDSMapOfPaveBlock = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map: BOPDSMapOfPaveBlock = Vec::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_add() {
        let mut map: BOPDSMapOfPaveBlock = Vec::new();
        map.push(1);
        map.push(2);
        assert_eq!(map.len(), 2);
    }
}
