// FILE: bin_tools_shape_writer.rs
// occt: BinTools_ShapeWriter

use std::io::Write;

/// Writes binary shape data to streams.
pub struct BinToolsShapeWriter;

impl BinToolsShapeWriter {
    /// Write shape data to bytes.
    pub fn write_shape(shape_data: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        // Write header
        result.extend((1u32).to_le_bytes()); // version
        result.extend((shape_data.len() as u32).to_le_bytes()); // size
        result.extend_from_slice(shape_data);
        result
    }

    /// Write shape to a writer.
    pub fn write_shape_to_writer(
        shape_data: &[u8],
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        writer.write_all(&(1u32).to_le_bytes())?; // version
        writer.write_all(&(shape_data.len() as u32).to_le_bytes())?; // size
        writer.write_all(shape_data)?;
        Ok(())
    }

    /// Write shape header.
    pub fn write_header(version: u32, size: u32) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(version.to_le_bytes());
        result.extend(size.to_le_bytes());
        result
    }

    /// Calculate total size needed.
    pub fn calculate_size(shape_data: &[u8]) -> u32 {
        8 + shape_data.len() as u32 // 8 bytes for header + data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_shape() {
        let data = vec![1, 2, 3];
        let result = BinToolsShapeWriter::write_shape(&data);

        // Check header
        let version = u32::from_le_bytes([result[0], result[1], result[2], result[3]]);
        let size = u32::from_le_bytes([result[4], result[5], result[6], result[7]]);

        assert_eq!(version, 1);
        assert_eq!(size, 3);
        assert_eq!(&result[8..], &data[..]);
    }

    #[test]
    fn test_write_header() {
        let header = BinToolsShapeWriter::write_header(2, 10);
        assert_eq!(header.len(), 8);

        let version = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        let size = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);

        assert_eq!(version, 2);
        assert_eq!(size, 10);
    }

    #[test]
    fn test_calculate_size() {
        let data = vec![1, 2, 3, 4, 5];
        let size = BinToolsShapeWriter::calculate_size(&data);
        assert_eq!(size, 8 + 5);
    }
}
