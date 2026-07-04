// FILE: interface_report_entity.rs
// occt: Interface_ReportEntity

/// Reports on an entity
pub struct InterfaceReportEntity {
    concerned: bool,
    message: String,
}

impl InterfaceReportEntity {
    pub fn new(concerned: bool, message: &str) -> Self {
        InterfaceReportEntity {
            concerned,
            message: message.to_string(),
        }
    }

    pub fn concerned(&self) -> bool {
        self.concerned
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Default for InterfaceReportEntity {
    fn default() -> Self {
        InterfaceReportEntity {
            concerned: false,
            message: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let report = InterfaceReportEntity::new(true, "test");
        assert!(report.concerned());
        assert_eq!(report.message(), "test");
    }
}
