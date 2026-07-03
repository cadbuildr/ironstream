// FILE: ch_fi3d_fillet_shape.rs
// occt: ChFi3d_FilletShape

/// Types of fillet shapes for 3D fillet representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChFi3dFilletShape {
    /// Standard NURBS representation of circles (default)
    Rational,
    /// NURBS representation where parameters match circle parameters
    QuasiAngular,
    /// Polynomial approximation of circles
    Polynomial,
}

impl Default for ChFi3dFilletShape {
    fn default() -> Self {
        Self::Rational
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(ChFi3dFilletShape::Rational, ChFi3dFilletShape::Rational);
        assert_ne!(ChFi3dFilletShape::Rational, ChFi3dFilletShape::Polynomial);
    }

    #[test]
    fn test_default() {
        assert_eq!(ChFi3dFilletShape::default(), ChFi3dFilletShape::Rational);
    }

    #[test]
    fn test_copy_semantics() {
        let shape = ChFi3dFilletShape::QuasiAngular;
        let shape2 = shape;
        assert_eq!(shape, shape2);
    }
}
