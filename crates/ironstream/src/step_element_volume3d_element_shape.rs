// FILE: step_element_volume3d_element_shape.rs
// occt: StepElement_Volume3dElementShape

/// Enumeration for 3D volume element shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Volume3dElementShape {
    Hexahedron,
    Wedge,
    Tetrahedron,
    Pyramid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_variants() {
        assert_ne!(Volume3dElementShape::Hexahedron, Volume3dElementShape::Wedge);
        assert_ne!(Volume3dElementShape::Tetrahedron, Volume3dElementShape::Pyramid);
        assert_eq!(Volume3dElementShape::Hexahedron, Volume3dElementShape::Hexahedron);
    }

    #[test]
    fn test_copy() {
        let shape = Volume3dElementShape::Tetrahedron;
        let shape2 = shape;
        assert_eq!(shape, shape2);
    }

    #[test]
    fn test_debug() {
        let shape = Volume3dElementShape::Pyramid;
        assert_eq!(format!("{:?}", shape), "Pyramid");
    }
}
