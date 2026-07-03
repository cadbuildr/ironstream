// FILE: geom2d_grid_eval_curve.rs
// occt: Geom2dGridEval_Curve

/// Grid-based evaluator for 2D curves
#[derive(Debug, Clone)]
pub struct Geom2dGridEvalCurve {
    grid_count: usize,
    values: Vec<(f64, f64)>,
}

impl Geom2dGridEvalCurve {
    pub fn new(grid_count: usize) -> Self {
        Geom2dGridEvalCurve {
            grid_count,
            values: vec![(0.0, 0.0); grid_count],
        }
    }

    pub fn set_value(&mut self, idx: usize, x: f64, y: f64) {
        if idx < self.grid_count {
            self.values[idx] = (x, y);
        }
    }

    pub fn get_value(&self, idx: usize) -> Option<(f64, f64)> {
        if idx < self.grid_count {
            Some(self.values[idx])
        } else {
            None
        }
    }

    pub fn grid_count(&self) -> usize {
        self.grid_count
    }

    pub fn interpolate(&self, t: f64) -> (f64, f64) {
        if self.grid_count < 2 {
            return if self.grid_count > 0 {
                self.values[0]
            } else {
                (0.0, 0.0)
            };
        }

        let idx_float = t * (self.grid_count - 1) as f64;
        let idx = idx_float.floor() as usize;
        let frac = idx_float - idx as f64;

        if idx + 1 < self.grid_count {
            let p1 = self.values[idx];
            let p2 = self.values[idx + 1];
            (
                (1.0 - frac) * p1.0 + frac * p2.0,
                (1.0 - frac) * p1.1 + frac * p2.1,
            )
        } else {
            self.values[self.grid_count - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_eval_curve_new() {
        let curve = Geom2dGridEvalCurve::new(10);
        assert_eq!(curve.grid_count(), 10);
    }

    #[test]
    fn test_grid_eval_curve_set_get() {
        let mut curve = Geom2dGridEvalCurve::new(5);
        curve.set_value(2, 1.5, 2.5);
        assert_eq!(curve.get_value(2), Some((1.5, 2.5)));
    }

    #[test]
    fn test_grid_eval_curve_interpolate() {
        let mut curve = Geom2dGridEvalCurve::new(2);
        curve.set_value(0, 0.0, 0.0);
        curve.set_value(1, 1.0, 1.0);
        let (x, y) = curve.interpolate(0.5);
        assert!((x - 0.5).abs() < 0.0001);
        assert!((y - 0.5).abs() < 0.0001);
    }
}
