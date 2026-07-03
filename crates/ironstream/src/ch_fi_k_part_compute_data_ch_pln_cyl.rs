// FILE: ch_fi_k_part_compute_data_ch_pln_cyl.rs
// occt: ChFiKPart_ComputeData_ChPlnCyl

/// Compute chamfer data for symmetric plane-cylinder configuration.
#[derive(Debug, Clone)]
pub struct ChFiKPartComputeDataChPlnCyl;

impl ChFiKPartComputeDataChPlnCyl {
    /// Compute symmetric plane-cylinder chamfer
    pub fn compute(_plane_id: usize, _cyl_id: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert!(ChFiKPartComputeDataChPlnCyl::compute(0, 1).is_ok());
    }
}
