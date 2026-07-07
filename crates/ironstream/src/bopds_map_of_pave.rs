// FILE: bopds_map_of_pave.rs
// occt: BOPDS_MapOfPave

//! NCollection alias: Map<BOPDS_Pave>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_MapOfPave
pub type BOPDSMapOfPave = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map: BOPDSMapOfPave = Vec::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_add() {
        let mut map: BOPDSMapOfPave = Vec::new();
        map.push(1);
        map.push(2);
        assert_eq!(map.len(), 2);
        assert!(map.contains(&1));
    }
}
