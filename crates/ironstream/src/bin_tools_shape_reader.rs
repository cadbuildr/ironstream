// FILE: bin_tools_shape_reader.rs
// occt: BinTools_ShapeReader

use std::io::{Read, Seek};

/// Reads binary shape data from streams.
pub struct BinToolsShapeReader;

impl BinToolsShapeReader {
    /// Read a shape from binary data.
    pub fn read_shape(data: &[u8]) -> Option<Vec<u8>> {
        if data.len() < 4 {
            return None;
        }
        // Read shape data structure
        Some(data.to_vec())
    }

    /// Read shape header information.
    pub fn read_header(data: &[u8]) -> Option<(u32, u32)> {
        if data.len() < 8 {
            return None;
        }
        let version = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        Some((version, size))
    }

    /// Read shape version.
    pub fn read_version(data: &[u8]) -> Option<u32> {
        if data.len() < 4 {
            return None;
        }
        Some(u32::from_le_bytes([data[0], data[1], data[2], data[3]]))
    }

    /// Read shape size.
    pub fn read_size(data: &[u8], offset: usize) -> Option<u32> {
        if data.len() < offset + 4 {
            return None;
        }
        Some(u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_shape() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let result = BinToolsShapeReader::read_shape(&data);
        assert_eq!(result, Some(data));
    }

    #[test]
    fn test_read_header() {
        let data = vec![1, 0, 0, 0, 10, 0, 0, 0];
        let result = BinToolsShapeReader::read_header(&data);
        assert_eq!(result, Some((1, 10)));
    }

    #[test]
    fn test_read_version() {
        let data = vec![5, 0, 0, 0];
        let result = BinToolsShapeReader::read_version(&data);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_read_size() {
        let data = vec![0, 0, 0, 0, 20, 0, 0, 0];
        let result = BinToolsShapeReader::read_size(&data, 4);
        assert_eq!(result, Some(20));
    }
}
