// FILE: brep_make_wire.rs
// occt: BRepBuilderAPI_MakeWire, BRepLib_MakeWire, ShapeAnalysis_Wire

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WireError {
    EmptyWire,
    DisconnectedEdge,
    NonManifold,
    Done,
}

/// Edge in a wire build operation.
#[derive(Clone, Debug)]
pub struct WireEdgeDef {
    pub id: usize,
    pub start: [f64; 3],
    pub end: [f64; 3],
    pub reversed: bool,
}

impl WireEdgeDef {
    pub fn new(id: usize, start: [f64; 3], end: [f64; 3]) -> Self {
        Self { id, start, end, reversed: false }
    }

    pub fn length(&self) -> f64 {
        let dx=self.end[0]-self.start[0]; let dy=self.end[1]-self.start[1]; let dz=self.end[2]-self.start[2];
        (dx*dx+dy*dy+dz*dz).sqrt()
    }
}

// occt: BRepBuilderAPI_MakeWire
#[derive(Clone, Debug, Default)]
pub struct MakeWire {
    pub edges: Vec<WireEdgeDef>,
    pub error: Option<WireError>,
    pub is_closed: bool,
}

impl MakeWire {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, edge: WireEdgeDef) {
        if let Some(last) = self.edges.last() {
            let gap = dist(last.end, edge.start);
            if gap > 1e-7 {
                self.error = Some(WireError::DisconnectedEdge);
                return;
            }
        }
        self.edges.push(edge);
        self.update_closed();
    }

    fn update_closed(&mut self) {
        if self.edges.len() < 2 { self.is_closed = false; return; }
        let first = self.edges.first().unwrap().start;
        let last = self.edges.last().unwrap().end;
        self.is_closed = dist(first, last) < 1e-7;
    }

    pub fn is_done(&self) -> bool { self.error.is_none() && !self.edges.is_empty() }
    pub fn nb_edges(&self) -> usize { self.edges.len() }

    pub fn total_length(&self) -> f64 {
        self.edges.iter().map(|e| e.length()).sum()
    }

    pub fn bounding_box(&self) -> Option<([f64; 3], [f64; 3])> {
        if self.edges.is_empty() { return None; }
        let mut mn = [f64::INFINITY; 3]; let mut mx = [f64::NEG_INFINITY; 3];
        for e in &self.edges {
            for pt in [e.start, e.end].iter() {
                for i in 0..3 {
                    mn[i] = mn[i].min(pt[i]);
                    mx[i] = mx[i].max(pt[i]);
                }
            }
        }
        Some((mn, mx))
    }
}

// occt: ShapeAnalysis_Wire gap analysis
pub struct WireGapAnalysis;

impl WireGapAnalysis {
    pub fn max_gap(edges: &[WireEdgeDef]) -> f64 {
        let mut max = 0.0f64;
        for w in edges.windows(2) {
            max = max.max(dist(w[0].end, w[1].start));
        }
        max
    }

    pub fn has_gaps(edges: &[WireEdgeDef], tol: f64) -> bool {
        Self::max_gap(edges) > tol
    }

    pub fn nb_gaps(edges: &[WireEdgeDef], tol: f64) -> usize {
        edges.windows(2).filter(|w| dist(w[0].end, w[1].start) > tol).count()
    }
}

fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx=a[0]-b[0]; let dy=a[1]-b[1]; let dz=a[2]-b[2];
    (dx*dx+dy*dy+dz*dz).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_wire_connected() {
        let mut w = MakeWire::new();
        w.add(WireEdgeDef::new(0, [0.0,0.0,0.0], [1.0,0.0,0.0]));
        w.add(WireEdgeDef::new(1, [1.0,0.0,0.0], [1.0,1.0,0.0]));
        assert!(w.is_done());
        assert_eq!(w.nb_edges(), 2);
        assert!((w.total_length() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn make_wire_disconnected() {
        let mut w = MakeWire::new();
        w.add(WireEdgeDef::new(0, [0.0,0.0,0.0], [1.0,0.0,0.0]));
        w.add(WireEdgeDef::new(1, [5.0,0.0,0.0], [6.0,0.0,0.0]));
        assert!(!w.is_done());
    }

    #[test]
    fn make_wire_closed() {
        let mut w = MakeWire::new();
        w.add(WireEdgeDef::new(0, [0.0,0.0,0.0], [1.0,0.0,0.0]));
        w.add(WireEdgeDef::new(1, [1.0,0.0,0.0], [0.0,0.0,0.0]));
        assert!(w.is_closed);
    }

    #[test]
    fn wire_gap_analysis() {
        let edges = vec![
            WireEdgeDef::new(0, [0.0,0.0,0.0], [1.0,0.0,0.0]),
            WireEdgeDef::new(1, [2.0,0.0,0.0], [3.0,0.0,0.0]),
        ];
        assert!(WireGapAnalysis::has_gaps(&edges, 1e-6));
        assert_eq!(WireGapAnalysis::nb_gaps(&edges, 1e-6), 1);
    }
}
