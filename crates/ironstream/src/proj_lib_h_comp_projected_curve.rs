// FILE: proj_lib_h_comp_projected_curve.rs
// occt: ProjLib_HCompProjectedCurve

/// Handle to composite projected curve.
pub struct ProjLibHCompProjectedCurve {
    curve: Box<super::proj_lib_comp_projected_curve::ProjLibCompProjectedCurve>,
}

impl ProjLibHCompProjectedCurve {
    /// Create a handle to a composite projected curve.
    pub fn new() -> Self {
        ProjLibHCompProjectedCurve {
            curve: Box::new(super::proj_lib_comp_projected_curve::ProjLibCompProjectedCurve::new()),
        }
    }

    /// Get reference to the curve.
    pub fn curve(&self) -> &super::proj_lib_comp_projected_curve::ProjLibCompProjectedCurve {
        &self.curve
    }

    /// Get mutable reference to the curve.
    pub fn curve_mut(&mut self) -> &mut super::proj_lib_comp_projected_curve::ProjLibCompProjectedCurve {
        &mut self.curve
    }
}

impl Default for ProjLibHCompProjectedCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_handle() {
        let handle = ProjLibHCompProjectedCurve::new();
        assert_eq!(handle.curve().nb_parts(), 0);
    }

    #[test]
    fn test_mutate_curve() {
        let mut handle = ProjLibHCompProjectedCurve::new();
        handle.curve_mut().add_part();
        assert_eq!(handle.curve().nb_parts(), 1);
    }
}
