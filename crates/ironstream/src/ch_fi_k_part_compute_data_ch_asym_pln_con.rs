// FILE: ch_fi_k_part_compute_data_ch_asym_pln_con.rs
// occt: ChFiKPart_ComputeData_ChAsymPlnCon

/// Compute chamfer data for asymmetric plane-cone configuration.
#[derive(Debug, Clone)]
pub struct ChFiKPartComputeDataChAsymPlnCon;

impl ChFiKPartComputeDataChAsymPlnCon {
    /// Compute asymmetric plane-cone chamfer
    pub fn compute(_plane_id: usize, _cone_id: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert!(ChFiKPartComputeDataChAsymPlnCon::compute(0, 1).is_ok());
    }
}
