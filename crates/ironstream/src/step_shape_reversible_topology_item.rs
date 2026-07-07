// FILE: step_shape_reversible_topology_item.rs
// occt: StepShape_ReversibleTopologyItem

use std::sync::Arc;

/// Placeholder types for the various topology items
pub struct Edge {
    id: usize,
}

pub struct Path {
    id: usize,
}

pub struct Face {
    id: usize,
}

pub struct FaceBound {
    id: usize,
}

pub struct ClosedShell {
    id: usize,
}

pub struct OpenShell {
    id: usize,
}

/// A discriminated union type representing a reversible topology item.
/// Can be one of: Edge, Path, Face, FaceBound, ClosedShell, or OpenShell.
pub enum ReversibleTopologyItem {
    /// Case 1: Edge
    Edge(Arc<Edge>),
    /// Case 2: Path
    Path(Arc<Path>),
    /// Case 3: Face
    Face(Arc<Face>),
    /// Case 4: FaceBound
    FaceBound(Arc<FaceBound>),
    /// Case 5: ClosedShell
    ClosedShell(Arc<ClosedShell>),
    /// Case 6: OpenShell
    OpenShell(Arc<OpenShell>),
}

impl ReversibleTopologyItem {
    /// Create a new ReversibleTopologyItem from an Edge
    pub fn from_edge(edge: Arc<Edge>) -> Self {
        ReversibleTopologyItem::Edge(edge)
    }

    /// Create a new ReversibleTopologyItem from a Path
    pub fn from_path(path: Arc<Path>) -> Self {
        ReversibleTopologyItem::Path(path)
    }

    /// Create a new ReversibleTopologyItem from a Face
    pub fn from_face(face: Arc<Face>) -> Self {
        ReversibleTopologyItem::Face(face)
    }

    /// Create a new ReversibleTopologyItem from a FaceBound
    pub fn from_face_bound(face_bound: Arc<FaceBound>) -> Self {
        ReversibleTopologyItem::FaceBound(face_bound)
    }

    /// Create a new ReversibleTopologyItem from a ClosedShell
    pub fn from_closed_shell(shell: Arc<ClosedShell>) -> Self {
        ReversibleTopologyItem::ClosedShell(shell)
    }

    /// Create a new ReversibleTopologyItem from an OpenShell
    pub fn from_open_shell(shell: Arc<OpenShell>) -> Self {
        ReversibleTopologyItem::OpenShell(shell)
    }

    /// Get the case number (kind) of this item
    /// 1 -> Edge
    /// 2 -> Path
    /// 3 -> Face
    /// 4 -> FaceBound
    /// 5 -> ClosedShell
    /// 6 -> OpenShell
    pub fn case_num(&self) -> usize {
        match self {
            ReversibleTopologyItem::Edge(_) => 1,
            ReversibleTopologyItem::Path(_) => 2,
            ReversibleTopologyItem::Face(_) => 3,
            ReversibleTopologyItem::FaceBound(_) => 4,
            ReversibleTopologyItem::ClosedShell(_) => 5,
            ReversibleTopologyItem::OpenShell(_) => 6,
        }
    }

    /// Try to get as an Edge, returns None if not an Edge
    pub fn as_edge(&self) -> Option<&Arc<Edge>> {
        match self {
            ReversibleTopologyItem::Edge(e) => Some(e),
            _ => None,
        }
    }

    /// Try to get as a Path, returns None if not a Path
    pub fn as_path(&self) -> Option<&Arc<Path>> {
        match self {
            ReversibleTopologyItem::Path(p) => Some(p),
            _ => None,
        }
    }

    /// Try to get as a Face, returns None if not a Face
    pub fn as_face(&self) -> Option<&Arc<Face>> {
        match self {
            ReversibleTopologyItem::Face(f) => Some(f),
            _ => None,
        }
    }

    /// Try to get as a FaceBound, returns None if not a FaceBound
    pub fn as_face_bound(&self) -> Option<&Arc<FaceBound>> {
        match self {
            ReversibleTopologyItem::FaceBound(fb) => Some(fb),
            _ => None,
        }
    }

    /// Try to get as a ClosedShell, returns None if not a ClosedShell
    pub fn as_closed_shell(&self) -> Option<&Arc<ClosedShell>> {
        match self {
            ReversibleTopologyItem::ClosedShell(cs) => Some(cs),
            _ => None,
        }
    }

    /// Try to get as an OpenShell, returns None if not an OpenShell
    pub fn as_open_shell(&self) -> Option<&Arc<OpenShell>> {
        match self {
            ReversibleTopologyItem::OpenShell(os) => Some(os),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num_edge() {
        let edge = Arc::new(Edge { id: 1 });
        let item = ReversibleTopologyItem::from_edge(edge);
        assert_eq!(item.case_num(), 1);
    }

    #[test]
    fn test_case_num_path() {
        let path = Arc::new(Path { id: 2 });
        let item = ReversibleTopologyItem::from_path(path);
        assert_eq!(item.case_num(), 2);
    }

    #[test]
    fn test_case_num_face() {
        let face = Arc::new(Face { id: 3 });
        let item = ReversibleTopologyItem::from_face(face);
        assert_eq!(item.case_num(), 3);
    }

    #[test]
    fn test_case_num_face_bound() {
        let fb = Arc::new(FaceBound { id: 4 });
        let item = ReversibleTopologyItem::from_face_bound(fb);
        assert_eq!(item.case_num(), 4);
    }

    #[test]
    fn test_case_num_closed_shell() {
        let cs = Arc::new(ClosedShell { id: 5 });
        let item = ReversibleTopologyItem::from_closed_shell(cs);
        assert_eq!(item.case_num(), 5);
    }

    #[test]
    fn test_case_num_open_shell() {
        let os = Arc::new(OpenShell { id: 6 });
        let item = ReversibleTopologyItem::from_open_shell(os);
        assert_eq!(item.case_num(), 6);
    }

    #[test]
    fn test_as_edge() {
        let edge = Arc::new(Edge { id: 10 });
        let item = ReversibleTopologyItem::from_edge(edge.clone());
        assert!(item.as_edge().is_some());
        assert_eq!(item.as_edge().unwrap().id, 10);
        assert!(item.as_path().is_none());
    }

    #[test]
    fn test_as_path() {
        let path = Arc::new(Path { id: 20 });
        let item = ReversibleTopologyItem::from_path(path.clone());
        assert!(item.as_path().is_some());
        assert_eq!(item.as_path().unwrap().id, 20);
        assert!(item.as_edge().is_none());
    }

    #[test]
    fn test_as_face() {
        let face = Arc::new(Face { id: 30 });
        let item = ReversibleTopologyItem::from_face(face.clone());
        assert!(item.as_face().is_some());
        assert_eq!(item.as_face().unwrap().id, 30);
    }

    #[test]
    fn test_discriminated_union() {
        let edge = ReversibleTopologyItem::from_edge(Arc::new(Edge { id: 1 }));
        let path = ReversibleTopologyItem::from_path(Arc::new(Path { id: 2 }));

        assert_eq!(edge.case_num(), 1);
        assert_eq!(path.case_num(), 2);
    }
}
