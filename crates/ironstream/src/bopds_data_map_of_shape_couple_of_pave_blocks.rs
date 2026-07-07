// FILE: bopds_data_map_of_shape_couple_of_pave_blocks.rs
// occt: BOPDS_DataMapOfShapeCoupleOfPaveBlocks

//! NCollection alias: DataMap<TopoDS_Shape, BOPDS_CoupleOfPaveBlocks>
//! Deprecated type for backward compatibility.

use std::collections::HashMap;

/// Deprecated: BOPDS_DataMapOfShapeCoupleOfPaveBlocks
pub type BOPDSDataMapOfShapeCoupleOfPaveBlocks = HashMap<u32, (u32, u32)>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map: BOPDSDataMapOfShapeCoupleOfPaveBlocks = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_insert() {
        let mut map: BOPDSDataMapOfShapeCoupleOfPaveBlocks = HashMap::new();
        map.insert(1, (2, 3));
        assert_eq!(map.get(&1), Some(&(2, 3)));
    }
}
