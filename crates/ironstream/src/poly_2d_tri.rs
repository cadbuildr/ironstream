// FILE: poly_2d_tri.rs
// occt: Poly_2DTriangulation, Poly_Triangle, Poly_Array1OfTriangle,
//       Poly_MakeLoops, Poly_CoherentTriangle

/// A triangle defined by three node indices (1-based, OCCT convention).
/// occt: Poly_Triangle
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PolyTriangle {
    pub n1: u32,
    pub n2: u32,
    pub n3: u32,
}

impl PolyTriangle {
    pub fn new(n1: u32, n2: u32, n3: u32) -> Self { Self { n1, n2, n3 } }

    pub fn indices(&self) -> [u32; 3] { [self.n1, self.n2, self.n3] }

    pub fn get(&self, i: usize) -> u32 {
        match i { 0 => self.n1, 1 => self.n2, _ => self.n3 }
    }

    pub fn reversed(&self) -> Self { Self::new(self.n3, self.n2, self.n1) }
}

/// 1-based array of triangles.
/// occt: Poly_Array1OfTriangle
#[derive(Clone, Debug, Default)]
pub struct PolyArray1OfTriangle {
    pub data: Vec<PolyTriangle>,
    pub lower: usize,
}

impl PolyArray1OfTriangle {
    pub fn new(lower: usize, upper: usize) -> Self {
        let len = upper.saturating_sub(lower) + 1;
        Self { data: vec![PolyTriangle::default(); len], lower }
    }

    pub fn set_value(&mut self, idx: usize, t: PolyTriangle) {
        if let Some(slot) = self.data.get_mut(idx.saturating_sub(self.lower)) {
            *slot = t;
        }
    }

    pub fn value(&self, idx: usize) -> Option<PolyTriangle> {
        self.data.get(idx.saturating_sub(self.lower)).copied()
    }

    pub fn length(&self) -> usize { self.data.len() }
    pub fn upper(&self) -> usize { self.lower + self.data.len().saturating_sub(1) }
}

/// 2D triangulation (UV mesh).
/// occt: Poly_2DTriangulation
#[derive(Clone, Debug, Default)]
pub struct Poly2DTriangulation {
    pub nodes: Vec<[f64; 2]>,           // UV coordinates
    pub triangles: Vec<PolyTriangle>,
    pub deflection: f64,
}

impl Poly2DTriangulation {
    pub fn new() -> Self { Self::default() }

    pub fn from_nodes_and_triangles(nodes: Vec<[f64; 2]>, triangles: Vec<PolyTriangle>) -> Self {
        Self { nodes, triangles, deflection: 0.0 }
    }

    pub fn add_node(&mut self, uv: [f64; 2]) -> u32 {
        let idx = self.nodes.len() as u32 + 1;  // 1-based
        self.nodes.push(uv);
        idx
    }

    pub fn add_triangle(&mut self, t: PolyTriangle) { self.triangles.push(t); }

    pub fn node(&self, idx: u32) -> Option<[f64; 2]> {
        self.nodes.get(idx.saturating_sub(1) as usize).copied()
    }

    pub fn triangle(&self, idx: usize) -> Option<PolyTriangle> {
        self.triangles.get(idx.saturating_sub(1)).copied()
    }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn nb_triangles(&self) -> usize { self.triangles.len() }

    pub fn set_deflection(&mut self, d: f64) { self.deflection = d.max(0.0); }

    /// Validate: check all triangle indices are in [1, nb_nodes].
    pub fn is_valid(&self) -> bool {
        let n = self.nb_nodes() as u32;
        self.triangles.iter().all(|t| t.n1 >= 1 && t.n1 <= n && t.n2 >= 1 && t.n2 <= n && t.n3 >= 1 && t.n3 <= n)
    }

    /// Compute signed area of a single triangle (UV space).
    pub fn triangle_area_uv(&self, t: &PolyTriangle) -> f64 {
        let p1 = self.nodes.get(t.n1.saturating_sub(1) as usize).copied().unwrap_or([0.0; 2]);
        let p2 = self.nodes.get(t.n2.saturating_sub(1) as usize).copied().unwrap_or([0.0; 2]);
        let p3 = self.nodes.get(t.n3.saturating_sub(1) as usize).copied().unwrap_or([0.0; 2]);
        0.5 * ((p2[0]-p1[0])*(p3[1]-p1[1]) - (p3[0]-p1[0])*(p2[1]-p1[1]))
    }

    pub fn total_area_uv(&self) -> f64 {
        self.triangles.iter().map(|t| self.triangle_area_uv(t).abs()).sum()
    }
}

