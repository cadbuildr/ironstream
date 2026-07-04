// FILE: topo_ds_alert_attribute.rs
// occt: TopoDS_AlertAttribute

//! Alert attribute for shape validation and diagnostic information.

/// Alert attribute for shape diagnostics
#[derive(Clone, Debug)]
pub struct TopoDS_AlertAttribute {
    message: String,
    severity: AlertSeverity,
}

/// Alert severity level
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertSeverity {
    Info = 0,
    Warning = 1,
    Error = 2,
    Critical = 3,
}

impl TopoDS_AlertAttribute {
    /// Creates new alert attribute
    pub fn new(message: String, severity: AlertSeverity) -> Self {
        TopoDS_AlertAttribute { message, severity }
    }

    /// Returns alert message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns alert severity
    pub fn severity(&self) -> AlertSeverity {
        self.severity
    }

    /// Returns whether alert is critical
    pub fn is_critical(&self) -> bool {
        self.severity == AlertSeverity::Critical
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_attribute_new() {
        let alert = TopoDS_AlertAttribute::new("Test message".to_string(), AlertSeverity::Warning);
        assert_eq!(alert.message(), "Test message");
        assert_eq!(alert.severity(), AlertSeverity::Warning);
    }

    #[test]
    fn test_alert_attribute_is_critical() {
        let alert_warn = TopoDS_AlertAttribute::new("Warning".to_string(), AlertSeverity::Warning);
        let alert_crit = TopoDS_AlertAttribute::new("Critical".to_string(), AlertSeverity::Critical);
        assert!(!alert_warn.is_critical());
        assert!(alert_crit.is_critical());
    }
}
