// FILE: b_rep_class3d_bnd_box_tree.rs
// occt: BRepClass3d_BndBoxTree
// occt: BRepClass3d_BndBoxTreeSelectorPoint
// occt: BRepClass3d_BndBoxTreeSelectorLine

/// Tree selector for point-box collisions
pub struct BndBoxTreeSelectorPoint {
    point: [f64; 3],
}

impl BndBoxTreeSelectorPoint {
    pub fn new() -> Self {
        BndBoxTreeSelectorPoint { point: [0.0; 3] }
    }

    pub fn set_current_point(&mut self, pt: [f64; 3]) {
        self.point = pt;
    }

    pub fn reject_box(&self, _box_min: [f64; 3], _box_max: [f64; 3]) -> bool {
        false
    }
}

impl Default for BndBoxTreeSelectorPoint {
    fn default() -> Self {
        Self::new()
    }
}

/// Edge parameter information
pub struct EdgeParam {
    pub edge_idx: usize,
    pub edge_param: f64,
    pub line_param: f64,
}

/// Vertex parameter information
pub struct VertParam {
    pub vertex_idx: usize,
    pub line_param: f64,
}

/// Tree selector for line-box collisions
pub struct BndBoxTreeSelectorLine {
    line_point: [f64; 3],
    line_direction: [f64; 3],
    edge_params: Vec<EdgeParam>,
    vert_params: Vec<VertParam>,
    is_valid: bool,
}

impl BndBoxTreeSelectorLine {
    pub fn new() -> Self {
        BndBoxTreeSelectorLine {
            line_point: [0.0; 3],
            line_direction: [0.0; 3],
            edge_params: Vec::new(),
            vert_params: Vec::new(),
            is_valid: true,
        }
    }

    pub fn set_current_line(&mut self, pt: [f64; 3], dir: [f64; 3]) {
        self.line_point = pt;
        self.line_direction = dir;
    }

    pub fn reject_box(&self, _box_min: [f64; 3], _box_max: [f64; 3]) -> bool {
        false
    }

    pub fn get_edge_param(&self, idx: usize) -> Option<(&EdgeParam)> {
        self.edge_params.get(idx)
    }

    pub fn get_vert_param(&self, idx: usize) -> Option<(&VertParam)> {
        self.vert_params.get(idx)
    }

    pub fn num_edge_params(&self) -> usize {
        self.edge_params.len()
    }

    pub fn num_vert_params(&self) -> usize {
        self.vert_params.len()
    }

    pub fn clear_results(&mut self) {
        self.edge_params.clear();
        self.vert_params.clear();
        self.is_valid = true;
    }

    pub fn is_correct(&self) -> bool {
        self.is_valid
    }
}

impl Default for BndBoxTreeSelectorLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_selector() {
        let mut sel = BndBoxTreeSelectorPoint::new();
        sel.set_current_point([1.0, 2.0, 3.0]);
        assert!(!sel.reject_box([0.0, 0.0, 0.0], [2.0, 2.0, 2.0]));
    }

    #[test]
    fn test_line_selector() {
        let mut sel = BndBoxTreeSelectorLine::new();
        sel.set_current_line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert!(sel.is_correct());
        sel.clear_results();
        assert_eq!(sel.num_edge_params(), 0);
        assert_eq!(sel.num_vert_params(), 0);
    }
}
