// FILE: bin_m_data_xtd_position_driver.rs
// occt: BinMDataXtd_PositionDriver

/// Binary serialization/deserialization driver for position attributes.
/// Handles persistence of 3D point position (X, Y, Z coordinates).
pub struct BinMDataXtdPositionDriver {
    _message_driver: Option<String>,
}

impl BinMDataXtdPositionDriver {
    pub fn new(_message_driver: Option<String>) -> Self {
        BinMDataXtdPositionDriver {
            _message_driver,
        }
    }

    /// Create a new empty position attribute.
    pub fn new_empty(&self) -> MockPosition {
        MockPosition::new()
    }

    /// Deserialize position from binary source.
    /// Reads: X coordinate, Y coordinate, Z coordinate.
    pub fn paste_read(&self, source: &[u8], offset: usize) -> Result<(MockPosition, usize), String> {
        let mut current_offset = offset;

        let (x, next_offset) = read_f64(source, current_offset)?;
        current_offset = next_offset;

        let (y, next_offset) = read_f64(source, current_offset)?;
        current_offset = next_offset;

        let (z, next_offset) = read_f64(source, current_offset)?;
        current_offset = next_offset;

        let position = MockPosition::new().with_coords(x, y, z);

        Ok((position, current_offset))
    }

    /// Serialize position to binary target.
    /// Writes: X coordinate, Y coordinate, Z coordinate.
    pub fn paste_write(&self, position: &MockPosition, target: &mut Vec<u8>) {
        write_f64(target, position.x);
        write_f64(target, position.y);
        write_f64(target, position.z);
    }
}

/// Mock position attribute for testing serialization.
#[derive(Clone, Debug, PartialEq)]
pub struct MockPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl MockPosition {
    pub fn new() -> Self {
        MockPosition {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn with_coords(mut self, x: f64, y: f64, z: f64) -> Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }
}

impl Default for MockPosition {
    fn default() -> Self {
        MockPosition::new()
    }
}

fn read_f64(source: &[u8], offset: usize) -> Result<(f64, usize), String> {
    if offset + 8 > source.len() {
        return Err("Insufficient data".to_string());
    }
    let bytes: [u8; 8] = [
        source[offset],
        source[offset + 1],
        source[offset + 2],
        source[offset + 3],
        source[offset + 4],
        source[offset + 5],
        source[offset + 6],
        source[offset + 7],
    ];
    Ok((f64::from_le_bytes(bytes), offset + 8))
}

fn write_f64(target: &mut Vec<u8>, value: f64) {
    target.extend_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let driver = BinMDataXtdPositionDriver::new(None);
        let pos = driver.new_empty();
        assert_eq!(pos.x, 0.0);
        assert_eq!(pos.y, 0.0);
        assert_eq!(pos.z, 0.0);
    }

    #[test]
    fn test_position_with_coords() {
        let pos = MockPosition::new().with_coords(1.5, 2.5, 3.5);
        assert_eq!(pos.x, 1.5);
        assert_eq!(pos.y, 2.5);
        assert_eq!(pos.z, 3.5);
    }

    #[test]
    fn test_serialize_deserialize_origin() {
        let driver = BinMDataXtdPositionDriver::new(None);

        let original = MockPosition::new();

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.x, 0.0);
        assert_eq!(deserialized.y, 0.0);
        assert_eq!(deserialized.z, 0.0);
    }

    #[test]
    fn test_serialize_deserialize_positive_coords() {
        let driver = BinMDataXtdPositionDriver::new(None);

        let original = MockPosition::new().with_coords(10.5, 20.75, 30.125);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert!((deserialized.x - 10.5).abs() < 1e-10);
        assert!((deserialized.y - 20.75).abs() < 1e-10);
        assert!((deserialized.z - 30.125).abs() < 1e-10);
    }

    #[test]
    fn test_serialize_deserialize_negative_coords() {
        let driver = BinMDataXtdPositionDriver::new(None);

        let original = MockPosition::new().with_coords(-5.5, -10.25, -15.75);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert!((deserialized.x - (-5.5)).abs() < 1e-10);
        assert!((deserialized.y - (-10.25)).abs() < 1e-10);
        assert!((deserialized.z - (-15.75)).abs() < 1e-10);
    }

    #[test]
    fn test_serialize_deserialize_mixed_coords() {
        let driver = BinMDataXtdPositionDriver::new(None);

        let original = MockPosition::new().with_coords(100.0, -50.5, 0.001);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert!((deserialized.x - 100.0).abs() < 1e-10);
        assert!((deserialized.y - (-50.5)).abs() < 1e-10);
        assert!((deserialized.z - 0.001).abs() < 1e-10);
    }

    #[test]
    fn test_read_insufficient_data() {
        let driver = BinMDataXtdPositionDriver::new(None);
        let short_buffer = vec![1, 2, 3];
        let result = driver.paste_read(&short_buffer, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_round_trip_preserves_position() {
        let driver = BinMDataXtdPositionDriver::new(None);
        let mut buffer = Vec::new();

        let pos1 = MockPosition::new().with_coords(12.34, 56.78, 90.12);
        driver.paste_write(&pos1, &mut buffer);

        let (pos2, _) = driver.paste_read(&buffer, 0).unwrap();

        assert!((pos1.x - pos2.x).abs() < 1e-10);
        assert!((pos1.y - pos2.y).abs() < 1e-10);
        assert!((pos1.z - pos2.z).abs() < 1e-10);
    }
}
