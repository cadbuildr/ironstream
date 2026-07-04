// FILE: step_visual_tessellated_edge_or_vertex.rs
// occt: StepVisual_TessellatedEdgeOrVertex

pub struct TessellatedEdge;
pub struct Vertex;

pub struct TessellatedEdgeOrVertex {
    case: Option<SelectCase>,
    value: Option<Box<dyn std::any::Any>>,
}

#[derive(Clone, Copy)]
enum SelectCase {
    TessellatedEdge = 1,
    Vertex = 2,
}

impl TessellatedEdgeOrVertex {
    pub fn new() -> Self {
        TessellatedEdgeOrVertex {
            case: None,
            value: None,
        }
    }

    pub fn case_num(&self) -> i32 {
        match self.case {
            Some(SelectCase::TessellatedEdge) => 1,
            Some(SelectCase::Vertex) => 2,
            None => 0,
        }
    }

    pub fn tessellated_edge(&self) -> Option<&TessellatedEdge> {
        if matches!(self.case, Some(SelectCase::TessellatedEdge)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<TessellatedEdge>())
        } else {
            None
        }
    }

    pub fn vertex(&self) -> Option<&Vertex> {
        if matches!(self.case, Some(SelectCase::Vertex)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<Vertex>())
        } else {
            None
        }
    }

    pub fn set_tessellated_edge(&mut self, edge: TessellatedEdge) {
        self.case = Some(SelectCase::TessellatedEdge);
        self.value = Some(Box::new(edge));
    }

    pub fn set_vertex(&mut self, vertex: Vertex) {
        self.case = Some(SelectCase::Vertex);
        self.value = Some(Box::new(vertex));
    }
}

impl Default for TessellatedEdgeOrVertex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let teov = TessellatedEdgeOrVertex::new();
        assert_eq!(teov.case_num(), 0);
        assert!(teov.tessellated_edge().is_none());
        assert!(teov.vertex().is_none());
    }

    #[test]
    fn test_set_edge() {
        let mut teov = TessellatedEdgeOrVertex::new();
        teov.set_tessellated_edge(TessellatedEdge);
        assert_eq!(teov.case_num(), 1);
        assert!(teov.tessellated_edge().is_some());
    }

    #[test]
    fn test_set_vertex() {
        let mut teov = TessellatedEdgeOrVertex::new();
        teov.set_vertex(Vertex);
        assert_eq!(teov.case_num(), 2);
        assert!(teov.vertex().is_some());
    }
}
