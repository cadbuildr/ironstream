// FILE: wire_exp.rs
// occt-ref: BRepTools_WireExplorer, BRepTools_ReShape

/// Orientation of an edge within a wire.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeOrientation {
    Forward,
    Reversed,
    Internal,
    External,
}

impl Default for EdgeOrientation {
    fn default() -> Self { Self::Forward }
}

/// Status of a ReShape substitution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReShapeStatus {
    Ok,
    NoChange,
    Removed,
    Replaced,
    Error,
}

impl Default for ReShapeStatus {
    fn default() -> Self { Self::Ok }
}

impl ReShapeStatus {
    pub fn is_ok(&self) -> bool { *self == ReShapeStatus::Ok || *self == ReShapeStatus::Replaced }
}

/// A minimal description of an edge for exploration purposes.
#[derive(Clone, Debug)]
pub struct ExploredEdge {
    pub id: u32,
    pub orientation: EdgeOrientation,
    pub first_vertex_id: u32,
    pub last_vertex_id: u32,
    pub is_degenerated: bool,
}

impl ExploredEdge {
    pub fn new(id: u32, orientation: EdgeOrientation, first: u32, last: u32) -> Self {
        Self { id, orientation, first_vertex_id: first, last_vertex_id: last, is_degenerated: false }
    }
}

// occt-ref: BRepTools_WireExplorer // — traverses edges of a wire in order
#[derive(Clone, Debug, Default)]
pub struct WireExplorer {
    pub wire_id: u32,
    pub face_id: Option<u32>,
    edges: Vec<ExploredEdge>,
    cursor: usize,
}

impl WireExplorer {
    pub fn new() -> Self { Self::default() }

    /// Initialize exploration of a wire (optionally constrained to a face).
    pub fn init(&mut self, wire_id: u32, face_id: Option<u32>, edges: Vec<ExploredEdge>) {
        self.wire_id = wire_id;
        self.face_id = face_id;
        self.edges = edges;
        self.cursor = 0;
    }

    pub fn more(&self) -> bool { self.cursor < self.edges.len() }

    pub fn next(&mut self) {
        if self.cursor < self.edges.len() { self.cursor += 1; }
    }

    pub fn current_edge(&self) -> Option<&ExploredEdge> {
        self.edges.get(self.cursor)
    }

    pub fn current_vertex(&self) -> Option<u32> {
        self.current_edge().map(|e| e.first_vertex_id)
    }

    pub fn orientation(&self) -> Option<EdgeOrientation> {
        self.current_edge().map(|e| e.orientation)
    }

    pub fn nb_edges(&self) -> usize { self.edges.len() }
    pub fn is_closed(&self) -> bool {
        if self.edges.len() < 2 { return false; }
        let first = &self.edges[0];
        let last = &self.edges[self.edges.len() - 1];
        first.first_vertex_id == last.last_vertex_id
    }

    pub fn clear(&mut self) {
        self.edges.clear();
        self.cursor = 0;
    }

    pub fn collect_edges(&self) -> Vec<u32> {
        self.edges.iter().map(|e| e.id).collect()
    }
}

/// A substitution rule for ReShape.
#[derive(Clone, Debug)]
pub struct Substitution {
    pub old_id: u32,
    pub new_id: Option<u32>,   // None = removal
    pub orientation: EdgeOrientation,
}

// occt-ref: BRepTools_ReShape // — replaces or removes sub-shapes in a shape
#[derive(Clone, Debug, Default)]
pub struct ReShape {
    pub substitutions: Vec<Substitution>,
}

impl ReShape {
    pub fn new() -> Self { Self::default() }

    pub fn replace(&mut self, old_id: u32, new_id: u32) {
        self.substitutions.push(Substitution { old_id, new_id: Some(new_id), orientation: EdgeOrientation::Forward });
    }

    pub fn replace_with_orientation(&mut self, old_id: u32, new_id: u32, orientation: EdgeOrientation) {
        self.substitutions.push(Substitution { old_id, new_id: Some(new_id), orientation });
    }

    pub fn remove(&mut self, old_id: u32) {
        self.substitutions.push(Substitution { old_id, new_id: None, orientation: EdgeOrientation::Forward });
    }

    pub fn apply(&self, id: u32) -> ReShapeStatus {
        if let Some(sub) = self.substitutions.iter().rev().find(|s| s.old_id == id) {
            if sub.new_id.is_some() { ReShapeStatus::Replaced } else { ReShapeStatus::Removed }
        } else {
            ReShapeStatus::NoChange
        }
    }

    pub fn has_replace(&self, id: u32) -> bool {
        self.substitutions.iter().any(|s| s.old_id == id && s.new_id.is_some())
    }

    pub fn has_remove(&self, id: u32) -> bool {
        self.substitutions.iter().any(|s| s.old_id == id && s.new_id.is_none())
    }

    pub fn new_id(&self, old_id: u32) -> Option<u32> {
        self.substitutions.iter().rev().find(|s| s.old_id == old_id)?.new_id
    }

    pub fn clear(&mut self) { self.substitutions.clear(); }
    pub fn nb_substitutions(&self) -> usize { self.substitutions.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_explorer_traversal() {
        let edges = vec![
            ExploredEdge::new(1, EdgeOrientation::Forward, 10, 11),
            ExploredEdge::new(2, EdgeOrientation::Reversed, 11, 12),
            ExploredEdge::new(3, EdgeOrientation::Forward, 12, 10),
        ];
        let mut exp = WireExplorer::new();
        exp.init(100, None, edges);
        assert!(exp.more());
        assert_eq!(exp.current_edge().unwrap().id, 1);
        exp.next();
        assert_eq!(exp.current_edge().unwrap().id, 2);
        exp.next();
        exp.next();
        assert!(!exp.more());
    }

    #[test]
    fn wire_explorer_closed() {
        let edges = vec![
            ExploredEdge::new(1, EdgeOrientation::Forward, 5, 6),
            ExploredEdge::new(2, EdgeOrientation::Forward, 6, 5),
        ];
        let mut exp = WireExplorer::new();
        exp.init(99, Some(50), edges);
        assert!(exp.is_closed());
    }

    #[test]
    fn wire_explorer_collect_edges() {
        let edges = vec![
            ExploredEdge::new(10, EdgeOrientation::Forward, 1, 2),
            ExploredEdge::new(20, EdgeOrientation::Forward, 2, 3),
        ];
        let mut exp = WireExplorer::new();
        exp.init(1, None, edges);
        assert_eq!(exp.collect_edges(), vec![10, 20]);
    }

    #[test]
    fn reshape_replace_and_apply() {
        let mut rs = ReShape::new();
        rs.replace(10, 20);
        assert_eq!(rs.apply(10), ReShapeStatus::Replaced);
        assert_eq!(rs.new_id(10), Some(20));
        assert!(rs.has_replace(10));
    }

    #[test]
    fn reshape_remove() {
        let mut rs = ReShape::new();
        rs.remove(5);
        assert_eq!(rs.apply(5), ReShapeStatus::Removed);
        assert!(rs.has_remove(5));
        assert!(!rs.has_replace(5));
    }

    #[test]
    fn reshape_no_change() {
        let rs = ReShape::new();
        assert_eq!(rs.apply(99), ReShapeStatus::NoChange);
    }
}
