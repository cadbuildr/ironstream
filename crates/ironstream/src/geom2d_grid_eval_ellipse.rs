// FILE: geom2d_grid_eval_ellipse.rs
// occt: Geom2dGridEval_Ellipse

/// Grid evaluator for ellipses
#[derive(Debug, Clone)]
pub struct Geom2dGridEvalEllipse {
    center_x: f64,
    center_y: f64,
    major_axis: f64,
    minor_axis: f64,
    rotation: f64,
    grid_size: usize,
    grid_values: Vec<(f64, f64)>,
}

impl Geom2dGridEvalEllipse {
    pub fn new(
        center_x: f64,
        center_y: f64,
        major_axis: f64,
        minor_axis: f64,
        rotation: f64,
        grid_size: usize,
    ) -> Self {
        let mut eval = Geom2dGridEvalEllipse {
            center_x,
            center_y,
            major_axis,
            minor_axis,
            rotation,
            grid_size,
            grid_values: vec![(0.0, 0.0); grid_size],
        };
        eval.precompute_grid();
        eval
    }

    fn precompute_grid(&mut self) {
        for i in 0..self.grid_size {
            let theta = (i as f64 / self.grid_size as f64) * 2.0 * std::f64::consts::PI;
            let x = self.major_axis * theta.cos();
            let y = self.minor_axis * theta.sin();

            // Apply rotation
            let cos_r = self.rotation.cos();
            let sin_r = self.rotation.sin();
            let rotated_x = x * cos_r - y * sin_r;
            let rotated_y = x * sin_r + y * cos_r;

            self.grid_values[i] = (
                self.center_x + rotated_x,
                self.center_y + rotated_y,
            );
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
    fn test_ellipse_grid_new() {
        let ellipse = Geom2dGridEvalEllipse::new(0.0, 0.0, 2.0, 1.0, 0.0, 8);
        assert_eq!(ellipse.grid_size(), 8);
    }

    #[test]
    fn test_ellipse_grid_value() {
        let ellipse = Geom2dGridEvalEllipse::new(1.0, 2.0, 3.0, 2.0, 0.0, 4);
        assert!(ellipse.grid_value(0).is_some());
    }
}
