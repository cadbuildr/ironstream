// FILE: iges_select_set_global_parameter.rs
// occt: IGESSelect_SetGlobalParameter

/// Sets a Global (Header) Parameter to a new value in an IGES file.
/// Controls the form of the parameter (Integer, Real, String) but not consistency
/// with the rest of the file.
pub struct IgesSelectSetGlobalParameter {
    param_number: i32,
    value: Option<String>,
}

impl IgesSelectSetGlobalParameter {
    /// Creates a SetGlobalParameter modifier for the given global parameter number.
    ///
    /// # Arguments
    /// - `numpar`: The global parameter number (1-based)
    pub fn new(numpar: i32) -> Self {
        IgesSelectSetGlobalParameter {
            param_number: numpar,
            value: None,
        }
    }

    /// Returns the global parameter number this modifier applies to.
    pub fn global_number(&self) -> i32 {
        self.param_number
    }

    /// Sets the text value for the global parameter.
    ///
    /// # Arguments
    /// - `text`: The new parameter value as a string
    pub fn set_value(&mut self, text: Option<String>) {
        self.value = text;
    }

    /// Returns the value to set to the global parameter.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Applies the modification. Checks the form of the new value regarding
    /// the parameter number.
    pub fn perform(&self, _target: Option<&dyn std::any::Any>) {
        // Real implementation would:
        // 1. Validate the value matches the parameter type (integer, real, or string)
        // 2. Apply the change to the target IGES model
    }

    /// Returns a descriptive label for this modifier.
    pub fn label(&self) -> String {
        match &self.value {
            None => format!("Sets Global Parameter {} to (unset)", self.param_number),
            Some(val) => format!("Sets Global Parameter {} to {}", self.param_number, val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_global_parameter_creation() {
        let sgp = IgesSelectSetGlobalParameter::new(5);
        assert_eq!(sgp.global_number(), 5);
        assert_eq!(sgp.value(), None);
    }

    #[test]
    fn test_set_global_parameter_set_value() {
        let mut sgp = IgesSelectSetGlobalParameter::new(3);
        sgp.set_value(Some("Test Value".to_string()));
        assert_eq!(sgp.value(), Some("Test Value"));
    }

    #[test]
    fn test_set_global_parameter_label_no_value() {
        let sgp = IgesSelectSetGlobalParameter::new(2);
        assert_eq!(
            sgp.label(),
            "Sets Global Parameter 2 to (unset)".to_string()
        );
    }

    #[test]
    fn test_set_global_parameter_label_with_value() {
        let mut sgp = IgesSelectSetGlobalParameter::new(7);
        sgp.set_value(Some("MyValue".to_string()));
        assert_eq!(
            sgp.label(),
            "Sets Global Parameter 7 to MyValue".to_string()
        );
    }

    #[test]
    fn test_set_global_parameter_perform() {
        let sgp = IgesSelectSetGlobalParameter::new(1);
        sgp.perform(None); // Should not panic
    }
}
