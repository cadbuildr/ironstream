// FILE: hlr_test_shape_data.rs
// occt: HLRTest_ShapeData

//! Shape data for HLR test processing.

#[derive(Clone, Debug)]
pub struct ShapeData {
    pub shape_id: usize,
    pub edge_count: usize,
    pub face_count: usize,
}

impl ShapeData {
    pub fn new(shape_id: usize) -> Self {
        ShapeData {
            shape_id,
            edge_count: 0,
            face_count: 0,
        }
    }

    pub fn set_edge_count(&mut self, count: usize) {
        self.edge_count = count;
    }

    pub fn set_face_count(&mut self, count: usize) {
        self.face_count = count;
    }

    pub fn total_elements(&self) -> usize {
        self.edge_count + self.face_count
    }

    pub fn to_string(&self) -> String {
        format!(
            "Shape {}: {} edges, {} faces",
            self.shape_id, self.edge_count, self.face_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let data = ShapeData::new(1);
        assert_eq!(data.shape_id, 1);
        assert_eq!(data.edge_count, 0);
        assert_eq!(data.face_count, 0);
    }

    #[test]
    fn test_set_counts() {
        let mut data = ShapeData::new(1);
        data.set_edge_count(12);
        data.set_face_count(6);

        assert_eq!(data.edge_count, 12);
        assert_eq!(data.face_count, 6);
    }

    #[test]
    fn test_total_elements() {
        let mut data = ShapeData::new(1);
        data.set_edge_count(10);
        data.set_face_count(5);

        assert_eq!(data.total_elements(), 15);
    }

    #[test]
    fn test_to_string() {
        let mut data = ShapeData::new(42);
        data.set_edge_count(12);
        data.set_face_count(6);

        let s = data.to_string();
        assert!(s.contains("42"));
        assert!(s.contains("12 edges"));
        assert!(s.contains("6 faces"));
    }
}
