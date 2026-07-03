// FILE: message_status_type.rs
// occt: Message_StatusType

/// Status type enumeration for message categorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageStatusType {
    Info = 0,
    Warning = 1,
    Error = 2,
    Critical = 3,
}

impl MessageStatusType {
    pub fn name(&self) -> &'static str {
        match self {
            MessageStatusType::Info => "Info",
            MessageStatusType::Warning => "Warning",
            MessageStatusType::Error => "Error",
            MessageStatusType::Critical => "Critical",
        }
    }

    pub fn severity(&self) -> i32 {
        match self {
            MessageStatusType::Info => 0,
            MessageStatusType::Warning => 1,
            MessageStatusType::Error => 2,
            MessageStatusType::Critical => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_names() {
        assert_eq!(MessageStatusType::Info.name(), "Info");
        assert_eq!(MessageStatusType::Error.name(), "Error");
    }

    #[test]
    fn test_severity() {
        assert_eq!(MessageStatusType::Info.severity(), 0);
        assert_eq!(MessageStatusType::Critical.severity(), 3);
    }
}
