// FILE: hlr_algo_poly_data.rs
// occt: HLRAlgo_PolyData

/// Triangle data for polygonal surfaces.
#[derive(Clone)]
pub struct HlrAlgoTriangleData {
    pub node1: i32,
    pub node2: i32,
    pub node3: i32,
    pub flags: i32,
}

/// Data structure of a set of triangles (polygonal surface representation).
#[derive(Clone)]
pub struct HlrAlgoPolyData {
    face_index: i32,
    triangles: Vec<HlrAlgoTriangleData>,
    nodes: Vec<[f64; 3]>, // 3D node coordinates
    hiding: bool,
}

impl HlrAlgoPolyData {
    /// Create new polygon data.
    pub fn new() -> Self {
        HlrAlgoPolyData {
            face_index: 0,
            triangles: Vec::new(),
            nodes: Vec::new(),
            hiding: false,
        }
    }

    /// Set face index.
    pub fn set_face_index(&mut self, idx: i32) {
        self.face_index = idx;
    }

    /// Get face index.
    pub fn face_index(&self) -> i32 {
        self.face_index
    }

    /// Get number of triangles.
    pub fn nb_triangles(&self) -> usize {
        self.triangles.len()
    }

    /// Get number of nodes.
    pub fn nb_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Add a triangle.
    pub fn add_triangle(&mut self, t: HlrAlgoTriangleData) {
        self.triangles.push(t);
    }

    /// Get triangle at index.
    pub fn triangle(&self, idx: usize) -> Option<&HlrAlgoTriangleData> {
        self.triangles.get(idx)
    }

    /// Add a node.
    pub fn add_node(&mut self, pt: [f64; 3]) {
        self.nodes.push(pt);
    }

    /// Get node at index.
    pub fn node(&self, idx: usize) -> Option<[f64; 3]> {
        self.nodes.get(idx).copied()
    }

    /// Check if this is a hiding surface.
    pub fn hiding(&self) -> bool {
        self.hiding
    }

    /// Set hiding flag.
    pub fn set_hiding(&mut self, h: bool) {
        self.hiding = h;
    }

    /// Clear all data.
    pub fn clear(&mut self) {
        self.triangles.clear();
        self.nodes.clear();
        self.face_index = 0;
        self.hiding = false;
    }
}

impl Default for HlrAlgoPolyData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let poly = HlrAlgoPolyData::new();
        assert_eq!(poly.face_index(), 0);
        assert_eq!(poly.nb_triangles(), 0);
        assert_eq!(poly.nb_nodes(), 0);
        assert!(!poly.hiding());
    }

    #[test]
    fn test_add_triangle() {
        let mut poly = HlrAlgoPolyData::new();
        let tri = HlrAlgoTriangleData {
            node1: 0,
            node2: 1,
            node3: 2,
            flags: 0,
        };
        poly.add_triangle(tri);
        assert_eq!(poly.nb_triangles(), 1);
        assert!(poly.triangle(0).is_some());
    }

    #[test]
    fn test_add_node() {
        let mut poly = HlrAlgoPolyData::new();
        poly.add_node([0.0, 0.0, 0.0]);
        poly.add_node([1.0, 1.0, 1.0]);
        assert_eq!(poly.nb_nodes(), 2);
        assert_eq!(poly.node(0), Some([0.0, 0.0, 0.0]));
        assert_eq!(poly.node(1), Some([1.0, 1.0, 1.0]));
    }

    #[test]
    fn test_face_index() {
        let mut poly = HlrAlgoPolyData::new();
        poly.set_face_index(42);
        assert_eq!(poly.face_index(), 42);
    }

    #[test]
    fn test_hiding() {
        let mut poly = HlrAlgoPolyData::new();
        assert!(!poly.hiding());
        poly.set_hiding(true);
        assert!(poly.hiding());
    }

    #[test]
    fn test_clear() {
        let mut poly = HlrAlgoPolyData::new();
        poly.add_node([1.0, 2.0, 3.0]);
        poly.set_face_index(10);
        poly.clear();
        assert_eq!(poly.nb_nodes(), 0);
        assert_eq!(poly.face_index(), 0);
    }
}
