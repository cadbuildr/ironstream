// FILE: step_data.rs
// occt: StepData

//! Basic data definition for STEP Interface.
//! Manages header protocols and initialization for STEP data models.

use std::sync::Mutex;

/// Simplified representation of a Protocol
#[derive(Debug, Clone)]
pub struct Protocol {
    name: String,
}

impl Protocol {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Thread-safe global protocol storage
static HEADER_PROTOCOL: Mutex<Option<Protocol>> = Mutex::new(None);
static PROTOCOL: Mutex<Option<Protocol>> = Mutex::new(None);

/// Gives basic data definition for STEP Interface.
pub struct StepData;

impl StepData {
    /// Returns the recorded HeaderProtocol
    pub fn header_protocol() -> Option<Protocol> {
        HEADER_PROTOCOL.lock().ok().and_then(|guard| guard.clone())
    }

    /// Adds a new Header Protocol to the Header Definition
    pub fn add_header_protocol(protocol: Protocol) {
        if let Ok(mut guard) = HEADER_PROTOCOL.lock() {
            *guard = Some(protocol);
        }
    }

    /// Prepares general data required to work with this package
    pub fn init() {
        // Initialize default protocol if not already done
        if let Ok(guard) = PROTOCOL.lock() {
            if guard.is_none() {
                drop(guard);
                let default_protocol = Protocol::new("StepData_Protocol".to_string());
                if let Ok(mut guard) = PROTOCOL.lock() {
                    *guard = Some(default_protocol);
                }
            }
        }
    }

    /// Returns a Protocol from StepData
    pub fn protocol() -> Option<Protocol> {
        PROTOCOL.lock().ok().and_then(|guard| guard.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_new() {
        let proto = Protocol::new("TestProto".to_string());
        assert_eq!(proto.name(), "TestProto");
    }

    #[test]
    fn test_header_protocol_initially_none() {
        // Clear any prior state
        if let Ok(mut guard) = HEADER_PROTOCOL.lock() {
            *guard = None;
        }
        assert!(StepData::header_protocol().is_none());
    }

    #[test]
    fn test_add_header_protocol() {
        let proto = Protocol::new("HeaderProto".to_string());
        StepData::add_header_protocol(proto);
        let retrieved = StepData::header_protocol();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "HeaderProto");
    }

    #[test]
    fn test_init() {
        // Reset protocol
        if let Ok(mut guard) = PROTOCOL.lock() {
            *guard = None;
        }
        StepData::init();
        let proto = StepData::protocol();
        assert!(proto.is_some());
        assert_eq!(proto.unwrap().name(), "StepData_Protocol");
    }

    #[test]
    fn test_protocol() {
        StepData::init();
        let proto = StepData::protocol();
        assert!(proto.is_some());
    }
}
