// FILE: geom2d_grid_eval_bezier_curve.rs
// occt: Geom2dGridEval_BezierCurve

/// Grid evaluator for Bezier curves
#[derive(Debug, Clone)]
pub struct Geom2dGridEvalBezierCurve {
    poles: Vec<(f64, f64)>,
    grid_values: Vec<(f64, f64)>,
    grid_size: usize,
}

impl Geom2dGridEvalBezierCurve {
    pub fn new(poles: Vec<(f64, f64)>, grid_size: usize) -> Self {
        let mut eval = Geom2dGridEvalBezierCurve {
            poles,
            grid_values: vec![(0.0, 0.0); grid_size],
            grid_size,
        };
        eval.precompute_grid();
        eval
    }

    fn precompute_grid(&mut self) {
        for i in 0..self.grid_size {
            let t = i as f64 / (self.grid_size.saturating_sub(1).max(1)) as f64;
            self.grid_values[i] = self.evaluate_at(t);
        }
    }

    fn evaluate_at(&self, t: f64) -> (f64, f64) {
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

    pub fn grid_value(&self, idx: usize) -> Option<(f64, f64)> {
        if idx < self.grid_size {
            Some(self.grid_values[idx])
        } else {
            None
        }
    }

    pub fn grid_size(&self) -> usize {
        self.grid_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bezier_grid_new() {
        let poles = vec![(0.0, 0.0), (1.0, 1.0)];
        let curve = Geom2dGridEvalBezierCurve::new(poles, 10);
        assert_eq!(curve.grid_size(), 10);
    }

    #[test]
    fn test_bezier_grid_value() {
        let poles = vec![(0.0, 0.0), (1.0, 1.0)];
        let curve = Geom2dGridEvalBezierCurve::new(poles, 3);
        assert!(curve.grid_value(0).is_some());
        assert!(curve.grid_value(2).is_some());
    }
}