/// Coherent triangle: edge connectivity for topology analysis.
/// occt: Poly_CoherentTriangle
#[derive(Clone, Debug, Default)]
pub struct PolyCoherentTriangle {
    pub node_indices: [u32; 3],
    pub neighbours: [Option<u32>; 3],  // neighbour triangle index per edge, 0-based
}

impl PolyCoherentTriangle {
    pub fn new(n0: u32, n1: u32, n2: u32) -> Self {
        Self { node_indices: [n0, n1, n2], neighbours: [None; 3] }
    }

    pub fn set_neighbour(&mut self, edge: usize, tri_idx: u32) {
        if edge < 3 { self.neighbours[edge] = Some(tri_idx); }
    }

    pub fn nb_neighbours(&self) -> usize { self.neighbours.iter().filter(|n| n.is_some()).count() }
    pub fn is_boundary_edge(&self, edge: usize) -> bool { edge < 3 && self.neighbours[edge].is_none() }
}

/// Loop builder for polygon edge loops.
/// occt: Poly_MakeLoops (simplified)
#[derive(Clone, Debug, Default)]
pub struct PolyMakeLoops {
    pub edges: Vec<(u32, u32)>,     // directed edges (from, to)
    pub loops: Vec<Vec<u32>>,
}

impl PolyMakeLoops {
    pub fn new() -> Self { Self::default() }

    pub fn add_edge(&mut self, from: u32, to: u32) { self.edges.push((from, to)); }

    /// Find loops by chaining edges.
    pub fn perform(&mut self) {
        self.loops.clear();
        let mut remaining: Vec<(u32, u32)> = self.edges.clone();
        while !remaining.is_empty() {
            let start = remaining[0].0;
            let mut loop_nodes = vec![start];
            let mut current = remaining[0].1;
            remaining.remove(0);

            let mut max_iter = remaining.len() + 1;
            while current != start && max_iter > 0 {
                max_iter -= 1;
                loop_nodes.push(current);
                if let Some(pos) = remaining.iter().position(|(f, _)| *f == current) {
                    let next = remaining[pos].1;
                    remaining.remove(pos);
                    current = next;
                } else {
                    break;
                }
            }
            if current == start { self.loops.push(loop_nodes); }
        }
    }

    pub fn nb_loops(&self) -> usize { self.loops.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_indices() {
        let t = PolyTriangle::new(1, 2, 3);
        assert_eq!(t.indices(), [1, 2, 3]);
        assert_eq!(t.reversed().n1, 3);
    }

    #[test]
    fn array1_of_triangle() {
        let mut a = PolyArray1OfTriangle::new(1, 3);
        a.set_value(1, PolyTriangle::new(1, 2, 3));
        a.set_value(3, PolyTriangle::new(4, 5, 6));
        assert_eq!(a.length(), 3);
        assert_eq!(a.value(1).unwrap().n1, 1);
        assert_eq!(a.value(3).unwrap().n2, 5);
    }

    #[test]
    fn poly_2d_triangulation_valid() {
        let mut mesh = Poly2DTriangulation::new();
        let a = mesh.add_node([0.0, 0.0]);
        let b = mesh.add_node([1.0, 0.0]);
        let c = mesh.add_node([0.0, 1.0]);
        mesh.add_triangle(PolyTriangle::new(a, b, c));
        assert!(mesh.is_valid());
        assert_eq!(mesh.nb_nodes(), 3);
        assert_eq!(mesh.nb_triangles(), 1);
    }

    #[test]
    fn poly_2d_area() {
        let mut mesh = Poly2DTriangulation::new();
        mesh.add_node([0.0, 0.0]);
        mesh.add_node([1.0, 0.0]);
        mesh.add_node([0.0, 1.0]);
        mesh.add_triangle(PolyTriangle::new(1, 2, 3));
        let area = mesh.total_area_uv();
        assert!((area - 0.5).abs() < 1e-10);
    }

    #[test]
    fn coherent_triangle_neighbours() {
        let mut ct = PolyCoherentTriangle::new(0, 1, 2);
        ct.set_neighbour(0, 5);
        assert_eq!(ct.nb_neighbours(), 1);
        assert!(ct.is_boundary_edge(1));
        assert!(!ct.is_boundary_edge(0));
    }

    #[test]
    fn make_loops_simple() {
        let mut ml = PolyMakeLoops::new();
        // Triangle: 0→1, 1→2, 2→0
        ml.add_edge(0, 1);
        ml.add_edge(1, 2);
        ml.add_edge(2, 0);
        ml.perform();
        assert_eq!(ml.nb_loops(), 1);
        assert_eq!(ml.loops[0].len(), 3);
    }
}
