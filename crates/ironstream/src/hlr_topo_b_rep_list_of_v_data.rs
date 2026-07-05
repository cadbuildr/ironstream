// FILE: hlr_topo_b_rep_list_of_v_data.rs
// occt: HLRTopoBRep_ListOfVData

//! Deprecated: Use Vec<VData> directly.
//! List of vertex data for topology.

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

pub type HLRTopoListOfVData = Vec<VData>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_creation() {
        let mut list: HLRTopoListOfVData = Vec::new();
        list.push(VData::new(1, 0.5));

        assert_eq!(list.len(), 1);
        assert_eq!(list[0].vertex_id, 1);
        assert_eq!(list[0].param, 0.5);
    }

    #[test]
    fn test_list_operations() {
        let list = vec![
            VData::new(1, 0.0),
            VData::new(2, 0.5),
            VData::new(3, 1.0),
        ];

        assert_eq!(list.len(), 3);
        assert_eq!(list[1].param, 0.5);
    }

    #[test]
    fn test_list_iteration() {
        let list = vec![
            VData::new(10, 0.1),
            VData::new(20, 0.2),
        ];

        let params: Vec<f64> = list.iter().map(|v| v.param).collect();
        assert_eq!(params.len(), 2);
        assert_eq!(params[0], 0.1);
    }
}
