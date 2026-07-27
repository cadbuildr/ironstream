// FILE: brep_wire_tools.rs
// occt: BRepCheck_Wire
// occt-ref: BRepTools_WireExplorer, ShapeAnalysis_Wire

// occt-ref: BRepTools_WireExplorer
/// Iterates the edges of a wire in topological order.
#[derive(Clone, Debug)]
pub struct WireExplorer {
    pub edges: Vec<WireEdge>,
    index: usize,
}

#[derive(Clone, Debug)]
pub struct WireEdge {
    pub id: usize,
    pub start: [f64; 3],
    pub end: [f64; 3],
    pub is_forward: bool,
}

impl WireEdge {
    pub fn new(id: usize, start: [f64; 3], end: [f64; 3]) -> Self {
        Self { id, start, end, is_forward: true }
    }

    pub fn length(&self) -> f64 {
        let dx = self.end[0]-self.start[0];
        let dy = self.end[1]-self.start[1];
        let dz = self.end[2]-self.start[2];
        (dx*dx+dy*dy+dz*dz).sqrt()
    }

    pub fn reversed(&self) -> Self {
        WireEdge { id: self.id, start: self.end, end: self.start, is_forward: !self.is_forward }
    }
}

impl WireExplorer {
    pub fn new(edges: Vec<WireEdge>) -> Self { Self { edges, index: 0 } }
    pub fn more(&self) -> bool { self.index < self.edges.len() }
    pub fn next(&mut self) { self.index += 1; }
    pub fn current(&self) -> Option<&WireEdge> { self.edges.get(self.index) }
    pub fn nb_edges(&self) -> usize { self.edges.len() }
    pub fn reset(&mut self) { self.index = 0; }
}

impl Iterator for WireExplorer {
    type Item = WireEdge;
    fn next(&mut self) -> Option<Self::Item> {
        let v = self.edges.get(self.index).cloned();
        self.index += 1;
        v
    }
}

// occt: BRepCheck_Wire
#[derive(Clone, Debug)]
pub struct WireCheck {
    pub edges: Vec<WireEdge>,
    pub errors: Vec<WireError>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WireError {
    NoError,
    NotConnected,
    NotClosed,
    SelfIntersecting,
    LackingPart,
    EdgeWithoutNeighbour,
}

impl WireCheck {
    pub fn new(edges: Vec<WireEdge>) -> Self {
        let mut check = Self { edges, errors: Vec::new() };
        check.perform();
        check
    }

    fn perform(&mut self) {
        if self.edges.is_empty() { return; }
        // Check connectivity: end of edge[i] ≈ start of edge[i+1]
        for i in 0..self.edges.len()-1 {
            let e1 = &self.edges[i];
            let e2 = &self.edges[i+1];
            let dx = e1.end[0]-e2.start[0];
            let dy = e1.end[1]-e2.start[1];
            let dz = e1.end[2]-e2.start[2];
            if (dx*dx+dy*dy+dz*dz).sqrt() > 1e-6 {
                self.errors.push(WireError::NotConnected);
                break;
            }
        }
        // Check closed
        let first = &self.edges[0];
        let last = &self.edges[self.edges.len()-1];
        let dx = last.end[0]-first.start[0];
        let dy = last.end[1]-first.start[1];
        let dz = last.end[2]-first.start[2];
        if (dx*dx+dy*dy+dz*dz).sqrt() > 1e-6 {
            if !self.errors.contains(&WireError::NotClosed) {
                self.errors.push(WireError::NotClosed);
            }
        }
    }

    pub fn is_valid(&self) -> bool { self.errors.is_empty() }
    pub fn nb_errors(&self) -> usize { self.errors.len() }
    pub fn total_length(&self) -> f64 { self.edges.iter().map(|e| e.length()).sum() }
}

// occt-ref: ShapeAnalysis_Wire
#[derive(Clone, Debug)]
pub struct WireAnalysis {
    pub edges: Vec<WireEdge>,
}

impl WireAnalysis {
    pub fn new(edges: Vec<WireEdge>) -> Self { Self { edges } }

    pub fn check_order(&self) -> bool {
        self.edges.windows(2).all(|w| {
            let dx = w[0].end[0]-w[1].start[0];
            let dy = w[0].end[1]-w[1].start[1];
            let dz = w[0].end[2]-w[1].start[2];
            (dx*dx+dy*dy+dz*dz).sqrt() < 1e-6
        })
    }

    pub fn is_closed(&self) -> bool {
        if self.edges.is_empty() { return false; }
        let first = &self.edges[0];
        let last = &self.edges[self.edges.len()-1];
        let dx = last.end[0]-first.start[0];
        let dy = last.end[1]-first.start[1];
        let dz = last.end[2]-first.start[2];
        (dx*dx+dy*dy+dz*dz).sqrt() < 1e-6
    }

    pub fn nb_edges(&self) -> usize { self.edges.len() }
    pub fn perimeter(&self) -> f64 { self.edges.iter().map(|e| e.length()).sum() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square_wire() -> Vec<WireEdge> {
        vec![
            WireEdge::new(0, [0.0,0.0,0.0], [1.0,0.0,0.0]),
            WireEdge::new(1, [1.0,0.0,0.0], [1.0,1.0,0.0]),
            WireEdge::new(2, [1.0,1.0,0.0], [0.0,1.0,0.0]),
            WireEdge::new(3, [0.0,1.0,0.0], [0.0,0.0,0.0]),
        ]
    }

    #[test]
    fn wire_explorer() {
        let mut exp = WireExplorer::new(square_wire());
        assert_eq!(exp.nb_edges(), 4);
        assert!(exp.more());
        exp.next();
        assert!(exp.more());
    }

    #[test]
    fn wire_check_valid() {
        let c = WireCheck::new(square_wire());
        assert!(c.is_valid());
        assert!((c.total_length() - 4.0).abs() < 1e-8);
    }

    #[test]
    fn wire_check_open() {
        let edges = vec![
            WireEdge::new(0, [0.0,0.0,0.0], [1.0,0.0,0.0]),
            WireEdge::new(1, [1.0,0.0,0.0], [2.0,0.0,0.0]),
        ];
        let c = WireCheck::new(edges);
        assert!(!c.is_valid());
    }

    #[test]
    fn wire_analysis_closed() {
        let a = WireAnalysis::new(square_wire());
        assert!(a.is_closed());
        assert!(a.check_order());
        assert!((a.perimeter() - 4.0).abs() < 1e-8);
    }

    #[test]
    fn wire_edge_reversed() {
        let e = WireEdge::new(0, [0.0,0.0,0.0], [1.0,0.0,0.0]);
        let r = e.reversed();
        assert!(!r.is_forward);
        assert_eq!(r.start, [1.0,0.0,0.0]);
    }
}
