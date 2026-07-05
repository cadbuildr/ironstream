// FILE: hlr_topo_b_rep_map_of_shape_list_of_v_data.rs
// occt: HLRTopoBRep_MapOfShapeListOfVData

//! Deprecated: Use HashMap<ShapeId, Vec<VData>> directly.
//! Map of vertex data lists indexed by shape.

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct VData {
    pub vertex_id: usize,
    pub param: f64,
}

impl VData {
    pub fn new(vertex_id: usize, param: f64) -> Self {
        VData { vertex_id, param }
    }
}

pub type HLRTopoMapOfShapeListOfVData = HashMap<usize, Vec<VData>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let mut map: HLRTopoMapOfShapeListOfVData = HashMap::new();
        let mut vertices = Vec::new();
        vertices.push(VData::new(1, 0.5));
        map.insert(1, vertices);

        assert_eq!(map.len(), 1);
        assert_eq!(map[&1].len(), 1);
    }

    #[test]
    fn test_map_operations() {
        let mut map: HLRTopoMapOfShapeListOfVData = HashMap::new();

        let mut v1 = Vec::new();
        v1.push(VData::new(1, 0.0));
        v1.push(VData::new(2, 0.5));
        map.insert(1, v1);

        let mut v2 = Vec::new();
        v2.push(VData::new(3, 1.0));
        map.insert(2, v2);

        assert_eq!(map.len(), 2);
        assert_eq!(map[&1].len(), 2);
        assert_eq!(map[&2].len(), 1);
    }

    #[test]
    fn test_map_iteration() {
        let mut map: HLRTopoMapOfShapeListOfVData = HashMap::new();

        let mut vdata = Vec::new();
        vdata.push(VData::new(10, 0.1));
        vdata.push(VData::new(20, 0.2));
        map.insert(1, vdata);

        let total_vertices: usize = map.values().map(|list| list.len()).sum();
        assert_eq!(total_vertices, 2);
    }
}
