// FILE: aspect_type_of_deflection.rs
// occt: Aspect_TypeOfDeflection

/// Type of deflection for meshing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectTypeOfDeflection {
    /// Relative deflection (fraction of size).
    Relative = 0,
    /// Absolute deflection (world units).
    Absolute = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(AspectTypeOfDeflection::Relative as i32, 0);
        assert_eq!(AspectTypeOfDeflection::Absolute as i32, 1);
    }
}
