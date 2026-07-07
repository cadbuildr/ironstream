// FILE: prs_dim_equal_radius_relation.rs
// occt: PrsDim_EqualRadiusRelation

/// Stub for PrsDim_EqualRadiusRelation from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_EqualRadiusRelation {}

impl PrsDim_EqualRadiusRelation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_EqualRadiusRelation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_equal_radius_relation_creation() {
        let _obj = PrsDim_EqualRadiusRelation::new();
        let _def = PrsDim_EqualRadiusRelation::default();
    }
}
