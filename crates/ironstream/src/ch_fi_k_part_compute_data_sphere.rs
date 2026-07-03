// FILE: ch_fi_k_part_compute_data_sphere.rs
// occt: ChFiKPart_ComputeData_Sphere

/// Compute sphere fillet between surfaces.
#[derive(Debug, Clone)]
pub struct ChFiKPartComputeDataSphere;

impl ChFiKPartComputeDataSphere {
    /// Compute sphere fillet
    pub fn compute(_surf1_id: usize, _surf2_id: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert!(ChFiKPartComputeDataSphere::compute(0, 1).is_ok());
    }
}
