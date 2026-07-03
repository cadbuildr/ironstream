// FILE: prs_dim_parallel_relation.rs
// occt: PrsDim_ParallelRelation

/// Stub for PrsDim_ParallelRelation from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_ParallelRelation {}

impl PrsDim_ParallelRelation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_ParallelRelation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_parallel_relation_creation() {
        let _obj = PrsDim_ParallelRelation::new();
        let _def = PrsDim_ParallelRelation::default();
    }
}
