// FILE: standard_dump.rs
// occt: Standard_Dump

/// Utility for dumping object information to streams.
pub struct StandardDump;

impl StandardDump {
    /// Dumps a formatted header to a string.
    pub fn format_header(name: &str, depth: i32) -> String {
        let indent = if depth > 0 {
            " ".repeat(depth as usize * 2)
        } else {
            String::new()
        };
        format!("{}[{}]", indent, name)
    }

    /// Dumps a named field with a single value.
    pub fn format_field(name: &str, value: &str, depth: i32) -> String {
        let indent = if depth > 0 {
            " ".repeat(depth as usize * 2)
        } else {
            String::new()
        };
        format!("{}{}: {}", indent, name, value)
    }

    /// Dumps a named field with multiple values.
    pub fn format_field_values(name: &str, values: &[&str], depth: i32) -> String {
        let indent = if depth > 0 {
            " ".repeat(depth as usize * 2)
        } else {
            String::new()
        };
        format!("{}{}: [{}]", indent, name, values.join(", "))
    }

    /// Dumps numerical field values.
    pub fn format_numerical_field(name: &str, values: &[f64], depth: i32) -> String {
        let indent = if depth > 0 {
            " ".repeat(depth as usize * 2)
        } else {
            String::new()
        };
        let val_strs: Vec<String> = values.iter().map(|v| format!("{}", v)).collect();
        format!("{}{}: [{}]", indent, name, val_strs.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_header() {
        let header = StandardDump::format_header("MyClass", 0);
        assert_eq!(header, "[MyClass]");

        let header_indented = StandardDump::format_header("MyClass", 1);
        assert_eq!(header_indented, "  [MyClass]");
    }

    #[test]
    fn test_format_field() {
        let field = StandardDump::format_field("field", "value", 0);
        assert_eq!(field, "field: value");

        let field_indented = StandardDump::format_field("field", "value", 1);
        assert_eq!(field_indented, "  field: value");
    }

    #[test]
    fn test_format_field_values() {
        let vals = StandardDump::format_field_values("coords", &["1.0", "2.0", "3.0"], 0);
        assert_eq!(vals, "coords: [1.0, 2.0, 3.0]");
    }

    #[test]
    fn test_format_numerical_field() {
        let vals = StandardDump::format_numerical_field("data", &[1.5, 2.5, 3.5], 0);
        assert!(vals.contains("1.5"));
        assert!(vals.contains("2.5"));
        assert!(vals.contains("3.5"));
    }
}
