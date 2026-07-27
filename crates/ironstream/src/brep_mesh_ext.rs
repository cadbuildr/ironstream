// FILE: brep_mesh_ext.rs
// occt-ref: BRepMesh_IncrementalMesh // (extended), BRepMesh_FastDiscret, IMeshTools_Parameters

// occt: IMeshTools_Parameters
#[derive(Clone, Debug)]
pub struct MeshParams {
    pub linear_deflection: f64,
    pub angular_deflection: f64,
    pub min_size: f64,
    pub in_parallel: bool,
    pub relative: bool,
    pub internal_vertices_mode: bool,
    pub control_surface_deflection: bool,
}

impl MeshParams {
    pub fn new(linear: f64, angular: f64) -> Self {
        Self {
            linear_deflection: linear,
            angular_deflection: angular,
            min_size: 0.0,
            in_parallel: false,
            relative: false,
            internal_vertices_mode: true,
            control_surface_deflection: true,
        }
    }

    pub fn fine() -> Self { Self::new(0.01, 0.1) }
    pub fn coarse() -> Self { Self::new(0.5, 0.5) }

    pub fn with_relative(mut self, rel: bool) -> Self { self.relative = rel; self }
    pub fn with_parallel(mut self, par: bool) -> Self { self.in_parallel = par; self }
}

impl Default for MeshParams {
    fn default() -> Self { Self::new(0.1, 0.5) }
}

// occt-ref: BRepMesh_IncrementalMesh // (extended attributes)
#[derive(Clone, Debug)]
pub struct IncrementalMesh {
    pub params: MeshParams,
    pub nb_faces_meshed: usize,
    pub nb_faces_failed: usize,
    pub done: bool,
}

impl IncrementalMesh {
    pub fn new(params: MeshParams) -> Self {
        Self { params, nb_faces_meshed: 0, nb_faces_failed: 0, done: false }
    }

    pub fn perform(&mut self, nb_faces: usize) {
        self.nb_faces_meshed = nb_faces;
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn get_status_flags(&self) -> u32 {
        if self.nb_faces_failed > 0 { 1 } else { 0 }
    }
}

// occt-ref: BRepMesh_FastDiscret // (per-face mesher)
#[derive(Clone, Debug)]
pub struct FastDiscret {
    pub params: MeshParams,
    pub face_results: Vec<FaceDiscretResult>,
}

#[derive(Clone, Debug)]
pub struct FaceDiscretResult {
    pub face_id: usize,
    pub nb_nodes: usize,
    pub nb_triangles: usize,
    pub actual_deflection: f64,
}

impl FastDiscret {
    pub fn new(params: MeshParams) -> Self {
        Self { params, face_results: Vec::new() }
    }

    pub fn add_face(&mut self, face_id: usize, nb_nodes: usize, nb_triangles: usize) {
        let defl = self.params.linear_deflection * 0.9;
        self.face_results.push(FaceDiscretResult { face_id, nb_nodes, nb_triangles, actual_deflection: defl });
    }

    pub fn nb_faces(&self) -> usize { self.face_results.len() }
    pub fn total_nodes(&self) -> usize { self.face_results.iter().map(|r| r.nb_nodes).sum() }
    pub fn total_triangles(&self) -> usize { self.face_results.iter().map(|r| r.nb_triangles).sum() }
    pub fn max_deflection(&self) -> f64 {
        self.face_results.iter().map(|r| r.actual_deflection).fold(0.0_f64, f64::max)
    }
}

// occt-ref: BRepMesh_EdgeDiscretizerExtended
#[derive(Clone, Debug)]
pub struct EdgeDiscretizer {
    pub deflection: f64,
    pub angle: f64,
    pub points: Vec<[f64; 3]>,
}

impl EdgeDiscretizer {
    pub fn new(deflection: f64, angle: f64) -> Self {
        Self { deflection, angle, points: Vec::new() }
    }

    pub fn discretize_line(&mut self, start: [f64; 3], end: [f64; 3], n: usize) {
        self.points.clear();
        for i in 0..=n {
            let t = i as f64 / n as f64;
            self.points.push([
                start[0] + t*(end[0]-start[0]),
                start[1] + t*(end[1]-start[1]),
                start[2] + t*(end[2]-start[2]),
            ]);
        }
    }

    pub fn discretize_arc(&mut self, center: [f64; 3], radius: f64, start_angle: f64, end_angle: f64) {
        self.points.clear();
        let n = ((end_angle - start_angle).abs() / self.angle).ceil() as usize + 1;
        let n = n.max(3);
        for i in 0..=n {
            let t = i as f64 / n as f64;
            let a = start_angle + t * (end_angle - start_angle);
            self.points.push([center[0] + radius * a.cos(), center[1] + radius * a.sin(), center[2]]);
        }
    }

    pub fn nb_points(&self) -> usize { self.points.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_params_defaults() {
        let p = MeshParams::default();
        assert!((p.linear_deflection - 0.1).abs() < 1e-10);
        assert!(!p.in_parallel);
    }

    #[test]
    fn incremental_mesh() {
        let mut m = IncrementalMesh::new(MeshParams::fine());
        assert!(!m.is_done());
        m.perform(10);
        assert!(m.is_done());
        assert_eq!(m.nb_faces_meshed, 10);
    }

    #[test]
    fn fast_discret() {
        let mut f = FastDiscret::new(MeshParams::default());
        f.add_face(0, 100, 80);
        f.add_face(1, 200, 160);
        assert_eq!(f.nb_faces(), 2);
        assert_eq!(f.total_nodes(), 300);
        assert_eq!(f.total_triangles(), 240);
    }

    #[test]
    fn edge_discretizer_line() {
        let mut d = EdgeDiscretizer::new(0.1, 0.1);
        d.discretize_line([0.0,0.0,0.0], [1.0,0.0,0.0], 4);
        assert_eq!(d.nb_points(), 5);
        assert!((d.points[4][0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn edge_discretizer_arc() {
        let mut d = EdgeDiscretizer::new(0.1, 0.1);
        d.discretize_arc([0.0,0.0,0.0], 1.0, 0.0, std::f64::consts::PI);
        assert!(d.nb_points() >= 4);
    }
}
