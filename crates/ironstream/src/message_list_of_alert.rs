// FILE: message_list_of_alert.rs
// occt: Message_ListOfAlert

use std::rc::Rc;
use std::cell::RefCell;

/// Message_Alert represents an alert/warning message.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageAlert {
    message: String,
    severity: AlertSeverity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl MessageAlert {
    pub fn new(message: String, severity: AlertSeverity) -> Self {
        MessageAlert { message, severity }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn severity(&self) -> AlertSeverity {
        self.severity
    }

    pub fn is_critical(&self) -> bool {
        self.severity == AlertSeverity::Critical
    }
}

/// A handle/reference-counted wrapper for Message_Alert.
pub type MessageAlertHandle = Rc<RefCell<MessageAlert>>;

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_List<opencascade::handle<Message_Alert>>`
pub type MessageListOfAlert = Vec<MessageAlertHandle>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_creation() {
        let alert = MessageAlert::new("Test Alert".to_string(), AlertSeverity::Warning);
        assert_eq!(alert.message(), "Test Alert");
        assert_eq!(alert.severity(), AlertSeverity::Warning);
        assert!(!alert.is_critical());
    }

    #[test]
    fn test_alert_severity_levels() {
        let info = MessageAlert::new("Info".to_string(), AlertSeverity::Info);
        let warning = MessageAlert::new("Warning".to_string(), AlertSeverity::Warning);
        let error = MessageAlert::new("Error".to_string(), AlertSeverity::Error);
        let critical = MessageAlert::new("Critical".to_string(), AlertSeverity::Critical);

        assert_eq!(info.severity(), AlertSeverity::Info);
        assert_eq!(warning.severity(), AlertSeverity::Warning);
        assert_eq!(error.severity(), AlertSeverity::Error);
        assert!(critical.is_critical());
    }

    #[test]
    fn test_list_creation() {
        let list: MessageListOfAlert = Vec::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_push_and_access() {
        let mut list: MessageListOfAlert = Vec::new();

        let alert1 = Rc::new(RefCell::new(MessageAlert::new(
            "Alert 1".to_string(),
            AlertSeverity::Warning,
        )));
        let alert2 = Rc::new(RefCell::new(MessageAlert::new(
            "Alert 2".to_string(),
            AlertSeverity::Error,
        )));

        list.push(alert1.clone());
        list.push(alert2.clone());

        assert_eq!(list.len(), 2);
        assert_eq!(list[0].borrow().severity(), AlertSeverity::Warning);
        assert_eq!(list[1].borrow().severity(), AlertSeverity::Error);
    }

    #[test]
    fn test_list_iteration() {
        let mut list: MessageListOfAlert = Vec::new();

        for i in 0..3 {
            let alert = Rc::new(RefCell::new(MessageAlert::new(
                format!("Alert {}", i),
                AlertSeverity::Warning,
            )));
            list.push(alert);
        }

        assert_eq!(list.len(), 3);

        let mut count = 0;
        for alert_handle in &list {
            assert!(alert_handle.borrow().message().starts_with("Alert"));
            count += 1;
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn test_list_remove() {
        let mut list: MessageListOfAlert = Vec::new();

        let alert1 = Rc::new(RefCell::new(MessageAlert::new(
            "Alert 1".to_string(),
            AlertSeverity::Info,
        )));
        let alert2 = Rc::new(RefCell::new(MessageAlert::new(
            "Alert 2".to_string(),
            AlertSeverity::Warning,
        )));

        list.push(alert1.clone());
        list.push(alert2.clone());

        assert_eq!(list.len(), 2);
        list.remove(0);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].borrow().severity(), AlertSeverity::Warning);
    }

    #[test]
    fn test_list_with_critical_alerts() {
        let mut list: MessageListOfAlert = Vec::new();

        let mut has_critical = false;
        for severity in &[
            AlertSeverity::Info,
            AlertSeverity::Warning,
            AlertSeverity::Critical,
        ] {
            let alert = Rc::new(RefCell::new(MessageAlert::new(
                format!("Alert {:?}", severity),
                *severity,
            )));
            list.push(alert);
        }

        for alert_handle in &list {
            if alert_handle.borrow().is_critical() {
                has_critical = true;
            }
        }
        assert!(has_critical);
    }
}
