// FILE: bin_m_data_xtd_presentation_driver.rs
// occt: BinMDataXtd_PresentationDriver

use std::fmt::Debug;

/// Binary serialization/deserialization driver for presentation attributes.
/// Handles persistence of presentation properties: display status, GUID, color, material, transparency, width, mode.
pub struct BinMDataXtdPresentationDriver {
    _message_driver: Option<String>,
}

impl BinMDataXtdPresentationDriver {
    pub fn new(_message_driver: Option<String>) -> Self {
        BinMDataXtdPresentationDriver {
            _message_driver,
        }
    }

    /// Create a new empty presentation attribute.
    pub fn new_empty(&self) -> MockPresentation {
        MockPresentation::new()
    }

    /// Deserialize presentation from binary source.
    pub fn paste_read(
        &self,
        source: &[u8],
        offset: usize,
    ) -> Result<(MockPresentation, usize), String> {
        let mut current_offset = offset;

        // Read display status
        let (displayed_int, next_offset) = read_i32(source, current_offset)?;
        current_offset = next_offset;
        let displayed = displayed_int != 0;

        // Read GUID (16 bytes)
        let guid = read_guid(source, current_offset)?;
        current_offset += 16;

        // Read color
        let (color_value, next_offset) = read_i32(source, current_offset)?;
        current_offset = next_offset;
        let color = if color_value != -1 {
            Some(color_value)
        } else {
            None
        };

        // Read material
        let (material_value, next_offset) = read_i32(source, current_offset)?;
        current_offset = next_offset;
        let material = if material_value != -1 {
            Some(material_value)
        } else {
            None
        };

        // Read transparency
        let (transparency_value, next_offset) = read_f64(source, current_offset)?;
        current_offset = next_offset;
        let transparency = if (transparency_value - (-1.0)).abs() > 1e-10 {
            Some(transparency_value)
        } else {
            None
        };

        // Read width
        let (width_value, next_offset) = read_f64(source, current_offset)?;
        current_offset = next_offset;
        let width = if (width_value - (-1.0)).abs() > 1e-10 {
            Some(width_value)
        } else {
            None
        };

        // Read mode
        let (mode_value, next_offset) = read_i32(source, current_offset)?;
        current_offset = next_offset;
        let mode = if mode_value != -1 {
            Some(mode_value)
        } else {
            None
        };

        let presentation = MockPresentation {
            displayed,
            driver_guid: guid,
            color,
            material,
            transparency,
            width,
            mode,
        };

        Ok((presentation, current_offset))
    }

    /// Serialize presentation to binary target.
    pub fn paste_write(&self, presentation: &MockPresentation, target: &mut Vec<u8>) {
        // Write display status
        write_i32(target, if presentation.displayed { 1 } else { 0 });

        // Write GUID
        write_guid(target, &presentation.driver_guid);

        // Write color
        write_i32(target, presentation.color.unwrap_or(-1));

        // Write material
        write_i32(target, presentation.material.unwrap_or(-1));

        // Write transparency
        write_f64(target, presentation.transparency.unwrap_or(-1.0));

        // Write width
        write_f64(target, presentation.width.unwrap_or(-1.0));

        // Write mode
        write_i32(target, presentation.mode.unwrap_or(-1));
    }
}

/// Mock presentation attribute for testing serialization.
#[derive(Clone, Debug, PartialEq)]
pub struct MockPresentation {
    pub displayed: bool,
    pub driver_guid: [u8; 16],
    pub color: Option<i32>,
    pub material: Option<i32>,
    pub transparency: Option<f64>,
    pub width: Option<f64>,
    pub mode: Option<i32>,
}

impl MockPresentation {
    pub fn new() -> Self {
        MockPresentation {
            displayed: false,
            driver_guid: [0; 16],
            color: None,
            material: None,
            transparency: None,
            width: None,
            mode: None,
        }
    }

    pub fn with_displayed(mut self, displayed: bool) -> Self {
        self.displayed = displayed;
        self
    }

