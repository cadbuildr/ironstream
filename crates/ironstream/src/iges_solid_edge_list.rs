// FILE: iges_solid_edge_list.rs
// occt: IGESSolid_EdgeList

//! Edge List entity (IGES Type 504, Form 1).
//!
//! Contains one or more edge tuples, where each edge connects two vertices
//! from specified vertex lists via a curve.

/// Vertex list reference
#[derive(Clone)]
pub struct VertexList {
    id: usize,
}

impl VertexList {
    pub fn new(id: usize) -> Self {
        VertexList { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

/// Curve entity reference
#[derive(Clone)]
pub struct CurveEntity {
    id: usize,
}

impl CurveEntity {
    pub fn new(id: usize) -> Self {
        CurveEntity { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

/// Edge definition: curve with start and end vertex references
#[derive(Clone)]
pub struct Edge {
    curve: CurveEntity,
    start_vertex_list: VertexList,
    start_vertex_index: usize,
    end_vertex_list: VertexList,
    end_vertex_index: usize,
}

impl Edge {
    pub fn new(
        curve: CurveEntity,
        start_vlist: VertexList,
        start_idx: usize,
        end_vlist: VertexList,
        end_idx: usize,
    ) -> Self {
        Edge {
            curve,
            start_vertex_list: start_vlist,
            start_vertex_index: start_idx,
            end_vertex_list: end_vlist,
            end_vertex_index: end_idx,
        }
    }

    pub fn curve(&self) -> &CurveEntity {
        &self.curve
    }

    pub fn start_vertex_list(&self) -> &VertexList {
        &self.start_vertex_list
    }

    pub fn start_vertex_index(&self) -> usize {
        self.start_vertex_index
    }

    pub fn end_vertex_list(&self) -> &VertexList {
        &self.end_vertex_list
    }

    pub fn end_vertex_index(&self) -> usize {
        self.end_vertex_index
    }
}

/// Edge list containing edges
pub struct IGESSolidEdgeList {
    edges: Vec<Edge>,
}

impl IGESSolidEdgeList {
    /// Creates a new empty edge list
    pub fn new() -> Self {
        IGESSolidEdgeList {
            edges: Vec::new(),
        }
    }

    /// Initializes the edge list with array of edge components
    pub fn init(
        &mut self,
        curves: Vec<CurveEntity>,
        start_vertex_lists: Vec<VertexList>,
        start_vertex_indices: Vec<usize>,
        end_vertex_lists: Vec<VertexList>,
        end_vertex_indices: Vec<usize>,
    ) -> Result<(), String> {
        // Validate array sizes match
        let len = curves.len();
        if start_vertex_lists.len() != len
            || start_vertex_indices.len() != len
            || end_vertex_lists.len() != len
            || end_vertex_indices.len() != len
        {
            return Err(
                "IGESSolid_EdgeList: all arrays must have the same length".to_string(),
            );
        }

        // Build edges
        self.edges.clear();
        for i in 0..len {
            let edge = Edge::new(
                curves[i].clone(),
                start_vertex_lists[i].clone(),
                start_vertex_indices[i],
                end_vertex_lists[i].clone(),
                end_vertex_indices[i],
            );
            self.edges.push(edge);
        }

        Ok(())
    }

    /// Returns the number of edges in the list
    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }

    /// Returns the num-th curve (0-indexed)
    pub fn curve(&self, num: usize) -> Option<&CurveEntity> {
        self.edges.get(num).map(|e| &e.curve)
    }

    /// Returns the num-th start vertex list
    pub fn start_vertex_list(&self, num: usize) -> Option<&VertexList> {
        self.edges.get(num).map(|e| &e.start_vertex_list)
    }

    /// Returns the index of the num-th start vertex
    pub fn start_vertex_index(&self, num: usize) -> Option<usize> {
        self.edges.get(num).map(|e| e.start_vertex_index)
    }

    /// Returns the num-th end vertex list
    pub fn end_vertex_list(&self, num: usize) -> Option<&VertexList> {
        self.edges.get(num).map(|e| &e.end_vertex_list)
    }

    /// Returns the index of the num-th end vertex
    pub fn end_vertex_index(&self, num: usize) -> Option<usize> {
        self.edges.get(num).map(|e| e.end_vertex_index)
    }

    /// Returns all edges
    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_list_creation() {
        let vl = VertexList::new(42);
        assert_eq!(vl.id(), 42);
        assert!(!vl.is_null());
    }

    #[test]
    fn test_vertex_list_null() {
        let vl = VertexList::new(0);
        assert!(vl.is_null());
    }

    #[test]
    fn test_curve_entity_creation() {
        let ce = CurveEntity::new(10);
        assert_eq!(ce.id(), 10);
        assert!(!ce.is_null());
    }

    #[test]
    fn test_curve_entity_null() {
        let ce = CurveEntity::new(0);
        assert!(ce.is_null());
    }

    #[test]
    fn test_edge_creation() {
        let curve = CurveEntity::new(1);
        let sv_list = VertexList::new(2);
        let ev_list = VertexList::new(3);

        let edge = Edge::new(curve, sv_list, 5, ev_list, 7);

        assert_eq!(edge.curve().id(), 1);
        assert_eq!(edge.start_vertex_list().id(), 2);
        assert_eq!(edge.start_vertex_index(), 5);
        assert_eq!(edge.end_vertex_list().id(), 3);
        assert_eq!(edge.end_vertex_index(), 7);
    }

    #[test]
    fn test_edge_list_creation() {
        let el = IGESSolidEdgeList::new();
        assert_eq!(el.nb_edges(), 0);
    }

    #[test]
    fn test_edge_list_init_valid() {
        let mut el = IGESSolidEdgeList::new();
        let curves = vec![CurveEntity::new(1), CurveEntity::new(2)];
        let sv_lists = vec![VertexList::new(10), VertexList::new(11)];
        let sv_indices = vec![1, 2];
        let ev_lists = vec![VertexList::new(20), VertexList::new(21)];
        let ev_indices = vec![3, 4];

        let result = el.init(curves, sv_lists, sv_indices, ev_lists, ev_indices);

        assert!(result.is_ok());
        assert_eq!(el.nb_edges(), 2);
    }

    #[test]
    fn test_edge_list_init_mismatched_size() {
        let mut el = IGESSolidEdgeList::new();
        let curves = vec![CurveEntity::new(1)];
        let sv_lists = vec![VertexList::new(10), VertexList::new(11)];
        let sv_indices = vec![1];
        let ev_lists = vec![VertexList::new(20)];
        let ev_indices = vec![3];

        let result = el.init(curves, sv_lists, sv_indices, ev_lists, ev_indices);

        assert!(result.is_err());
    }

    #[test]
    fn test_edge_list_curve() {
        let mut el = IGESSolidEdgeList::new();
        let curves = vec![CurveEntity::new(42)];
        let sv_lists = vec![VertexList::new(1)];
        let sv_indices = vec![1];
        let ev_lists = vec![VertexList::new(2)];
        let ev_indices = vec![2];

        el.init(curves, sv_lists, sv_indices, ev_lists, ev_indices)
            .unwrap();

        let curve = el.curve(0);
        assert!(curve.is_some());
        assert_eq!(curve.unwrap().id(), 42);
    }

    #[test]
    fn test_edge_list_vertex_lists() {
        let mut el = IGESSolidEdgeList::new();
        let curves = vec![CurveEntity::new(1)];
        let sv_lists = vec![VertexList::new(100)];
        let sv_indices = vec![5];
        let ev_lists = vec![VertexList::new(200)];
        let ev_indices = vec![7];

        el.init(curves, sv_lists, sv_indices, ev_lists, ev_indices)
            .unwrap();

        assert_eq!(el.start_vertex_index(0), Some(5));
        assert_eq!(el.end_vertex_index(0), Some(7));
        assert_eq!(el.start_vertex_list(0).unwrap().id(), 100);
        assert_eq!(el.end_vertex_list(0).unwrap().id(), 200);
    }

    #[test]
    fn test_edge_list_out_of_bounds() {
        let el = IGESSolidEdgeList::new();

        assert!(el.curve(0).is_none());
        assert!(el.start_vertex_list(0).is_none());
    }
}
