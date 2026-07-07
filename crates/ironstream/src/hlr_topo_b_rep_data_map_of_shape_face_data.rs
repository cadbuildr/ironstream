// FILE: hlr_topo_b_rep_data_map_of_shape_face_data.rs
// occt: HLRTopoBRep_DataMapOfShapeFaceData

//! Deprecated: Use HashMap<ShapeId, FaceData> directly.
//! Map of face data indexed by shape.

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct FaceData {
    pub face_id: usize,
    pub visible: bool,
}

impl FaceData {
    pub fn new(face_id: usize, visible: bool) -> Self {
        FaceData { face_id, visible }
    }
}

pub type HLRTopoDataMapOfShapeFaceData = HashMap<usize, FaceData>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let mut map: HLRTopoDataMapOfShapeFaceData = HashMap::new();
        map.insert(1, FaceData::new(10, true));

        assert_eq!(map.len(), 1);
        assert_eq!(map[&1].face_id, 10);
    }

    #[test]
    fn test_map_operations() {
        let mut map: HLRTopoDataMapOfShapeFaceData = HashMap::new();
        map.insert(1, FaceData::new(10, true));
        map.insert(2, FaceData::new(20, false));

        assert_eq!(map.len(), 2);
        assert!(map[&1].visible);
        assert!(!map[&2].visible);
    }

    #[test]
    fn test_map_iteration() {
        let mut map: HLRTopoDataMapOfShapeFaceData = HashMap::new();
        map.insert(1, FaceData::new(10, true));
        map.insert(2, FaceData::new(20, false));

        let visible_count = map.values().filter(|f| f.visible).count();
        assert_eq!(visible_count, 1);
    }
}
