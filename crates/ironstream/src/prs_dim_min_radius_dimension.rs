// FILE: prs_dim_min_radius_dimension.rs
// occt: PrsDim_MinRadiusDimension

/// Stub for PrsDim_MinRadiusDimension from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_MinRadiusDimension {}

impl PrsDim_MinRadiusDimension {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_MinRadiusDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_min_radius_dimension_creation() {
        let _obj = PrsDim_MinRadiusDimension::new();
        let _def = PrsDim_MinRadiusDimension::default();
    }
}
