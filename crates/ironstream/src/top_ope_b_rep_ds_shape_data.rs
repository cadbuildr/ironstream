// FILE: top_ope_b_rep_ds_shape_data.rs
// occt: TopOpeBRepDS_ShapeData

use crate::top_ope_b_rep_ds_config::Config;

/// Shape data: contains interference list and domain information
#[derive(Debug, Clone)]
pub struct ShapeData {
    /// List of interference indices
    interferences: Vec<usize>,
    /// Same domain shape indices
    same_domain: Vec<usize>,
    /// Same domain reference index
    same_domain_ref: i32,
    /// Same domain orientation
    same_domain_ori: Config,
    /// Same domain index
    same_domain_ind: i32,
    /// Orientation (0=Forward, 1=Reversed, 2=Internal, 3=External)
    orientation: u8,
    /// Orientation defined flag
    orientation_def: bool,
    /// Ancestor rank
    ancestor_rank: i32,
    /// Keep flag
    keep: bool,
}

impl ShapeData {
    /// Create new ShapeData
    pub fn new() -> Self {
        ShapeData {
            interferences: Vec::new(),
            same_domain: Vec::new(),
            same_domain_ref: -1,
            same_domain_ori: Config::UnshGeometry,
            same_domain_ind: -1,
            orientation: 0,
            orientation_def: false,
            ancestor_rank: 0,
            keep: false,
        }
    }

    /// Get interferences (read-only)
    pub fn interferences(&self) -> &[usize] {
        &self.interferences
    }

    /// Get mutable interferences
    pub fn interferences_mut(&mut self) -> &mut Vec<usize> {
        &mut self.interferences
    }

    /// Add an interference
    pub fn add_interference(&mut self, i: usize) {
        self.interferences.push(i);
    }

    /// Check keep flag
    pub fn keep(&self) -> bool {
        self.keep
    }

    /// Set keep flag
    pub fn set_keep(&mut self, b: bool) {
        self.keep = b;
    }

    /// Get same domain shapes
    pub fn same_domain(&self) -> &[usize] {
        &self.same_domain
    }

    /// Get mutable same domain shapes
    pub fn same_domain_mut(&mut self) -> &mut Vec<usize> {
        &mut self.same_domain
    }

    /// Add same domain shape
    pub fn add_same_domain(&mut self, shape_id: usize) {
        self.same_domain.push(shape_id);
    }

    /// Get same domain reference
    pub fn same_domain_ref(&self) -> i32 {
        self.same_domain_ref
    }

    /// Set same domain reference
    pub fn set_same_domain_ref(&mut self, ref_idx: i32) {
        self.same_domain_ref = ref_idx;
    }

    /// Get same domain orientation
    pub fn same_domain_ori(&self) -> Config {
        self.same_domain_ori
    }

    /// Set same domain orientation
    pub fn set_same_domain_ori(&mut self, ori: Config) {
        self.same_domain_ori = ori;
    }

    /// Get same domain index
    pub fn same_domain_ind(&self) -> i32 {
        self.same_domain_ind
    }

    /// Set same domain index
    pub fn set_same_domain_ind(&mut self, ind: i32) {
        self.same_domain_ind = ind;
    }

    /// Get orientation
    pub fn orientation(&self) -> u8 {
        self.orientation
    }

    /// Set orientation
    pub fn set_orientation(&mut self, ori: u8) {
        self.orientation = ori;
    }

    /// Check if orientation is defined
    pub fn orientation_def(&self) -> bool {
        self.orientation_def
    }

    /// Set orientation defined flag
    pub fn set_orientation_def(&mut self, b: bool) {
        self.orientation_def = b;
    }

    /// Get ancestor rank
    pub fn ancestor_rank(&self) -> i32 {
        self.ancestor_rank
    }

    /// Set ancestor rank
    pub fn set_ancestor_rank(&mut self, rank: i32) {
        self.ancestor_rank = rank;
    }
}

impl Default for ShapeData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_data_new() {
        let sd = ShapeData::new();
        assert_eq!(sd.interferences().len(), 0);
        assert!(!sd.keep());
        assert_eq!(sd.ancestor_rank(), 0);
    }

    #[test]
    fn test_interferences() {
        let mut sd = ShapeData::new();
        sd.add_interference(1);
        sd.add_interference(2);
        assert_eq!(sd.interferences().len(), 2);
        assert_eq!(sd.interferences()[0], 1);
        assert_eq!(sd.interferences()[1], 2);
    }

    #[test]
    fn test_keep_flag() {
        let mut sd = ShapeData::new();
        assert!(!sd.keep());
        sd.set_keep(true);
        assert!(sd.keep());
    }

    #[test]
    fn test_same_domain() {
        let mut sd = ShapeData::new();
        sd.add_same_domain(5);
        sd.add_same_domain(10);
        assert_eq!(sd.same_domain().len(), 2);
        assert_eq!(sd.same_domain()[0], 5);
    }

    #[test]
    fn test_same_domain_ref_ori_ind() {
        let mut sd = ShapeData::new();
        sd.set_same_domain_ref(7);
        sd.set_same_domain_ori(Config::SameOriented);
        sd.set_same_domain_ind(3);
        assert_eq!(sd.same_domain_ref(), 7);
        assert_eq!(sd.same_domain_ori(), Config::SameOriented);
        assert_eq!(sd.same_domain_ind(), 3);
    }

    #[test]
    fn test_orientation() {
        let mut sd = ShapeData::new();
        assert!(!sd.orientation_def());
        sd.set_orientation(1);
        sd.set_orientation_def(true);
        assert_eq!(sd.orientation(), 1);
        assert!(sd.orientation_def());
    }

    #[test]
    fn test_ancestor_rank() {
        let mut sd = ShapeData::new();
        sd.set_ancestor_rank(2);
        assert_eq!(sd.ancestor_rank(), 2);
    }
}
