// FILE: aspect_rectangular_grid.rs
// occt: Aspect_RectangularGrid

/// Rectangular grid.
pub struct AspectRectangularGrid {
    x_step: f64,
    y_step: f64,
    x_origin: f64,
    y_origin: f64,
    rotation_angle: f64,
}

impl AspectRectangularGrid {
    /// Create rectangular grid.
    pub fn new(
        x_step: f64,
        y_step: f64,
        x_origin: f64,
        y_origin: f64,
        rotation_angle: f64,
    ) -> Self {
        AspectRectangularGrid {
            x_step,
            y_step,
            x_origin,
            y_origin,
            rotation_angle,
        }
    }

    /// Set X step.
    pub fn set_x_step(&mut self, step: f64) {
        self.x_step = step;
    }

    /// Get X step.
    pub fn x_step(&self) -> f64 {
        self.x_step
    }

    /// Set Y step.
    pub fn set_y_step(&mut self, step: f64) {
        self.y_step = step;
    }

    /// Get Y step.
    pub fn y_step(&self) -> f64 {
        self.y_step
    }

    /// Compute closest grid point.
    pub fn compute(&self, x: f64, y: f64) -> (f64, f64) {
        let grid_x = ((x - self.x_origin) / self.x_step).round() * self.x_step + self.x_origin;
        let grid_y = ((y - self.y_origin) / self.y_step).round() * self.y_step + self.y_origin;
        (grid_x, grid_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let grid = AspectRectangularGrid::new(1.0, 2.0, 0.0, 0.0, 0.0);
        assert_eq!(grid.x_step(), 1.0);
        assert_eq!(grid.y_step(), 2.0);
    }

    #[test]
    fn test_compute() {
        let grid = AspectRectangularGrid::new(1.0, 1.0, 0.0, 0.0, 0.0);
        let (x, y) = grid.compute(0.7, 0.3);
        assert_eq!(x, 1.0);
        assert_eq!(y, 0.0);
    }
}
