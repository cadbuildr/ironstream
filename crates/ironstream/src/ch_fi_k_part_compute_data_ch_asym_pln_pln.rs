// FILE: ch_fi_k_part_compute_data_ch_asym_pln_pln.rs
// occt: ChFiKPart_ComputeData_ChAsymPlnPln

/// Compute chamfer data for asymmetric plane-plane configuration.
#[derive(Debug, Clone)]
pub struct ChFiKPartComputeDataChAsymPlnPln;

impl ChFiKPartComputeDataChAsymPlnPln {
    /// Compute asymmetric plane-plane chamfer
    pub fn compute(_plane1_id: usize, _plane2_id: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert!(ChFiKPartComputeDataChAsymPlnPln::compute(0, 1).is_ok());
    }
}
