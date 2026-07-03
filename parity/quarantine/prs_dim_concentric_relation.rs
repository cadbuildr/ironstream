// FILE: prs_dim_concentric_relation.rs
// occt: PrsDim_ConcentricRelation

/// Stub for PrsDim_ConcentricRelation from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_ConcentricRelation {}

impl PrsDim_ConcentricRelation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_ConcentricRelation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_concentric_relation_creation() {
        let _obj = PrsDim_ConcentricRelation::new();
        let _def = PrsDim_ConcentricRelation::default();
    }
}
