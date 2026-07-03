// FILE: geom_lib_log_sample.rs
// occt: GeomLib_LogSample

/// Logarithmic sampling for numerical integration.
pub struct GeomLibLogSample {
    num_points: usize,
}

impl GeomLibLogSample {
    /// Create a logarithmic sampler.
    pub fn new(num_points: usize) -> Self {
        GeomLibLogSample { num_points }
    }

    /// Get sample points in logarithmic distribution.
    pub fn points(&self) -> Vec<f64> {
        let mut result = Vec::new();
        if self.num_points < 2 {
            return result;
        }

        for i in 0..self.num_points {
            let t = (i as f64) / ((self.num_points - 1) as f64);
            // Logarithmic distribution: more points at the beginning
            let x = (1.0 + t * (std::f64::consts::E - 1.0)).ln();
            result.push(x / std::f64::consts::E.ln());
        }
        result
    }

    /// Get the number of points.
    pub fn num_points(&self) -> usize {
        self.num_points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sampler() {
        let sampler = GeomLibLogSample::new(10);
        assert_eq!(sampler.num_points(), 10);
    }

    #[test]
    fn test_points_count() {
        let sampler = GeomLibLogSample::new(5);
        let points = sampler.points();
        assert_eq!(points.len(), 5);
    }

    #[test]
    fn test_points_bounds() {
        let sampler = GeomLibLogSample::new(10);
        let points = sampler.points();
        assert!(points[0] >= 0.0);
        assert!(points[points.len() - 1] <= 1.0 + 1e-6);
    }

    #[test]
    fn test_points_increasing() {
        let sampler = GeomLibLogSample::new(10);
        let points = sampler.points();
        for i in 1..points.len() {
            assert!(points[i] >= points[i - 1]);
        }
    }
}
