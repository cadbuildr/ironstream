// FILE: std_object_gp_axes.rs
// occt: StdObject_gp_Axes

/// Persistent representation of geometric axes (coordinate system)
#[derive(Clone, Debug)]
pub struct GpAxes {
    main_dir: [f64; 3],
    x_dir: [f64; 3],
}

impl GpAxes {
    /// Create new axes
    pub fn new() -> Self {
        GpAxes {
            main_dir: [0.0, 0.0, 1.0],
            x_dir: [1.0, 0.0, 0.0],
        }
    }

    /// Create standard axes (Z-axis as main direction, X-axis as x direction)
    pub fn standard() -> Self {
        GpAxes {
            main_dir: [0.0, 0.0, 1.0],
            x_dir: [1.0, 0.0, 0.0],
        }
    }

    /// Get main direction
    pub fn main_dir(&self) -> &[f64; 3] {
        &self.main_dir
    }

    /// Set main direction
    pub fn set_main_dir(&mut self, dir: [f64; 3]) {
        self.main_dir = dir;
    }

    /// Get X direction
    pub fn x_dir(&self) -> &[f64; 3] {
        &self.x_dir
    }

    /// Set X direction
    pub fn set_x_dir(&mut self, dir: [f64; 3]) {
        self.x_dir = dir;
    }
}

impl Default for GpAxes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let axes = GpAxes::new();
        assert_eq!(*axes.main_dir(), [0.0, 0.0, 1.0]);
        assert_eq!(*axes.x_dir(), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_standard() {
        let axes = GpAxes::standard();
        assert_eq!(*axes.main_dir(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_set_directions() {
        let mut axes = GpAxes::new();
        axes.set_main_dir([0.0, 1.0, 0.0]);
        axes.set_x_dir([0.0, 0.0, 1.0]);

        assert_eq!(*axes.main_dir(), [0.0, 1.0, 0.0]);
        assert_eq!(*axes.x_dir(), [0.0, 0.0, 1.0]);
    }
}
