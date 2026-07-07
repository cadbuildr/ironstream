// FILE: prs_dim_mid_point_relation.rs
// occt: PrsDim_MidPointRelation

/// Stub for PrsDim_MidPointRelation from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_MidPointRelation {}

impl PrsDim_MidPointRelation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_MidPointRelation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_mid_point_relation_creation() {
        let _obj = PrsDim_MidPointRelation::new();
        let _def = PrsDim_MidPointRelation::default();
    }
}
