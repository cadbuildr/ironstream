// FILE: select3_d_interior_sensitive_point_set.rs
// occt: Select3D_InteriorSensitivePointSet

/// Stub for Select3D_InteriorSensitivePointSet from OCCT.
#[derive(Clone, Debug)]
pub struct Select3D_InteriorSensitivePointSet {}

impl Select3D_InteriorSensitivePointSet {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Select3D_InteriorSensitivePointSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select3_d_interior_sensitive_point_set_creation() {
        let _obj = Select3D_InteriorSensitivePointSet::new();
        let _def = Select3D_InteriorSensitivePointSet::default();
    }
}
