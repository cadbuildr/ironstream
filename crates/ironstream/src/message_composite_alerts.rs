// FILE: message_composite_alerts.rs
// occt: Message_CompositeAlerts

/// Composite collection of alerts.
pub struct MessageCompositeAlerts {
    alerts: Vec<String>,
}

impl MessageCompositeAlerts {
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
        }
    }

    pub fn add_alert(&mut self, alert: &str) {
        self.alerts.push(alert.to_string());
    }

    pub fn alerts(&self) -> &[String] {
        &self.alerts
    }

    pub fn clear(&mut self) {
        self.alerts.clear();
    }
}

impl Default for MessageCompositeAlerts {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ca = MessageCompositeAlerts::new();
        assert!(ca.alerts().is_empty());
    }

    #[test]
    fn test_add_alert() {
        let mut ca = MessageCompositeAlerts::new();
        ca.add_alert("Alert 1");
        ca.add_alert("Alert 2");
        assert_eq!(ca.alerts().len(), 2);
    }
}
