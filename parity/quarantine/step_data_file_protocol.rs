// FILE: step_data_file_protocol.rs
// occt: StepData_FileProtocol

use std::collections::HashMap;

//! A FileProtocol is defined as the addition of several already
//! existing Protocols
pub struct StepDataFileProtocol {
    protocols: Vec<String>,
}

impl StepDataFileProtocol {
    //! Creates an empty FileProtocol
    pub fn new() -> Self {
        StepDataFileProtocol {
            protocols: Vec::new(),
        }
    }

    //! Adds a Protocol to the definition list
    pub fn add(&mut self, protocol_name: &str) {
        if !self.protocols.contains(&protocol_name.to_string()) {
            self.protocols.push(protocol_name.to_string());
        }
    }

    //! Gives the count of Protocols
    pub fn nb_resources(&self) -> usize {
        self.protocols.len()
    }

    //! Returns a Protocol, given a rank
    pub fn resource(&self, num: usize) -> Option<&str> {
        if num < 1 || num > self.protocols.len() {
            return None;
        }
        Some(&self.protocols[num - 1])
    }

    //! Returns the schema name
    pub fn schema_name(&self) -> &str {
        ""
    }
}

impl Default for StepDataFileProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_protocol_new() {
        let proto = StepDataFileProtocol::new();
        assert_eq!(proto.nb_resources(), 0);
    }

    #[test]
    fn test_add_protocol() {
        let mut proto = StepDataFileProtocol::new();
        proto.add("Protocol1");
        proto.add("Protocol2");
        assert_eq!(proto.nb_resources(), 2);
    }

    #[test]
    fn test_no_duplicates() {
        let mut proto = StepDataFileProtocol::new();
        proto.add("Protocol1");
        proto.add("Protocol1");
        assert_eq!(proto.nb_resources(), 1);
    }

    #[test]
    fn test_schema_name() {
        let proto = StepDataFileProtocol::new();
        assert_eq!(proto.schema_name(), "");
    }
}
