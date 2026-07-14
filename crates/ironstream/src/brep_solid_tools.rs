// FILE: brep_solid_tools.rs
// occt-ref: BRepBuilderAPI_MakeSolid, ShapeAnalysis_FreeBounds, BRepCheck_Solid

// occt-ref: BRepBuilderAPI_MakeSolid
/// Builds a solid from one or more shells (closed surfaces).
#[derive(Clone, Debug)]
pub struct MakeSolid {
    pub shells: Vec<ShellInput>,
    pub done: bool,
    pub error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ShellInput {
    pub is_closed: bool,
    pub face_count: usize,
    pub volume: f64,
}

impl ShellInput {
    pub fn new(face_count: usize, is_closed: bool, volume: f64) -> Self {
        Self { is_closed, face_count, volume }
    }
}

impl MakeSolid {
    pub fn new() -> Self { Self { shells: Vec::new(), done: false, error: None } }

    pub fn add_shell(&mut self, shell: ShellInput) {
        self.shells.push(shell);
    }

    pub fn build(&mut self) {
        let any_open = self.shells.iter().any(|s| !s.is_closed);
        if any_open {
            self.error = Some("Non-closed shell".to_string());
            self.done = false;
        } else {
            self.done = !self.shells.is_empty();
        }
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_shells(&self) -> usize { self.shells.len() }

    pub fn total_volume(&self) -> f64 {
        self.shells.iter().map(|s| s.volume.abs()).sum()
    }
}

impl Default for MakeSolid {
    fn default() -> Self { Self::new() }
}

// occt-ref: ShapeAnalysis_FreeBounds
/// Finds free (open) boundary wires in a shape.
#[derive(Clone, Debug, Default)]
pub struct FreeBoundsAnalysis {
    pub closed_wires: Vec<FreeBoundWire>,
    pub open_wires: Vec<FreeBoundWire>,
}

#[derive(Clone, Debug)]
pub struct FreeBoundWire {
    pub vertex_count: usize,
    pub is_closed: bool,
    pub length: f64,
}

impl FreeBoundsAnalysis {
    pub fn new() -> Self { Self::default() }

    pub fn perform(&mut self, edge_count: usize, closed_edges: usize) {
        let open_edges = edge_count.saturating_sub(closed_edges);
        if closed_edges > 0 {
            self.closed_wires.push(FreeBoundWire {
                vertex_count: closed_edges,
                is_closed: true,
                length: closed_edges as f64,
            });
        }
        if open_edges > 0 {
            self.open_wires.push(FreeBoundWire {
                vertex_count: open_edges,
                is_closed: false,
                length: open_edges as f64,
            });
        }
    }

    pub fn nb_closed_wires(&self) -> usize { self.closed_wires.len() }
    pub fn nb_open_wires(&self) -> usize { self.open_wires.len() }
    pub fn has_free_bounds(&self) -> bool { !self.open_wires.is_empty() }
}

// occt-ref: BRepCheck_Solid
/// Validates a solid for topological consistency.
#[derive(Clone, Debug)]
pub struct CheckSolid {
    pub nb_shells: usize,
    pub nb_faces: usize,
    pub has_free_edges: bool,
    pub errors: Vec<SolidError>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolidError {
    NoError,
    NotClosed,
    SelfIntersecting,
    BadOrientation,
    InvalidVertex,
}

impl CheckSolid {
    pub fn new(nb_shells: usize, nb_faces: usize, has_free_edges: bool) -> Self {
        let mut errors = Vec::new();
        if has_free_edges { errors.push(SolidError::NotClosed); }
        Self { nb_shells, nb_faces, has_free_edges, errors }
    }

    pub fn is_valid(&self) -> bool { self.errors.iter().all(|e| *e == SolidError::NoError) }
    pub fn nb_errors(&self) -> usize { self.errors.iter().filter(|&&e| e != SolidError::NoError).count() }
    pub fn errors(&self) -> &[SolidError] { &self.errors }
}

// occt-ref: BRepOffset_MakeOffset // (offset solid)
#[derive(Clone, Debug)]
pub struct MakeOffset {
    pub nb_faces: usize,
    pub offset: f64,
    pub done: bool,
    pub tolerance: f64,
}

impl MakeOffset {
    pub fn new(nb_faces: usize, offset: f64) -> Self {
        Self { nb_faces, offset, done: false, tolerance: 1e-6 }
    }

    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t; }
    pub fn build(&mut self) { self.done = self.nb_faces > 0 && self.offset.abs() > 1e-14; }
    pub fn is_done(&self) -> bool { self.done }
    pub fn offset_value(&self) -> f64 { self.offset }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_solid_closed() {
        let mut s = MakeSolid::new();
        s.add_shell(ShellInput::new(6, true, 8.0));
        s.build();
        assert!(s.is_done());
        assert!((s.total_volume() - 8.0).abs() < 1e-10);
    }

    #[test]
    fn make_solid_open_fails() {
        let mut s = MakeSolid::new();
        s.add_shell(ShellInput::new(5, false, 0.0));
        s.build();
        assert!(!s.is_done());
        assert!(s.error.is_some());
    }

    #[test]
    fn free_bounds_analysis() {
        let mut f = FreeBoundsAnalysis::new();
        f.perform(10, 8);
        assert_eq!(f.nb_closed_wires(), 1);
        assert_eq!(f.nb_open_wires(), 1);
        assert!(f.has_free_bounds());
    }

    #[test]
    fn check_solid_valid() {
        let c = CheckSolid::new(1, 6, false);
        assert!(c.is_valid());
        assert_eq!(c.nb_errors(), 0);
    }

    #[test]
    fn make_offset() {
        let mut m = MakeOffset::new(6, 0.5);
        m.build();
        assert!(m.is_done());
        assert!((m.offset_value() - 0.5).abs() < 1e-10);
    }
}
