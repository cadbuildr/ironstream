// FILE: int_tools_surface_range_localize_data.rs
// occt: IntTools_SurfaceRangeLocalizeData

use super::int_tools_surface_range_sample::IntToolsSurfaceRangeSample;
use std::collections::{HashMap, HashSet};

/// Localized data for surface range (2D bounding boxes, grid points, etc.).
#[derive(Clone, Debug)]
pub struct IntToolsSurfaceRangeLocalizeData {
    nb_sample_u: i32,
    nb_sample_v: i32,
    min_range_u: f64,
    min_range_v: f64,
    map_range_out: HashSet<i32>,
    map_box: HashMap<i32, (f64, f64, f64, f64)>,
    u_params: Vec<f64>,
    v_params: Vec<f64>,
    deflection: f64,
}

impl IntToolsSurfaceRangeLocalizeData {
    /// Create an empty surface range localize data.
    pub fn new() -> Self {
        IntToolsSurfaceRangeLocalizeData {
            nb_sample_u: 0,
            nb_sample_v: 0,
            min_range_u: 0.0,
            min_range_v: 0.0,
            map_range_out: HashSet::new(),
            map_box: HashMap::new(),
            u_params: Vec::new(),
            v_params: Vec::new(),
            deflection: 0.0,
        }
    }

    /// Create with sample counts and min ranges.
    pub fn with_params(nb_u: i32, nb_v: i32, min_u: f64, min_v: f64) -> Self {
        IntToolsSurfaceRangeLocalizeData {
            nb_sample_u: nb_u,
            nb_sample_v: nb_v,
            min_range_u: min_u,
            min_range_v: min_v,
            map_range_out: HashSet::new(),
            map_box: HashMap::new(),
            u_params: Vec::new(),
            v_params: Vec::new(),
            deflection: 0.0,
        }
    }

    /// Get number of U samples.
    pub fn get_nb_sample_u(&self) -> i32 {
        self.nb_sample_u
    }

    /// Get number of V samples.
    pub fn get_nb_sample_v(&self) -> i32 {
        self.nb_sample_v
    }

    /// Get minimum U range.
    pub fn get_min_range_u(&self) -> f64 {
        self.min_range_u
    }

    /// Get minimum V range.
    pub fn get_min_range_v(&self) -> f64 {
        self.min_range_v
    }

    /// Add an out-range.
    pub fn add_out_range(&mut self, sample: &IntToolsSurfaceRangeSample) {
        let hash = (sample.get_index_u() << 16) | sample.get_index_v();
        self.map_range_out.insert(hash);
    }

    /// Add a bounding box.
    pub fn add_box(&mut self, sample: &IntToolsSurfaceRangeSample, bbox: (f64, f64, f64, f64)) {
        let hash = (sample.get_index_u() << 16) | sample.get_index_v();
        self.map_box.insert(hash, bbox);
    }

    /// Find a bounding box.
    pub fn find_box(&self, sample: &IntToolsSurfaceRangeSample) -> Option<(f64, f64, f64, f64)> {
        let hash = (sample.get_index_u() << 16) | sample.get_index_v();
        self.map_box.get(&hash).copied()
    }

    /// Check if range is out.
    pub fn is_range_out(&self, sample: &IntToolsSurfaceRangeSample) -> bool {
        let hash = (sample.get_index_u() << 16) | sample.get_index_v();
        self.map_range_out.contains(&hash)
    }

    /// Set grid deflection.
    pub fn set_grid_deflection(&mut self, deflection: f64) {
        self.deflection = deflection;
    }

    /// Get grid deflection.
    pub fn get_grid_deflection(&self) -> f64 {
        self.deflection
    }

    /// Set range U grid size.
    pub fn set_range_u_grid(&mut self, nb_u: i32) {
        self.u_params.resize(nb_u as usize, 0.0);
    }

    /// Get range U grid size.
    pub fn get_range_u_grid(&self) -> i32 {
        self.u_params.len() as i32
    }

    /// Set U parameter at index.
    pub fn set_u_param(&mut self, index: i32, param: f64) {
        if index >= 0 && (index as usize) < self.u_params.len() {
            self.u_params[index as usize] = param;
        }
    }

    /// Get U parameter at index.
    pub fn get_u_param(&self, index: i32) -> f64 {
        if index >= 0 && (index as usize) < self.u_params.len() {
            self.u_params[index as usize]
        } else {
            0.0
        }
    }

    /// Set range V grid size.
    pub fn set_range_v_grid(&mut self, nb_v: i32) {
        self.v_params.resize(nb_v as usize, 0.0);
    }

    /// Get range V grid size.
    pub fn get_range_v_grid(&self) -> i32 {
        self.v_params.len() as i32
    }

    /// Set V parameter at index.
    pub fn set_v_param(&mut self, index: i32, param: f64) {
        if index >= 0 && (index as usize) < self.v_params.len() {
            self.v_params[index as usize] = param;
        }
    }

    /// Get V parameter at index.
    pub fn get_v_param(&self, index: i32) -> f64 {
        if index >= 0 && (index as usize) < self.v_params.len() {
            self.v_params[index as usize]
        } else {
            0.0
        }
    }
}

impl Default for IntToolsSurfaceRangeLocalizeData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = IntToolsSurfaceRangeLocalizeData::new();
        assert_eq!(data.get_nb_sample_u(), 0);
        assert_eq!(data.get_nb_sample_v(), 0);
    }

    #[test]
    fn test_with_params() {
        let data = IntToolsSurfaceRangeLocalizeData::with_params(5, 5, 0.01, 0.01);
        assert_eq!(data.get_nb_sample_u(), 5);
        assert_eq!(data.get_nb_sample_v(), 5);
    }

    #[test]
    fn test_grid_deflection() {
        let mut data = IntToolsSurfaceRangeLocalizeData::new();
        data.set_grid_deflection(0.001);
        assert_eq!(data.get_grid_deflection(), 0.001);
    }
}
