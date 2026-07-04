// FILE: topo_ds_to_step_faceted_error.rs
// occt: TopoDSToStep_FacetedError

/// Error enumeration for TopoDSToStep faceted operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FacetedError {
    FacetedDone,
    SurfaceNotPlane,
    PCurveNotLinear,
}

impl FacetedError {
    pub fn is_success(&self) -> bool {
        matches!(self, FacetedError::FacetedDone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert!(FacetedError::FacetedDone.is_success());
        assert!(!FacetedError::SurfaceNotPlane.is_success());
        assert!(!FacetedError::PCurveNotLinear.is_success());
    }
}
