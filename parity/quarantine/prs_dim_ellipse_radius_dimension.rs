// FILE: prs_dim_ellipse_radius_dimension.rs
// occt: PrsDim_EllipseRadiusDimension

/// Stub for PrsDim_EllipseRadiusDimension from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_EllipseRadiusDimension {}

impl PrsDim_EllipseRadiusDimension {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_EllipseRadiusDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_ellipse_radius_dimension_creation() {
        let _obj = PrsDim_EllipseRadiusDimension::new();
        let _def = PrsDim_EllipseRadiusDimension::default();
    }
}