    pub fn with_color(mut self, color: i32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_material(mut self, material: i32) -> Self {
        self.material = Some(material);
        self
    }

    pub fn with_transparency(mut self, transparency: f64) -> Self {
        self.transparency = Some(transparency);
        self
    }

    pub fn with_width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_mode(mut self, mode: i32) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn with_guid(mut self, guid: [u8; 16]) -> Self {
        self.driver_guid = guid;
        self
    }
}

impl Default for MockPresentation {
    fn default() -> Self {
        MockPresentation::new()
    }
}

fn read_i32(source: &[u8], offset: usize) -> Result<(i32, usize), String> {
    if offset + 4 > source.len() {
        return Err("Insufficient data for i32".to_string());
    }
    let bytes: [u8; 4] = [source[offset], source[offset + 1], source[offset + 2], source[offset + 3]];
    Ok((i32::from_le_bytes(bytes), offset + 4))
}

fn read_f64(source: &[u8], offset: usize) -> Result<(f64, usize), String> {
    if offset + 8 > source.len() {
        return Err("Insufficient data for f64".to_string());
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

fn read_guid(source: &[u8], offset: usize) -> Result<[u8; 16], String> {
    if offset + 16 > source.len() {
        return Err("Insufficient data for GUID".to_string());
    }
    let mut guid = [0u8; 16];
    guid.copy_from_slice(&source[offset..offset + 16]);
    Ok(guid)
}

fn write_i32(target: &mut Vec<u8>, value: i32) {
    target.extend_from_slice(&value.to_le_bytes());
}

fn write_f64(target: &mut Vec<u8>, value: f64) {
    target.extend_from_slice(&value.to_le_bytes());
}

fn write_guid(target: &mut Vec<u8>, guid: &[u8; 16]) {
    target.extend_from_slice(guid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_creation() {
        let driver = BinMDataXtdPresentationDriver::new(None);
        let pres = driver.new_empty();
        assert!(!pres.displayed);
        assert_eq!(pres.color, None);
        assert_eq!(pres.material, None);
    }

    #[test]
    fn test_empty_presentation_serialize() {
        let driver = BinMDataXtdPresentationDriver::new(None);

        let original = MockPresentation::new();

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.displayed, false);
        assert_eq!(deserialized.color, None);
        assert_eq!(deserialized.material, None);
        assert_eq!(deserialized.transparency, None);
        assert_eq!(deserialized.width, None);
        assert_eq!(deserialized.mode, None);
    }

    #[test]
    fn test_presentation_with_all_properties() {
        let driver = BinMDataXtdPresentationDriver::new(None);
        let guid = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        let original = MockPresentation::new()
            .with_displayed(true)
            .with_color(5)
            .with_material(10)
            .with_transparency(0.5)
            .with_width(2.0)
            .with_mode(3)
            .with_guid(guid);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.displayed, true);
        assert_eq!(deserialized.color, Some(5));
        assert_eq!(deserialized.material, Some(10));
        assert!((deserialized.transparency.unwrap() - 0.5).abs() < 1e-10);
        assert!((deserialized.width.unwrap() - 2.0).abs() < 1e-10);
        assert_eq!(deserialized.mode, Some(3));
        assert_eq!(deserialized.driver_guid, guid);
    }

    #[test]
    fn test_presentation_partial_properties() {
        let driver = BinMDataXtdPresentationDriver::new(None);

        let original = MockPresentation::new()
            .with_displayed(true)
            .with_color(3);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.displayed, true);
        assert_eq!(deserialized.color, Some(3));
        assert_eq!(deserialized.material, None);
        assert_eq!(deserialized.transparency, None);
    }

    #[test]
    fn test_guid_serialization() {
        let driver = BinMDataXtdPresentationDriver::new(None);
        let guid = [255, 128, 64, 32, 16, 8, 4, 2, 1, 0, 200, 150, 100, 50, 25, 12];

        let original = MockPresentation::new().with_guid(guid);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.driver_guid, guid);
    }

    #[test]
    fn test_transparency_edge_values() {
        let driver = BinMDataXtdPresentationDriver::new(None);

        let original = MockPresentation::new().with_transparency(0.0);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert!((deserialized.transparency.unwrap() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_read_insufficient_data() {
        let driver = BinMDataXtdPresentationDriver::new(None);
        let short_buffer = vec![1, 2];
        let result = driver.paste_read(&short_buffer, 0);
        assert!(result.is_err());
    }
}
