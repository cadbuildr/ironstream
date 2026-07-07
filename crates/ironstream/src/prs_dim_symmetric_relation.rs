// FILE: prs_dim_symmetric_relation.rs
// occt: PrsDim_SymmetricRelation

/// Stub for PrsDim_SymmetricRelation from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_SymmetricRelation {}

impl PrsDim_SymmetricRelation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_SymmetricRelation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_symmetric_relation_creation() {
        let _obj = PrsDim_SymmetricRelation::new();
        let _def = PrsDim_SymmetricRelation::default();
    }
}
