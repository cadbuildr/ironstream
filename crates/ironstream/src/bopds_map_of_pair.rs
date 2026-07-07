// FILE: bopds_map_of_pair.rs
// occt: BOPDS_MapOfPair

//! NCollection alias: Map<BOPDS_Pair>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_MapOfPair
pub type BOPDSMapOfPair = Vec<(i32, i32)>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map: BOPDSMapOfPair = Vec::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_add() {
        let mut map: BOPDSMapOfPair = Vec::new();
        map.push((1, 2));
        assert_eq!(map.len(), 1);
    }
}
