// FILE: prs_dim_chamf3d_dimension.rs
// occt: PrsDim_Chamf3dDimension

/// Stub for PrsDim_Chamf3dDimension from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_Chamf3dDimension {}

impl PrsDim_Chamf3dDimension {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_Chamf3dDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_chamf3d_dimension_creation() {
        let _obj = PrsDim_Chamf3dDimension::new();
        let _def = PrsDim_Chamf3dDimension::default();
    }
}
