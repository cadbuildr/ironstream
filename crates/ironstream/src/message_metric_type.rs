// FILE: message_metric_type.rs
// occt: Message_MetricType

/// Metric type enumeration for performance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageMetricType {
    Time = 0,
    Memory = 1,
    Count = 2,
    Bytes = 3,
}

impl MessageMetricType {
    pub fn name(&self) -> &'static str {
        match self {
            MessageMetricType::Time => "Time",
            MessageMetricType::Memory => "Memory",
            MessageMetricType::Count => "Count",
            MessageMetricType::Bytes => "Bytes",
        }
    }

    pub fn unit(&self) -> &'static str {
        match self {
            MessageMetricType::Time => "ms",
            MessageMetricType::Memory => "MB",
            MessageMetricType::Count => "",
            MessageMetricType::Bytes => "B",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_names() {
        assert_eq!(MessageMetricType::Time.name(), "Time");
        assert_eq!(MessageMetricType::Memory.name(), "Memory");
    }

    #[test]
    fn test_metric_units() {
        assert_eq!(MessageMetricType::Time.unit(), "ms");
        assert_eq!(MessageMetricType::Memory.unit(), "MB");
    }
}
