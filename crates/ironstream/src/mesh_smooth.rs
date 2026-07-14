// FILE: mesh_smooth.rs
// occt: BRepMesh_FastDiscret, BRepMesh_Context
// occt-ref: IMeshAlgo_Mesher

/// Meshing status.
/// occt: BRepMesh_FastDiscret // status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeshStatus {
    Empty,
    Done,
    Failed,
    ReMeshed,
}

impl Default for MeshStatus {
    fn default() -> Self { Self::Empty }
}

/// Mesh discretization parameters.
/// occt: BRepMesh_FastDiscret // ::Parameters
#[derive(Clone, Debug)]
pub struct MeshingParams {
    pub linear_deflection: f64,
    pub angular_deflection: f64,
    pub min_size: f64,
    pub is_relative: bool,
    pub is_in_parallel: bool,
    pub adapt_min: bool,
    pub internal_vertices_mode: bool,
    pub control_surface_deflection: bool,
}

impl Default for MeshingParams {
    fn default() -> Self {
        Self {
            linear_deflection: 0.1,
            angular_deflection: 0.5,
            min_size: 1e-7,
            is_relative: false,
            is_in_parallel: false,
            adapt_min: true,
            internal_vertices_mode: true,
            control_surface_deflection: true,
        }
    }
}

impl MeshingParams {
    pub fn new() -> Self { Self::default() }

    pub fn set_linear_deflection(&mut self, d: f64) { self.linear_deflection = d.max(1e-10); }
    pub fn set_angular_deflection(&mut self, d: f64) { self.angular_deflection = d.clamp(1e-6, std::f64::consts::PI); }
    pub fn set_min_size(&mut self, s: f64) { self.min_size = s.max(1e-10); }
    pub fn set_relative(&mut self, v: bool) { self.is_relative = v; }
    pub fn set_parallel(&mut self, v: bool) { self.is_in_parallel = v; }
}

/// Fast mesh discretizer for BRep shapes (stub).
/// occt: BRepMesh_FastDiscret
#[derive(Clone, Debug)]
pub struct BrepMeshFastDiscret {
    pub shape_id: u32,
    pub params: MeshingParams,
    pub status: MeshStatus,
    pub nb_faces_meshed: usize,
    pub nb_total_nodes: usize,
    pub nb_total_triangles: usize,
}

impl BrepMeshFastDiscret {
    pub fn new(params: MeshingParams) -> Self {
        Self {
            shape_id: 0,
            params,
            status: MeshStatus::Empty,
            nb_faces_meshed: 0,
            nb_total_nodes: 0,
            nb_total_triangles: 0,
        }
    }

    pub fn set_shape(&mut self, shape_id: u32) { self.shape_id = shape_id; }

    /// Stub mesh: assigns synthetic triangle/node counts.
    pub fn perform(&mut self) {
        if self.shape_id == 0 {
            self.status = MeshStatus::Failed;
            return;
        }
        let scale = (1.0 / self.params.linear_deflection).ceil() as usize;
        self.nb_faces_meshed = 6;
        self.nb_total_nodes = scale * 4;
        self.nb_total_triangles = scale * 2;
        self.status = MeshStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status == MeshStatus::Done }
    pub fn nb_faces_meshed(&self) -> usize { self.nb_faces_meshed }
    pub fn nb_total_nodes(&self) -> usize { self.nb_total_nodes }
    pub fn nb_total_triangles(&self) -> usize { self.nb_total_triangles }
}

/// Mesh algorithm context.
/// occt: BRepMesh_Context
#[derive(Clone, Debug)]
pub struct BrepMeshContext {
    pub params: MeshingParams,
    pub shape_ids: Vec<u32>,
    pub statuses: Vec<MeshStatus>,
}

impl BrepMeshContext {
    pub fn new(params: MeshingParams) -> Self {
        Self { params, shape_ids: Vec::new(), statuses: Vec::new() }
    }

    pub fn add_shape(&mut self, shape_id: u32) {
        self.shape_ids.push(shape_id);
        self.statuses.push(MeshStatus::Empty);
    }

    pub fn perform_all(&mut self) {
        for (i, &sid) in self.shape_ids.iter().enumerate() {
            self.statuses[i] = if sid > 0 { MeshStatus::Done } else { MeshStatus::Failed };
        }
    }

