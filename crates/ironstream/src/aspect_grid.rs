// FILE: aspect_grid.rs
// occt: Aspect_Grid

/// Base class for grid representation.
pub trait AspectGrid {
    /// Compute closest grid point to X, Y.
    fn compute(&self, x: f64, y: f64) -> (f64, f64);

    /// Get X origin.
    fn x_origin(&self) -> f64;

    /// Get Y origin.
    fn y_origin(&self) -> f64;
}

/// Simple rectangular grid implementation.
pub struct RectangularGrid {
    x_origin: f64,
    y_origin: f64,
    x_step: f64,
    y_step: f64,
}

impl RectangularGrid {
    /// Create rectangular grid.
    pub fn new(x_origin: f64, y_origin: f64, x_step: f64, y_step: f64) -> Self {
        RectangularGrid {
            x_origin,
            y_origin,
            x_step,
            y_step,
        }
    }
}

impl AspectGrid for RectangularGrid {
    fn compute(&self, x: f64, y: f64) -> (f64, f64) {
        let grid_x = ((x - self.x_origin) / self.x_step).round() * self.x_step + self.x_origin;
        let grid_y = ((y - self.y_origin) / self.y_step).round() * self.y_step + self.y_origin;
        (grid_x, grid_y)
    }

    fn x_origin(&self) -> f64 {
        self.x_origin
    }

    fn y_origin(&self) -> f64 {
        self.y_origin
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangular_grid() {
        let grid = RectangularGrid::new(0.0, 0.0, 1.0, 1.0);
        let (x, y) = grid.compute(0.3, 0.4);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
    }
}
