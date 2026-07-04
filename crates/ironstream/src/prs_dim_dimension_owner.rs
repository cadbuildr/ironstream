// FILE: prs_dim_dimension_owner.rs
// occt: PrsDim_DimensionOwner

/// Stub for PrsDim_DimensionOwner from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_DimensionOwner {}

impl PrsDim_DimensionOwner {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_DimensionOwner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_dimension_owner_creation() {
        let _obj = PrsDim_DimensionOwner::new();
        let _def = PrsDim_DimensionOwner::default();
    }
}
