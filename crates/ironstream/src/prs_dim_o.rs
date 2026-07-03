// FILE: prs_dim_o.rs
// occt: PrsDim

/// Stub for PrsDim from OCCT.
#[derive(Clone, Debug)]
pub struct PrsDim {}

impl PrsDim {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsDim {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_o_creation() {
        let _obj = PrsDim::new();
        let _def = PrsDim::default();
    }
}