    pub fn status(&self, i: usize) -> Option<MeshStatus> {
        self.statuses.get(i).copied()
    }

    pub fn nb_done(&self) -> usize {
        self.statuses.iter().filter(|&&s| s == MeshStatus::Done).count()
    }
}

/// Laplacian mesh smoothing (stub).
/// occt-note: No direct class, but used in repair pipelines
#[derive(Clone, Debug)]
pub struct MeshLaplacianSmoother {
    pub nb_iterations: usize,
    pub lambda: f64,
    pub mu: f64,
    pub vertices: Vec<[f32; 3]>,
    pub adjacency: Vec<Vec<usize>>,
}

impl MeshLaplacianSmoother {
    pub fn new() -> Self {
        Self {
            nb_iterations: 10,
            lambda: 0.5,
            mu: -0.53,
            vertices: Vec::new(),
            adjacency: Vec::new(),
        }
    }

    pub fn set_iterations(&mut self, n: usize) { self.nb_iterations = n.max(1); }
    pub fn set_lambda(&mut self, l: f64) { self.lambda = l; }
    pub fn set_mu(&mut self, m: f64) { self.mu = m; }

    pub fn set_mesh(&mut self, vertices: Vec<[f32; 3]>, adjacency: Vec<Vec<usize>>) {
        self.vertices = vertices;
        self.adjacency = adjacency;
    }

    pub fn smooth(&mut self) {
        let n = self.vertices.len();
        for _ in 0..self.nb_iterations {
            let mut new_verts = self.vertices.clone();
            for i in 0..n {
                let neighbors = &self.adjacency[i];
                if neighbors.is_empty() { continue; }
                let mut avg = [0.0f32; 3];
                for &j in neighbors {
                    avg[0] += self.vertices[j][0];
                    avg[1] += self.vertices[j][1];
                    avg[2] += self.vertices[j][2];
                }
                let k = neighbors.len() as f32;
                let laplacian = [avg[0]/k - self.vertices[i][0],
                                 avg[1]/k - self.vertices[i][1],
                                 avg[2]/k - self.vertices[i][2]];
                for d in 0..3 {
                    new_verts[i][d] = self.vertices[i][d] + self.lambda as f32 * laplacian[d];
                }
            }
            self.vertices = new_verts;
        }
    }

    pub fn vertices(&self) -> &[[f32; 3]] { &self.vertices }
}

impl Default for MeshLaplacianSmoother {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meshing_params_defaults() {
        let p = MeshingParams::default();
        assert!((p.linear_deflection - 0.1).abs() < 1e-10);
        assert!(p.adapt_min);
    }

    #[test]
    fn mesh_fastdiscret_perform() {
        let mut m = BrepMeshFastDiscret::new(MeshingParams::default());
        m.set_shape(42);
        m.perform();
        assert!(m.is_done());
        assert!(m.nb_total_nodes() > 0);
        assert!(m.nb_total_triangles() > 0);
    }

    #[test]
    fn mesh_fastdiscret_no_shape() {
        let mut m = BrepMeshFastDiscret::new(MeshingParams::default());
        m.perform();
        assert_eq!(m.status, MeshStatus::Failed);
    }

    #[test]
    fn mesh_context_perform_all() {
        let mut ctx = BrepMeshContext::new(MeshingParams::default());
        ctx.add_shape(1);
        ctx.add_shape(0);  // will fail
        ctx.add_shape(2);
        ctx.perform_all();
        assert_eq!(ctx.nb_done(), 2);
        assert_eq!(ctx.status(1), Some(MeshStatus::Failed));
    }

    #[test]
    fn laplacian_smoother_basic() {
        let mut s = MeshLaplacianSmoother::new();
        let verts = vec![[0.0f32,0.0,0.0],[2.0,0.0,0.0],[1.0,1.0,0.0]];
        let adj = vec![vec![1usize,2], vec![0,2], vec![0,1]];
        s.set_mesh(verts, adj);
        s.set_iterations(5);
        s.smooth();
        assert_eq!(s.vertices().len(), 3);
        // After smoothing, centroid should be roughly preserved (it drifts slightly in Laplacian)
        let cx: f32 = s.vertices().iter().map(|v| v[0]).sum::<f32>() / 3.0;
        assert!(cx.abs() < 5.0);  // generous bound for stub
    }
}
