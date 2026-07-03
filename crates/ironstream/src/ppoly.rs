// FILE: ppoly.rs
// occt: PPoly_Polygon3D, PPoly_Triangle, PPoly_Triangulation

// occt: PPoly_Triangle
/// A triangle defined by three vertex indices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPolyTriangle {
    pub n1: u32,
    pub n2: u32,
    pub n3: u32,
}

impl PPolyTriangle {
    pub fn new(n1: u32, n2: u32, n3: u32) -> Self {
        Self { n1, n2, n3 }
    }

    pub fn indices(&self) -> [u32; 3] {
        [self.n1, self.n2, self.n3]
    }

    pub fn has_vertex(&self, v: u32) -> bool {
        self.n1 == v || self.n2 == v || self.n3 == v
    }
}

// occt: PPoly_Polygon3D
/// A 3-D polygon (polyline) with optional parametric data.
#[derive(Clone, Debug)]
pub struct PPolyPolygon3D {
    pub nodes: Vec<[f64; 3]>,
    pub parameters: Vec<f64>,
    pub deflection: f64,
}

impl PPolyPolygon3D {
    pub fn new(nodes: Vec<[f64; 3]>, deflection: f64) -> Self {
        Self { nodes, parameters: Vec::new(), deflection }
    }

    pub fn with_parameters(mut self, params: Vec<f64>) -> Self {
        self.parameters = params;
        self
    }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }

    pub fn length(&self) -> f64 {
        self.nodes.windows(2).map(|w| {
            let dx = w[1][0]-w[0][0]; let dy = w[1][1]-w[0][1]; let dz = w[1][2]-w[0][2];
            (dx*dx+dy*dy+dz*dz).sqrt()
        }).sum()
    }
}

// occt: PPoly_Triangulation
/// Persistent triangulation: a set of 3-D nodes and indexed triangles.
#[derive(Clone, Debug)]
pub struct PPolyTriangulation {
    pub nodes: Vec<[f64; 3]>,
    pub triangles: Vec<PPolyTriangle>,
    pub normals: Vec<[f32; 3]>,
    pub uv_nodes: Vec<[f64; 2]>,
    pub deflection: f64,
}

impl PPolyTriangulation {
    pub fn new(nodes: Vec<[f64; 3]>, triangles: Vec<PPolyTriangle>) -> Self {
        Self { nodes, triangles, normals: Vec::new(), uv_nodes: Vec::new(), deflection: 0.0 }
    }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn nb_triangles(&self) -> usize { self.triangles.len() }

    pub fn has_normals(&self) -> bool { self.normals.len() == self.nodes.len() }
    pub fn has_uv(&self) -> bool { self.uv_nodes.len() == self.nodes.len() }

    pub fn set_deflection(&mut self, d: f64) { self.deflection = d; }

    /// Compute and store vertex normals as average of adjacent face normals.
    pub fn compute_normals(&mut self) {
        let n = self.nodes.len();
        let mut sums = vec![[0.0f64; 3]; n];
        let mut counts = vec![0usize; n];
        for tri in &self.triangles {
            let idx = tri.indices();
            let p: Vec<[f64; 3]> = idx.iter().map(|&i| self.nodes[i as usize]).collect();
            let u = [p[1][0]-p[0][0], p[1][1]-p[0][1], p[1][2]-p[0][2]];
            let v = [p[2][0]-p[0][0], p[2][1]-p[0][1], p[2][2]-p[0][2]];
            let normal = [u[1]*v[2]-u[2]*v[1], u[2]*v[0]-u[0]*v[2], u[0]*v[1]-u[1]*v[0]];
            for &i in &idx {
                let i = i as usize;
                sums[i][0] += normal[0]; sums[i][1] += normal[1]; sums[i][2] += normal[2];
                counts[i] += 1;
            }
        }
        self.normals = sums.iter().zip(counts.iter()).map(|(s, &c)| {
            if c == 0 { return [0.0f32; 3]; }
            let inv = 1.0 / c as f64;
            let len = ((s[0]*inv).powi(2)+(s[1]*inv).powi(2)+(s[2]*inv).powi(2)).sqrt();
            if len < 1e-14 { [0.0, 0.0, 1.0] } else {
                [(s[0]*inv/len) as f32, (s[1]*inv/len) as f32, (s[2]*inv/len) as f32]
            }
        }).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_indices() {
        let t = PPolyTriangle::new(0, 1, 2);
        assert_eq!(t.indices(), [0, 1, 2]);
        assert!(t.has_vertex(1));
        assert!(!t.has_vertex(5));
    }

    #[test]
    fn polygon3d_length() {
        let nodes = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[1.0,1.0,0.0]];
        let p = PPolyPolygon3D::new(nodes, 0.01);
        assert!((p.length() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn triangulation_normals() {
        let nodes = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[0.0,1.0,0.0]];
        let tris = vec![PPolyTriangle::new(0, 1, 2)];
        let mut mesh = PPolyTriangulation::new(nodes, tris);
        mesh.compute_normals();
        assert!(mesh.has_normals());
        let nz = mesh.normals[0][2].abs();
        assert!(nz > 0.5, "normal should point in Z direction");
    }
}
