// FILE: poly_triangulation.rs
// occt: BRep_Polygon3D
// occt-ref: Poly_Triangulation, Poly_Triangle, Poly_Array1OfTriangle

/// A triangle by three node indices (1-based in OCCT, 0-based here).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Triangle {
    pub n1: usize,
    pub n2: usize,
    pub n3: usize,
}

impl Triangle {
    pub fn new(n1: usize, n2: usize, n3: usize) -> Self { Self { n1, n2, n3 } }
    pub fn indices(&self) -> [usize; 3] { [self.n1, self.n2, self.n3] }

    pub fn contains_node(&self, n: usize) -> bool { self.n1 == n || self.n2 == n || self.n3 == n }
}

// occt-ref: Poly_Triangulation
#[derive(Clone, Debug, Default)]
pub struct Triangulation {
    pub nodes: Vec<[f64; 3]>,
    pub triangles: Vec<Triangle>,
    pub normals: Vec<[f64; 3]>,
    pub uvs: Vec<[f64; 2]>,
    pub deflection: f64,
}

impl Triangulation {
    pub fn new() -> Self { Self { deflection: 1e-3, ..Default::default() } }

    pub fn with_capacity(nb_nodes: usize, nb_triangles: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(nb_nodes),
            triangles: Vec::with_capacity(nb_triangles),
            normals: Vec::new(), uvs: Vec::new(),
            deflection: 1e-3,
        }
    }

    pub fn add_node(&mut self, p: [f64; 3]) -> usize { self.nodes.push(p); self.nodes.len() - 1 }
    pub fn add_triangle(&mut self, t: Triangle) { self.triangles.push(t); }
    pub fn add_normal(&mut self, n: [f64; 3]) { self.normals.push(n); }
    pub fn add_uv(&mut self, uv: [f64; 2]) { self.uvs.push(uv); }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn nb_triangles(&self) -> usize { self.triangles.len() }
    pub fn has_normals(&self) -> bool { self.normals.len() == self.nodes.len() }
    pub fn has_uvs(&self) -> bool { self.uvs.len() == self.nodes.len() }

    pub fn triangle_normal(&self, t: &Triangle) -> [f64; 3] {
        let p0 = self.nodes[t.n1]; let p1 = self.nodes[t.n2]; let p2 = self.nodes[t.n3];
        let e1 = sub(p1, p0); let e2 = sub(p2, p0);
        normalize(cross(e1, e2))
    }

    pub fn triangle_area(&self, t: &Triangle) -> f64 {
        let p0 = self.nodes[t.n1]; let p1 = self.nodes[t.n2]; let p2 = self.nodes[t.n3];
        let e1 = sub(p1, p0); let e2 = sub(p2, p0);
        mag(cross(e1, e2)) / 2.0
    }

    pub fn total_area(&self) -> f64 {
        self.triangles.iter().map(|t| self.triangle_area(t)).sum()
    }

    pub fn bounding_box(&self) -> Option<([f64; 3], [f64; 3])> {
        if self.nodes.is_empty() { return None; }
        let mut mn = [f64::INFINITY; 3]; let mut mx = [f64::NEG_INFINITY; 3];
        for p in &self.nodes {
            for i in 0..3 { mn[i] = mn[i].min(p[i]); mx[i] = mx[i].max(p[i]); }
        }
        Some((mn, mx))
    }

    pub fn centroid(&self) -> Option<[f64; 3]> {
        if self.nodes.is_empty() { return None; }
        let n = self.nodes.len() as f64;
        let cx: f64 = self.nodes.iter().map(|p| p[0]).sum::<f64>() / n;
        let cy: f64 = self.nodes.iter().map(|p| p[1]).sum::<f64>() / n;
        let cz: f64 = self.nodes.iter().map(|p| p[2]).sum::<f64>() / n;
        Some([cx, cy, cz])
    }

    /// Flip all triangle winding orders.
    pub fn flip_normals(&mut self) {
        for t in &mut self.triangles { std::mem::swap(&mut t.n2, &mut t.n3); }
        for n in &mut self.normals { *n = [n[0]*-1.0, n[1]*-1.0, n[2]*-1.0]; }
    }

    /// Build a simple triangulated quad (4 verts → 2 triangles).
    pub fn from_quad(p00: [f64; 3], p10: [f64; 3], p11: [f64; 3], p01: [f64; 3]) -> Self {
        let mut t = Self::new();
        let i00 = t.add_node(p00);
        let i10 = t.add_node(p10);
        let i11 = t.add_node(p11);
        let i01 = t.add_node(p01);
        t.add_triangle(Triangle::new(i00, i10, i11));
        t.add_triangle(Triangle::new(i00, i11, i01));
        t
    }
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]-b[0],a[1]-b[1],a[2]-b[2]] }
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]] }
fn mag(v: [f64; 3]) -> f64 { (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt() }
fn normalize(v: [f64; 3]) -> [f64; 3] { let m=mag(v); if m>1e-14{[v[0]/m,v[1]/m,v[2]/m]}else{[0.0,0.0,1.0]} }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangulation_basic() {
        let mut tri = Triangulation::new();
        tri.add_node([0.0,0.0,0.0]);
        tri.add_node([1.0,0.0,0.0]);
        tri.add_node([0.0,1.0,0.0]);
        tri.add_triangle(Triangle::new(0,1,2));
        assert_eq!(tri.nb_nodes(), 3);
        assert_eq!(tri.nb_triangles(), 1);
    }

    #[test]
    fn triangle_area() {
        let mut tri = Triangulation::new();
        tri.add_node([0.0,0.0,0.0]);
        tri.add_node([2.0,0.0,0.0]);
        tri.add_node([0.0,2.0,0.0]);
        let t = Triangle::new(0,1,2);
        let area = tri.triangle_area(&t);
        assert!((area - 2.0).abs() < 1e-10);
    }

    #[test]
    fn triangulation_from_quad() {
        let t = Triangulation::from_quad([0.0,0.0,0.0],[1.0,0.0,0.0],[1.0,1.0,0.0],[0.0,1.0,0.0]);
        assert_eq!(t.nb_nodes(), 4);
        assert_eq!(t.nb_triangles(), 2);
        assert!((t.total_area() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn triangulation_bbox() {
        let t = Triangulation::from_quad([0.0,0.0,0.0],[5.0,0.0,0.0],[5.0,3.0,0.0],[0.0,3.0,0.0]);
        let (mn, mx) = t.bounding_box().unwrap();
        assert!((mx[0] - 5.0).abs() < 1e-10);
        assert!((mx[1] - 3.0).abs() < 1e-10);
    }
}
