// FILE: geom2d_grid_eval_parabola.rs
// occt: Geom2dGridEval_Parabola

/// Grid evaluator for parabolas
#[derive(Debug, Clone)]
pub struct Geom2dGridEvalParabola {
    focus_x: f64,
    focus_y: f64,
    directrix_offset: f64,
    grid_size: usize,
    grid_values: Vec<(f64, f64)>,
}

impl Geom2dGridEvalParabola {
    pub fn new(focus_x: f64, focus_y: f64, directrix_offset: f64, grid_size: usize) -> Self {
        let mut eval = Geom2dGridEvalParabola {
            focus_x,
            focus_y,
            directrix_offset,
            grid_size,
            grid_values: vec![(0.0, 0.0); grid_size],
        };
        eval.precompute_grid();
        eval
    }

    fn precompute_grid(&mut self) {
        for i in 0..self.grid_size {
            let t = -3.0 + (i as f64 / self.grid_size as f64) * 6.0;
            // Parabola: y^2 = 4ax (parametric: (at^2, 2at))
            let x = self.directrix_offset * t * t;
            let y = 2.0 * self.directrix_offset * t;
            self.grid_values[i] = (self.focus_x + x, self.focus_y + y);
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
    fn test_parabola_grid_new() {
        let parabola = Geom2dGridEvalParabola::new(0.0, 0.0, 1.0, 10);
        assert_eq!(parabola.grid_size(), 10);
    }

    #[test]
    fn test_parabola_grid_value() {
        let parabola = Geom2dGridEvalParabola::new(1.0, 1.0, 0.5, 5);
        assert!(parabola.grid_value(0).is_some());
    }
}
