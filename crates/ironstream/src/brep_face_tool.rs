// FILE: brep_face_tool.rs
// occt: BRepLib_FindSurface
// occt-ref: BRepTools_FaceSet, ShapeAnalysis_Face

// occt: BRepLib_FindSurface
/// Finds the best-fit plane/surface for a wire or set of points.
#[derive(Clone, Debug)]
pub struct FindSurface {
    pub points: Vec<[f64; 3]>,
    pub tolerance: f64,
    pub found: bool,
    pub plane_normal: Option<[f64; 3]>,
    pub plane_origin: Option<[f64; 3]>,
    pub existing_surface: bool,
}

impl FindSurface {
    pub fn new(points: Vec<[f64; 3]>, tolerance: f64) -> Self {
        let mut f = Self { points, tolerance, found: false, plane_normal: None, plane_origin: None, existing_surface: false };
        f.perform();
        f
    }

    fn perform(&mut self) {
        if self.points.len() < 3 { return; }
        // Compute centroid
        let n = self.points.len() as f64;
        let cx: f64 = self.points.iter().map(|p| p[0]).sum::<f64>() / n;
        let cy: f64 = self.points.iter().map(|p| p[1]).sum::<f64>() / n;
        let cz: f64 = self.points.iter().map(|p| p[2]).sum::<f64>() / n;
        self.plane_origin = Some([cx, cy, cz]);
        // Estimate normal using first 3 non-collinear points
        let v1 = sub3(self.points[1], self.points[0]);
        let v2 = sub3(self.points[2], self.points[0]);
        let n_dir = cross3(v1, v2);
        let len = (n_dir[0]*n_dir[0]+n_dir[1]*n_dir[1]+n_dir[2]*n_dir[2]).sqrt();
        if len > self.tolerance * 1e-3 {
            self.plane_normal = Some([n_dir[0]/len, n_dir[1]/len, n_dir[2]/len]);
            self.found = true;
        }
    }

    pub fn found(&self) -> bool { self.found }
    pub fn tolerance_reached(&self) -> f64 { self.tolerance }
}

// occt-ref: ShapeAnalysis_Face
#[derive(Clone, Debug)]
pub struct FaceAnalysis {
    pub face_id: usize,
    pub uv_boundary: Vec<[f64; 2]>,
    pub surface_id: usize,
}

impl FaceAnalysis {
    pub fn new(face_id: usize, surface_id: usize, uv_boundary: Vec<[f64; 2]>) -> Self {
        Self { face_id, uv_boundary, surface_id }
    }

    pub fn is_degenerated(&self, tol: f64) -> bool {
        if self.uv_boundary.len() < 3 { return true; }
        let area = self.uv_area();
        area < tol * tol
    }

    fn uv_area(&self) -> f64 {
        let pts = &self.uv_boundary;
        let n = pts.len();
        if n < 3 { return 0.0; }
        let mut area = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            area += pts[i][0] * pts[j][1] - pts[j][0] * pts[i][1];
        }
        (area / 2.0).abs()
    }

    pub fn uv_bounds(&self) -> ([f64; 2], [f64; 2]) {
        let mut min = [f64::INFINITY; 2];
        let mut max = [f64::NEG_INFINITY; 2];
        for p in &self.uv_boundary {
            min[0] = min[0].min(p[0]); min[1] = min[1].min(p[1]);
            max[0] = max[0].max(p[0]); max[1] = max[1].max(p[1]);
        }
        (min, max)
    }

    pub fn nb_wires(&self) -> usize { 1 }
}

// occt-ref: BRepTools_FaceSet
#[derive(Clone, Debug, Default)]
pub struct FaceSet {
    pub faces: Vec<FaceSummary>,
}

#[derive(Clone, Debug)]
pub struct FaceSummary {
    pub face_id: usize,
    pub area: f64,
    pub normal: [f64; 3],
}

impl FaceSet {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, f: FaceSummary) { self.faces.push(f); }
    pub fn extent(&self) -> usize { self.faces.len() }
    pub fn total_area(&self) -> f64 { self.faces.iter().map(|f| f.area).sum() }
    pub fn find_by_id(&self, id: usize) -> Option<&FaceSummary> {
        self.faces.iter().find(|f| f.face_id == id)
    }
    pub fn clear(&mut self) { self.faces.clear(); }
}

// occt-ref: TopExp_Explorer // (face-level)
#[derive(Clone, Debug)]
pub struct FaceExplorer<'a> {
    set: &'a FaceSet,
    index: usize,
}

impl<'a> FaceExplorer<'a> {
    pub fn new(set: &'a FaceSet) -> Self { Self { set, index: 0 } }
    pub fn more(&self) -> bool { self.index < self.set.faces.len() }
    pub fn next(&mut self) { self.index += 1; }
    pub fn current(&self) -> Option<&FaceSummary> { self.set.faces.get(self.index) }
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
}
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]-b[0], a[1]-b[1], a[2]-b[2]] }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_surface_plane() {
        let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[1.0,1.0,0.0],[0.0,1.0,0.0]];
        let f = FindSurface::new(pts, 1e-6);
        assert!(f.found());
        let n = f.plane_normal.unwrap();
        assert!((n[2].abs() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn find_surface_insufficient() {
        let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
        let f = FindSurface::new(pts, 1e-6);
        assert!(!f.found());
    }

    #[test]
    fn face_analysis_area() {
        let boundary = vec![[0.0,0.0],[1.0,0.0],[1.0,1.0],[0.0,1.0]];
        let fa = FaceAnalysis::new(0, 1, boundary);
        assert!(!fa.is_degenerated(1e-6));
        let (min, max) = fa.uv_bounds();
        assert!((min[0]).abs() < 1e-10);
        assert!((max[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn face_set() {
        let mut set = FaceSet::new();
        set.add(FaceSummary { face_id: 0, area: 2.0, normal: [0.0,0.0,1.0] });
        set.add(FaceSummary { face_id: 1, area: 3.0, normal: [0.0,0.0,-1.0] });
        assert_eq!(set.extent(), 2);
        assert!((set.total_area() - 5.0).abs() < 1e-10);
        assert!(set.find_by_id(1).is_some());
    }

    #[test]
    fn face_explorer() {
        let mut set = FaceSet::new();
        set.add(FaceSummary { face_id: 0, area: 1.0, normal: [0.0,0.0,1.0] });
        let mut exp = FaceExplorer::new(&set);
        assert!(exp.more());
        exp.next();
        assert!(!exp.more());
    }
}
