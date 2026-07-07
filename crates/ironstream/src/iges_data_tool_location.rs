// FILE: iges_data_tool_location.rs
// occt: IGESData_ToolLocation

//! Tool for handling location and transformation data in IGES.

#[derive(Clone, Debug)]
pub struct ToolLocation {
    x: f64,
    y: f64,
    z: f64,
}

impl ToolLocation {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        ToolLocation { x, y, z }
    }

    pub fn coordinates(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }
}

impl Default for ToolLocation {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let loc = ToolLocation::new(1.0, 2.0, 3.0);
        assert_eq!(loc.coordinates(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_set_coordinates() {
        let mut loc = ToolLocation::new(0.0, 0.0, 0.0);
        loc.set_x(5.0);
        loc.set_y(6.0);
        loc.set_z(7.0);
        assert_eq!(loc.coordinates(), (5.0, 6.0, 7.0));
    }

    #[test]
    fn test_default() {
        let loc = ToolLocation::default();
        assert_eq!(loc.coordinates(), (0.0, 0.0, 0.0));
    }
}
