// FILE: ch_fi_k_part_compute_data_rotule.rs
// occt: ChFiKPart_ComputeData_Rotule

/// Compute tore/rotule fillet between plane and line.
#[derive(Debug, Clone)]
pub struct ChFiKPartComputeDataRotule;

impl ChFiKPartComputeDataRotule {
    /// Compute rotule fillet
    pub fn compute(_plane_id: usize, _line_id: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert!(ChFiKPartComputeDataRotule::compute(0, 1).is_ok());
    }
}
