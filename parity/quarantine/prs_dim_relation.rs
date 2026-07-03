// FILE: prs_dim_relation.rs
// occt: PrsDim_Relation

/// Stub for PrsDim_Relation from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_Relation {}

impl PrsDim_Relation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_Relation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_relation_creation() {
        let _obj = PrsDim_Relation::new();
        let _def = PrsDim_Relation::default();
    }
}
