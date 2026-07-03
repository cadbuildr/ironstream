// FILE: rust/ironstream/crates/ironstream/src/geom_int.rs

// occt: GeomInt_SurfaceTool result stub
/// occt: GeomInt_SurfaceTool result stub — stores nb_lines and is_done state.
pub struct GeomIntSsResult {
    nb_lines: usize,
    is_done: bool,
}

impl GeomIntSsResult {
    pub fn new() -> Self {
        Self { nb_lines: 0, is_done: false }
    }

    pub fn nb_lines(&self) -> usize {
        self.nb_lines
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn mark_done(&mut self) {
        self.is_done = true;
    }
}

impl Default for GeomIntSsResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt: GeomInt_IntSS — surface-surface intersection query stub
/// occt: GeomInt_IntSS — surface-surface intersection query stub.
pub struct GeomIntIntSs {
    pub is_done: bool,
    pub nb_lines: usize,
}

impl GeomIntIntSs {
    pub fn new() -> Self {
        Self { is_done: false, nb_lines: 0 }
    }

    /// occt: GeomInt_IntSS::Perform — sets is_done=true, nb_lines=0 (stub).
    pub fn perform(&mut self) {
        self.is_done = true;
        self.nb_lines = 0;
    }

    pub fn nb_lines(&self) -> usize {
        self.nb_lines
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

impl Default for GeomIntIntSs {
    fn default() -> Self {
        Self::new()
    }
}

// occt: GeomInt_LineTool — helper for intersection lines
/// occt: GeomInt_LineTool — helper for intersection lines.
pub struct GeomIntLineTool;

impl GeomIntLineTool {
    /// occt: GeomInt_LineTool::NbVertex — returns nb for stub.
    pub fn nb_vertex(nb: usize) -> usize {
        nb
    }

    /// occt: GeomInt_LineTool::FirstVertex — returns 0 (stub vertex index).
    pub fn first_vertex() -> usize {
        0
    }

    /// occt: GeomInt_LineTool::LastVertex — returns 0 (stub vertex index).
    pub fn last_vertex() -> usize {
        0
    }
}

// occt: GeomInt_LineTool type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomIntLineType {
    Walking,
    Restriction,
    Analytic,
}

/// occt: GeomInt_IntSS result point
pub struct GeomIntIntSSPoint {
    pub u1: f64,
    pub v1: f64, // params on surface 1
    pub u2: f64,
    pub v2: f64, // params on surface 2
    pub xyz: [f64; 3],
}

impl GeomIntIntSSPoint {
    pub fn new(u1: f64, v1: f64, u2: f64, v2: f64, xyz: [f64; 3]) -> Self {
        Self { u1, v1, u2, v2, xyz }
    }
}

/// occt: GeomInt_IntSS line
pub struct GeomIntIntSSLine {
    pub line_type: GeomIntLineType,
    pub points: Vec<GeomIntIntSSPoint>,
    pub has_first_point: bool,
    pub has_last_point: bool,
}

impl GeomIntIntSSLine {
    pub fn new(line_type: GeomIntLineType) -> Self {
        Self {
            line_type,
            points: Vec::new(),
            has_first_point: false,
            has_last_point: false,
        }
    }

    pub fn add_point(&mut self, p: GeomIntIntSSPoint) {
        self.points.push(p);
    }

    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    /// 1-based index
    pub fn point(&self, idx: usize) -> Option<&GeomIntIntSSPoint> {
        if idx == 0 {
            return None;
        }
        self.points.get(idx - 1)
    }
}

/// occt: GeomInt_IntSS
pub struct GeomIntIntSS {
    lines: Vec<GeomIntIntSSLine>,
    points: Vec<GeomIntIntSSPoint>,
    is_done: bool,
    tolerance: f64,
}

impl GeomIntIntSS {
    pub fn new(tolerance: f64) -> Self {
        Self {
            lines: Vec::new(),
            points: Vec::new(),
            is_done: false,
            tolerance,
        }
    }

    /// Stub: marks done, produces no lines or points
    pub fn perform(&mut self) {
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    /// 1-based index
    pub fn line(&self, idx: usize) -> Option<&GeomIntIntSSLine> {
        if idx == 0 {
            return None;
        }
        self.lines.get(idx - 1)
    }

    /// 1-based index
    pub fn point(&self, idx: usize) -> Option<&GeomIntIntSSPoint> {
        if idx == 0 {
            return None;
        }
        self.points.get(idx - 1)
    }

    /// For testing: manually insert a line
    pub fn add_line(&mut self, l: GeomIntIntSSLine) {
        self.lines.push(l);
    }
}
