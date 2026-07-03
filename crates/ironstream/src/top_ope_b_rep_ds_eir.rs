// FILE: top_ope_b_rep_ds_eir.rs
// occt: TopOpeBRepDS_EIR

/// Edge Interference Record
/// Stores edge interference information for analysis
#[derive(Debug, Clone)]
pub struct EIR {
    /// Edge index
    edge_idx: usize,
    /// Interference index
    interference_idx: usize,
    /// Parameter on edge
    param: f64,
}

impl EIR {
    /// Create new EIR
    pub fn new(edge_idx: usize, interference_idx: usize, param: f64) -> Self {
        EIR {
            edge_idx,
            interference_idx,
            param,
        }
    }

    /// Get edge index
    pub fn edge_idx(&self) -> usize {
        self.edge_idx
    }

    /// Get interference index
    pub fn interference_idx(&self) -> usize {
        self.interference_idx
    }

    /// Get parameter
    pub fn param(&self) -> f64 {
        self.param
    }

    /// Set parameter
    pub fn set_param(&mut self, p: f64) {
        self.param = p;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eir_new() {
        let eir = EIR::new(5, 10, 2.5);
        assert_eq!(eir.edge_idx(), 5);
        assert_eq!(eir.interference_idx(), 10);
        assert_eq!(eir.param(), 2.5);
    }

    #[test]
    fn test_eir_set_param() {
        let mut eir = EIR::new(1, 2, 0.5);
        eir.set_param(3.5);
        assert_eq!(eir.param(), 3.5);
    }
}
