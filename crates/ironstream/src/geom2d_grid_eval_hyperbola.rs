// FILE: geom2d_grid_eval_hyperbola.rs
// occt: Geom2dGridEval_Hyperbola

/// Grid evaluator for hyperbolas
#[derive(Debug, Clone)]
pub struct Geom2dGridEvalHyperbola {
    center_x: f64,
    center_y: f64,
    major_axis: f64,
    minor_axis: f64,
    grid_size: usize,
    grid_values: Vec<(f64, f64)>,
}

impl Geom2dGridEvalHyperbola {
    pub fn new(
        center_x: f64,
        center_y: f64,
        major_axis: f64,
        minor_axis: f64,
        grid_size: usize,
    ) -> Self {
        let mut eval = Geom2dGridEvalHyperbola {
            center_x,
            center_y,
            major_axis,
            minor_axis,
            grid_size,
            grid_values: vec![(0.0, 0.0); grid_size],
        };
        eval.precompute_grid();
        eval
    }

    fn precompute_grid(&mut self) {
        for i in 0..self.grid_size {
            let t = -3.0 + (i as f64 / self.grid_size as f64) * 6.0;
            let x = self.major_axis * t.cosh();
            let y = self.minor_axis * t.sinh();
            self.grid_values[i] = (self.center_x + x, self.center_y + y);
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
    fn test_hyperbola_grid_new() {
        let hyperbola = Geom2dGridEvalHyperbola::new(0.0, 0.0, 2.0, 1.0, 10);
        assert_eq!(hyperbola.grid_size(), 10);
    }

    #[test]
    fn test_hyperbola_grid_value() {
        let hyperbola = Geom2dGridEvalHyperbola::new(1.0, 1.0, 2.0, 1.5, 5);
        assert!(hyperbola.grid_value(0).is_some());
    }
}
