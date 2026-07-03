// FILE: bop_tools_couple_of_shape.rs
// occt: BOPTools_CoupleOfShape

/// Represents a couple (pair) of shapes.
#[derive(Clone, Debug)]
pub struct BopToolsCoupleOfShape {
    shape1: Option<Vec<u8>>, // Placeholder for TopoDS_Shape
    shape2: Option<Vec<u8>>, // Placeholder for TopoDS_Shape
}

impl BopToolsCoupleOfShape {
    /// Create an empty couple of shapes.
    pub fn new() -> Self {
        BopToolsCoupleOfShape {
            shape1: None,
            shape2: None,
        }
    }

    /// Set the first shape.
    pub fn set_shape1(&mut self, shape: Vec<u8>) {
        self.shape1 = Some(shape);
    }

    /// Get the first shape.
    pub fn shape1(&self) -> Option<&[u8]> {
        self.shape1.as_deref()
    }

    /// Set the second shape.
    pub fn set_shape2(&mut self, shape: Vec<u8>) {
        self.shape2 = Some(shape);
    }

    /// Get the second shape.
    pub fn shape2(&self) -> Option<&[u8]> {
        self.shape2.as_deref()
    }
}

impl Default for BopToolsCoupleOfShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let couple = BopToolsCoupleOfShape::new();
        assert_eq!(couple.shape1(), None);
        assert_eq!(couple.shape2(), None);
    }

    #[test]
    fn test_set_shapes() {
        let mut couple = BopToolsCoupleOfShape::new();
        couple.set_shape1(vec![1, 2, 3]);
        couple.set_shape2(vec![4, 5, 6]);

        assert_eq!(couple.shape1(), Some(&[1, 2, 3][..]));
        assert_eq!(couple.shape2(), Some(&[4, 5, 6][..]));
    }

    #[test]
    fn test_default() {
        let couple = BopToolsCoupleOfShape::default();
        assert_eq!(couple.shape1(), None);
        assert_eq!(couple.shape2(), None);
    }
}
