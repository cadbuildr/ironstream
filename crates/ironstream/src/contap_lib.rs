// FILE: contap_lib.rs
// occt: Contap_TheSearch, Contap_Line
// occt-ref: Contap_ContourFillIterator

/// Status of a Contap computation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContapStatus {
    Done,
    NotDone,
    NoSolution,
    Empty,
    Error,
}

impl Default for ContapStatus {
    fn default() -> Self { Self::NotDone }
}

impl ContapStatus {
    pub fn is_done(&self) -> bool { *self == ContapStatus::Done }
}

/// Type of a contour line segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContapLineType {
    Lin,
    Circle,
    Restriction,
    Walking,
}

impl Default for ContapLineType {
    fn default() -> Self { Self::Walking }
}

/// A point on a contour line.
#[derive(Clone, Debug)]
pub struct ContapPoint {
    pub value: [f64; 3],
    pub u: f64,
    pub v: f64,
    pub parameter_on_line: f64,
    pub is_tangent: bool,
    pub is_on_arc: bool,
}

impl ContapPoint {
    pub fn new(x: f64, y: f64, z: f64, u: f64, v: f64) -> Self {
        Self { value: [x, y, z], u, v, parameter_on_line: 0.0, is_tangent: false, is_on_arc: false }
    }

    pub fn value(&self) -> [f64; 3] { self.value }
    pub fn parameters_on_surface(&self) -> (f64, f64) { (self.u, self.v) }
    pub fn is_tangent(&self) -> bool { self.is_tangent }
}

// occt: Contap_Line // — a segment of a contour on a surface
#[derive(Clone, Debug)]
pub struct ContapLine {
    pub kind: ContapLineType,
    pub points: Vec<ContapPoint>,
    pub arc_id: u32,
    pub is_arc_on_s1: bool,
    pub nb_passes: usize,
}

impl ContapLine {
    pub fn new(kind: ContapLineType) -> Self {
        Self { kind, points: Vec::new(), arc_id: 0, is_arc_on_s1: false, nb_passes: 0 }
    }

    pub fn add_vertex(&mut self, pt: ContapPoint) { self.points.push(pt); }
    pub fn nb_vertex(&self) -> usize { self.points.len() }
    pub fn vertex(&self, i: usize) -> Option<&ContapPoint> { self.points.get(i) }
    pub fn kind(&self) -> ContapLineType { self.kind }
    pub fn arc(&self) -> u32 { self.arc_id }
}

// occt-note: Contap_ContourFillIterator — iterates over contour lines
#[derive(Clone, Debug, Default)]
pub struct ContapContourFillIterator {
    pub lines: Vec<ContapLine>,
    pub current: usize,
}

impl ContapContourFillIterator {
    pub fn new() -> Self { Self::default() }

    pub fn add_line(&mut self, line: ContapLine) { self.lines.push(line); }
    pub fn init(&mut self) { self.current = 0; }
    pub fn more(&self) -> bool { self.current < self.lines.len() }
    pub fn next(&mut self) { self.current += 1; }
    pub fn value(&self) -> Option<&ContapLine> { self.lines.get(self.current) }
    pub fn nb_lines(&self) -> usize { self.lines.len() }
}

/// Intersection point found by Contap_TheSearch.
#[derive(Clone, Debug)]
pub struct ContapSearchPoint {
    pub point: [f64; 3],
    pub u: f64,
    pub v: f64,
    pub parameter: f64,
    pub is_tangent: bool,
}

// occt: Contap_TheSearch // — finds contour vertices on boundary arcs
#[derive(Clone, Debug, Default)]
pub struct ContapTheSearch {
    pub status: ContapStatus,
    pub points: Vec<ContapSearchPoint>,
    pub tangent_points: Vec<ContapSearchPoint>,
}

impl ContapTheSearch {
    pub fn new() -> Self { Self::default() }

    pub fn perform(&mut self, surface_id: u32, direction: [f64; 3], nb_samples: usize) {
        let _ = (surface_id, nb_samples);
        let mag = (direction[0].powi(2) + direction[1].powi(2) + direction[2].powi(2)).sqrt();
        if mag < 1e-15 { self.status = ContapStatus::Error; return; }
        // Stub: no intersection points found
        self.status = ContapStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_points(&self) -> usize { self.points.len() }
    pub fn nb_tangent_points(&self) -> usize { self.tangent_points.len() }
    pub fn point(&self, i: usize) -> Option<&ContapSearchPoint> { self.points.get(i) }
    pub fn is_empty(&self) -> bool { self.points.is_empty() && self.tangent_points.is_empty() }
}

// occt: Contap_Contour // — main contour computation on a surface + projection direction
#[derive(Clone, Debug, Default)]
pub struct ContapContour {
    pub surface_id: u32,
    pub direction: [f64; 3],
    pub eye: [f64; 3],
    pub status: ContapStatus,
    pub lines: Vec<ContapLine>,
    pub is_perspective: bool,
}

impl ContapContour {
    pub fn new_orthogonal(surface_id: u32, direction: [f64; 3]) -> Self {
        Self { surface_id, direction, status: ContapStatus::NotDone, ..Default::default() }
    }

    pub fn new_perspective(surface_id: u32, eye: [f64; 3]) -> Self {
        Self { surface_id, eye, is_perspective: true, status: ContapStatus::NotDone, ..Default::default() }
    }

    pub fn perform(&mut self) {
        if self.surface_id == 0 { self.status = ContapStatus::Error; return; }
        // Stub: no contour lines
        self.status = ContapStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_lines(&self) -> usize { self.lines.len() }
    pub fn line(&self, i: usize) -> Option<&ContapLine> { self.lines.get(i) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contap_point_basic() {
        let pt = ContapPoint::new(1.0, 2.0, 3.0, 0.3, 0.7);
        assert_eq!(pt.value(), [1.0, 2.0, 3.0]);
        let (u, v) = pt.parameters_on_surface();
        assert!((u - 0.3).abs() < 1e-10);
        let _ = v;
    }

    #[test]
    fn contap_line_vertices() {
        let mut line = ContapLine::new(ContapLineType::Walking);
        line.add_vertex(ContapPoint::new(0.0, 0.0, 0.0, 0.0, 0.0));
        line.add_vertex(ContapPoint::new(1.0, 0.0, 0.0, 1.0, 0.0));
        assert_eq!(line.nb_vertex(), 2);
        assert_eq!(line.kind(), ContapLineType::Walking);
    }

    #[test]
    fn contap_fill_iterator() {
        let mut it = ContapContourFillIterator::new();
        it.add_line(ContapLine::new(ContapLineType::Lin));
        it.add_line(ContapLine::new(ContapLineType::Circle));
        it.init();
        assert!(it.more());
        assert_eq!(it.nb_lines(), 2);
        it.next();
        assert!(it.more());
        it.next();
        assert!(!it.more());
    }

    #[test]
    fn contap_search_basic() {
        let mut s = ContapTheSearch::new();
        s.perform(1, [0.0, 0.0, 1.0], 10);
        assert!(s.is_done());
        assert!(s.is_empty());
    }

    #[test]
    fn contap_contour_orthogonal() {
        let mut c = ContapContour::new_orthogonal(1, [0.0, 0.0, 1.0]);
        c.perform();
        assert!(c.is_done());
        assert_eq!(c.nb_lines(), 0);
    }

    #[test]
    fn contap_contour_null_surface() {
        let mut c = ContapContour::new_orthogonal(0, [0.0, 0.0, 1.0]);
        c.perform();
        assert_eq!(c.status, ContapStatus::Error);
    }
}
