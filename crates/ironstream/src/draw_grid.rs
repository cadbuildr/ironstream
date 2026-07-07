// FILE: draw_grid.rs
// occt: Draw_Grid

//! A grid drawable for the Draw application.
//! Displays a grid with configurable step sizes along X, Y, and Z axes.

/// Represents a drawable grid in 3D space
pub struct DrawGrid {
    step_x: f64,
    step_y: f64,
    step_z: f64,
    is_active: bool,
}

impl DrawGrid {
    /// Create a new grid with default steps
    pub fn new() -> Self {
        DrawGrid {
            step_x: 1.0,
            step_y: 1.0,
            step_z: 1.0,
            is_active: true,
        }
    }

    /// Set the step size along X, Y, and Z axes
    pub fn set_steps(&mut self, step_x: f64, step_y: f64, step_z: f64) {
        self.step_x = step_x;
        self.step_y = step_y;
        self.step_z = step_z;
    }

    /// Get the step size along the X axis
    pub fn step_x(&self) -> f64 {
        self.step_x
    }

    /// Get the step size along the Y axis
    pub fn step_y(&self) -> f64 {
        self.step_y
    }

    /// Get the step size along the Z axis
    pub fn step_z(&self) -> f64 {
        self.step_z
    }

    /// Check if the grid is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Set the active state of the grid
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

impl Default for DrawGrid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = DrawGrid::new();
        assert_eq!(grid.step_x(), 1.0);
        assert_eq!(grid.step_y(), 1.0);
        assert_eq!(grid.step_z(), 1.0);
        assert!(grid.is_active());
    }

    #[test]
    fn test_grid_set_steps() {
        let mut grid = DrawGrid::new();
        grid.set_steps(2.0, 3.0, 4.0);
        assert_eq!(grid.step_x(), 2.0);
        assert_eq!(grid.step_y(), 3.0);
        assert_eq!(grid.step_z(), 4.0);
    }

    #[test]
    fn test_grid_active() {
        let mut grid = DrawGrid::new();
        assert!(grid.is_active());
        grid.set_active(false);
        assert!(!grid.is_active());
    }
}
