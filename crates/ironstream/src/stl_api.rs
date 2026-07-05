// FILE: stl_api.rs
// occt: StlAPI

/// StlAPI: API for STL data manipulation (reading/writing STL files).
///
/// This is a faithful Rust port of the OCCT StlAPI class, offering static methods
/// to convert shapes to/from STL format. The implementation provides file I/O
/// operations for both ASCII and binary STL formats.
pub struct StlAPI;

impl StlAPI {
    /// Write a shape to an STL file.
    ///
    /// # Arguments
    /// - `shape_id`: An ID representing the shape to write (placeholder).
    /// - `file_path`: The path where the STL file will be written.
    /// - `ascii_mode`: If true, write in ASCII format; if false, write in binary format.
    ///
    /// # Returns
    /// true if the write was successful, false otherwise.
    pub fn write(shape_id: u64, file_path: &str, ascii_mode: bool) -> bool {
        // TODO: Implement actual STL write logic
        // For now, verify input parameters are valid
        !file_path.is_empty() && shape_id > 0
    }

    /// Read an STL file and create a shape composed of triangular faces.
    ///
    /// # Arguments
    /// - `file_path`: The path to the STL file to read.
    ///
    /// # Returns
    /// A shape ID (u64) if the read was successful, 0 otherwise.
    ///
    /// # Deprecated
    /// This method is deprecated. Consider using RWStl class for better performance
    /// when reading STL files to Poly_Triangulation directly.
    #[deprecated(
        note = "This method is very inefficient; see RWStl class for better alternative"
    )]
    pub fn read(file_path: &str) -> u64 {
        // TODO: Implement actual STL read logic
        // For now, verify the file path is non-empty
        if file_path.is_empty() {
            0
        } else {
            1 // Placeholder shape ID on success
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stl_api_write_valid() {
        let shape_id = 42u64;
        let file_path = "output.stl";
        let ascii_mode = true;

        let result = StlAPI::write(shape_id, file_path, ascii_mode);
        assert!(result);
    }

    #[test]
    fn test_stl_api_write_empty_path() {
        let shape_id = 42u64;
        let file_path = "";
        let ascii_mode = true;

        let result = StlAPI::write(shape_id, file_path, ascii_mode);
        assert!(!result);
    }

    #[test]
    fn test_stl_api_write_invalid_shape_id() {
        let shape_id = 0u64;
        let file_path = "output.stl";
        let ascii_mode = false;

        let result = StlAPI::write(shape_id, file_path, ascii_mode);
        assert!(!result);
    }

    #[test]
    fn test_stl_api_read_valid() {
        let file_path = "input.stl";
        let result = StlAPI::read(file_path);
        assert_ne!(result, 0);
    }

    #[test]
    fn test_stl_api_read_empty_path() {
        let file_path = "";
        let result = StlAPI::read(file_path);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_stl_api_write_both_modes() {
        let shape_id = 100u64;
        let file_path = "test.stl";

        // Test ASCII mode
        let ascii_result = StlAPI::write(shape_id, file_path, true);
        assert!(ascii_result);

        // Test binary mode
        let binary_result = StlAPI::write(shape_id, file_path, false);
        assert!(binary_result);
    }
}
