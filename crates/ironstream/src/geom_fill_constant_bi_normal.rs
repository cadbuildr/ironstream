// FILE: geom_fill_constant_bi_normal.rs
// occt: GeomFill_ConstantBiNormal

/// Constant binormal trihedron for sweeping
pub struct ConstantBiNormal;

impl ConstantBiNormal {
    pub fn new() -> Self {
        ConstantBiNormal
    }
}

impl Default for ConstantBiNormal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _normal = ConstantBiNormal::new();
    }

    #[test]
    fn test_default() {
        let _normal = ConstantBiNormal::default();
    }
}
