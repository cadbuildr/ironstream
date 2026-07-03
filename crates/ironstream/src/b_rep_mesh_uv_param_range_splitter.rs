// FILE: b_rep_mesh_uv_param_range_splitter.rs
// occt: BRepMesh_UVParamRangeSplitter

use std::collections::BTreeMap;

/// Intended to generate internal mesh nodes using UV parameters of boundary discrete points.
pub struct UVParamRangeSplitter {
    u_params: BTreeMap<i32, f64>,
    v_params: BTreeMap<i32, f64>,
}

impl UVParamRangeSplitter {
    /// Constructor.
    pub fn new() -> Self {
        UVParamRangeSplitter {
            u_params: BTreeMap::new(),
            v_params: BTreeMap::new(),
        }
    }

    /// Resets this splitter.
    pub fn reset(&mut self) {
        self.u_params.clear();
        self.v_params.clear();
    }

    /// Returns U parameters.
    pub fn get_parameters_u(&self) -> &BTreeMap<i32, f64> {
        &self.u_params
    }

    /// Returns U parameters (mutable).
    pub fn get_parameters_u_mut(&mut self) -> &mut BTreeMap<i32, f64> {
        &mut self.u_params
    }

    /// Returns V parameters.
    pub fn get_parameters_v(&self) -> &BTreeMap<i32, f64> {
        &self.v_params
    }

    /// Returns V parameters (mutable).
    pub fn get_parameters_v_mut(&mut self) -> &mut BTreeMap<i32, f64> {
        &mut self.v_params
    }

    /// Adds a U parameter.
    pub fn add_u_parameter(&mut self, key: i32, value: f64) {
        self.u_params.insert(key, value);
    }

    /// Adds a V parameter.
    pub fn add_v_parameter(&mut self, key: i32, value: f64) {
        self.v_params.insert(key, value);
    }
}

impl Default for UVParamRangeSplitter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uv_param_range_splitter_new() {
        let splitter = UVParamRangeSplitter::new();
        assert!(splitter.get_parameters_u().is_empty());
        assert!(splitter.get_parameters_v().is_empty());
    }

    #[test]
    fn test_uv_param_range_splitter_add_parameters() {
        let mut splitter = UVParamRangeSplitter::new();
        splitter.add_u_parameter(0, 0.5);
        splitter.add_v_parameter(0, 0.7);

        assert_eq!(splitter.get_parameters_u().len(), 1);
        assert_eq!(splitter.get_parameters_v().len(), 1);
        assert_eq!(splitter.get_parameters_u()[&0], 0.5);
        assert_eq!(splitter.get_parameters_v()[&0], 0.7);
    }

    #[test]
    fn test_uv_param_range_splitter_reset() {
        let mut splitter = UVParamRangeSplitter::new();
        splitter.add_u_parameter(0, 0.5);
        splitter.add_v_parameter(0, 0.7);
        splitter.reset();

        assert!(splitter.get_parameters_u().is_empty());
        assert!(splitter.get_parameters_v().is_empty());
    }

    #[test]
    fn test_uv_param_range_splitter_mutable_access() {
        let mut splitter = UVParamRangeSplitter::new();
        splitter.get_parameters_u_mut().insert(1, 0.3);
        splitter.get_parameters_v_mut().insert(1, 0.6);

        assert_eq!(splitter.get_parameters_u()[&1], 0.3);
        assert_eq!(splitter.get_parameters_v()[&1], 0.6);
    }
}
