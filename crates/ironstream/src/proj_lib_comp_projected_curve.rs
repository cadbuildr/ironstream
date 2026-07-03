// FILE: proj_lib_comp_projected_curve.rs
// occt: ProjLib_CompProjectedCurve

/// Composite projected curve.
pub struct ProjLibCompProjectedCurve {
    nb_parts: usize,
}

impl ProjLibCompProjectedCurve {
    /// Create a composite projected curve.
    pub fn new() -> Self {
        ProjLibCompProjectedCurve { nb_parts: 0 }
    }

    /// Add a part to the curve.
    pub fn add_part(&mut self) {
        self.nb_parts += 1;
    }

    /// Get the number of parts.
    pub fn nb_parts(&self) -> usize {
        self.nb_parts
    }
}

impl Default for ProjLibCompProjectedCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_curve() {
        let curve = ProjLibCompProjectedCurve::new();
        assert_eq!(curve.nb_parts(), 0);
    }

    #[test]
    fn test_add_parts() {
        let mut curve = ProjLibCompProjectedCurve::new();
        curve.add_part();
        curve.add_part();
        assert_eq!(curve.nb_parts(), 2);
    }
}
