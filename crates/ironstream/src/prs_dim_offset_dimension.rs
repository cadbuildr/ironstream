// FILE: prs_dim_offset_dimension.rs
// occt: PrsDim_OffsetDimension

/// Stub for PrsDim_OffsetDimension from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim_OffsetDimension {}

impl PrsDim_OffsetDimension {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim_OffsetDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_offset_dimension_creation() {
        let _obj = PrsDim_OffsetDimension::new();
        let _def = PrsDim_OffsetDimension::default();
    }
}
