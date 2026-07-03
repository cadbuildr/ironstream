// FILE: select_mgr_type_of_depth_tolerance.rs
// occt: SelectMgr_TypeOfDepthTolerance

/// Enumeration for depth tolerance types in selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfDepthTolerance {
    /// Uniform tolerance applied to all depths.
    Uniform,
    /// Tolerance scales with depth value.
    Uniform3D,
}

impl TypeOfDepthTolerance {
    pub fn is_uniform(&self) -> bool {
        *self == TypeOfDepthTolerance::Uniform
    }

    pub fn is_uniform_3d(&self) -> bool {
        *self == TypeOfDepthTolerance::Uniform3D
    }

    pub fn as_str(&self) -> &str {
        match self {
            TypeOfDepthTolerance::Uniform => "Uniform",
            TypeOfDepthTolerance::Uniform3D => "Uniform3D",
        }
    }
}

impl Default for TypeOfDepthTolerance {
    fn default() -> Self {
        TypeOfDepthTolerance::Uniform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_comparison() {
        assert_eq!(TypeOfDepthTolerance::Uniform, TypeOfDepthTolerance::Uniform);
        assert_ne!(TypeOfDepthTolerance::Uniform, TypeOfDepthTolerance::Uniform3D);
    }

    #[test]
    fn type_checks() {
        assert!(TypeOfDepthTolerance::Uniform.is_uniform());
        assert!(TypeOfDepthTolerance::Uniform3D.is_uniform_3d());
    }

    #[test]
    fn as_str() {
        assert_eq!(TypeOfDepthTolerance::Uniform.as_str(), "Uniform");
        assert_eq!(TypeOfDepthTolerance::Uniform3D.as_str(), "Uniform3D");
    }

    #[test]
    fn default() {
        assert_eq!(TypeOfDepthTolerance::default(), TypeOfDepthTolerance::Uniform);
    }
}
