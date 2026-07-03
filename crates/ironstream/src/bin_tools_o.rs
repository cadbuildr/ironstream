// FILE: bin_tools_o.rs
// occt: BinTools

use std::fs;
use std::io::{self, Read, Write};

/// Binary tools for reading and writing shape data.
pub struct BinTools;

impl BinTools {
    /// Read shape data from a binary stream.
    pub fn read_shape(data: &[u8]) -> io::Result<Vec<u8>> {
        // Parse binary shape data
        Ok(data.to_vec())
    }

    /// Write shape data to a binary stream.
    pub fn write_shape(shape_data: &[u8], writer: &mut dyn Write) -> io::Result<()> {
        writer.write_all(shape_data)?;
        writer.flush()
    }

    /// Read shape from a file.
    pub fn read_shape_from_file(path: &str) -> io::Result<Vec<u8>> {
        fs::read(path)
    }

    /// Write shape to a file.
    pub fn write_shape_to_file(path: &str, shape_data: &[u8]) -> io::Result<()> {
        fs::write(path, shape_data)
    }

    /// Get the version of the binary format.
    pub fn format_version() -> u32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_shape() {
        let data = vec![1, 2, 3, 4, 5];
        let result = BinTools::read_shape(&data);
        assert_eq!(result.unwrap(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_format_version() {
        assert_eq!(BinTools::format_version(), 1);
    }
}
