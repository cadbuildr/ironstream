// FILE: t_data_xtd_position.rs
// occt: TDataXtd_Position

/// Position of a Label
pub struct TDataXtdPosition {
    position: (f64, f64, f64),
}

impl TDataXtdPosition {
    pub fn new() -> Self {
        TDataXtdPosition {
            position: (0.0, 0.0, 0.0),
        }
    }

    pub fn with_position(x: f64, y: f64, z: f64) -> Self {
        TDataXtdPosition {
            position: (x, y, z),
        }
    }

    pub fn get_id() -> [u8; 16] {
        [0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    }

    pub fn id(&self) -> [u8; 16] {
        Self::get_id()
    }

    pub fn position(&self) -> (f64, f64, f64) {
        self.position
    }

    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.position = (x, y, z);
    }

    pub fn x(&self) -> f64 {
        self.position.0
    }

    pub fn y(&self) -> f64 {
        self.position.1
    }

    pub fn z(&self) -> f64 {
        self.position.2
    }

    pub fn dump(&self) -> std::string::String {
        std::format!(
            "TDataXtd_Position: ({}, {}, {})",
            self.position.0, self.position.1, self.position.2
        )
    }
}

impl Default for TDataXtdPosition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let pos = TDataXtdPosition::new();
        let (x, y, z) = pos.position();
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }

    #[test]
    fn test_create_with_position() {
        let pos = TDataXtdPosition::with_position(1.0, 2.0, 3.0);
        assert_eq!(pos.x(), 1.0);
        assert_eq!(pos.y(), 2.0);
        assert_eq!(pos.z(), 3.0);
    }

    #[test]
    fn test_set_position() {
        let mut pos = TDataXtdPosition::new();
        pos.set_position(5.0, 6.0, 7.0);
        assert_eq!(pos.x(), 5.0);
        assert_eq!(pos.y(), 6.0);
        assert_eq!(pos.z(), 7.0);
    }

    #[test]
    fn test_get_id() {
        let id = TDataXtdPosition::get_id();
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn test_instance_id() {
        let pos = TDataXtdPosition::new();
        let id = pos.id();
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn test_dump() {
        let pos = TDataXtdPosition::with_position(1.5, 2.5, 3.5);
        let dump = pos.dump();
        assert!(dump.contains("TDataXtd_Position"));
    }

    #[test]
    fn test_default_trait() {
        let pos = TDataXtdPosition::default();
        assert_eq!(pos.x(), 0.0);
    }

    #[test]
    fn test_negative_coordinates() {
        let pos = TDataXtdPosition::with_position(-1.0, -2.0, -3.0);
        assert_eq!(pos.x(), -1.0);
        assert_eq!(pos.y(), -2.0);
        assert_eq!(pos.z(), -3.0);
    }
}
