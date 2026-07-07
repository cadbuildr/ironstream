// FILE: step_geom_direction.rs
// occt: StepGeom_Direction

use std::sync::Arc;

/// Direction: A unit or non-unit direction vector with 2 or 3 components.
#[derive(Clone)]
pub struct Direction {
    name: Arc<String>,
    nb_coord: i32,
    direction_ratios: [f64; 3],
}

impl Direction {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            nb_coord: 0,
            direction_ratios: [0.0; 3],
        }
    }

    pub fn init(&mut self, name: String, direction_ratios: [f64; 3]) {
        self.name = Arc::new(name);
        self.direction_ratios = direction_ratios;
        self.nb_coord = 3;
    }

    pub fn init_2d(&mut self, name: String, x: f64, y: f64) {
        self.name = Arc::new(name);
        self.direction_ratios = [x, y, 0.0];
        self.nb_coord = 2;
    }

    pub fn init_3d(&mut self, name: String, x: f64, y: f64, z: f64) {
        self.name = Arc::new(name);
        self.direction_ratios = [x, y, z];
        self.nb_coord = 3;
    }

    pub fn set_direction_ratios(&mut self, ratios: [f64; 3]) {
        self.direction_ratios = ratios;
    }

    pub fn direction_ratios(&self) -> [f64; 3] {
        self.direction_ratios
    }

    pub fn direction_ratios_value(&self, index: i32) -> Option<f64> {
        match index {
            1 => Some(self.direction_ratios[0]),
            2 => Some(self.direction_ratios[1]),
            3 => Some(self.direction_ratios[2]),
            _ => None,
        }
    }

    pub fn set_nb_direction_ratios(&mut self, size: i32) {
        self.nb_coord = size;
    }

    pub fn nb_direction_ratios(&self) -> i32 {
        self.nb_coord
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let dir = Direction::new();
        assert_eq!(dir.nb_direction_ratios(), 0);
    }

    #[test]
    fn test_init_3d() {
        let mut dir = Direction::new();
        dir.init_3d("z_axis".to_string(), 0.0, 0.0, 1.0);
        assert_eq!(dir.nb_direction_ratios(), 3);
        assert_eq!(dir.direction_ratios_value(3), Some(1.0));
    }

    #[test]
    fn test_init_2d() {
        let mut dir = Direction::new();
        dir.init_2d("x_axis_2d".to_string(), 1.0, 0.0);
        assert_eq!(dir.nb_direction_ratios(), 2);
        assert_eq!(dir.direction_ratios_value(1), Some(1.0));
    }
}
