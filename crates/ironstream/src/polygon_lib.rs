// FILE: polygon_lib.rs
// occt: Poly_Polygon3D, Poly_Polygon2D, Poly_PolygonOnTriangulation

/// A 3D polygon (piecewise-linear approximation of a curve).
#[derive(Clone, Debug)]
pub struct PolyPolygon3d {
    pub nodes: Vec<[f64; 3]>,
    pub deflection: f64,
    pub has_parameters: bool,
    pub parameters: Vec<f64>,
}

impl PolyPolygon3d {
    pub fn new(nodes: Vec<[f64; 3]>, deflection: f64) -> Self {
        let n = nodes.len();
        Self { nodes, deflection, has_parameters: false, parameters: vec![0.0; n] }
    }

    pub fn with_parameters(mut self, params: Vec<f64>) -> Self {
        assert_eq!(params.len(), self.nodes.len());
        self.parameters = params;
        self.has_parameters = true;
        self
    }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn deflection(&self) -> f64 { self.deflection }
    pub fn node(&self, i: usize) -> Option<[f64; 3]> {
        if i == 0 { None } else { self.nodes.get(i - 1).copied() }
    }
    pub fn parameter(&self, i: usize) -> Option<f64> {
        if i == 0 || !self.has_parameters { None } else { self.parameters.get(i - 1).copied() }
    }
    pub fn has_parameters(&self) -> bool { self.has_parameters }

    pub fn length(&self) -> f64 {
        let mut total = 0.0;
        for i in 1..self.nodes.len() {
            let a = self.nodes[i-1];
            let b = self.nodes[i];
            let dx = b[0]-a[0]; let dy = b[1]-a[1]; let dz = b[2]-a[2];
            total += (dx*dx + dy*dy + dz*dz).sqrt();
        }
        total
    }
}

/// A 2D polygon (UV curve approximation).
#[derive(Clone, Debug)]
pub struct PolyPolygon2d {
    pub nodes: Vec<[f64; 2]>,
    pub deflection: f64,
    pub parameters: Vec<f64>,
}

impl PolyPolygon2d {
    pub fn new(nodes: Vec<[f64; 2]>, deflection: f64) -> Self {
        let n = nodes.len();
        Self { nodes, deflection, parameters: vec![0.0; n] }
    }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn node(&self, i: usize) -> Option<[f64; 2]> {
        if i == 0 { None } else { self.nodes.get(i - 1).copied() }
    }
    pub fn deflection(&self) -> f64 { self.deflection }
}

// occt: Poly_PolygonOnTriangulation // — links polygon nodes to triangulation nodes
#[derive(Clone, Debug)]
pub struct PolyPolygonOnTriangulation {
    pub node_indices: Vec<usize>,
    pub deflection: f64,
    pub has_parameters: bool,
    pub parameters: Vec<f64>,
}

impl PolyPolygonOnTriangulation {
    pub fn new(node_indices: Vec<usize>, deflection: f64) -> Self {
        let n = node_indices.len();
        Self { node_indices, deflection, has_parameters: false, parameters: vec![0.0; n] }
    }

    pub fn with_parameters(mut self, params: Vec<f64>) -> Self {
        self.parameters = params;
        self.has_parameters = true;
        self
    }

    pub fn nb_nodes(&self) -> usize { self.node_indices.len() }
    pub fn node(&self, i: usize) -> Option<usize> {
        if i == 0 { None } else { self.node_indices.get(i - 1).copied() }
    }
    pub fn deflection(&self) -> f64 { self.deflection }
    pub fn parameter(&self, i: usize) -> Option<f64> {
        if i == 0 || !self.has_parameters { None } else { self.parameters.get(i - 1).copied() }
    }
    pub fn has_parameters(&self) -> bool { self.has_parameters }
}

// occt-ref: Poly_MakeLoops // — converts a sequence of edges into loops
#[derive(Clone, Debug, Default)]
pub struct PolyMakeLoops {
    pub edge_ids: Vec<u32>,
    pub loops: Vec<Vec<u32>>,
    pub free_edges: Vec<u32>,
    pub is_done: bool,
}

impl PolyMakeLoops {
    pub fn new() -> Self { Self::default() }

    pub fn add_edge(&mut self, edge_id: u32) { self.edge_ids.push(edge_id); }

    pub fn perform(&mut self) {
        if self.edge_ids.is_empty() { return; }
        // Stub: all edges form one loop
        self.loops.push(self.edge_ids.clone());
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_loops(&self) -> usize { self.loops.len() }
    pub fn loop_edges(&self, i: usize) -> &[u32] {
        self.loops.get(i).map_or(&[], |l| l.as_slice())
    }
    pub fn nb_free_edges(&self) -> usize { self.free_edges.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polygon3d_nodes() {
        let p = PolyPolygon3d::new(vec![[0.0;3],[1.0,0.0,0.0],[1.0,1.0,0.0]], 1e-4);
        assert_eq!(p.nb_nodes(), 3);
        assert_eq!(p.node(1), Some([0.0;3]));
        assert_eq!(p.node(0), None); // 1-based
    }

    #[test]
    fn polygon3d_length() {
        let p = PolyPolygon3d::new(vec![[0.0;3],[1.0,0.0,0.0],[2.0,0.0,0.0]], 0.0);
        assert!((p.length() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn polygon3d_parameters() {
        let p = PolyPolygon3d::new(vec![[0.0;3],[1.0,0.0,0.0]], 0.0)
            .with_parameters(vec![0.0, 1.0]);
        assert!(p.has_parameters());
        assert_eq!(p.parameter(2), Some(1.0));
    }

    #[test]
    fn polygon2d_nodes() {
        let p = PolyPolygon2d::new(vec![[0.0;2],[1.0,0.0],[1.0,1.0]], 1e-4);
        assert_eq!(p.nb_nodes(), 3);
        assert_eq!(p.node(2), Some([1.0, 0.0]));
    }

    #[test]
    fn polygon_on_triangulation() {
        let p = PolyPolygonOnTriangulation::new(vec![0, 1, 2, 3], 1e-4)
            .with_parameters(vec![0.0, 1.0, 2.0, 3.0]);
        assert_eq!(p.nb_nodes(), 4);
        assert_eq!(p.node(1), Some(0));
        assert!(p.has_parameters());
        assert_eq!(p.parameter(3), Some(2.0));
    }

    #[test]
    fn poly_make_loops() {
        let mut ml = PolyMakeLoops::new();
        ml.add_edge(10); ml.add_edge(20); ml.add_edge(30);
        ml.perform();
        assert!(ml.is_done());
        assert_eq!(ml.nb_loops(), 1);
        assert_eq!(ml.loop_edges(0), &[10, 20, 30]);
    }
}
