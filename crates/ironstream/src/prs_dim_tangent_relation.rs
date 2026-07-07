// FILE: prs_dim_tangent_relation.rs
// occt: PrsDim_TangentRelation

/// Stub for PrsDim_TangentRelation from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_TangentRelation {}

impl PrsDim_TangentRelation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_TangentRelation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_tangent_relation_creation() {
        let _obj = PrsDim_TangentRelation::new();
        let _def = PrsDim_TangentRelation::default();
    }
}
