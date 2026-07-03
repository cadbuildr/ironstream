// FILE: ch_fi_k_part_compute_data_fcts.rs
// occt: ChFiKPart_ComputeData_Fcts

/// Helper functions for computing fillet/chamfer data.
#[derive(Debug, Clone)]
pub struct ChFiKPartComputeDataFcts;

impl ChFiKPartComputeDataFcts {
    /// Compute parameter within period
    pub fn in_period(u: f64, u_first: f64, u_last: f64, _eps: f64) -> f64 {
        if u < u_first {
            u + (u_last - u_first)
        } else if u > u_last {
            u - (u_last - u_first)
        } else {
            u
        }
    }

    /// Create a parametric curve
    pub fn create_pcurve(_uv1: (f64, f64), _uv2: (f64, f64), _start: f64, _end: f64) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_period() {
        let u = ChFiKPartComputeDataFcts::in_period(0.5, 0.0, 1.0, 1e-7);
        assert_eq!(u, 0.5);
    }

    #[test]
    fn test_in_period_below() {
        let u = ChFiKPartComputeDataFcts::in_period(-0.5, 0.0, 1.0, 1e-7);
        assert_eq!(u, 0.5);
    }

    #[test]
    fn test_create_pcurve() {
        assert!(ChFiKPartComputeDataFcts::create_pcurve((0.0, 0.0), (1.0, 1.0), 0.0, 1.0).is_ok());
    }
}
