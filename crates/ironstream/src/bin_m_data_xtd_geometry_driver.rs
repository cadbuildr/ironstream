// FILE: bin_m_data_xtd_geometry_driver.rs
// occt: BinMDataXtd_GeometryDriver

/// Binary serialization/deserialization driver for geometry attributes.
/// Handles persistence of geometry type information.
pub struct BinMDataXtdGeometryDriver {
    _message_driver: Option<String>,
}

impl BinMDataXtdGeometryDriver {
    pub fn new(_message_driver: Option<String>) -> Self {
        BinMDataXtdGeometryDriver {
            _message_driver,
        }
    }

    /// Create a new empty geometry attribute.
    pub fn new_empty(&self) -> MockGeometry {
        MockGeometry::new()
    }

    /// Deserialize geometry from binary source.
    /// Reads: geometry type.
    pub fn paste_read(&self, source: &[u8], offset: usize) -> Result<(MockGeometry, usize), String> {
        let (geom_type, next_offset) = read_i32(source, offset)?;
        let mut geometry = MockGeometry::new();
        geometry.geom_type = geom_type;
        Ok((geometry, next_offset))
    }

    /// Serialize geometry to binary target.
    /// Writes: geometry type.
    pub fn paste_write(&self, geometry: &MockGeometry, target: &mut Vec<u8>) {
        write_i32(target, geometry.geom_type);
    }
}

/// Mock geometry attribute for testing serialization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockGeometry {
    pub geom_type: i32,
}

impl MockGeometry {
    pub fn new() -> Self {
        MockGeometry { geom_type: 0 }
    }

    pub fn with_type(mut self, t: i32) -> Self {
        self.geom_type = t;
        self
    }
}

impl Default for MockGeometry {
    fn default() -> Self {
        MockGeometry::new()
    }
}

fn read_i32(source: &[u8], offset: usize) -> Result<(i32, usize), String> {
    if offset + 4 > source.len() {
        return Err("Insufficient data".to_string());
    }
    let bytes: [u8; 4] = [source[offset], source[offset + 1], source[offset + 2], source[offset + 3]];
    Ok((i32::from_le_bytes(bytes), offset + 4))
}

fn write_i32(target: &mut Vec<u8>, value: i32) {
    target.extend_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_creation() {
        let driver = BinMDataXtdGeometryDriver::new(None);
        let geom = driver.new_empty();
        assert_eq!(geom.geom_type, 0);
    }

    #[test]
    fn test_geometry_with_type() {
        let geom = MockGeometry::new().with_type(5);
        assert_eq!(geom.geom_type, 5);
    }

    #[test]
    fn test_serialize_deserialize() {
        let driver = BinMDataXtdGeometryDriver::new(None);

        let original = MockGeometry::new().with_type(3);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.geom_type, original.geom_type);
    }

    #[test]
    fn test_serialize_deserialize_various_types() {
        let driver = BinMDataXtdGeometryDriver::new(None);

        for t in &[0, 1, 5, 10, 127, 255, -1, -100] {
            let original = MockGeometry::new().with_type(*t);

            let mut buffer = Vec::new();
            driver.paste_write(&original, &mut buffer);

            let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

            assert_eq!(deserialized.geom_type, *t);
        }
    }

    #[test]
    fn test_read_insufficient_data() {
        let driver = BinMDataXtdGeometryDriver::new(None);
        let empty_buffer = vec![];
        let result = driver.paste_read(&empty_buffer, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_round_trip_preserves_type() {
        let driver = BinMDataXtdGeometryDriver::new(None);
        let mut buffer = Vec::new();

        let geom1 = MockGeometry::new().with_type(42);
        driver.paste_write(&geom1, &mut buffer);

        let (geom2, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(geom1, geom2);
    }
}
