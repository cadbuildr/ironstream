// FILE: mesh_vs_sensitive_segment.rs
// occt: MeshVS_SensitiveSegment

//! Sensitive line segment for mesh selection.

#[derive(Clone, Debug)]
pub struct SensitiveSegment {
    pub entity_id: u32,
    pub segment_id: u32,
    pub start_vertex: u32,
    pub end_vertex: u32,
}

impl SensitiveSegment {
    pub fn new(entity_id: u32, segment_id: u32) -> Self {
        SensitiveSegment {
            entity_id,
            segment_id,
            start_vertex: 0,
            end_vertex: 0,
        }
    }

    pub fn with_vertices(mut self, start: u32, end: u32) -> Self {
        self.start_vertex = start;
        self.end_vertex = end;
        self
    }

    pub fn has_vertices(&self) -> bool {
        self.start_vertex != self.end_vertex
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_segment_creation() {
        let seg = SensitiveSegment::new(1, 10);
        assert_eq!(seg.entity_id, 1);
        assert_eq!(seg.segment_id, 10);
        assert!(!seg.has_vertices());
    }

    #[test]
    fn sensitive_segment_vertices() {
        let seg = SensitiveSegment::new(1, 10)
            .with_vertices(5, 8);

        assert!(seg.has_vertices());
        assert_eq!(seg.start_vertex, 5);
        assert_eq!(seg.end_vertex, 8);
    }
}
