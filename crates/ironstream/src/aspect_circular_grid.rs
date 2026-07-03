// FILE: aspect_circular_grid.rs
// occt: Aspect_CircularGrid

/// Circular grid representation.
pub struct AspectCircularGrid {
    radius_step: f64,
    division_number: i32,
    x_origin: f64,
    y_origin: f64,
    rotation_angle: f64,
    radius: f64,
    z_offset: f64,
    angle_start: f64,
    angle_end: f64,
}

impl AspectCircularGrid {
    /// Create circular grid with parameters.
    pub fn new(
        radius_step: f64,
        division_number: i32,
        x_origin: f64,
        y_origin: f64,
        rotation_angle: f64,
    ) -> Self {
        AspectCircularGrid {
            radius_step,
            division_number,
            x_origin,
            y_origin,
            rotation_angle,
            radius: 0.0,
            z_offset: 0.0,
            angle_start: 0.0,
            angle_end: 0.0,
        }
    }

    /// Set radius step.
    pub fn set_radius_step(&mut self, step: f64) {
        self.radius_step = step;
    }

    /// Get radius step.
    pub fn radius_step(&self) -> f64 {
        self.radius_step
    }

    /// Set division number.
    pub fn set_division_number(&mut self, number: i32) {
        self.division_number = number;
    }

    /// Get division number.
    pub fn division_number(&self) -> i32 {
        self.division_number
    }

    /// Set radius.
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    /// Get radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Set Z offset.
    pub fn set_z_offset(&mut self, offset: f64) {
        self.z_offset = offset;
    }

    /// Get Z offset.
    pub fn z_offset(&self) -> f64 {
        self.z_offset
    }

    /// Set arc range.
    pub fn set_arc_range(&mut self, start: f64, end: f64) {
        self.angle_start = start;
        self.angle_end = end;
    }

    /// Get angle start.
    pub fn angle_start(&self) -> f64 {
        self.angle_start
    }

    /// Get angle end.
    pub fn angle_end(&self) -> f64 {
        self.angle_end
    }

    /// Is arc.
    pub fn is_arc(&self) -> bool {
        (self.angle_start - self.angle_end).abs() > 1e-10
    }

    /// Compute grid point closest to given X, Y.
    pub fn compute(&self, x: f64, y: f64) -> (f64, f64) {
        // Simplified: return polar conversion
        let r = (x * x + y * y).sqrt();
        let snapped_r = (r / self.radius_step).round() * self.radius_step;
        (snapped_r, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let grid = AspectCircularGrid::new(1.0, 12, 0.0, 0.0, 0.0);
        assert_eq!(grid.radius_step(), 1.0);
        assert_eq!(grid.division_number(), 12);
    }

    #[test]
    fn test_set_radius() {
        let mut grid = AspectCircularGrid::new(1.0, 12, 0.0, 0.0, 0.0);
        grid.set_radius(5.0);
        assert_eq!(grid.radius(), 5.0);
    }

    #[test]
    fn test_is_arc() {
        let mut grid = AspectCircularGrid::new(1.0, 12, 0.0, 0.0, 0.0);
        assert!(!grid.is_arc());
        grid.set_arc_range(0.0, 1.5);
        assert!(grid.is_arc());
    }
}
