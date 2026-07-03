// FILE: geom2d_eval_t_bezier_curve.rs
// occt: Geom2dEval_TBezierCurve

/// Bezier curve evaluator
#[derive(Debug, Clone)]
pub struct Geom2dEvalTBezierCurve {
    poles: Vec<(f64, f64)>,
    weights: Vec<f64>,
    degree: i32,
}

impl Geom2dEvalTBezierCurve {
    pub fn new(poles: Vec<(f64, f64)>, degree: i32) -> Self {
        let count = poles.len();
        Geom2dEvalTBezierCurve {
            poles,
            weights: vec![1.0; count],
            degree,
        }
    }

    pub fn with_weights(poles: Vec<(f64, f64)>, weights: Vec<f64>, degree: i32) -> Self {
        Geom2dEvalTBezierCurve {
            poles,
            weights,
            degree,
        }
    }

    pub fn poles(&self) -> &[(f64, f64)] {
        &self.poles
    }

    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    pub fn degree(&self) -> i32 {
        self.degree
    }

    pub fn value(&self, t: f64) -> (f64, f64) {
        if self.poles.is_empty() {
            return (0.0, 0.0);
        }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_bezier_new() {
        let poles = vec![(0.0, 0.0), (1.0, 1.0)];
        let curve = Geom2dEvalTBezierCurve::new(poles, 1);
        assert_eq!(curve.degree(), 1);
    }

    #[test]
    fn test_t_bezier_value() {
        let poles = vec![(0.0, 0.0), (1.0, 1.0)];
        let curve = Geom2dEvalTBezierCurve::new(poles, 1);
        let (x, y) = curve.value(0.5);
        assert!((x - 0.5).abs() < 0.0001);
        assert!((y - 0.5).abs() < 0.0001);
    }
}
