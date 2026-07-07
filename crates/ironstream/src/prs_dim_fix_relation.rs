// FILE: prs_dim_fix_relation.rs
// occt: PrsDim_FixRelation

/// Stub for PrsDim_FixRelation from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_FixRelation {}

impl PrsDim_FixRelation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_FixRelation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_fix_relation_creation() {
        let _obj = PrsDim_FixRelation::new();
        let _def = PrsDim_FixRelation::default();
    }
}
