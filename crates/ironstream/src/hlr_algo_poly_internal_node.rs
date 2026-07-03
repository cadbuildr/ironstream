// FILE: hlr_algo_poly_internal_node.rs
// occt: HLRAlgo_PolyInternalNode

/// Node data for polygon outline processing.
#[derive(Clone)]
pub struct HlrAlgoPolyInternalNodeData {
    pub point: [f64; 3],    // 3D point
    pub normal: [f64; 3],   // Normal vector
    pub uv: [f64; 2],       // UV parameters
    pub pcu1: f64,          // Curvature 1
    pub pcu2: f64,          // Curvature 2
    pub scal: f64,          // Scalar field
}

impl HlrAlgoPolyInternalNodeData {
    /// Create default node data.
    pub fn new() -> Self {
        HlrAlgoPolyInternalNodeData {
            point: [0.0; 3],
            normal: [0.0; 3],
            uv: [0.0; 2],
            pcu1: 0.0,
            pcu2: 0.0,
            scal: 0.0,
        }
    }
}

/// Node indices for linking.
#[derive(Clone, Copy)]
pub struct HlrAlgoPolyInternalNodeIndices {
    pub nd_sg: i32,
    pub flag: i32,
    pub edg1: i32,
    pub edg2: i32,
}

impl HlrAlgoPolyInternalNodeIndices {
    /// Create default node indices.
    pub fn new() -> Self {
        HlrAlgoPolyInternalNodeIndices {
            nd_sg: 0,
            flag: 0,
            edg1: 0,
            edg2: 0,
        }
    }
}

/// Internal node for polygon outline representation.
#[derive(Clone)]
pub struct HlrAlgoPolyInternalNode {
    indices: HlrAlgoPolyInternalNodeIndices,
    data: HlrAlgoPolyInternalNodeData,
}

impl HlrAlgoPolyInternalNode {
    /// Create a new internal node.
    pub fn new() -> Self {
        HlrAlgoPolyInternalNode {
            indices: HlrAlgoPolyInternalNodeIndices::new(),
            data: HlrAlgoPolyInternalNodeData::new(),
        }
    }

    /// Get indices (mutable).
    pub fn indices_mut(&mut self) -> &mut HlrAlgoPolyInternalNodeIndices {
        &mut self.indices
    }

    /// Get indices (immutable).
    pub fn indices(&self) -> &HlrAlgoPolyInternalNodeIndices {
        &self.indices
    }

    /// Get data (mutable).
    pub fn data_mut(&mut self) -> &mut HlrAlgoPolyInternalNodeData {
        &mut self.data
    }

    /// Get data (immutable).
    pub fn data(&self) -> &HlrAlgoPolyInternalNodeData {
        &self.data
    }
}

impl Default for HlrAlgoPolyInternalNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for HlrAlgoPolyInternalNodeData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = HlrAlgoPolyInternalNode::new();
        assert_eq!(node.indices().nd_sg, 0);
        assert_eq!(node.data().point, [0.0; 3]);
    }

    #[test]
    fn test_data_access() {
        let mut node = HlrAlgoPolyInternalNode::new();
        node.data_mut().point = [1.0, 2.0, 3.0];
        node.data_mut().pcu1 = 0.5;
        assert_eq!(node.data().point, [1.0, 2.0, 3.0]);
        assert_eq!(node.data().pcu1, 0.5);
    }

    #[test]
    fn test_indices_access() {
        let mut node = HlrAlgoPolyInternalNode::new();
        node.indices_mut().nd_sg = 5;
        node.indices_mut().flag = 1;
        assert_eq!(node.indices().nd_sg, 5);
        assert_eq!(node.indices().flag, 1);
    }
}
