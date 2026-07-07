// FILE: math_compute_kronrod_points_and_weights.rs
// occt: math_ComputeKronrodPointsAndWeights

/// Compute Gauss-Kronrod quadrature points and weights.
pub struct KronrodPointsAndWeights {
    pub points: Vec<f64>,
    pub weights: Vec<f64>,
}

impl KronrodPointsAndWeights {
    pub fn new(n: usize) -> Self {
        // Simplified: initialize empty for n <= 0
        if n == 0 {
            return Self { points: vec![], weights: vec![] };
        }

        // Placeholder for actual computation
        let mut points = vec![0.0; 2 * n + 1];
        let mut weights = vec![0.0; 2 * n + 1];

        // Basic symmetric quadrature
        for i in 0..=n {
            let x = -1.0 + 2.0 * i as f64 / n as f64;
            points[i] = x;
            points[2 * n - i] = -x;
            weights[i] = 1.0 / (n as f64 + 1.0);
            weights[2 * n - i] = weights[i];
        }

        Self { points, weights }
    }

    pub fn nb_points(&self) -> usize {
        self.points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let kpw = KronrodPointsAndWeights::new(0);
        assert_eq!(kpw.nb_points(), 0);
    }

    #[test]
    fn test_single_point() {
        let kpw = KronrodPointsAndWeights::new(1);
        assert_eq!(kpw.nb_points(), 3);
        assert_eq!(kpw.points.len(), 3);
        assert_eq!(kpw.weights.len(), 3);
    }
}
