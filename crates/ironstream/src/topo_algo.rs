// FILE: topo_algo.rs
// occt: TopOpeBRep_FacesFiller, TopOpeBRep_LineInter, TopOpeBRepBuild_Builder

// occt: TopOpeBRep_LineInter
/// Represents an intersection line between two topological faces.
#[derive(Clone, Debug)]
pub struct LineInter {
    pub kind: LineInterKind,
    pub points: Vec<InterPoint>,
    pub is_arc: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineInterKind {
    Walking,
    Restriction,
    GLine,
    ALine,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct InterPoint {
    pub point: [f64; 3],
    pub parameter: f64,
    pub is_on_first: bool,
    pub is_on_second: bool,
}

impl LineInter {
    pub fn new(kind: LineInterKind, is_arc: bool) -> Self {
        Self { kind, points: Vec::new(), is_arc }
    }

    pub fn add_point(&mut self, p: InterPoint) { self.points.push(p); }
    pub fn nb_points(&self) -> usize { self.points.len() }
    pub fn is_walking(&self) -> bool { self.kind == LineInterKind::Walking }
    pub fn is_restriction(&self) -> bool { self.kind == LineInterKind::Restriction }
}

// occt: TopOpeBRep_FacesFiller
/// Handles the filling of intersection information between two faces.
#[derive(Clone, Debug, Default)]
pub struct FacesFiller {
    pub lines: Vec<LineInter>,
    pub done: bool,
}

impl FacesFiller {
    pub fn new() -> Self { Self::default() }

    pub fn insert(&mut self, line: LineInter) { self.lines.push(line); }
    pub fn perform(&mut self) { self.done = true; }
    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_lines(&self) -> usize { self.lines.len() }

    pub fn change_faces_intersector(&mut self) { }
}

// occt: TopOpeBRepBuild_Builder
/// Core builder for constructing result shapes from topology operations.
#[derive(Clone, Debug)]
pub struct TopoBuilder {
    pub shapes_in: Vec<TopoShape>,
    pub result_shapes: Vec<TopoShape>,
    pub state: BuilderState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuilderState {
    Initial,
    Running,
    Done,
    Failed,
}

#[derive(Clone, Debug)]
pub struct TopoShape {
    pub id: usize,
    pub kind: TopoKind,
    pub children: Vec<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopoKind {
    Vertex, Edge, Wire, Face, Shell, Solid, Compound,
}

impl TopoBuilder {
    pub fn new() -> Self {
        Self { shapes_in: Vec::new(), result_shapes: Vec::new(), state: BuilderState::Initial }
    }

    pub fn add_shape(&mut self, s: TopoShape) { self.shapes_in.push(s); }

    pub fn build(&mut self) {
        self.state = BuilderState::Running;
        // Stub: pass all input shapes to result
        self.result_shapes = self.shapes_in.clone();
        self.state = BuilderState::Done;
    }

    pub fn is_done(&self) -> bool { self.state == BuilderState::Done }
    pub fn nb_results(&self) -> usize { self.result_shapes.len() }
}

impl Default for TopoBuilder {
    fn default() -> Self { Self::new() }
}

// occt: TopOpeBRepTool_CLASSI
/// Classifies a 2D point relative to a 2D curve (in/out/on).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassifyResult2d {
    In,
    Out,
    On,
    Unknown,
}

pub fn classify_point_2d(point: [f64; 2], boundary: &[[f64; 2]]) -> ClassifyResult2d {
    if boundary.len() < 3 { return ClassifyResult2d::Unknown; }
    let n = boundary.len();
    let mut crossings = 0i32;
    let (px, py) = (point[0], point[1]);
    for i in 0..n {
        let j = (i + 1) % n;
        let (x0, y0) = (boundary[i][0], boundary[i][1]);
        let (x1, y1) = (boundary[j][0], boundary[j][1]);
        if ((y0 <= py && py < y1) || (y1 <= py && py < y0)) {
            let t = (py - y0) / (y1 - y0);
            let xi = x0 + t * (x1 - x0);
            if px < xi { crossings += 1; }
        }
    }
    if crossings % 2 == 1 { ClassifyResult2d::In } else { ClassifyResult2d::Out }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_inter() {
        let mut l = LineInter::new(LineInterKind::Walking, false);
        l.add_point(InterPoint { point: [0.0;3], parameter: 0.0, is_on_first: true, is_on_second: false });
        assert_eq!(l.nb_points(), 1);
        assert!(l.is_walking());
    }

    #[test]
    fn faces_filler() {
        let mut ff = FacesFiller::new();
        ff.insert(LineInter::new(LineInterKind::Walking, false));
        ff.perform();
        assert!(ff.is_done());
        assert_eq!(ff.nb_lines(), 1);
    }

    #[test]
    fn topo_builder() {
        let mut b = TopoBuilder::new();
        b.add_shape(TopoShape { id: 0, kind: TopoKind::Solid, children: Vec::new() });
        b.add_shape(TopoShape { id: 1, kind: TopoKind::Solid, children: Vec::new() });
        b.build();
        assert!(b.is_done());
        assert_eq!(b.nb_results(), 2);
    }

    #[test]
    fn classify_inside() {
        let sq = vec![[0.0,0.0],[1.0,0.0],[1.0,1.0],[0.0,1.0]];
        assert_eq!(classify_point_2d([0.5, 0.5], &sq), ClassifyResult2d::In);
    }

    #[test]
    fn classify_outside() {
        let sq = vec![[0.0,0.0],[1.0,0.0],[1.0,1.0],[0.0,1.0]];
        assert_eq!(classify_point_2d([2.0, 2.0], &sq), ClassifyResult2d::Out);
    }
}
