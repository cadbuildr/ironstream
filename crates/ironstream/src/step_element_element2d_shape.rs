// FILE: step_element_element2d_shape.rs
// occt: StepElement_Element2dShape

/// 2D shape enumeration for surface elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element2dShape {
    Quadrilateral,
    Triangle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element2d_shape_variants() {
        let quad = Element2dShape::Quadrilateral;
        let tri = Element2dShape::Triangle;
        assert_ne!(quad, tri);
    }

    #[test]
    fn test_element2d_shape_copy() {
        let shape = Element2dShape::Quadrilateral;
        let shape2 = shape;
        assert_eq!(shape, shape2);
    }

    #[test]
    fn test_element2d_shape_debug() {
        let shape = Element2dShape::Triangle;
        assert_eq!(format!("{:?}", shape), "Triangle");
    }
}
