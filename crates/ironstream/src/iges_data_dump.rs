// FILE: iges_data_dump.rs
// occt: IGESData_Dump

//! Macros and utilities to help dumping parts of IGES Entities.
//! Provides helper functions for outputting IGES entity data in a readable format.

/// Dump utility for formatting IGES entity data.
/// Provides helper functions for displaying simple data, coordinates, and lists.
#[derive(Clone, Debug)]
pub struct Dump;

impl Dump {
    /// Dumps a text value, handling null/empty cases
    pub fn dump_string(s: &str) -> String {
        if s.is_empty() {
            "(undefined)".to_string()
        } else {
            format!("\"{}\"", s)
        }
    }

    /// Dumps XY coordinates
    pub fn dump_xy(x: f64, y: f64) -> String {
        format!("({:.6}, {:.6})", x, y)
    }

    /// Dumps XYZ coordinates
    pub fn dump_xyz(x: f64, y: f64, z: f64) -> String {
        format!("({:.6}, {:.6}, {:.6})", x, y, z)
    }

    /// Dumps a list of values
    pub fn dump_list_header(lower: usize, upper: usize) -> String {
        if lower > upper {
            "Empty".to_string()
        } else {
            format!("({} - {})", lower, upper)
        }
    }

    /// Formats a list item index for display
    pub fn format_list_item(index: usize) -> String {
        format!("[{}]", index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dump_string_empty() {
        assert_eq!(Dump::dump_string(""), "(undefined)");
    }

    #[test]
    fn test_dump_string_nonempty() {
        let result = Dump::dump_string("test");
        assert!(result.contains("test"));
    }

    #[test]
    fn test_dump_xy() {
        let result = Dump::dump_xy(1.5, 2.5);
        assert!(result.contains("1.5"));
        assert!(result.contains("2.5"));
    }

    #[test]
    fn test_dump_xyz() {
        let result = Dump::dump_xyz(1.0, 2.0, 3.0);
        assert!(result.contains("1.0"));
        assert!(result.contains("2.0"));
        assert!(result.contains("3.0"));
    }

    #[test]
    fn test_dump_list_header_empty() {
        let result = Dump::dump_list_header(5, 3);
        assert_eq!(result, "Empty");
    }

    #[test]
    fn test_dump_list_header_nonempty() {
        let result = Dump::dump_list_header(1, 5);
        assert!(result.contains("1"));
        assert!(result.contains("5"));
    }

    #[test]
    fn test_format_list_item() {
        let result = Dump::format_list_item(42);
        assert_eq!(result, "[42]");
    }
}
