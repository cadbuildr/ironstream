// FILE: topo_edge.rs
// occt-ref: TopoDS_Edge, TopoDS_EdgeIterator, TopoDsEdgeBuilder

/// Edge: 1D topology element.
// occt-ref: TopoDS_Edge
#[derive(Clone, Debug)]
pub struct TopoDsEdge {
    pub edge_id: u32,
    pub vertices: Vec<u32>,  // [start_vertex, end_vertex]
    pub tolerance: f64,
    pub is_degenerate: bool,
}

impl TopoDsEdge {
    pub fn new(id: u32) -> Self {
        Self {
            edge_id: id,
            vertices: Vec::new(),
            tolerance: 1e-7,
            is_degenerate: false,
        }
    }

    pub fn set_vertices(&mut self, start: u32, end: u32) {
        self.vertices = vec![start, end];
    }

    pub fn start_vertex(&self) -> Option<u32> {
        self.vertices.first().copied()
    }

    pub fn end_vertex(&self) -> Option<u32> {
        self.vertices.get(1).copied()
    }

    pub fn mark_degenerate(&mut self) {
        self.is_degenerate = true;
    }
}

impl Default for TopoDsEdge {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Iterator over edges.
// occt-ref: TopoDS_EdgeIterator
#[derive(Clone, Debug)]
pub struct TopoDsEdgeIterator {
    pub edges: Vec<u32>,
    pub current_index: usize,
}

impl TopoDsEdgeIterator {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_edge(&mut self, edge_id: u32) {
        self.edges.push(edge_id);
    }

    pub fn init(&mut self) {
        self.current_index = 0;
    }

    pub fn more(&self) -> bool {
        self.current_index < self.edges.len()
    }

    pub fn current(&self) -> Option<u32> {
        if self.more() {
            Some(self.edges[self.current_index])
        } else {
            None
        }
    }

    pub fn next(&mut self) {
        if self.more() {
            self.current_index += 1;
        }
    }

    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }
}

impl Default for TopoDsEdgeIterator {
    fn default() -> Self {
        Self::new()
    }
}

/// Edge builder.
/// occt-note: TopoDsEdgeBuilder (simplified as BRepBuilderAPI_MakeEdge)
#[derive(Clone, Debug)]
pub struct TopoDsEdgeBuilder {
    pub edge: TopoDsEdge,
    pub curve_id: Option<u32>,
}

impl TopoDsEdgeBuilder {
    pub fn new(edge_id: u32) -> Self {
        Self {
            edge: TopoDsEdge::new(edge_id),
            curve_id: None,
        }
    }

    pub fn set_curve(&mut self, curve_id: u32) {
        self.curve_id = Some(curve_id);
    }

    pub fn set_vertices(&mut self, start: u32, end: u32) {
        self.edge.set_vertices(start, end);
    }

    pub fn make_edge(&self) -> Option<TopoDsEdge> {
        if self.edge.vertices.len() == 2 && self.curve_id.is_some() {
            Some(self.edge.clone())
        } else {
            None
        }
    }
}

impl Default for TopoDsEdgeBuilder {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topods_edge() {
        let mut e = TopoDsEdge::new(1);
        e.set_vertices(10, 20);
        assert_eq!(e.start_vertex(), Some(10));
        assert_eq!(e.end_vertex(), Some(20));
    }

    #[test]
    fn topods_edge_degenerate() {
        let mut e = TopoDsEdge::new(1);
        assert!(!e.is_degenerate);
        e.mark_degenerate();
        assert!(e.is_degenerate);
    }

    #[test]
    fn topods_edge_iterator() {
        let mut iter = TopoDsEdgeIterator::new();
        iter.add_edge(1);
        iter.add_edge(2);
        iter.add_edge(3);
        assert_eq!(iter.nb_edges(), 3);
        iter.init();
        assert_eq!(iter.current(), Some(1));
        iter.next();
        assert_eq!(iter.current(), Some(2));
    }

    #[test]
    fn edge_iterator_loop() {
        let mut iter = TopoDsEdgeIterator::new();
        for i in 1..=5 {
            iter.add_edge(i);
        }
        iter.init();
        let mut count = 0;
        while iter.more() {
            count += 1;
            iter.next();
        }
        assert_eq!(count, 5);
    }

    #[test]
    fn topods_edge_builder() {
        let mut eb = TopoDsEdgeBuilder::new(1);
        eb.set_vertices(10, 20);
        eb.set_curve(5);
        assert!(eb.make_edge().is_some());
    }
}
