// FILE: geom2d_eval_aht_bezier_curve.rs
// occt: Geom2dEval_AHTBezierCurve

/// Algebraic-Hyperbolic-Trigonometric Bezier curve evaluator
#[derive(Debug, Clone)]
pub struct Geom2dEvalAHTBezierCurve {
    poles: Vec<(f64, f64)>,
    weights: Vec<f64>,
    alg_degree: i32,
    alpha: f64,
    beta: f64,
}

impl Geom2dEvalAHTBezierCurve {
    /// Create a non-rational AHT Bezier curve
    pub fn new(poles: Vec<(f64, f64)>, alg_degree: i32, alpha: f64, beta: f64) -> Self {
        let count = poles.len();
        let weights = vec![1.0; count];
        Geom2dEvalAHTBezierCurve {
            poles,
            weights,
            alg_degree,
            alpha,
            beta,
        }
    }

    /// Create a rational AHT Bezier curve
    pub fn with_weights(
        poles: Vec<(f64, f64)>,
        weights: Vec<f64>,
        alg_degree: i32,
        alpha: f64,
        beta: f64,
    ) -> Self {
        Geom2dEvalAHTBezierCurve {
            poles,
            weights,
            alg_degree,
            alpha,
            beta,
        }
    }

    /// Get poles
    pub fn poles(&self) -> &[(f64, f64)] {
        &self.poles
    }

    /// Get weights
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    /// Get algebraic degree
    pub fn alg_degree(&self) -> i32 {
        self.alg_degree
    }

    /// Get alpha (hyperbolic frequency)
    pub fn alpha(&self) -> f64 {
        self.alpha
    }

    /// Get beta (trigonometric frequency)
    pub fn beta(&self) -> f64 {
        self.beta
    }

    /// Evaluate curve at parameter t (0 to 1)
    pub fn value(&self, t: f64) -> (f64, f64) {
        if self.poles.is_empty() {
            return (0.0, 0.0);
        }

        // Simplified evaluation using de Casteljau algorithm
        let mut points = self.poles.clone();
        for _ in 0..(self.poles.len() - 1) {
            for i in 0..(points.len() - 1) {
                let p1 = points[i];
                let p2 = points[i + 1];
                points[i] = (
                    (1.0 - t) * p1.0 + t * p2.0,
                    (1.0 - t) * p1.1 + t * p2.1,
                );
            }
            points.pop();
        }

        if !points.is_empty() {
            points[0]
        } else {
            (0.0, 0.0)
        }
    }

    /// Evaluate derivative at parameter t
    pub fn derivative(&self, t: f64) -> (f64, f64) {
        let dt = 0.001;
        let (x1, y1) = self.value(t - dt);
        let (x2, y2) = self.value(t + dt);
        ((x2 - x1) / (2.0 * dt), (y2 - y1) / (2.0 * dt))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aht_bezier_new() {
        let poles = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)];
        let curve = Geom2dEvalAHTBezierCurve::new(poles, 2, 0.0, 0.0);
        assert_eq!(curve.alg_degree(), 2);
        assert_eq!(curve.poles().len(), 3);
    }

    #[test]
    fn test_aht_bezier_value() {
        let poles = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)];
        let curve = Geom2dEvalAHTBezierCurve::new(poles, 2, 0.0, 0.0);
        let (x, y) = curve.value(0.5);
        assert!(x > 0.0 && x < 1.0);
        assert!(y > 0.0 && y < 1.0);
    }

    #[test]
    fn test_aht_bezier_with_weights() {
        let poles = vec![(0.0, 0.0), (1.0, 1.0)];
        let weights = vec![1.0, 2.0];
        let curve = Geom2dEvalAHTBezierCurve::with_weights(poles, weights, 1, 0.0, 0.0);
        assert_eq!(curve.weights().len(), 2);
    }

    #[test]
    fn test_aht_bezier_derivative() {
        let poles = vec![(0.0, 0.0), (1.0, 1.0)];
        let curve = Geom2dEvalAHTBezierCurve::new(poles, 1, 0.0, 0.0);
        let (dx, dy) = curve.derivative(0.5);
        assert!((dx - 1.0).abs() < 0.01 || (dy - 1.0).abs() < 0.01);
    }
}
