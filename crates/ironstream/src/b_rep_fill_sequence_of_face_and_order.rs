// FILE: b_rep_fill_sequence_of_face_and_order.rs
// occt: BRepFill_SequenceOfFaceAndOrder

//! Deprecated type alias for backward compatibility.
//! A sequence of face and order pairs.

/// Represents a face and order pair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaceAndOrder {
    pub face_id: usize,
    pub order: i32,
}

impl FaceAndOrder {
    pub fn new(face_id: usize, order: i32) -> Self {
        Self { face_id, order }
    }
}

/// A sequence of FaceAndOrder elements.
pub type BRepFillSequenceOfFaceAndOrder = Vec<FaceAndOrder>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_creation() {
        let elem = FaceAndOrder::new(1, 2);
        assert_eq!(elem.face_id, 1);
        assert_eq!(elem.order, 2);
    }

    #[test]
    fn test_sequence_creation() {
        let mut seq: BRepFillSequenceOfFaceAndOrder = Vec::new();
        seq.push(FaceAndOrder::new(10, 0));
        seq.push(FaceAndOrder::new(20, 1));
        seq.push(FaceAndOrder::new(30, 2));

        assert_eq!(seq.len(), 3);
        assert_eq!(seq[1].face_id, 20);
        assert_eq!(seq[2].order, 2);
    }

    #[test]
    fn test_sequence_iteration() {
        let mut seq: BRepFillSequenceOfFaceAndOrder = Vec::new();
        for i in 0..5 {
            seq.push(FaceAndOrder::new(i * 10, i as i32));
        }

        let total_order: i32 = seq.iter().map(|f| f.order).sum();
        assert_eq!(total_order, 0 + 1 + 2 + 3 + 4);
    }
}
