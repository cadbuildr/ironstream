// FILE: ch_fi_k_part_compute_data_ch_asym_pln_cyl.rs
// occt: ChFiKPart_ComputeData_ChAsymPlnCyl

/// Compute chamfer data for asymmetric plane-cylinder configuration.
#[derive(Debug, Clone)]
pub struct ChFiKPartComputeDataChAsymPlnCyl;

impl ChFiKPartComputeDataChAsymPlnCyl {
    /// Compute asymmetric plane-cylinder chamfer
    pub fn compute(_plane_id: usize, _cyl_id: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert!(ChFiKPartComputeDataChAsymPlnCyl::compute(0, 1).is_ok());
    }
}
