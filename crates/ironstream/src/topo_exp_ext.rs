// FILE: topo_exp_ext.rs
// occt: TopExp_Explorer extended, BRepTools_WireExplorer, TopExp

/// Orientation of a shape in its parent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Forward,
    Reversed,
    Internal,
    External,
}

impl Default for Orientation {
    fn default() -> Self { Self::Forward }
}

impl Orientation {
    pub fn reverse(self) -> Self {
        match self {
            Self::Forward => Self::Reversed,
            Self::Reversed => Self::Forward,
            o => o,
        }
    }

    pub fn is_forward(self) -> bool { self == Self::Forward }
    pub fn is_reversed(self) -> bool { self == Self::Reversed }
}

/// Represents a shape entry yielded during exploration.
#[derive(Clone, Debug)]
pub struct ExploredShape {
    pub shape_id: u32,
    pub shape_type: u8,  // 0=compound,1=solid,2=shell,3=face,4=wire,5=edge,6=vertex
    pub orientation: Orientation,
    pub depth: usize,
}

// occt: TopExp_Explorer extended — iterates sub-shapes of a given type
#[derive(Clone, Debug)]
pub struct TopExpExplorerExt {
    pub shape_id: u32,
    pub seek_type: u8,
    pub avoid_type: Option<u8>,
    pub entries: Vec<ExploredShape>,
    pub current: usize,
    pub has_more: bool,
}

impl TopExpExplorerExt {
    pub fn new(shape_id: u32, seek_type: u8) -> Self {
        Self {
            shape_id,
            seek_type,
            avoid_type: None,
            entries: Vec::new(),
            current: 0,
            has_more: false,
        }
    }

    pub fn with_avoid(mut self, avoid: u8) -> Self { self.avoid_type = Some(avoid); self }

    /// Populate entries from a provided list of sub-shapes.
    pub fn init(&mut self, sub_shapes: Vec<(u32, u8, Orientation)>) {
        self.entries = sub_shapes.into_iter()
            .filter(|(_, t, _)| *t == self.seek_type)
            .filter(|(_, t, _)| self.avoid_type.map_or(true, |a| *t != a))
            .map(|(id, t, o)| ExploredShape { shape_id: id, shape_type: t, orientation: o, depth: 1 })
            .collect();
        self.current = 0;
        self.has_more = !self.entries.is_empty();
    }

    pub fn more(&self) -> bool { self.current < self.entries.len() }
    pub fn next(&mut self) { if self.more() { self.current += 1; } }
    pub fn current(&self) -> Option<&ExploredShape> { self.entries.get(self.current) }
    pub fn reset(&mut self) { self.current = 0; }
    pub fn nb_entries(&self) -> usize { self.entries.len() }

    pub fn collect_all(&mut self) -> Vec<u32> {
        self.reset();
        let mut ids = Vec::new();
        while self.more() {
            if let Some(e) = self.current() { ids.push(e.shape_id); }
            self.next();
        }
        ids
    }
}

// occt: BRepTools_WireExplorer — iterates edges of a wire in order
#[derive(Clone, Debug)]
pub struct WireExplorerExt {
    pub wire_id: u32,
    pub edges: Vec<(u32, Orientation)>,
    pub current: usize,
}

impl WireExplorerExt {
    pub fn new(wire_id: u32) -> Self {
        Self { wire_id, edges: Vec::new(), current: 0 }
    }

    pub fn init(&mut self, ordered_edges: Vec<(u32, Orientation)>) {
        self.edges = ordered_edges;
        self.current = 0;
    }

    pub fn more(&self) -> bool { self.current < self.edges.len() }
    pub fn next(&mut self) { if self.more() { self.current += 1; } }

    pub fn current_edge(&self) -> Option<u32> {
        self.edges.get(self.current).map(|(id, _)| *id)
    }

    pub fn current_orientation(&self) -> Option<Orientation> {
        self.edges.get(self.current).map(|(_, o)| *o)
    }

    pub fn nb_edges(&self) -> usize { self.edges.len() }
    pub fn reset(&mut self) { self.current = 0; }
}

// occt: TopExp utilities — static helpers
pub struct TopExp;

impl TopExp {
    /// Get all vertices from an edge (returns two vertex ids, or fewer if degenerate).
    pub fn vertices_of_edge(edge_id: u32) -> (u32, u32) {
        // Stub: derive vertex ids from edge id
        (edge_id * 10, edge_id * 10 + 1)
    }

    /// Get all edges of a wire.
    pub fn map_shapes_and_ancestors(
        shape_id: u32,
        child_type: u8,
        parent_type: u8,
    ) -> Vec<(u32, Vec<u32>)> {
        if child_type >= parent_type { return Vec::new(); }
        // Stub: return two mock entries
        vec![
            (shape_id * 10 + 1, vec![shape_id]),
            (shape_id * 10 + 2, vec![shape_id]),
        ]
    }

    /// Get the common vertices between two edges.
    pub fn common_vertex(edge1: u32, edge2: u32) -> Option<u32> {
        let (v1a, v1b) = Self::vertices_of_edge(edge1);
        let (v2a, v2b) = Self::vertices_of_edge(edge2);
        if v1a == v2a || v1a == v2b { Some(v1a) }
        else if v1b == v2a || v1b == v2b { Some(v1b) }
        else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_reverse() {
        assert_eq!(Orientation::Forward.reverse(), Orientation::Reversed);
        assert_eq!(Orientation::Internal.reverse(), Orientation::Internal);
    }

    #[test]
    fn explorer_filter_by_type() {
        let mut exp = TopExpExplorerExt::new(1, 3); // seek faces (type 3)
        exp.init(vec![
            (10, 3, Orientation::Forward),
            (11, 5, Orientation::Forward), // edge - filtered out
            (12, 3, Orientation::Reversed),
        ]);
        assert_eq!(exp.nb_entries(), 2);
        let ids = exp.collect_all();
        assert_eq!(ids, vec![10, 12]);
    }

    #[test]
    fn explorer_more_next() {
        let mut exp = TopExpExplorerExt::new(1, 5);
        exp.init(vec![(20, 5, Orientation::Forward), (21, 5, Orientation::Forward)]);
        assert!(exp.more());
        exp.next();
        assert_eq!(exp.current().map(|e| e.shape_id), Some(21));
        exp.next();
        assert!(!exp.more());
    }

    #[test]
    fn wire_explorer_ordered_edges() {
        let mut we = WireExplorerExt::new(99);
        we.init(vec![(1, Orientation::Forward), (2, Orientation::Reversed)]);
        assert_eq!(we.nb_edges(), 2);
        assert_eq!(we.current_edge(), Some(1));
        we.next();
        assert_eq!(we.current_orientation(), Some(Orientation::Reversed));
    }

    #[test]
    fn top_exp_vertices_of_edge() {
        let (v1, v2) = TopExp::vertices_of_edge(5);
        assert_eq!(v1, 50);
        assert_eq!(v2, 51);
    }

    #[test]
    fn top_exp_common_vertex() {
        // edge 1 has vertices 10, 11; edge 2 has vertices 20, 21 — no common vertex
        let r = TopExp::common_vertex(1, 2);
        assert!(r.is_none());
    }
}
