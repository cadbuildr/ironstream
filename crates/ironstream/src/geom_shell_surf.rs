// FILE: geom_shell_surf.rs
// occt: BRepBuilderAPI_MakeShell, ShapeAnalysis_Shell, BRepClass3d_SolidExplorer

// occt: BRepBuilderAPI_MakeShell
/// Builds a shell from one or more faces.
#[derive(Clone, Debug, Default)]
pub struct MakeShell {
    pub faces: Vec<ShellFace>,
    pub done: bool,
    pub error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ShellFace {
    pub face_id: usize,
    pub surface_type: String,
    pub normal: [f64; 3],
    pub area: f64,
}

impl ShellFace {
    pub fn new(face_id: usize, surface_type: impl Into<String>, normal: [f64; 3], area: f64) -> Self {
        Self { face_id, surface_type: surface_type.into(), normal, area }
    }
}

impl MakeShell {
    pub fn new() -> Self { Self::default() }
    pub fn add_face(&mut self, face: ShellFace) { self.faces.push(face); }

    pub fn build(&mut self) {
        if self.faces.is_empty() {
            self.error = Some("No faces".into());
            self.done = false;
        } else {
            self.done = true;
        }
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_faces(&self) -> usize { self.faces.len() }
    pub fn total_area(&self) -> f64 { self.faces.iter().map(|f| f.area).sum() }
}

// occt: ShapeAnalysis_Shell
#[derive(Clone, Debug, Default)]
pub struct ShellAnalysis {
    pub faces: Vec<ShellFace>,
    pub free_edges: Vec<FreeEdge>,
}

#[derive(Clone, Debug)]
pub struct FreeEdge {
    pub start: [f64; 3],
    pub end: [f64; 3],
    pub adjacent_face_count: usize,
}

impl ShellAnalysis {
    pub fn new(faces: Vec<ShellFace>) -> Self {
        Self { faces, free_edges: Vec::new() }
    }

    pub fn check_closed(&self) -> bool { self.free_edges.is_empty() }
    pub fn nb_faces(&self) -> usize { self.faces.len() }
    pub fn nb_free_edges(&self) -> usize { self.free_edges.len() }
    pub fn has_free_edges(&self) -> bool { !self.free_edges.is_empty() }
    pub fn add_free_edge(&mut self, e: FreeEdge) { self.free_edges.push(e); }
}

// occt: BRepClass3d_SolidExplorer
/// Explores the solid-bounding shells to determine inside/outside.
#[derive(Clone, Debug)]
pub struct SolidClassifier {
    pub shells: Vec<ShellSummary>,
    pub tolerance: f64,
}

#[derive(Clone, Debug)]
pub struct ShellSummary {
    pub is_closed: bool,
    pub is_oriented_outward: bool,
    pub nb_faces: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsState {
    In,
    Out,
    On,
    Unknown,
}

impl SolidClassifier {
    pub fn new(shells: Vec<ShellSummary>, tolerance: f64) -> Self {
        Self { shells, tolerance }
    }

    pub fn classify(&self, _point: [f64; 3]) -> TopAbsState {
        // Stub: return Unknown if any shell is not closed
        if self.shells.iter().any(|s| !s.is_closed) { TopAbsState::Unknown }
        else { TopAbsState::Out }
    }

    pub fn nb_shells(&self) -> usize { self.shells.len() }
    pub fn all_closed(&self) -> bool { self.shells.iter().all(|s| s.is_closed) }
}

// occt: GeomFill_SimpleBound for shell boundary
#[derive(Clone, Debug)]
pub struct ShellBoundary {
    pub curves: Vec<Vec<[f64; 3]>>,
    pub is_closed: bool,
}

impl ShellBoundary {
    pub fn new() -> Self { Self { curves: Vec::new(), is_closed: false } }
    pub fn add_curve(&mut self, pts: Vec<[f64; 3]>) { self.curves.push(pts); }
    pub fn nb_curves(&self) -> usize { self.curves.len() }
    pub fn check_closed(&mut self) {
        if self.curves.len() < 2 { self.is_closed = false; return; }
        let first_start = self.curves.first().and_then(|c| c.first()).copied();
        let last_end = self.curves.last().and_then(|c| c.last()).copied();
        self.is_closed = match (first_start, last_end) {
            (Some(a), Some(b)) => {
                let dx=a[0]-b[0]; let dy=a[1]-b[1]; let dz=a[2]-b[2];
                (dx*dx+dy*dy+dz*dz).sqrt() < 1e-6
            },
            _ => false,
        };
    }
}

impl Default for ShellBoundary {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_shell_build() {
        let mut s = MakeShell::new();
        s.add_face(ShellFace::new(0, "Plane", [0.0,0.0,1.0], 4.0));
        s.add_face(ShellFace::new(1, "Plane", [0.0,0.0,-1.0], 4.0));
        s.build();
        assert!(s.is_done());
        assert_eq!(s.nb_faces(), 2);
        assert!((s.total_area() - 8.0).abs() < 1e-10);
    }

    #[test]
    fn make_shell_empty_fails() {
        let mut s = MakeShell::new();
        s.build();
        assert!(!s.is_done());
    }

    #[test]
    fn shell_analysis() {
        let faces = vec![ShellFace::new(0, "Sphere", [0.0,0.0,1.0], 12.57)];
        let a = ShellAnalysis::new(faces);
        assert!(a.check_closed());
        assert_eq!(a.nb_faces(), 1);
    }

    #[test]
    fn solid_classifier() {
        let shells = vec![
            ShellSummary { is_closed: true, is_oriented_outward: true, nb_faces: 6 },
        ];
        let c = SolidClassifier::new(shells, 1e-6);
        assert!(c.all_closed());
        assert_eq!(c.nb_shells(), 1);
    }

    #[test]
    fn shell_boundary() {
        let mut b = ShellBoundary::new();
        b.add_curve(vec![[0.0,0.0,0.0],[1.0,0.0,0.0]]);
        b.add_curve(vec![[1.0,0.0,0.0],[0.0,0.0,0.0]]);
        b.check_closed();
        assert!(b.is_closed);
    }
}
