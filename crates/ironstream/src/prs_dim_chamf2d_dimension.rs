// FILE: prs_dim_chamf2d_dimension.rs
// occt: PrsDim_Chamf2dDimension

/// Stub for PrsDim_Chamf2dDimension from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_Chamf2dDimension {}

impl PrsDim_Chamf2dDimension {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_Chamf2dDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_chamf2d_dimension_creation() {
        let _obj = PrsDim_Chamf2dDimension::new();
        let _def = PrsDim_Chamf2dDimension::default();
    }
}
