// FILE: topo_ds_alert_with_shape.rs
// occt: TopoDS_AlertWithShape

//! Alert attribute that includes shape reference for diagnostics.

use crate::topo_ds::{Shape, ShapeType};
use crate::topo_ds_alert_attribute::AlertSeverity;

/// Alert with associated shape
#[derive(Clone)]
pub struct TopoDS_AlertWithShape {
    message: String,
    severity: AlertSeverity,
    shape: Option<Shape>,
}

impl TopoDS_AlertWithShape {
    /// Creates alert with shape
    pub fn new(message: String, severity: AlertSeverity, shape: Option<Shape>) -> Self {
        TopoDS_AlertWithShape {
            message,
            severity,
            shape,
        }
    }

    /// Returns alert message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns alert severity
    pub fn severity(&self) -> AlertSeverity {
        self.severity
    }

    /// Returns associated shape
    pub fn shape(&self) -> Option<&Shape> {
        self.shape.as_ref()
    }

    /// Returns whether alert has shape
    pub fn has_shape(&self) -> bool {
        self.shape.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_with_shape_new() {
        let shape = Shape::new(ShapeType::Face);
        let alert = TopoDS_AlertWithShape::new(
            "Shape issue".to_string(),
            AlertSeverity::Error,
            Some(shape),
        );
        assert_eq!(alert.message(), "Shape issue");
        assert!(alert.has_shape());
    }

    #[test]
    fn test_alert_with_shape_no_shape() {
        let alert = TopoDS_AlertWithShape::new(
            "Generic issue".to_string(),
            AlertSeverity::Warning,
            None,
        );
        assert!(!alert.has_shape());
    }
}
