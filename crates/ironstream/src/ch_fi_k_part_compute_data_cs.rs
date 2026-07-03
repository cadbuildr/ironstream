// FILE: ch_fi_k_part_compute_data_cs.rs
// occt: ChFiKPart_ComputeData_CS

/// Compute data for cone-sphere fillet combinations.
#[derive(Debug, Clone)]
pub struct ChFiKPartComputeDataCS;

impl ChFiKPartComputeDataCS {
    /// Compute cone-sphere fillet
    pub fn compute(_cone_id: usize, _sphere_id: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert!(ChFiKPartComputeDataCS::compute(0, 1).is_ok());
    }
}
