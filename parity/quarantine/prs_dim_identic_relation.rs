// FILE: prs_dim_identic_relation.rs
// occt: PrsDim_IdenticRelation

/// Stub for PrsDim_IdenticRelation from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_IdenticRelation {}

impl PrsDim_IdenticRelation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_IdenticRelation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_identic_relation_creation() {
        let _obj = PrsDim_IdenticRelation::new();
        let _def = PrsDim_IdenticRelation::default();
    }
}
