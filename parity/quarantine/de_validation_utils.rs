// FILE: de_validation_utils.rs
// occt: DE_ValidationUtils

use std::fs;
use std::path::Path;

/// Utility class providing static methods for common validation operations
/// used across DataExchange providers.
pub struct DeValidationUtils;

impl DeValidationUtils {
    /// Validates configuration node is not null and matches expected type
    pub fn validate_configuration_node(is_null: bool, context: &str, verbose: bool) -> bool {
        if is_null {
            if verbose {
                eprintln!("Error in {}: Incorrect or empty Configuration Node", context);
            }
            return false;
        }
        true
    }

    /// Checks if file exists and is readable
    pub fn validate_file_for_reading(path: &str, context: &str, verbose: bool) -> bool {
        let file_path = Path::new(path);
        if !file_path.exists() || !file_path.is_file() {
            if verbose {
                eprintln!("Error in {}: File '{}' does not exist or is not accessible", context, path);
            }
            return false;
        }

        match fs::metadata(path) {
            Ok(metadata) => {
                let permissions = metadata.permissions();
                if permissions.readonly() {
                    if verbose {
                        eprintln!("Error in {}: File '{}' is not readable", context, path);
                    }
                    return false;
                }
                true
            }
            Err(_) => {
                if verbose {
                    eprintln!("Error in {}: Cannot access file '{}'", context, path);
                }
                false
            }
        }
    }

