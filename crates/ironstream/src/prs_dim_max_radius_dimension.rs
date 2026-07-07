// FILE: prs_dim_max_radius_dimension.rs
// occt: PrsDim_MaxRadiusDimension

/// Stub for PrsDim_MaxRadiusDimension from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_MaxRadiusDimension {}

impl PrsDim_MaxRadiusDimension {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_MaxRadiusDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_max_radius_dimension_creation() {
        let _obj = PrsDim_MaxRadiusDimension::new();
        let _def = PrsDim_MaxRadiusDimension::default();
    }
}
