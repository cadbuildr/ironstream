// FILE: step_shape_value_format_type_qualifier.rs
// occt: StepShape_ValueFormatTypeQualifier

use std::sync::Arc;

/// Represents a value format type qualifier for dimensional tolerances in STEP format.
pub struct ValueFormatTypeQualifier {
    format_type: Arc<str>,
}

impl ValueFormatTypeQualifier {
    /// Create a new ValueFormatTypeQualifier
    pub fn new() -> Self {
        ValueFormatTypeQualifier {
            format_type: Arc::from(""),
        }
    }

    /// Initialize with format type
    pub fn init(&mut self, format_type: Arc<str>) {
        self.format_type = format_type;
    }

    /// Get the format type
    pub fn format_type(&self) -> &str {
        &self.format_type
    }

    /// Set the format type
    pub fn set_format_type(&mut self, format_type: Arc<str>) {
        self.format_type = format_type;
    }
}

impl Default for ValueFormatTypeQualifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_format_type_qualifier_creation() {
        let vftq = ValueFormatTypeQualifier::new();
        assert_eq!(vftq.format_type(), "");
    }

    #[test]
    fn test_init_method() {
        let mut vftq = ValueFormatTypeQualifier::new();
        let format_type: Arc<str> = Arc::from("INTEGER");

        vftq.init(format_type.clone());

        assert_eq!(vftq.format_type(), "INTEGER");
    }

    #[test]
    fn test_set_format_type() {
        let mut vftq = ValueFormatTypeQualifier::new();
        let format_type: Arc<str> = Arc::from("REAL");

        vftq.set_format_type(format_type);

        assert_eq!(vftq.format_type(), "REAL");
    }

    #[test]
    fn test_format_type_changes() {
        let mut vftq = ValueFormatTypeQualifier::new();

        vftq.set_format_type(Arc::from("BOOLEAN"));
        assert_eq!(vftq.format_type(), "BOOLEAN");

        vftq.set_format_type(Arc::from("STRING"));
        assert_eq!(vftq.format_type(), "STRING");
    }
}
