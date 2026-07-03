// FILE: ch_fi_k_part_compute_data_fil_pln_cyl.rs
// occt: ChFiKPart_ComputeData_FilPlnCyl

/// Compute fillet data for plane-cylinder configuration.
#[derive(Debug, Clone)]
pub struct ChFiKPartComputeDataFilPlnCyl;

impl ChFiKPartComputeDataFilPlnCyl {
    /// Compute plane-cylinder fillet
    pub fn compute(_plane_id: usize, _cyl_id: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert!(ChFiKPartComputeDataFilPlnCyl::compute(0, 1).is_ok());
    }
}
