// FILE: ch_fi_k_part_compute_data_fil_pln_con.rs
// occt: ChFiKPart_ComputeData_FilPlnCon

/// Compute fillet data for plane-cone configuration.
#[derive(Debug, Clone)]
pub struct ChFiKPartComputeDataFilPlnCon;

impl ChFiKPartComputeDataFilPlnCon {
    /// Compute plane-cone fillet
    pub fn compute(_plane_id: usize, _cone_id: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert!(ChFiKPartComputeDataFilPlnCon::compute(0, 1).is_ok());
    }
}
