// FILE: prs_dim_equal_distance_relation.rs
// occt: PrsDim_EqualDistanceRelation

/// Stub for PrsDim_EqualDistanceRelation from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_EqualDistanceRelation {}

impl PrsDim_EqualDistanceRelation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_EqualDistanceRelation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_equal_distance_relation_creation() {
        let _obj = PrsDim_EqualDistanceRelation::new();
        let _def = PrsDim_EqualDistanceRelation::default();
    }
}
