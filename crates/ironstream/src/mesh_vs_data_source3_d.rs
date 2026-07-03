// FILE: mesh_vs_data_source3_d.rs
// occt: MeshVS_DataSource3D

//! 3D data source for mesh visualization.

#[derive(Clone, Debug)]
pub struct DataSource3D {
    pub source_id: u32,
    pub node_count: usize,
    pub element_count: usize,
}

impl DataSource3D {
    pub fn new(source_id: u32) -> Self {
        DataSource3D {
            source_id,
            node_count: 0,
            element_count: 0,
        }
    }

    pub fn with_counts(source_id: u32, nodes: usize, elements: usize) -> Self {
        DataSource3D {
            source_id,
            node_count: nodes,
            element_count: elements,
        }
    }

    pub fn total_entities(&self) -> usize {
        self.node_count + self.element_count
    }

    pub fn is_empty(&self) -> bool {
        self.node_count == 0 && self.element_count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_source3d_creation() {
        let src = DataSource3D::new(1);
        assert_eq!(src.source_id, 1);
        assert!(src.is_empty());
    }

    #[test]
    fn data_source3d_with_counts() {
        let src = DataSource3D::with_counts(1, 1000, 500);
        assert_eq!(src.node_count, 1000);
        assert_eq!(src.element_count, 500);
        assert_eq!(src.total_entities(), 1500);
        assert!(!src.is_empty());
    }
}