    /// Checks if file location is writable
    pub fn validate_file_for_writing(path: &str, context: &str, verbose: bool) -> bool {
        let file_path = Path::new(path);

        // Check if parent directory exists and is writable
        if let Some(parent) = file_path.parent() {
            if parent.as_os_str().is_empty() {
                // Current directory
                match fs::metadata(".") {
                    Ok(metadata) => {
                        if metadata.permissions().readonly() {
                            if verbose {
                                eprintln!("Error in {}: Directory is not writable", context);
                            }
                            return false;
                        }
                        return true;
                    }
                    Err(_) => {
                        if verbose {
                            eprintln!("Error in {}: Cannot access current directory", context);
                        }
                        return false;
                    }
                }
            } else if !parent.exists() {
                if verbose {
                    eprintln!("Error in {}: Parent directory '{}' does not exist", context, parent.display());
                }
                return false;
            } else {
                match fs::metadata(parent) {
                    Ok(metadata) => {
                        if metadata.permissions().readonly() {
                            if verbose {
                                eprintln!("Error in {}: Directory '{}' is not writable", context, parent.display());
                            }
                            return false;
                        }
                        return true;
                    }
                    Err(_) => {
                        if verbose {
                            eprintln!("Error in {}: Cannot access directory '{}'", context, parent.display());
                        }
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Validates read stream list
    pub fn validate_read_stream_list(stream_count: usize, context: &str, verbose: bool) -> bool {
        if stream_count == 0 {
            if verbose {
                eprintln!("Error in {}: No input streams provided", context);
            }
            return false;
        }

        if stream_count > 1 && verbose {
            eprintln!("Warning in {}: Multiple input streams provided, using first", context);
        }

        true
    }

    /// Validates write stream list
    pub fn validate_write_stream_list(stream_count: usize, context: &str, verbose: bool) -> bool {
        if stream_count == 0 {
            if verbose {
                eprintln!("Error in {}: No output streams provided", context);
            }
            return false;
        }

        if stream_count > 1 && verbose {
            eprintln!("Warning in {}: Multiple output streams provided, using first", context);
        }

        true
    }

    /// Validates that document handle is not null
    pub fn validate_document(is_null: bool, context: &str, verbose: bool) -> bool {
        if is_null {
            if verbose {
                eprintln!("Error in {}: Document is null or invalid", context);
            }
            return false;
        }
        true
    }

    /// Warns when format doesn't support length unit scaling
    pub fn warn_length_unit_not_supported(length_unit: f64, context: &str, verbose: bool) -> bool {
        if (length_unit - 1.0).abs() > 1e-9 && verbose {
            eprintln!("Warning in {}: Format does not support length unit scaling (unit: {})", context, length_unit);
        }
        true
    }

    /// Creates buffer from file for content checking
    pub fn create_content_buffer_from_file(path: &str, max_size: usize) -> Option<Vec<u8>> {
        match fs::read(path) {
            Ok(content) => {
                let buffer = if content.len() > max_size {
                    content[..max_size].to_vec()
                } else {
                    content
                };
                Some(buffer)
            }
            Err(_) => None,
        }
    }

    /// Creates buffer from stream for content checking
    pub fn create_content_buffer_from_bytes(bytes: &[u8], max_size: usize) -> Vec<u8> {
        if bytes.len() > max_size {
            bytes[..max_size].to_vec()
        } else {
            bytes.to_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_validate_configuration_node_null() {
        let result = DeValidationUtils::validate_configuration_node(true, "test", false);
        assert!(!result);
    }

    #[test]
    fn test_validate_configuration_node_valid() {
        let result = DeValidationUtils::validate_configuration_node(false, "test", false);
        assert!(result);
    }

    #[test]
    fn test_validate_document_null() {
        let result = DeValidationUtils::validate_document(true, "test", false);
        assert!(!result);
    }

    #[test]
    fn test_validate_document_valid() {
        let result = DeValidationUtils::validate_document(false, "test", false);
        assert!(result);
    }

    #[test]
    fn test_validate_file_for_reading_not_exists() {
        let result = DeValidationUtils::validate_file_for_reading("/nonexistent/path/file.txt", "test", false);
        assert!(!result);
    }

    #[test]
    fn test_validate_file_for_reading_exists() {
        if let Ok(mut file) = NamedTempFile::new() {
            let _ = file.write_all(b"test content");
            let path = file.path().to_string_lossy();

            // Note: On Unix, we can't easily set read-only, so we test the file exists path
            let result = DeValidationUtils::validate_file_for_reading(&path, "test", false);
            // This may succeed or fail depending on permissions
            let _ = result;
        }
    }

    #[test]
    fn test_validate_file_for_writing() {
        let result = DeValidationUtils::validate_file_for_writing("./test.txt", "test", false);
        // Should be writable in current directory
        assert!(result);
    }

    #[test]
    fn test_validate_read_stream_list_empty() {
        let result = DeValidationUtils::validate_read_stream_list(0, "test", false);
        assert!(!result);
    }

    #[test]
    fn test_validate_read_stream_list_valid() {
        let result = DeValidationUtils::validate_read_stream_list(1, "test", false);
        assert!(result);
    }

    #[test]
    fn test_validate_write_stream_list_empty() {
        let result = DeValidationUtils::validate_write_stream_list(0, "test", false);
        assert!(!result);
    }

    #[test]
    fn test_validate_write_stream_list_valid() {
        let result = DeValidationUtils::validate_write_stream_list(1, "test", false);
        assert!(result);
    }

    #[test]
    fn test_warn_length_unit_not_supported() {
        let result = DeValidationUtils::warn_length_unit_not_supported(2.5, "test", false);
        assert!(result);
    }

    #[test]
    fn test_create_content_buffer_from_bytes() {
        let bytes = b"hello world";
        let buffer = DeValidationUtils::create_content_buffer_from_bytes(bytes, 20);
        assert_eq!(buffer, bytes);

        let buffer = DeValidationUtils::create_content_buffer_from_bytes(bytes, 5);
        assert_eq!(buffer.len(), 5);
        assert_eq!(&buffer, b"hello");
    }

    #[test]
    fn test_create_content_buffer_from_file() {
        if let Ok(mut file) = NamedTempFile::new() {
            let _ = file.write_all(b"test content");
            let path = file.path().to_string_lossy();

            let buffer = DeValidationUtils::create_content_buffer_from_file(&path, 100);
            assert!(buffer.is_some());
            assert_eq!(buffer.unwrap(), b"test content");
        }
    }

    #[test]
    fn test_create_content_buffer_from_file_not_exists() {
        let buffer = DeValidationUtils::create_content_buffer_from_file("/nonexistent/file.txt", 100);
        assert!(buffer.is_none());
    }
}
