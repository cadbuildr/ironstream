// FILE: bopds_indexed_data_map_of_shape_couple_of_pave_blocks.rs
// occt: BOPDS_IndexedDataMapOfShapeCoupleOfPaveBlocks

//! NCollection alias: IndexedDataMap<TopoDS_Shape, BOPDS_CoupleOfPaveBlocks>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_IndexedDataMapOfShapeCoupleOfPaveBlocks
pub type BOPDSIndexedDataMapOfShapeCoupleOfPaveBlocks = Vec<(u32, (u32, u32))>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexed_map_creation() {
        let map: BOPDSIndexedDataMapOfShapeCoupleOfPaveBlocks = Vec::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_indexed_map_insert() {
        let mut map: BOPDSIndexedDataMapOfShapeCoupleOfPaveBlocks = Vec::new();
        map.push((1, (2, 3)));
        assert_eq!(map.len(), 1);
        assert_eq!(map[0], (1, (2, 3)));
    }
}
