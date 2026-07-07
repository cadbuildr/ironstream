// FILE: b_rep_fill_sequence_of_edge_face_and_order.rs
// occt: BRepFill_SequenceOfEdgeFaceAndOrder

//! Deprecated type alias for backward compatibility.
//! A sequence of edge, face, and order triples.

/// Represents an edge, face, and order combination.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgeFaceAndOrder {
    pub edge_id: usize,
    pub face_id: usize,
    pub order: i32,
}

impl EdgeFaceAndOrder {
    pub fn new(edge_id: usize, face_id: usize, order: i32) -> Self {
        Self {
            edge_id,
            face_id,
            order,
        }
    }
}

/// A sequence of EdgeFaceAndOrder elements.
pub type BRepFillSequenceOfEdgeFaceAndOrder = Vec<EdgeFaceAndOrder>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_creation() {
        let elem = EdgeFaceAndOrder::new(1, 2, 3);
        assert_eq!(elem.edge_id, 1);
        assert_eq!(elem.face_id, 2);
        assert_eq!(elem.order, 3);
    }

    #[test]
    fn test_sequence_creation() {
        let mut seq: BRepFillSequenceOfEdgeFaceAndOrder = Vec::new();
        seq.push(EdgeFaceAndOrder::new(1, 10, 0));
        seq.push(EdgeFaceAndOrder::new(2, 20, 1));
        seq.push(EdgeFaceAndOrder::new(3, 30, 2));

        assert_eq!(seq.len(), 3);
        assert_eq!(seq[1].face_id, 20);
        assert_eq!(seq[2].order, 2);
    }

    #[test]
    fn test_sequence_iteration() {
        let mut seq: BRepFillSequenceOfEdgeFaceAndOrder = Vec::new();
        for i in 0..5 {
            seq.push(EdgeFaceAndOrder::new(i, i * 10, i as i32));
        }

        let total_order: i32 = seq.iter().map(|e| e.order).sum();
        assert_eq!(total_order, 0 + 1 + 2 + 3 + 4);
    }
}
