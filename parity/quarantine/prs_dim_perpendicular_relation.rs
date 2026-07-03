// FILE: prs_dim_perpendicular_relation.rs
// occt: PrsDim_PerpendicularRelation

/// Stub for PrsDim_PerpendicularRelation from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_PerpendicularRelation {}

impl PrsDim_PerpendicularRelation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_PerpendicularRelation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_perpendicular_relation_creation() {
        let _obj = PrsDim_PerpendicularRelation::new();
        let _def = PrsDim_PerpendicularRelation::default();
    }
}
