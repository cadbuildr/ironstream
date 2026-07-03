// FILE: b_rep_fill_face_and_order_o.rs
// occt: BRepFill_FaceAndOrder

/// A data structure containing a face and the continuity order constraint.
#[derive(Clone)]
pub struct BRepFillFaceAndOrder {
    /// The face
    face: Option<Face>,
    /// The geometric continuity order
    order: ContinuityOrder,
}

/// A minimal face representation
#[derive(Clone)]
pub struct Face;

/// Continuity order for constraints
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContinuityOrder {
    /// No continuity
    C0,
    /// Continuous (C0)
    G0,
    /// Geometrically continuous (G1)
    G1,
    /// Geometrically continuous (G2)
    G2,
}

impl BRepFillFaceAndOrder {
    /// Creates an empty face and order structure.
    pub fn new() -> Self {
        Self {
            face: None,
            order: ContinuityOrder::C0,
        }
    }

    /// Creates a face and order structure with the given face and order.
    pub fn with_face_and_order(face: Face, order: ContinuityOrder) -> Self {
        Self {
            face: Some(face),
            order,
        }
    }

    /// Returns the face if present.
    pub fn face(&self) -> Option<&Face> {
        self.face.as_ref()
    }

    /// Returns the continuity order.
    pub fn order(&self) -> ContinuityOrder {
        self.order
    }

    /// Sets the face.
    pub fn set_face(&mut self, face: Face) {
        self.face = Some(face);
    }

    /// Sets the continuity order.
    pub fn set_order(&mut self, order: ContinuityOrder) {
        self.order = order;
    }
}

impl Default for BRepFillFaceAndOrder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_and_order_creation() {
        let fao = BRepFillFaceAndOrder::new();
        assert!(fao.face().is_none());
        assert_eq!(fao.order(), ContinuityOrder::C0);
    }

    #[test]
    fn test_face_and_order_with_values() {
        let face = Face;
        let fao = BRepFillFaceAndOrder::with_face_and_order(face, ContinuityOrder::G1);
        assert!(fao.face().is_some());
        assert_eq!(fao.order(), ContinuityOrder::G1);
    }

    #[test]
    fn test_face_and_order_set_face() {
        let mut fao = BRepFillFaceAndOrder::new();
        fao.set_face(Face);
        assert!(fao.face().is_some());
    }

    #[test]
    fn test_face_and_order_set_order() {
        let mut fao = BRepFillFaceAndOrder::new();
        fao.set_order(ContinuityOrder::G2);
        assert_eq!(fao.order(), ContinuityOrder::G2);
    }

    #[test]
    fn test_face_and_order_set_order_g0() {
        let mut fao = BRepFillFaceAndOrder::new();
        fao.set_order(ContinuityOrder::G0);
        assert_eq!(fao.order(), ContinuityOrder::G0);
    }

    #[test]
    fn test_face_and_order_g1_continuity() {
        let fao = BRepFillFaceAndOrder::with_face_and_order(Face, ContinuityOrder::G1);
        assert_eq!(fao.order(), ContinuityOrder::G1);
    }

    #[test]
    fn test_face_and_order_default() {
        let fao = BRepFillFaceAndOrder::default();
        assert!(fao.face().is_none());
        assert_eq!(fao.order(), ContinuityOrder::C0);
    }

    #[test]
    fn test_continuity_order_equality() {
        assert_eq!(ContinuityOrder::G0, ContinuityOrder::G0);
        assert_eq!(ContinuityOrder::G1, ContinuityOrder::G1);
        assert_ne!(ContinuityOrder::G0, ContinuityOrder::G1);
    }

    #[test]
    fn test_face_and_order_chain_operations() {
        let mut fao = BRepFillFaceAndOrder::new();
        fao.set_face(Face);
        fao.set_order(ContinuityOrder::G2);
        assert!(fao.face().is_some());
        assert_eq!(fao.order(), ContinuityOrder::G2);
    }
}
