// FILE: hlr_algo_poly_shell_data.rs
// occt: HLRAlgo_PolyShellData

/// Represents a bi-point (edge between two points)
#[derive(Clone, Debug)]
pub struct HLRAlgoBiPoint {
    point1: [f64; 3],
    point2: [f64; 3],
}

impl HLRAlgoBiPoint {
    pub fn new(p1: [f64; 3], p2: [f64; 3]) -> Self {
        HLRAlgoBiPoint {
            point1: p1,
            point2: p2,
        }
    }

    pub fn point1(&self) -> [f64; 3] {
        self.point1
    }

    pub fn point2(&self) -> [f64; 3] {
        self.point2
    }
}

/// Represents polygon data
#[derive(Clone, Debug)]
pub struct HLRAlgoPolyData {
    id: i32,
}

impl HLRAlgoPolyData {
    pub fn new(id: i32) -> Self {
        HLRAlgoPolyData { id }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

/// Shell indices range
#[derive(Clone, Debug)]
pub struct ShellIndices {
    pub min: i32,
    pub max: i32,
}

impl ShellIndices {
    pub fn new(min: i32, max: i32) -> Self {
        ShellIndices { min, max }
    }
}

/// Contains all the PolyData of a shell.
/// Manages arrays of polygon data and hiding polygon data,
/// edges, and bounding box information.
pub struct HLRAlgoPolyShellData {
    /// Indices for the shell
    indices: ShellIndices,
    /// Array of polygon data
    polygons: Vec<HLRAlgoPolyData>,
    /// Array of hiding polygon data
    hiding_polygons: Vec<HLRAlgoPolyData>,
    /// List of edges (bi-points)
    edges: Vec<HLRAlgoBiPoint>,
    /// Bounding box: [min_x, min_y, min_z, max_x, max_y, max_z]
    bounding_box: [f64; 6],
}

impl HLRAlgoPolyShellData {
    /// Create a new poly shell data with given number of faces
    pub fn new(nb_face: i32) -> Self {
        HLRAlgoPolyShellData {
            indices: ShellIndices::new(0, nb_face),
            polygons: Vec::with_capacity(nb_face as usize),
            hiding_polygons: Vec::new(),
            edges: Vec::new(),
            // Void box semantics (mirrors OCCT Bnd_Box): min components start at
            // +infinity, max components at -infinity, so the first update sets them.
            bounding_box: [
                f64::INFINITY,
                f64::INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
            ],
        }
    }

    /// Update the global bounding box
    pub fn update_global_min_max(&mut self, min: [f64; 3], max: [f64; 3]) {
        self.bounding_box[0] = self.bounding_box[0].min(min[0]);
        self.bounding_box[1] = self.bounding_box[1].min(min[1]);
        self.bounding_box[2] = self.bounding_box[2].min(min[2]);
        self.bounding_box[3] = self.bounding_box[3].max(max[0]);
        self.bounding_box[4] = self.bounding_box[4].max(max[1]);
        self.bounding_box[5] = self.bounding_box[5].max(max[2]);
    }

    /// Update hiding polygon count
    pub fn update_hiding(&mut self, nb_hiding: i32) {
        self.hiding_polygons.clear();
        for i in 0..nb_hiding {
            self.hiding_polygons.push(HLRAlgoPolyData::new(i));
        }
    }

    /// Check if there is hiding data
    pub fn has_hiding(&self) -> bool {
        !self.hiding_polygons.is_empty()
    }

    /// Get polygon data array
    pub fn poly_data(&self) -> &[HLRAlgoPolyData] {
        &self.polygons
    }

    /// Get mutable polygon data array
    pub fn poly_data_mut(&mut self) -> &mut Vec<HLRAlgoPolyData> {
        &mut self.polygons
    }

    /// Get hiding polygon data array
    pub fn hiding_poly_data(&self) -> &[HLRAlgoPolyData] {
        &self.hiding_polygons
    }

    /// Get mutable hiding polygon data array
    pub fn hiding_poly_data_mut(&mut self) -> &mut Vec<HLRAlgoPolyData> {
        &mut self.hiding_polygons
    }

    /// Get edges list
    pub fn edges(&self) -> &[HLRAlgoBiPoint] {
        &self.edges
    }

    /// Get mutable edges list
    pub fn edges_mut(&mut self) -> &mut Vec<HLRAlgoBiPoint> {
        &mut self.edges
    }

    /// Add an edge
    pub fn add_edge(&mut self, edge: HLRAlgoBiPoint) {
        self.edges.push(edge);
    }

    /// Get indices
    pub fn indices(&self) -> &ShellIndices {
        &self.indices
    }

    /// Get mutable indices
    pub fn indices_mut(&mut self) -> &mut ShellIndices {
        &mut self.indices
    }

    /// Get bounding box
    pub fn bounding_box(&self) -> [f64; 6] {
        self.bounding_box
    }

    /// Add polygon data
    pub fn add_poly_data(&mut self, poly: HLRAlgoPolyData) {
        self.polygons.push(poly);
    }

    /// Get number of faces
    pub fn nb_faces(&self) -> i32 {
        self.indices.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_poly_shell_data() {
        let data = HLRAlgoPolyShellData::new(5);
        assert_eq!(data.nb_faces(), 5);
        assert!(!data.has_hiding());
    }

    #[test]
    fn test_add_poly_data() {
        let mut data = HLRAlgoPolyShellData::new(3);
        data.add_poly_data(HLRAlgoPolyData::new(1));
        assert_eq!(data.poly_data().len(), 1);
    }

    #[test]
    fn test_update_hiding() {
        let mut data = HLRAlgoPolyShellData::new(5);
        data.update_hiding(3);
        assert!(data.has_hiding());
        assert_eq!(data.hiding_poly_data().len(), 3);
    }

    #[test]
    fn test_add_edge() {
        let mut data = HLRAlgoPolyShellData::new(2);
        let edge = HLRAlgoBiPoint::new([0.0; 3], [1.0; 3]);
        data.add_edge(edge);
        assert_eq!(data.edges().len(), 1);
    }

    #[test]
    fn test_update_global_min_max() {
        let mut data = HLRAlgoPolyShellData::new(1);
        data.update_global_min_max([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
        let bbox = data.bounding_box();
        assert_eq!(bbox[0], 0.0);
        assert_eq!(bbox[3], 10.0);
    }

    #[test]
    fn test_bi_point() {
        let p1 = [1.0, 2.0, 3.0];
        let p2 = [4.0, 5.0, 6.0];
        let edge = HLRAlgoBiPoint::new(p1, p2);
        assert_eq!(edge.point1(), p1);
        assert_eq!(edge.point2(), p2);
    }

    #[test]
    fn test_shell_indices() {
        let indices = ShellIndices::new(0, 10);
        assert_eq!(indices.min, 0);
        assert_eq!(indices.max, 10);
    }
}
