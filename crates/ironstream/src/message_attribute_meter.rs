// FILE: message_attribute_meter.rs
// occt: Message_AttributeMeter

/// Meter attribute for performance measurement.
pub struct MessageAttributeMeter {
    name: String,
    value: f64,
}

impl MessageAttributeMeter {
    pub fn new(name: &str, value: f64) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let meter = MessageAttributeMeter::new("time", 1.5);
        assert_eq!(meter.name(), "time");
        assert_eq!(meter.value(), 1.5);
    }
}
